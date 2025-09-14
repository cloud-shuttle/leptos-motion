# Leptos Motion Usage Guide

## Getting Started

### Installation

Add leptos-motion to your `Cargo.toml`:

```toml
[dependencies]
leptos = { version = "0.8.8", features = ["csr"] }
leptos-motion-dom = { version = "0.9.0" }
wasm-bindgen = "0.2"
```

### Basic Setup

```rust
use leptos::prelude::*;
use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
use leptos_motion_dom::*;
use std::collections::HashMap;

#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"Leptos Motion Demo"</h1>
            <AnimatedBox/>
        </div>
    }
}

#[component]
fn AnimatedBox() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    // Update animation signal when scale changes
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!("scale({})", scale.get())));
        set_animate_signal.set(animations);
    });

    let initial_values = {
        let mut initial = HashMap::new();
        initial.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        initial
    };

    let transition = Transition {
        duration: Some(0.3),
        delay: Some(0.0),
        ease: Easing::EaseInOut,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <ReactiveMotionDivV2
            initial=initial_values
            animate=animate_signal
            transition=transition
        >
            <div
                style="width: 100px; height: 100px; background: #4ecdc4; border-radius: 8px; cursor: pointer;"
                on:click=move |_| {
                    set_scale.set(if scale.get() == 1.0 { 1.5 } else { 1.0 });
                }
            >
                "Click me!"
            </div>
        </ReactiveMotionDivV2>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
```

## Animation Types

### 1. Scale Animations

```rust
#[component]
fn ScaleAnimation() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!("scale({})", scale.get())));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Scale Animation"</div>
        </ReactiveMotionDivV2>
    }
}
```

### 2. Rotation Animations

```rust
#[component]
fn RotationAnimation() -> impl IntoView {
    let (rotation, set_rotation) = create_signal(0.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!("rotate({}deg)", rotation.get())));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Rotation Animation"</div>
        </ReactiveMotionDivV2>
    }
}
```

### 3. Position Animations

```rust
#[component]
fn PositionAnimation() -> impl IntoView {
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!("translate({}px, {}px)", x.get(), y.get())));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Position Animation"</div>
        </ReactiveMotionDivV2>
    }
}
```

### 4. Opacity Animations

```rust
#[component]
fn OpacityAnimation() -> impl IntoView {
    let (opacity, set_opacity) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("opacity".to_string(), AnimationValue::Number(opacity.get()));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Opacity Animation"</div>
        </ReactiveMotionDivV2>
    }
}
```

### 5. Complex Transform Animations

```rust
#[component]
fn ComplexTransform() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (rotation, set_rotation) = create_signal(0.0);
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!(
            "translate({}px, {}px) scale({}) rotate({}deg)",
            x.get(), y.get(), scale.get(), rotation.get()
        )));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Complex Transform"</div>
        </ReactiveMotionDivV2>
    }
}
```

## Interactive Animations

### Hover Effects

```rust
#[component]
fn HoverEffect() -> impl IntoView {
    let (is_hovered, set_hovered) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_hovered.get() {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1.1) rotate(5deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(0.8));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1) rotate(0deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
        >
            <div>"Hover me!"</div>
        </ReactiveMotionDivV2>
    }
}
```

### Click Animations

```rust
#[component]
fn ClickAnimation() -> impl IntoView {
    let (is_clicked, set_clicked) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_clicked.get() {
            animations.insert("transform".to_string(), AnimationValue::String("scale(0.95)".to_string()));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1)".to_string()));
        }
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
            on:click=move |_| {
                set_clicked.set(true);
                // Reset after animation
                set_timeout(move || set_clicked.set(false), std::time::Duration::from_millis(150));
            }
        >
            <div>"Click me!"</div>
        </ReactiveMotionDivV2>
    }
}
```

## Advanced Features

### Staggered Animations

```rust
#[component]
fn StaggeredList() -> impl IntoView {
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];
    let (is_visible, set_visible) = create_signal(false);
    
    view! {
        <div>
            <button on:click=move |_| set_visible.set(!is_visible.get())>
                "Toggle List"
            </button>
            <div>
                {items.into_iter().enumerate().map(|(index, item)| {
                    let delay = index as f64 * 0.1; // 100ms stagger
                    let transition = Transition {
                        duration: Some(0.5),
                        delay: Some(delay),
                        ease: Easing::EaseOut,
                        repeat: RepeatConfig::Never,
                        stagger: None,
                    };
                    
                    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());
                    
                    Effect::new(move |_| {
                        let mut animations = HashMap::new();
                        if is_visible.get() {
                            animations.insert("transform".to_string(), AnimationValue::String("translateX(0px)".to_string()));
                            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
                        } else {
                            animations.insert("transform".to_string(), AnimationValue::String("translateX(-50px)".to_string()));
                            animations.insert("opacity".to_string(), AnimationValue::Number(0.0));
                        }
                        set_animate_signal.set(animations);
                    });
                    
                    view! {
                        <ReactiveMotionDivV2
                            initial=HashMap::new()
                            animate=animate_signal
                            transition=transition
                        >
                            <div>{item}</div>
                        </ReactiveMotionDivV2>
                    }
                }).collect::<Vec<_>>()}
            </div>
        </div>
    }
}
```

### Drag and Drop

```rust
#[component]
fn DraggableItem() -> impl IntoView {
    let drag_config = DragConfig {
        enabled: true,
        constraints: Some(DragConstraints {
            min_x: Some(-200.0),
            max_x: Some(200.0),
            min_y: Some(-200.0),
            max_y: Some(200.0),
        }),
        momentum: Some(DragMomentum {
            enabled: true,
            damping: 0.8,
            stiffness: 0.1,
        }),
    };

    view! {
        <DragMotionDiv drag=drag_config>
            <div style="width: 100px; height: 100px; background: #ff6b6b; border-radius: 8px; cursor: grab;">
                "Drag me!"
            </div>
        </DragMotionDiv>
    }
}
```

### Conditional Animations

```rust
#[component]
fn ConditionalAnimation() -> impl IntoView {
    let (condition, set_condition) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if condition.get() {
            animations.insert("transform".to_string(), AnimationValue::String("translateX(100px) rotate(180deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(0.5));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("translateX(0px) rotate(0deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }
        set_animate_signal.set(animations);
    });

    view! {
        <div>
            <button on:click=move |_| set_condition.set(!condition.get())>
                "Toggle Animation"
            </button>
            <ReactiveMotionDivV2
                initial=HashMap::new()
                animate=animate_signal
                transition=Transition::default()
            >
                <div>"Conditional Animation"</div>
            </ReactiveMotionDivV2>
        </div>
    }
}
```

## Performance Optimization

### Batch Updates

```rust
#[component]
fn OptimizedAnimation() -> impl IntoView {
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (scale, set_scale) = create_signal(1.0);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    // Batch all updates in a single effect
    Effect::new(move |_| {
        let mut animations = HashMap::new();
        animations.insert("transform".to_string(), AnimationValue::String(format!(
            "translate({}px, {}px) scale({})",
            x.get(), y.get(), scale.get()
        )));
        set_animate_signal.set(animations);
    });

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=Transition::default()
        >
            <div>"Optimized Animation"</div>
        </ReactiveMotionDivV2>
    }
}
```

### Use Appropriate Easing

```rust
// For UI interactions
let ui_transition = Transition {
    duration: Some(0.2),
    ease: Easing::EaseOut,
    repeat: RepeatConfig::Never,
    stagger: None,
};

// For page transitions
let page_transition = Transition {
    duration: Some(0.5),
    ease: Easing::EaseInOut,
    repeat: RepeatConfig::Never,
    stagger: None,
};

// For attention-grabbing animations
let attention_transition = Transition {
    duration: Some(0.8),
    ease: Easing::EaseOutBack,
    repeat: RepeatConfig::Never,
    stagger: None,
};
```

## Common Patterns

### Loading States

```rust
#[component]
fn LoadingAnimation() -> impl IntoView {
    let (is_loading, set_loading) = create_signal(true);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_loading.get() {
            animations.insert("transform".to_string(), AnimationValue::String("rotate(360deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(0.7));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("rotate(0deg)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        }
        set_animate_signal.set(animations);
    });

    let transition = Transition {
        duration: Some(1.0),
        ease: Easing::Linear,
        repeat: RepeatConfig::Infinite,
        stagger: None,
    };

    view! {
        <ReactiveMotionDivV2
            initial=HashMap::new()
            animate=animate_signal
            transition=transition
        >
            <div>"Loading..."</div>
        </ReactiveMotionDivV2>
    }
}
```

### Modal Animations

```rust
#[component]
fn Modal() -> impl IntoView {
    let (is_open, set_open) = create_signal(false);
    let (animate_signal, set_animate_signal) = create_signal(HashMap::new());

    Effect::new(move |_| {
        let mut animations = HashMap::new();
        if is_open.get() {
            animations.insert("transform".to_string(), AnimationValue::String("scale(1) translateY(0px)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(1.0));
        } else {
            animations.insert("transform".to_string(), AnimationValue::String("scale(0.8) translateY(-20px)".to_string()));
            animations.insert("opacity".to_string(), AnimationValue::Number(0.0));
        }
        set_animate_signal.set(animations);
    });

    view! {
        <div>
            <button on:click=move |_| set_open.set(true)>"Open Modal"</button>
            {move || if is_open.get() {
                view! {
                    <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100%; background: rgba(0,0,0,0.5); display: flex; align-items: center; justify-content: center;">
                        <ReactiveMotionDivV2
                            initial=HashMap::new()
                            animate=animate_signal
                            transition=Transition::default()
                        >
                            <div style="background: white; padding: 20px; border-radius: 8px;">
                                <h2>"Modal Title"</h2>
                                <p>"Modal content goes here"</p>
                                <button on:click=move |_| set_open.set(false)>"Close"</button>
                            </div>
                        </ReactiveMotionDivV2>
                    </div>
                }
            } else {
                view! { <div></div> }
            }}
        </div>
    }
}
```

## Troubleshooting

### Common Issues

1. **Animation not starting**
   - Check if the animation engine is started
   - Verify signal values are updating
   - Ensure proper initial values

2. **Janky animations**
   - Use appropriate easing functions
   - Avoid animating layout properties
   - Check for performance bottlenecks

3. **Reactive signal disposal**
   - Use `mount_to_body` instead of mounting to specific containers
   - Ensure proper component lifecycle management

### Debug Tips

```rust
// Add logging to track animation values
Effect::new(move |_| {
    let mut animations = HashMap::new();
    animations.insert("transform".to_string(), AnimationValue::String(format!("scale({})", scale.get())));
    set_animate_signal.set(animations);
    
    // Debug logging
    console::log_1(&format!("Scale: {}", scale.get()).into());
});
```

## Best Practices

1. **Use semantic animation names** for better maintainability
2. **Keep animations under 300ms** for UI interactions
3. **Use consistent easing functions** throughout your app
4. **Test on different devices** for performance
5. **Provide reduced motion alternatives** for accessibility
6. **Use transform and opacity** for best performance
7. **Avoid animating layout properties** like width, height, top, left

## Examples

Check out the complete examples in the repository:
- `simple-working-demo/` - Basic usage
- `performance-demo/` - Performance testing
- `phase2-reactive-demo/` - Advanced features
