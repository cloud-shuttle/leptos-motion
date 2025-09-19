# AGENTS.md - Leptos Motion Development Guide

## Build/Test Commands
- **Build**: `cargo build --all-features` or `pnpm build:release`
- **Test All**: `cargo test --workspace` or `pnpm test`
- **Test Single**: `cargo test --package <crate-name>` or `cargo test <test-name>`
- **Test Integration**: `pnpm test:integration`
- **E2E Tests**: `pnpm test:e2e` (Playwright)
- **Lint**: `cargo clippy --all-targets --all-features -- -D warnings` or `pnpm lint`
- **Format**: `cargo fmt --all` or `pnpm format`
- **Contract Tests**: `make contract-tests` or `cargo test --package leptos-motion-contracts`
- **Dev Server**: `pnpm dev` (runs showcase example)

## Architecture
**Workspace Structure**: 9 core crates in `/crates/` - leptos-motion (main), leptos-motion-core (engine), leptos-motion-dom (web APIs), leptos-motion-gestures, leptos-motion-layout, leptos-motion-scroll, leptos-motion-macros, leptos-motion-studio, leptos-motion-webgl. Examples in `/examples/`, demos in `/demos/`, comprehensive contract tests in `/tests/contracts/`. Uses Leptos 0.8.6 with WASM compilation.

## Code Style
**Rust 2024 Edition**, MSRV 1.89. Use workspace dependencies from root Cargo.toml. Follow standard Rust naming: snake_case for functions/variables, PascalCase for types. Import order: std, external crates, workspace crates, local modules. Use `anyhow` for errors, `thiserror` for custom errors. Prefer `leptos::prelude::*` imports. Components use reactive signals pattern. WASM-compatible code only (no std::thread, use wasm-bindgen-futures).
