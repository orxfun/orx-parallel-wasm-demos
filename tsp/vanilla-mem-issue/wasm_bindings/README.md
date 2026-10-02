# Allocator-disabled bindings

This crate has the same browser API as the [vanilla bindings](../../vanilla/wasm_bindings/README.md). The distinguishing change is in `Cargo.toml`: `orx-parallel` enables `wasm` but not `wasm-allocator`.

The Vite app's `ParallelWorker` integration handles worker/runtime setup. For the shared build flow, see the [vanilla app guide](../../vanilla/app/README.md); compare with the [allocator-enabled variant](../../vanilla-mem-fixed/README.md).
