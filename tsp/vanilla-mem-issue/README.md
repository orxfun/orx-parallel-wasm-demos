# TSP allocation demo: allocator disabled

Try the [allocator-disabled live demo](https://orx-parallel-wasm-demo-tsp-vanilla-mem-issue.pages.dev/).

This is the [vanilla TypeScript demo](../vanilla/README.md) with the same intentionally allocation-heavy workload as [`vanilla-mem-fixed`](../vanilla-mem-fixed/README.md). Its wasm bindings enable `orx-parallel`'s `wasm` feature but do not enable `wasm-allocator`.

The solver deliberately creates many temporary vectors in `computation/src/solver.rs` to put memory pressure on the browser worker. This is a diagnostic demonstration, not a recommended TSP implementation. Compare it with `vanilla-mem-fixed`, which differs by enabling `wasm-allocator` in `wasm_bindings/Cargo.toml`.

For app setup, build, and browser requirements, follow the [vanilla demo instructions](../vanilla/README.md).
