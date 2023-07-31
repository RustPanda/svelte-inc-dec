use axum::{
    extract::State,
    http::{header, Response, StatusCode, Uri},
    response::{
        sse::{Event, Sse},
        Html, IntoResponse,
    },
    routing::{get, post},
    Router,
};
use futures::stream::{self, Stream};
use rust_embed::RustEmbed;
use std::{sync::Arc, time::Duration};
use tokio::sync::{broadcast, Mutex};
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt as _;

static INDEX_HTML: &str = "index.html";

#[derive(RustEmbed)]
#[folder = "webui/dist"]
struct Dist;

#[derive(Clone)]
struct AppState {
    count: Arc<Mutex<i32>>,
    tx: broadcast::Sender<i32>,
}

#[tokio::main]
async fn main() {
    let app_state = {
        let count = Arc::new(Mutex::new(0i32));
        let (tx, _) = broadcast::channel::<i32>(2);
        AppState { count, tx }
    };

    let app = Router::new()
        .route("/counter", get(sse_handler))
        .route("/counter/increment", post(increment_handler))
        .route("/counter/decrement", post(decrement_handler))
        .with_state(app_state)
        .fallback(static_handler);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Server run on: http://localhost:8080");

    axum::serve(listener, app).await.unwrap();
}

async fn increment_handler(State(app_state): State<AppState>) -> Response<axum::body::Body> {
    let mut count_guard = app_state.count.lock().await;
    *count_guard += 1;
    app_state.tx.send(*count_guard).unwrap();
    StatusCode::OK.into_response()
}

async fn decrement_handler(State(app_state): State<AppState>) -> Response<axum::body::Body> {
    let mut count_guard = app_state.count.lock().await;
    *count_guard -= 1;
    app_state.tx.send(*count_guard).unwrap();
    StatusCode::OK.into_response()
}

async fn sse_handler(
    State(app_state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = BroadcastStream::new(app_state.tx.subscribe())
        .map(|i| Event::default().json_data(i.unwrap()));

    let first = stream::once(async move {
        let count = app_state.count.lock().await;
        Event::default().json_data(*count)
    });

    Sse::new(first.chain(stream)).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match Dist::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

async fn index_html() -> Response<axum::body::Body> {
    match Dist::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

async fn not_found() -> Response<axum::body::Body> {
    (StatusCode::NOT_FOUND, "404").into_response()
}
