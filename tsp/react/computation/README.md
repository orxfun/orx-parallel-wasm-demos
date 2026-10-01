# computation

This crate contains the pure Rust TSP implementation used by the wasm TSP example.

It is intentionally free of wasm-bindgen, JavaScript, and UI concerns, which keeps it easy to test and benchmark as an ordinary Rust crate.

## Responsibilities

- generate TSP instances
- build and improve tours
- run parallel search

## How it enables parallelization

This crate uses `orx-parallel` in `run_search`.

The computation crate keeps its dependency configuration independent of the browser runtime. Browser-specific setup belongs in `wasm_bindings`.

## How it fits into the example

The `wasm_bindings/` crate exposes the functions from this crate to JavaScript, and `app/` consumes those bindings from the browser.

The bindings crate depends on this crate and provides the browser-facing setup.
