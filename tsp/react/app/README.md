# Vite + React app

This directory contains the React + TypeScript browser UI. The TSP logic is in `../computation`, and `../wasm_bindings` provides its wasm API.

## Build and run

Run these commands from this directory:

```bash
npm install
npm run build:wasm
npm run dev
```

`npm run build:wasm` uses `orx-parallel-wasm` to generate `pkg/`. `npm run build` rebuilds wasm, type-checks TypeScript, and produces the Vite bundle in `dist/`.

## Runtime

`src/main.tsx` initializes the UI wasm package and creates one persistent `ParallelWorker`. `src/App.tsx` handles UI state and calls the worker wrapper in `src/search-runner.ts`; the components under `src/components/` render controls, status, canvas, and code examples.

The Vite plugin configures threaded wasm for local development. A production host must send `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp` so the browser can use shared memory and threads.
