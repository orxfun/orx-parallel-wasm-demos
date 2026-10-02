# components

This crate contains the Leptos UI for the wasm TSP example.

It is responsible for rendering the page, managing UI state, and invoking the search through the browser worker bridge.

## Responsibilities

- render the interactive UI with Leptos
- keep search settings and view state in Rust
- export `start_app()` for the browser host and send search requests through its JavaScript worker bridge
- hand off worker lifecycle concerns to the JavaScript host application

## How it fits into the example

This crate sits between the browser app and the computation bindings:

- `app/src/main.ts` loads the generated wasm package and calls `start_app()`
- `components/` renders the UI and prepares search requests
- `app/src/search-runner.ts` exposes a JavaScript function on `globalThis` so the Leptos UI can trigger a worker-backed search
- `app/src/search-runner.ts` uses `ParallelWorker` from `orx-parallel-wasm` to run the generated bindings in a worker

That split is deliberate. It keeps UI state and presentation in Rust while leaving browser-specific worker setup and bundler concerns in the Vite app.

## Exported entry point

The main browser entry point exported by this crate is:

- `start_app()`: mounts the Leptos application into the page after the generated wasm package has been initialized

This crate also calls into the wasm bindings to execute searches, but it does that internally as part of the UI flow rather than exposing a second public API layer.

## Parallel execution

The UI does not create the thread pool directly. The TypeScript bridge delegates worker and runtime setup to `ParallelWorker`; the UI sends it search settings and locations.

This separation matters because browser workers own their own wasm instances and runtime initialization.

## Build relationship

From `app/`, the `build:wasm` script builds this crate into `app/pkg`:

```bash
npm run build:wasm
```

The script uses `orx-parallel-wasm` to generate the wasm package consumed by `app/src/main.ts` and the worker bridge.