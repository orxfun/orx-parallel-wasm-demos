# Allocator-enabled bindings

This crate has the same browser API as the [vanilla bindings](../../vanilla/wasm_bindings/README.md). The distinguishing change is in `Cargo.toml`: `orx-parallel` enables `wasm` and `wasm-allocator`.

The Vite app's `ParallelWorker` integration handles worker/runtime setup. For the shared build flow, see the [vanilla app guide](../../vanilla/app/README.md); for why this variant enables the allocator, see the [demo overview](../README.md).
