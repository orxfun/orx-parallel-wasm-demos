# Vite + TypeScript app

This browser app implements the vanilla TypeScript UI for the TSP demo. The Rust computation lives in `../computation`; `../wasm_bindings` exposes the browser API.

## Build and run

Install dependencies, then run these commands from this directory:

```bash
npm install
npm run build:wasm
npm run dev
```

`npm run build:wasm` uses `orx-parallel-wasm` to build the bindings into `pkg/`. `npm run build` rebuilds wasm, type-checks TypeScript, and creates the Vite production bundle in `dist/`.

## Worker and browser requirements

`src/search-runner.ts` owns a persistent `ParallelWorker` from `orx-parallel-wasm`. It sends `run_search` calls to the worker; the integration manages wasm and parallel-runtime setup. The worker is terminated when the page unloads.

The Vite plugin configures the shared-memory wasm worker and the cross-origin isolation headers needed during development. A production host must also send `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` for `SharedArrayBuffer` and browser threads to work.
