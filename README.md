# Example of integrating Svelte into a Rust Axum web service

![Alt Text](/assets/1.gif)

This example consists of two parts:
- Frontend written in Svelte
- Backend written in Rust

The frontend part is built and linked into the web service code, so only the executable file is required to run.

## /src/main.rs
```rust
...
#[derive(RustEmbed)]
#[folder = "webui/dist"]
struct Dist;
...
```

The Svelte frontend part is built during the service compilation.
## /src/main.rs
```rust
...
fn main() {
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .set_path("webui")
        .init_env()
        .install(None)
        .run("build")
        .exec()
        .unwrap();
}
```

The frontend part subscribes to updates of the Count via Server-Sent Events (SSE) protocol. When the "Increment" or "Decrement" buttons are clicked, corresponding commands are sent to the server.

## /webui/src/App.svelte
```jsx
let count = 0;

const evtSource = new EventSource("/counter");

evtSource.onmessage = (event) => {
  count = JSON.parse(event.data);
};

async function handleIncrement() {
  await fetch("/counter/increment", { method: "POST"} );
}

async function handleDecrement() {
  await fetch("/counter/decrement", { method: "POST"} );
}
...
```
```html
...
<p>{count}</p>
<button on:click={handleDecrement}>Decrement</button>
<button on:click={handleIncrement}>Increment</button>
```

## Project build
To run the project, execute the following commands:
```sh
$ cd webui && npm install && cd ../
$ cargo run
```