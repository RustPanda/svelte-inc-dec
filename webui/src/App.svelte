<script>
	let count  = 0;

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
</script>

<style>
  main {
    font-family: sans-serif;
    text-align: center;
  }

  button {
	  background: #ff3e00;
	  color: white;
	  border: none;
	  padding: 8px 12px;
	  border-radius: 2px;
	}
</style>

<main>
  <h1>Hello Rust+Svelte</h1>
  <h2>Demonstrates the integration of svelte into a rust axum web-service</h2>
  
  <p>{count}</p>
  <button on:click={handleDecrement}>Decrement</button>
  <button on:click={handleIncrement}>Increment</button>
</main>