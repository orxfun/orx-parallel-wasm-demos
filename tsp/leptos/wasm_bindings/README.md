# wasm_bindings

This crate provides the WebAssembly bindings for the `computation` crate so the UI can solve TSP instances from JavaScript.

Its only responsibility is to expose a wasm-friendly API. All TSP logic lives in the `computation` crate, which keeps the implementation modular, easier to test, and easier to reuse outside the UI.

## Exposed API

- `locations(seed, num_cities)`: generates a random TSP instance and returns a JS array of locations.
- `run_search(iterations, seed, threads, chunk_size, locations)`: runs the parallel search and returns the best tour summary.

This crate does not define a JavaScript-facing `init_wasm_parallel_runtime` export. The Vite host uses `ParallelWorker` from `orx-parallel-wasm` to prepare the generated wasm package and worker runtime.

The `locations` argument passed to `run_search` must be a JS array of objects shaped like `{ x: number, y: number }`.

The returned object contains:

- `best_tour`
- `best_distance`
- `iterations`
- `elapsed_ms`

## Parallel execution

Parallel search requires a wasm target with thread support enabled, including atomic operations and shared memory. In practice, this means building for `wasm32-unknown-unknown` with the appropriate threading support in the browser or host environment.

The browser build must support atomics and shared memory, and the page must be cross-origin isolated. The Vite integration configures this for local development; deployed hosts must send COOP and COEP headers.

## Testing

Run native crate tests from this directory with:

```bash
cargo test
```
