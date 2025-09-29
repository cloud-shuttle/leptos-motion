# DOM Crate Remediation Plan

## Overview
**File**: `crates/leptos-motion-dom/`  
**Status**: Broken - 7 compilation errors  
**Lines of Code**: Multiple files >600 lines  
**Priority**: P0 (blocking builds)

## Current Issues

### Compilation Errors
- Import resolution failures (8 locations)
- Type system mismatches (6 locations)
- Missing required fields in component builders
- API contract violations

### Code Size Violations
- `event_driven_motion_div.rs`: 682 lines
- `keyframe_animation_tests.rs`: 726 lines
- `performance_regression_tests.rs`: 666 lines
- `drag_constraint_integration_tests.rs`: 658 lines

### Architecture Issues
- Mixed concerns in large files
- Inconsistent error handling
- Missing component abstractions

## Remediation Strategy

### Phase 1: Critical Build Fixes (Week 1)

#### Fix Import Resolution
**Problem**: Missing module exports in `lib.rs`
```rust
// Current (broken)
pub use elements::*; // unused import

// Fixed
pub mod motion_div;
pub mod event_handlers;
pub mod animation_engine;
// ... other modules

pub use motion_div::MotionDiv;
pub use event_handlers::*;
```

#### Fix Type Mismatches
**Problem**: Duration vs f64 conversions
```rust
// Current (broken)
cache.set("key1".to_string(), 42.0, Duration::from_secs(1));

// Fixed
cache.set("key1".to_string(), 42.0, Duration::from_secs(1).as_secs_f64());
```

#### Fix Component Builder Errors
**Problem**: Missing required fields
```rust
// Current (broken)
MotionDiv::builder()
    .animate(AnimateProp::Static(values))
    .build() // ERROR: missing children/node_ref

// Fixed
MotionDiv::builder()
    .animate(AnimateProp::Static(values))
    .children(children)
    .node_ref(node_ref)
    .build()
```

### Phase 2: Code Size Reduction (Week 2)

#### Split `event_driven_motion_div.rs` (682 lines)
**Target Structure**:
```
src/motion_div/
├── lib.rs (main component)
├── props.rs (component props)
├── state.rs (internal state)
├── event_handlers.rs (user interactions)
└── render.rs (view logic)
```

#### Implementation:
```rust
// motion_div/lib.rs
mod props;
mod state;
mod event_handlers;
mod render;

pub use props::*;
pub use state::*;
pub use event_handlers::*;
pub use render::*;

#[component]
pub fn MotionDiv(props: MotionDivProps) -> impl IntoView {
    // Implementation using sub-modules
}
```

### Phase 3: Test Consolidation (Week 3)

#### Current Test Files → Target Structure
**Before**: 8+ large test files
**After**: 3 focused test modules

```
src/
├── unit_tests/
│   ├── component_tests.rs (<300 lines)
│   ├── event_tests.rs (<300 lines)
│   └── animation_tests.rs (<300 lines)
├── integration_tests/
│   └── dom_integration_tests.rs (<300 lines)
└── performance_tests/
    └── dom_benchmarks.rs (<300 lines)
```

### Phase 4: API Standardization (Week 4)

#### Standardize AnimateProp Usage
```rust
#[derive(Clone)]
pub enum AnimateProp {
    Static(HashMap<String, AnimationValue>),
    Reactive(Signal<HashMap<String, AnimationValue>>),
    Derived(Memo<HashMap<String, AnimationValue>>),
    Fn(Rc<dyn Fn() -> HashMap<String, AnimationValue>>),
}
```

#### Fix Event Handler Types
```rust
pub struct EventHandlers {
    pub drag: Option<DragEventHandler>,
    pub hover: Option<HoverEventHandler>,
    pub tap: Option<TapEventHandler>,
    pub gesture: Option<GestureEventHandler>,
}
```

## Success Criteria

### Build Health
- [ ] 0 compilation errors
- [ ] All imports resolve correctly
- [ ] Type system consistent
- [ ] Component builders functional

### Code Quality
- [ ] All files <300 lines
- [ ] Clear module separation
- [ ] Consistent error handling
- [ ] Proper documentation

### Functionality
- [ ] MotionDiv component works
- [ ] Event handling functional
- [ ] Animation transitions smooth
- [ ] Memory leaks prevented

### Performance
- [ ] 60fps animation target
- [ ] Minimal DOM updates
- [ ] Efficient event handling
- [ ] Bundle size optimized

## Risk Mitigation

### Breaking Changes
- **Risk**: API changes break existing code
- **Mitigation**: Comprehensive contract testing
- **Validation**: All examples compile and run

### Performance Regression
- **Risk**: Module splitting introduces overhead
- **Mitigation**: Benchmark critical paths
- **Validation**: Performance within 5% of baseline

### Event Handling Complexity
- **Risk**: Complex event logic becomes unmaintainable
- **Mitigation**: Clear abstraction layers
- **Validation**: Event tests pass comprehensively

## Implementation Timeline

| Week | Task | Deliverables |
|------|------|-------------|
| 1 | Fix compilation errors | Clean build |
| 2 | Split large files | Modules <300 lines |
| 3 | Consolidate tests | 4 test files |
| 4 | API standardization | Consistent interfaces |

## Dependencies
- **Blocks**: WebGL crate, examples
- **Blocked by**: Core crate compilation
- **Enables**: Full system integration

## Resources Required
- **Engineers**: 2 senior Rust developers
- **Time**: 4 weeks
- **Tools**: cargo-check, wasm-pack, browser tests

## Validation Commands
```bash
# Build validation
cargo check --package leptos-motion-dom

# Component tests
cargo test --package leptos-motion-dom --lib

# WASM build
wasm-pack build --target web

# Integration tests
cargo test --package leptos-motion-contracts -- test_dom_contracts
```
