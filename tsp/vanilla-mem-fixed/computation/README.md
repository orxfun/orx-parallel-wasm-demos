# Allocation-heavy computation

This crate contains the same intentionally inefficient `random_tour` implementation as `vanilla-mem-issue`. Each candidate creates many temporary vectors to put memory pressure on the wasm worker; this is a demonstration workload, not a recommended TSP algorithm.

The allocator setting is not configured here. The distinction is in `../wasm_bindings/Cargo.toml`: this variant enables `orx-parallel` with both `wasm` and `wasm-allocator` features. See the [demo overview](../README.md) and the [vanilla computation guide](../../vanilla/computation/README.md) for shared context.
