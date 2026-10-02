# Vite host for the Yew UI

This directory builds and hosts the Rust UI in `../components`. The TSP computation and wasm API are in `../computation` and `../wasm_bindings`.

## Build and run

Run from this directory:

```bash
npm install
npm run build:wasm
npm run dev
```

`npm run build:wasm` uses `orx-parallel-wasm` to compile `../components` into `pkg/`. `npm run build` rebuilds wasm, type-checks the host, and creates the Vite bundle in `dist/`.

## Bootstrap and worker

`src/main.ts` initializes `pkg/components.js`, creates a persistent `ParallelWorker` through `src/search-runner.ts`, then calls `start_app()`. It also provides the code-highlighting callback used by the Yew UI. The worker integration manages wasm and parallel-runtime setup.

The Vite plugin configures threaded wasm for development. A production host must send `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` for shared memory and browser threads.