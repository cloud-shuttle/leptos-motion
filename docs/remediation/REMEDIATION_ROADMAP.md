# Leptos Motion Library - Remediation Roadmap

## Executive Summary

This roadmap provides a structured approach to fixing the `leptos-motion`
library and bringing it to production readiness. The library currently has one
critical issue preventing animations from working, along with several areas for
improvement.

## Current Status Assessment

### ✅ Completed Fixes

- Type system conflicts resolved
- Prop consistency issues fixed
- Component architecture improved
- Build system working
- Testing infrastructure established

### ❌ Critical Issue

- **Animation Reactivity**: MotionDiv component doesn't react to state changes

### 🔄 Areas for Improvement

- Missing animation features
- Performance optimization needed
- Documentation gaps
- Error handling improvements

## Phase 1: Critical Fix (Priority 1)

### 1.1 Fix Animation Reactivity Issue

**Problem**: The `Effect::new` in MotionDiv doesn't properly track dependencies
on signals used by animation closures.

**Solution**: Implement enhanced effect system with proper dependency tracking.

#### Step 1.1.1: Update MotionDiv Component

**File**: `crates/leptos-motion-dom/src/components.rs`

**Current Code** (lines 60-90):

```rust
// Update styles when dependencies change - make it reactive
Effect::new(move |_| {
    let mut styles = HashMap::new();

    // Apply initial styles
    if let Some(initial_target) = &initial {
        for (key, value) in initial_target {
            styles.insert(key.clone(), value.to_string_value());
        }
    }

    // Apply animate styles (reactive) - call the closure to get current values
    if let Some(animate_closure) = &animate {
        let animate_target = animate_closure();
        for (key, value) in &animate_target {
            styles.insert(key.clone(), value.to_string_value());
        }
    }

    // ... rest of effect
});
```

**Replace With**:

```rust
// Create a reactive signal for the current animation state
let (current_animation, set_current_animation) = signal(HashMap::new());

// Effect that tracks animation changes
Effect::new(move |_| {
    if let Some(animate_closure) = &animate {
        // Call the closure - this should establish reactive dependencies
        let animate_target = animate_closure();
        set_current_animation.set(animate_target);
    }
});

// Effect that applies styles
Effect::new(move |_| {
    let mut styles = HashMap::new();

    // Apply initial styles
    if let Some(initial_target) = &initial {
        for (key, value) in initial_target {
            styles.insert(key.clone(), value.to_string_value());
        }
    }

    // Apply current animation styles
    let animation_target = current_animation.get();
    for (key, value) in &animation_target {
        styles.insert(key.clone(), value.to_string_value());
    }

    // Apply hover styles
    if is_hovered.get() {
        if let Some(hover_target) = &while_hover {
            for (key, value) in hover_target {
                styles.insert(key.clone(), value.to_string_value());
            }
        }
    }

    // Apply tap styles
    if is_tapped.get() {
        if let Some(tap_target) = &while_tap {
            for (key, value) in tap_target {
                styles.insert(key.clone(), value.to_string_value());
            }
        }
    }

    set_styles.set(styles);
});
```

#### Step 1.1.2: Test the Fix

**Commands to Run**:

```bash
# Rebuild the library
wasm-pack build --target web --out-dir pkg

# Test the demo
curl -s http://localhost:8085/ | grep -A 5 -B 5 "animated-box"
```

**Expected Result**: The HTML should show inline styles being applied instead of
CSS classes.

#### Step 1.1.3: Add Debugging

**Add to MotionDiv component**:

```rust
// Add console logging for debugging
use web_sys::console;

// In the animation tracking effect
Effect::new(move |_| {
    if let Some(animate_closure) = &animate {
        let animate_target = animate_closure();
        console::log_1(&format!("Animation target updated: {:?}", animate_target).into());
        set_current_animation.set(animate_target);
    }
});
```

### 1.2 Alternative Solution (If Step 1.1 Doesn't Work)

**Use `create_effect` instead of `Effect::new`**:

```rust
// Replace Effect::new with create_effect
create_effect(move |_| {
    if let Some(animate_closure) = &animate {
        let animate_target = animate_closure();
        // Apply styles directly
        let mut styles = HashMap::new();
        // ... style application logic
        set_styles.set(styles);
    }
});
```

### 1.3 Validation

**Test Cases**:

1. Button click changes animation state
2. Animation styles are visually applied
3. Multiple animation modes work
4. Hover animations work
5. No console errors

**Success Criteria**:

- ✅ Animations are visually applied
- ✅ State changes trigger animation updates
- ✅ No performance issues
- ✅ All existing tests pass

## Phase 2: Core Features Implementation (Priority 2)

### 2.1 Drag Animations

**Timeline**: 1-2 weeks

**Implementation**:

```rust
// Add to MotionDiv component
#[prop(optional)]
drag: Option<DragConfig>,

// Implement drag handling
if let Some(drag_config) = &drag {
    // Add drag event listeners
    // Implement drag constraints
    // Apply drag styles
}
```

**Files to Modify**:

- `crates/leptos-motion-dom/src/components.rs`
- `crates/leptos-motion-gestures/src/lib.rs`

### 2.2 Layout Animations (FLIP)

**Timeline**: 2-3 weeks

**Implementation**:

```rust
// Add to MotionDiv component
#[prop(optional)]
layout: Option<bool>,

// Implement FLIP animations
if layout.unwrap_or(false) {
    // Track element position changes
    // Implement FLIP algorithm
    // Apply smooth transitions
}
```

**Files to Modify**:

- `crates/leptos-motion-dom/src/components.rs`
- `crates/leptos-motion-layout/src/lib.rs`

### 2.3 Keyframe Animations

**Timeline**: 1-2 weeks

**Implementation**:

```rust
// Add keyframe support
#[prop(optional)]
keyframes: Option<Vec<AnimationTarget>>,

// Implement keyframe sequencing
if let Some(keyframes) = &keyframes {
    // Sequence through keyframes
    // Apply timing and easing
}
```

### 2.4 Stagger Animations

**Timeline**: 1 week

**Implementation**:

```rust
// Add stagger support
#[prop(optional)]
stagger: Option<f64>,

// Implement staggered timing
if let Some(stagger_delay) = &stagger {
    // Apply delays to child elements
}
```

## Phase 3: Performance & Optimization (Priority 3)

### 3.1 Performance Monitoring

**Timeline**: 1 week

**Implementation**:

```rust
// Add performance tracking
use web_sys::Performance;

// Track animation performance
let start_time = performance.now();
// ... animation logic
let end_time = performance.now();
console::log_1(&format!("Animation took: {}ms", end_time - start_time).into());
```

### 3.2 Animation Cancellation

**Timeline**: 1 week

**Implementation**:

```rust
// Add cancellation support
#[prop(optional)]
on_cancel: Option<Callback<()>>,

// Implement cancellation logic
if let Some(cancel_callback) = &on_cancel {
    // Cancel ongoing animations
    // Clean up resources
}
```

### 3.3 Animation Queuing

**Timeline**: 2 weeks

**Implementation**:

```rust
// Add queuing support
#[prop(optional)]
queue: Option<Vec<AnimationTarget>>,

// Implement queuing logic
if let Some(animation_queue) = &queue {
    // Process animations in sequence
    // Handle queue management
}
```

## Phase 4: Documentation & Testing (Priority 4)

### 4.1 API Documentation

**Timeline**: 1-2 weeks

**Tasks**:

- Document all MotionDiv props
- Add usage examples
- Create migration guide
- Add troubleshooting section

**Files to Create**:

- `docs/API.md`
- `docs/EXAMPLES.md`
- `docs/MIGRATION.md`
- `docs/TROUBLESHOOTING.md`

### 4.2 Comprehensive Testing

**Timeline**: 2-3 weeks

**Test Categories**:

1. **Unit Tests**
   - Component prop validation
   - Animation logic testing
   - Error handling tests

2. **Integration Tests**
   - Demo application tests
   - Cross-browser compatibility
   - Performance benchmarks

3. **End-to-End Tests**
   - Playwright test expansion
   - User interaction testing
   - Animation visual verification

**Files to Create**:

- `tests/unit/`
- `tests/integration/`
- `tests/e2e/`

### 4.3 Examples & Demos

**Timeline**: 1-2 weeks

**Examples to Create**:

- Basic animations
- Complex animations
- Performance demos
- Error handling demos

**Files to Create**:

- `examples/basic-animations/`
- `examples/complex-animations/`
- `examples/performance-demo/`
- `examples/error-handling/`

## Phase 5: Production Readiness (Priority 5)

### 5.1 Error Handling

**Timeline**: 1 week

**Implementation**:

```rust
// Add comprehensive error handling
#[derive(Debug, Clone)]
pub enum AnimationError {
    InvalidTarget(String),
    InvalidTransition(String),
    PerformanceError(String),
}

// Implement error handling in components
pub fn MotionDiv(/* props */) -> Result<impl IntoView, AnimationError> {
    // Validate props
    // Handle errors gracefully
    // Provide helpful error messages
}
```

### 5.2 Type Safety Improvements

**Timeline**: 1 week

**Tasks**:

- Add compile-time validation
- Improve type constraints
- Add generic support where appropriate

### 5.3 Ecosystem Integration

**Timeline**: 2-3 weeks

**Tasks**:

- Create Leptos integration examples
- Add framework-specific optimizations
- Document best practices

## Implementation Timeline

### Week 1-2: Critical Fix

- [ ] Fix animation reactivity issue
- [ ] Test and validate fix
- [ ] Add debugging and monitoring

### Week 3-4: Core Features

- [ ] Implement drag animations
- [ ] Add layout animations (FLIP)
- [ ] Implement keyframe animations

### Week 5-6: Advanced Features

- [ ] Add stagger animations
- [ ] Implement performance monitoring
- [ ] Add animation cancellation

### Week 7-8: Documentation

- [ ] Create comprehensive API docs
- [ ] Add usage examples
- [ ] Create migration guide

### Week 9-10: Testing & Validation

- [ ] Expand test coverage
- [ ] Performance optimization
- [ ] Cross-browser testing

### Week 11-12: Production Readiness

- [ ] Error handling improvements
- [ ] Type safety enhancements
- [ ] Final validation and release

## Risk Assessment

### High Risk

- **Animation Reactivity Fix**: If the proposed solution doesn't work, may
  require significant architectural changes
- **Performance**: Complex animations may cause performance issues

### Medium Risk

- **Browser Compatibility**: Some animation features may not work across all
  browsers
- **API Changes**: Adding new features may require breaking changes

### Low Risk

- **Documentation**: Straightforward but time-consuming
- **Testing**: Well-defined scope and requirements

## Success Metrics

### Technical Metrics

- ✅ All animations work correctly
- ✅ Performance benchmarks met
- ✅ Test coverage > 90%
- ✅ Zero critical bugs

### User Experience Metrics

- ✅ Smooth animations
- ✅ Intuitive API
- ✅ Good documentation
- ✅ Easy migration path

### Business Metrics

- ✅ Library adoption
- ✅ Community feedback
- ✅ Production usage
- ✅ Maintenance burden

## Dependencies

### External Dependencies

- Leptos framework updates
- Web standards evolution
- Browser compatibility

### Internal Dependencies

- Core animation system
- Gesture detection system
- Layout tracking system

## Rollback Plan

### If Critical Fix Fails

1. Revert to previous working state
2. Implement alternative approach
3. Consider architectural redesign

### If Performance Issues

1. Implement performance monitoring
2. Add optimization options
3. Provide fallback mechanisms

### If API Changes Required

1. Maintain backward compatibility
2. Provide migration tools
3. Document breaking changes

## Conclusion

This roadmap provides a structured approach to fixing the leptos-motion library
and bringing it to production readiness. The critical fix for animation
reactivity should be the immediate priority, followed by implementing core
features and improving documentation.

The timeline is aggressive but achievable with focused effort. The key is to
prioritize the critical fix first, then systematically work through the
remaining phases.

## Next Steps

1. **Immediate**: Implement the animation reactivity fix
2. **Short-term**: Test and validate the fix
3. **Medium-term**: Implement core animation features
4. **Long-term**: Complete documentation and testing

This roadmap provides a clear path forward for making the leptos-motion library
production-ready.
