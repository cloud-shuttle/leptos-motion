# Leptos Motion Migration Guide

## From 0.7.x to 0.8.x

This guide helps you migrate your leptos-motion code from version 0.7.x to 0.8.x.

## Breaking Changes

### 1. Transition Property Naming

**Before (0.7.x)**:
```rust
<MotionDiv transition=Transition { duration: 0.6, ease: Easing::EaseInOut }>
```

**After (0.8.x)**:
```rust
<MotionDiv transition=Transition { duration: 0.6, ease: Easing::EaseInOut }>
```

**Note**: The property name is now consistent. If you were using `_transition`, you can now use `transition`.

### 2. CubicBezier Usage

**Before (0.7.x)**:
```rust
// This didn't exist in 0.7.x
ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0))
```

**After (0.8.x)**:
```rust
// Now you can use either:
ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0))
// Or the tuple format:
ease: Easing::Bezier(0.4, 0.0, 0.2, 1.0)
```

### 3. Key Property Support

**Before (0.7.x)**:
```rust
// Key property didn't exist
<MotionSpan>Text</MotionSpan>
```

**After (0.8.x)**:
```rust
// Now you can use key for React-like re-rendering
<MotionSpan key=current_text_index.get()>Text</MotionSpan>
```

## New Features in 0.8.x

### 1. Enhanced Easing Functions

```rust
use leptos_motion::*;

// Standard easing functions
ease: Easing::EaseInOut
ease: Easing::CircIn
ease: Easing::BackOut

// Custom cubic bezier curves
ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0))
ease: Easing::Bezier(0.4, 0.0, 0.2, 1.0)

// Spring physics (with approx feature)
ease: Easing::Spring(SpringConfig::new())
```

### 2. Key-based Re-rendering

```rust
let (current_text, set_current_text) = create_signal(0);

view! {
    <MotionSpan 
        key=current_text.get()
        animate=motion_target!(
            opacity: 0.0,
            y: 20.0,
        )
        transition=Transition {
            duration: 0.3,
            ease: Easing::EaseOut,
        }
    >
        {move || texts[current_text.get()]}
    </MotionSpan>
}
```

### 3. Improved Transition API

```rust
<MotionDiv
    initial=motion_target!(opacity: 0.0, scale: 0.8)
    animate=motion_target!(opacity: 1.0, scale: 1.0)
    transition=Transition {
        duration: 0.6,
        ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0)),
        delay: 0.1,
    }
>
    Content
</MotionDiv>
```

## Common Migration Patterns

### 1. Form Input Handling

**Before (Leptos 0.7.x)**:
```rust
<textarea value=form_data.get().message />
```

**After (Leptos 0.8.x)**:
```rust
<textarea prop:value=form_data.get().message />
```

### 2. Exit Animations

**Before (using non-existent exit property)**:
```rust
<MotionDiv exit=motion_target!(opacity: 0.0, y: 20.0)>
```

**After (using conditional rendering)**:
```rust
<MotionDiv 
    animate=if should_exit.get() {
        Some(motion_target!(opacity: 0.0, y: 20.0))
    } else {
        None
    }
>
```

### 3. Intersection Observer Animations

**Before (using non-existent while_in_view)**:
```rust
<MotionDiv while_in_view=motion_target!(opacity: 1.0, y: 0.0)>
```

**After (using while_hover or custom implementation)**:
```rust
<MotionDiv while_hover=motion_target!(opacity: 1.0, y: 0.0)>
```

### 4. Interval Management

**Before (incorrect usage)**:
```rust
let interval = set_interval(/* ... */);
interval.clear(); // This doesn't work
```

**After (correct usage)**:
```rust
let interval_id = set_interval(/* ... */);
// Later:
clear_interval(interval_id);
```

## Version Compatibility

| leptos-motion | leptos | Status | Notes |
|---------------|--------|--------|-------|
| 0.8.1 | 0.8.5 | ✅ Compatible | Stable |
| 0.8.2 | 0.8.8 | ✅ Compatible | Latest |
| 0.7.x | 0.7.x | ⚠️ Deprecated | Use 0.8.x |

## Troubleshooting

### Common Errors and Solutions

#### 1. "no method named transition found"
**Error**: `no method named transition found for struct MotionDivPropsBuilder`
**Solution**: Use `transition` (not `_transition`) in 0.8.x

#### 2. "no variant or associated item named CubicBezier found"
**Error**: `no variant or associated item named CubicBezier found for enum leptos_motion::Easing`
**Solution**: Use `Easing::CubicBezier(CubicBezier::new(...))` or `Easing::Bezier(...)`

#### 3. "no method named key found"
**Error**: `no method named key found for struct MotionSpanPropsBuilder`
**Solution**: Update to leptos-motion 0.8.2+ where key support was added

#### 4. "no method named exit found"
**Error**: `no method named exit found for struct MotionDivPropsBuilder`
**Solution**: Use conditional rendering with `animate` property instead

#### 5. "no method named while_in_view found"
**Error**: `no method named while_in_view found for struct MotionDivPropsBuilder`
**Solution**: Use `while_hover` or implement custom intersection observer

## Best Practices

### 1. Use Consistent Easing
```rust
// Good: Use descriptive easing names
ease: Easing::EaseInOut

// Good: Use cubic bezier for custom curves
ease: Easing::CubicBezier(CubicBezier::new(0.4, 0.0, 0.2, 1.0))

// Avoid: Inconsistent naming
ease: Easing::Bezier(0.4, 0.0, 0.2, 1.0) // Less clear than CubicBezier
```

### 2. Leverage Key Properties
```rust
// Good: Use keys for list items
{move || items.get().into_iter().map(|item| {
    view! {
        <MotionDiv 
            key=item.id
            animate=motion_target!(opacity: 1.0, y: 0.0)
        >
            {item.content}
        </MotionDiv>
    }
}).collect::<Vec<_>>()}
```

### 3. Handle Exit Animations Properly
```rust
// Good: Use conditional rendering
<MotionDiv 
    animate=if is_visible.get() {
        Some(motion_target!(opacity: 1.0, scale: 1.0))
    } else {
        Some(motion_target!(opacity: 0.0, scale: 0.8))
    }
    transition=Transition {
        duration: 0.3,
        ease: Easing::EaseInOut,
    }
>
```

## Performance Tips

### 1. Use Appropriate Easing Functions
```rust
// Fast animations: Use simple easing
ease: Easing::EaseOut

// Complex animations: Use cubic bezier
ease: Easing::CubicBezier(CubicBezier::new(0.25, 0.1, 0.25, 1.0))
```

### 2. Optimize Key Usage
```rust
// Good: Use stable keys
key=item.id

// Avoid: Using changing values as keys
key=item.content // This changes frequently
```

### 3. Batch Animations
```rust
// Good: Animate multiple properties together
animate=motion_target!(
    opacity: 1.0,
    scale: 1.0,
    y: 0.0,
)

// Avoid: Multiple separate animations
// This is less efficient
```

## Getting Help

If you encounter issues during migration:

1. Check the [API Documentation](https://docs.rs/leptos-motion)
2. Review the [Examples](../examples/)
3. Open an issue on [GitHub](https://github.com/your-org/leptos-motion)
4. Join the [Discord Community](https://discord.gg/leptos)

## Conclusion

The migration to leptos-motion 0.8.x brings improved API consistency, better performance, and new features. While there are some breaking changes, they are designed to make the API more intuitive and powerful.

Take your time with the migration, test thoroughly, and don't hesitate to ask for help if you run into issues!
