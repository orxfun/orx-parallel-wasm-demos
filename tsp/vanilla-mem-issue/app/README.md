# Allocator-disabled app variant

This app is the [vanilla Vite + TypeScript app](../../vanilla/app/README.md) with the allocation-heavy computation from this demo. Follow that guide for installation, build, development, and browser-isolation requirements.

The app scripts and UI are otherwise the same. Unlike `vanilla-mem-fixed`, `../wasm_bindings/Cargo.toml` enables `wasm` without enabling `wasm-allocator`. Run `npm run build:wasm` after changing Rust code.
