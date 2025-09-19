# Dependency Fixes

## Problem
Two versions of `leptos_motion_core` (v0.9.1 vs v1.1.0) causing type conflicts.

## Solution
Update all `Cargo.toml` files to use local paths:

```toml
# In each crate's Cargo.toml
leptos-motion-core = { path = "../leptos-motion-core" }
```

## Files to Update
- `crates/leptos-motion/Cargo.toml`
- `crates/leptos-motion-scroll/Cargo.toml` 
- `crates/leptos-motion-webgl/Cargo.toml`

## Validation
```bash
cargo tree --package leptos-motion-dom
# Should show single version usage
```

## Status
✅ **COMPLETED** - All files updated
