# Type System Conflicts

## Problem
Cross-crate type mismatches:
```bash
error[E0308]: expected `leptos_motion_dom::Transition`, found `leptos_motion_core::Transition`
```

## Solution
Use re-exports for type consistency:

```rust
// In each crate's lib.rs
pub use leptos_motion_core::Transition;
pub use leptos_motion_core::AnimationValue;
pub use leptos_motion_core::Easing;
```

## Root Cause
Different versions of `leptos_motion_core` create different types even if they look the same.

## Files to Update
- All crate `lib.rs` files
- Test files using `Transition` type
- Example files with type conflicts

## Status
⏳ **PENDING** - Need to implement
