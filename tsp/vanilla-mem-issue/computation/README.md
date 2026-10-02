# Allocation-heavy computation

This crate contains the same intentionally inefficient `random_tour` implementation as `vanilla-mem-fixed`. Each candidate creates many temporary vectors to put memory pressure on the wasm worker; this is a demonstration workload, not a recommended TSP algorithm.

The allocator setting is not configured here. This variant's `../wasm_bindings/Cargo.toml` enables `orx-parallel` with `wasm` but not `wasm-allocator`. See the [demo overview](../README.md) and the [vanilla computation guide](../../vanilla/computation/README.md) for shared context.
