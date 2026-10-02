# Allocator-enabled app variant

This app is the [vanilla Vite + TypeScript app](../../vanilla/app/README.md) with the allocation-heavy computation from this demo. Follow that guide for installation, build, development, and browser-isolation requirements.

The app scripts and UI are otherwise the same. The distinguishing setting is in `../wasm_bindings/Cargo.toml`, where `orx-parallel` enables both `wasm` and `wasm-allocator` features. Run `npm run build:wasm` after changing Rust code.
