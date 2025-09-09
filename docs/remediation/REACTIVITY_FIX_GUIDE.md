# Leptos Motion - Animation Reactivity Fix Guide

## Problem Statement

The MotionDiv component's `Effect::new` is not being triggered when the state
changes. The effect needs to be watching the signals that the animation closure
depends on, but the current implementation isn't properly tracking those
dependencies.

## Root Cause Analysis

### Current Implementation Issues

1. **Effect Not Reactive**: The `Effect::new` only runs once when the component
   is created
2. **Missing Dependency Tracking**: The effect doesn't establish reactive
   dependencies on the signals used by the animation closure
3. **Closure Isolation**: The animation closure is isolated from the effect's
   reactivity system

### Technical Details

```rust
// ❌ CURRENT: Effect doesn't track closure dependencies
Effect::new(move |_| {
    // This only runs once, not when is_visible or animation_mode change
    if let Some(animate_closure) = &animate {
        let animate_target = animate_closure(); // Calls closure but doesn't track its dependencies
        // ... apply styles
    }
});
```

The issue is that `animate_closure()` calls the closure but doesn't establish
reactive dependencies on the signals it uses internally.

## Solution Approaches

### Approach 1: Direct Signal Access in Effect

**Concept**: Make the effect directly access the signals that the animation
closure depends on.

```rust
// ✅ PROPOSED: Direct signal access
Effect::new(move |_| {
    // Access signals directly to establish reactive dependencies
    let visible = is_visible.get();
    let mode = animation_mode.get();

    let mut styles = HashMap::new();

    // Apply initial styles
    if let Some(initial_target) = &initial {
        for (key, value) in initial_target {
            styles.insert(key.clone(), value.to_string_value());
        }
    }

    // Apply animate styles using direct signal access
    if let Some(animate_closure) = &animate {
        let animate_target = animate_closure();
        for (key, value) in &animate_target {
            styles.insert(key.clone(), value.to_string_value());
        }
    }

    // ... rest of the effect

    set_styles.set(styles);
});
```

**Problem**: This approach requires the MotionDiv to know about the specific
signals used by the animation closure, which breaks encapsulation.

### Approach 2: Signal Injection Pattern

**Concept**: Pass the signals as parameters to the animation closure.

```rust
// ✅ PROPOSED: Signal injection
pub fn MotionDiv(
    // ... other props
    #[prop(optional)] animate: Option<Rc<dyn Fn() -> AnimationTarget>>,
    // Add signal parameters
    #[prop(optional)] animate_signals: Option<Vec<Rc<dyn Fn() -> ()>>>,
    // ... rest of props
) -> impl IntoView {
    // ... component setup

    Effect::new(move |_| {
        // Call signal functions to establish reactive dependencies
        if let Some(signals) = &animate_signals {
            for signal_fn in signals {
                signal_fn();
            }
        }

        // ... rest of effect
    });
}
```

**Problem**: This approach is complex and requires changes to the API.

### Approach 3: Reactive Closure Wrapper

**Concept**: Create a wrapper that makes the closure reactive by tracking its
dependencies.

```rust
// ✅ PROPOSED: Reactive closure wrapper
pub struct ReactiveAnimation {
    closure: Rc<dyn Fn() -> AnimationTarget>,
    dependencies: Vec<Rc<dyn Fn() -> ()>>,
}

impl ReactiveAnimation {
    pub fn new<F>(closure: F, dependencies: Vec<Rc<dyn Fn() -> ()>>) -> Self
    where
        F: Fn() -> AnimationTarget + 'static
    {
        Self {
            closure: Rc::new(closure),
            dependencies,
        }
    }

    pub fn get_target(&self) -> AnimationTarget {
        // Call dependencies to establish reactive tracking
        for dep in &self.dependencies {
            dep();
        }
        (self.closure)()
    }
}
```

**Problem**: This approach is complex and requires significant API changes.

### Approach 4: Leptos Signal Integration

**Concept**: Use Leptos's built-in signal tracking mechanisms.

```rust
// ✅ PROPOSED: Leptos signal integration
use leptos::prelude::*;

pub fn MotionDiv(
    // ... other props
    #[prop(optional)] animate: Option<Rc<dyn Fn() -> AnimationTarget>>,
    // ... rest of props
) -> impl IntoView {
    // ... component setup

    // Create a reactive signal for the animation target
    let (animation_target, set_animation_target) = signal(HashMap::new());

    // Create effect that updates the animation target
    Effect::new(move |_| {
        if let Some(animate_closure) = &animate {
            let target = animate_closure();
            set_animation_target.set(target);
        }
    });

    // Create effect that applies styles based on the animation target
    Effect::new(move |_| {
        let target = animation_target.get();
        let mut styles = HashMap::new();

        // Apply initial styles
        if let Some(initial_target) = &initial {
            for (key, value) in initial_target {
                styles.insert(key.clone(), value.to_string_value());
            }
        }

        // Apply animate styles
        for (key, value) in &target {
            styles.insert(key.clone(), value.to_string_value());
        }

        // ... rest of style application

        set_styles.set(styles);
    });
}
```

**Problem**: This approach creates two effects and may not properly track
dependencies.

## Recommended Solution

### Approach 5: Enhanced Effect with Dependency Tracking

**Concept**: Enhance the current effect to properly track dependencies by
calling the closure and using Leptos's built-in dependency tracking.

```rust
// ✅ RECOMMENDED: Enhanced effect with dependency tracking
use leptos::prelude::*;

pub fn MotionDiv(
    // ... other props
    #[prop(optional)] animate: Option<Rc<dyn Fn() -> AnimationTarget>>,
    // ... rest of props
) -> impl IntoView {
    // ... component setup

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
}
```

## Implementation Steps

### Step 1: Update MotionDiv Component

1. Add a reactive signal for the current animation state
2. Create separate effects for animation tracking and style application
3. Ensure proper dependency tracking

### Step 2: Test the Fix

1. Update the demo to use the new implementation
2. Test that animations work when state changes
3. Verify that the effect is triggered on state changes

### Step 3: Add Debugging

1. Add console logging to track when effects are triggered
2. Add logging to track when animation targets change
3. Verify that the reactivity system is working correctly

## Alternative: Use Leptos's create_effect

If the above approach doesn't work, consider using Leptos's `create_effect`
instead of `Effect::new`:

```rust
// ✅ ALTERNATIVE: Use create_effect
use leptos::prelude::*;

pub fn MotionDiv(
    // ... props
) -> impl IntoView {
    // ... setup

    // Use create_effect for better dependency tracking
    create_effect(move |_| {
        if let Some(animate_closure) = &animate {
            let animate_target = animate_closure();
            // Apply styles directly
            let mut styles = HashMap::new();
            // ... style application logic
            set_styles.set(styles);
        }
    });
}
```

## Testing Strategy

### Unit Tests

1. Test that the effect is triggered when signals change
2. Test that animation styles are applied correctly
3. Test that the component re-renders when animations change

### Integration Tests

1. Test with the demo application
2. Test with different animation configurations
3. Test with multiple MotionDiv components

### Performance Tests

1. Test that animations don't cause excessive re-renders
2. Test that the effect system is efficient
3. Test with complex animation sequences

## Conclusion

The recommended solution is to enhance the current effect system with proper
dependency tracking. This approach maintains the current API while fixing the
reactivity issue. If this doesn't work, the alternative approach using
`create_effect` should be considered.

The key insight is that the effect needs to establish reactive dependencies on
the signals used by the animation closure. This can be achieved by calling the
closure within the effect and using Leptos's built-in dependency tracking
mechanisms.
