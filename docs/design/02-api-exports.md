# API Export Fixes

## Problem
Missing component exports causing import errors:
```bash
error[E0432]: unresolved import `leptos_motion_dom::ReactiveMotionDiv`
error[E0432]: unresolved import `leptos_motion_dom::MotionDiv`
```

## Solution
Export all components from main lib:

```rust
// crates/leptos-motion-dom/src/lib.rs
pub use event_driven_motion_div::EventDrivenMotionDiv as MotionDiv;
pub use event_driven_motion_div::EventDrivenMotionDiv as ReactiveMotionDiv;
pub use event_driven_motion_div::EventDrivenMotionDiv as DragMotionDiv;

// Legacy compatibility
pub type SimpleMotionDiv = MotionDiv;
pub type CleanMotionDiv = MotionDiv;
```

## Files to Update
- `crates/leptos-motion-dom/src/lib.rs`
- All example files with import errors
- All test files with import errors

## Status
⏳ **PENDING** - Need to implement
