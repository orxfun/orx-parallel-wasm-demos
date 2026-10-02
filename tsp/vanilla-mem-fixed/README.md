# TSP allocation demo: allocator enabled

Try the [allocator-enabled live demo](https://orx-parallel-wasm-demo-tsp-vanilla-mem-fixed.pages.dev/).

This is the [vanilla TypeScript demo](../vanilla/README.md) with an intentionally allocation-heavy TSP workload. Its wasm bindings enable the `wasm-allocator` feature of `orx-parallel`, demonstrating the allocator-enabled configuration.

The comparison is with [`vanilla-mem-issue`](../vanilla-mem-issue/README.md), which runs the same workload without that feature. The deliberately inefficient tour construction in `computation/src/solver.rs` creates many temporary vectors; it is a memory-pressure demonstration, not a recommended TSP implementation.

For setup, build, and browser requirements, follow the [vanilla demo instructions](../vanilla/README.md). The only relevant configuration difference is in `wasm_bindings/Cargo.toml`: this demo enables `orx-parallel` with `features = ["wasm", "wasm-allocator"]`.
