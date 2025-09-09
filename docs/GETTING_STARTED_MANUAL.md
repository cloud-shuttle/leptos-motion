# Getting Started with Leptos Motion 🚀

A step-by-step guide to building your first animations with Leptos Motion.

## Prerequisites

- Rust 1.70+ installed
- Basic knowledge of Rust and Leptos
- A text editor or IDE

## Step 1: Project Setup

### Create a New Leptos Project

```bash
# Create new Leptos project
cargo new my-leptos-motion-app
cd my-leptos-motion-app
```

### Add Dependencies

Add to your `Cargo.toml`:

```toml
[package]
name = "my-leptos-motion-app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = "0.8"
leptos-motion = "0.4"
console_error_panic_hook = "0.1"
console_log = "1.0"
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib", "rlib"]
```

### Configure for WebAssembly

Create `Cargo.toml` with web target:

```toml
[dependencies]
# ... other dependencies

[target.'cfg(target_arch = "wasm32")'.dependencies]
getrandom = { version = "0.2", features = ["js"] }
```

## Step 2: Your First Animation

### Basic Setup

Create `src/lib.rs`:

```rust
use leptos::*;
use leptos_motion::*;
use leptos_motion_core::{AnimationTarget, AnimationValue, Transition, Easing, RepeatConfig};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;

/// Your first animated component
#[component]
fn AnimatedBox() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);

    // Define initial state (hidden)
    let initial = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(0.5));
        map
    };

    // Define target state (visible)
    let animate = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    };

    // Define transition
    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    view! {
        <div style="padding: 2rem; text-align: center;">
            <h1>"My First Leptos Motion Animation"</h1>

            <button
                on:click=move |_| set_is_visible.set(!is_visible.get())
                style="padding: 1rem 2rem; margin: 1rem; font-size: 1.2rem;"
            >
                {move || if is_visible.get() { "Hide" } else { "Show" }}
            </button>

            <div style="margin-top: 2rem;">
                <MotionDiv
                    class=Some("animated-box".to_string())
                    initial=Some(initial)
                    animate=Some(animate)
                    _transition=Some(transition)
                    style="
                        padding: 2rem;
                        background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                        color: white;
                        border-radius: 10px;
                        font-size: 1.2rem;
                        font-weight: bold;
                        box-shadow: 0 4px 15px rgba(0,0,0,0.2);
                    "
                >
                    "🎉 Hello Leptos Motion!"
                </MotionDiv>
            </div>
        </div>
    }
}

/// Main app component
#[component]
fn App() -> impl IntoView {
    view! {
        <AnimatedBox />
    }
}

/// Entry point
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    console_log::init().expect("Failed to initialize console log");

    mount_to_body(|| view! { <App/> })
}
```

### HTML File

Create `index.html`:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Leptos Motion Demo</title>
    <style>
      body {
        margin: 0;
        padding: 0;
        font-family:
          -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        min-height: 100vh;
      }

      .animated-box {
        transition: all 0.3s ease;
      }
    </style>
  </head>
  <body>
    <script type="module">
      import init from './pkg/my_leptos_motion_app.js';
      await init();
    </script>
  </body>
</html>
```

## Step 3: Build and Run

### Build for WebAssembly

```bash
# Install wasm-pack if you haven't already
cargo install wasm-pack

# Build the project
wasm-pack build --target web --out-dir pkg
```

### Serve Locally

```bash
# Using Python (if installed)
python3 -m http.server 8080

# Or using Node.js
npx serve .

# Or using any other static file server
```

Open `http://localhost:8080` in your browser to see your first animation!

## Step 4: Adding More Animations

### Hover Effects

```rust
#[component]
fn HoverButton() -> impl IntoView {
    // Normal state
    let normal = {
        let mut map = HashMap::new();
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map
    };

    // Hover state
    let hover = {
        let mut map = HashMap::new();
        map.insert("scale".to_string(), AnimationValue::Number(1.05));
        map.insert("y".to_string(), AnimationValue::Pixels(-2.0));
        map
    };

    view! {
        <MotionDiv
            _while_hover=Some(hover)
            style="
                padding: 1rem 2rem;
                background: linear-gradient(45deg, #667eea, #764ba2);
                color: white;
                border: none;
                border-radius: 25px;
                cursor: pointer;
                font-size: 1.1rem;
                font-weight: 600;
                box-shadow: 0 4px 15px rgba(0,0,0,0.2);
            "
        >
            "Hover me!"
        </MotionDiv>
    }
}
```

### Slide Animations

```rust
#[component]
fn SlideCard() -> impl IntoView {
    let (is_slided, set_is_slided) = signal(false);

    let slide_in = {
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(-300.0));
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map
    };

    let slide_out = {
        let mut map = HashMap::new();
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map
    };

    view! {
        <div>
            <button on:click=move |_| set_is_slided.set(!is_slided.get())>
                "Toggle Slide"
            </button>

            <MotionDiv
                initial=Some(slide_in)
                animate=Some(slide_out)
                _transition=Some(Transition {
                    duration: Some(0.6),
                    ease: Easing::EaseInOut,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                style="
                    margin-top: 1rem;
                    padding: 2rem;
                    background: white;
                    border-radius: 10px;
                    box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                "
            >
                "This card slides in from the left!"
            </MotionDiv>
        </div>
    }
}
```

### Staggered Animations

```rust
#[component]
fn StaggeredList() -> impl IntoView {
    let items = vec!["Item 1", "Item 2", "Item 3", "Item 4"];

    let initial = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("y".to_string(), AnimationValue::Pixels(20.0));
        map
    };

    let animate = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("y".to_string(), AnimationValue::Pixels(0.0));
        map
    };

    let stagger_transition = Transition {
        duration: Some(0.3),
        ease: Easing::EaseOut,
        delay: None,
        repeat: RepeatConfig::Never,
        stagger: Some(StaggerConfig {
            delay: 0.1,
            from: StaggerFrom::First,
        }),
    };

    view! {
        <div>
            <h3>"Staggered Animation"</h3>
            {items.into_iter().enumerate().map(|(i, item)| {
                view! {
                    <MotionDiv
                        key=i
                        initial=Some(initial.clone())
                        animate=Some(animate.clone())
                        _transition=Some(stagger_transition.clone())
                        style="
                            padding: 1rem;
                            margin: 0.5rem 0;
                            background: linear-gradient(45deg, #ff9a9e, #fecfef);
                            border-radius: 8px;
                            color: #333;
                        "
                    >
                        {item}
                    </MotionDiv>
                }
            }).collect_view()}
        </div>
    }
}
```

## Step 5: Advanced Patterns

### Conditional Animations

```rust
#[component]
fn ConditionalAnimation() -> impl IntoView {
    let (mode, set_mode) = signal(0);

    let get_animation = move || {
        let mut map = HashMap::new();
        match mode.get() {
            0 => {
                // Fade animation
                map.insert("opacity".to_string(), AnimationValue::Number(0.0));
                map.insert("scale".to_string(), AnimationValue::Number(0.8));
            },
            1 => {
                // Slide animation
                map.insert("x".to_string(), AnimationValue::Pixels(-200.0));
                map.insert("opacity".to_string(), AnimationValue::Number(0.0));
            },
            2 => {
                // Rotate animation
                map.insert("rotate".to_string(), AnimationValue::Degrees(-180.0));
                map.insert("scale".to_string(), AnimationValue::Number(0.5));
            },
            _ => {}
        }
        map
    };

    let get_target = move || {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map.insert("x".to_string(), AnimationValue::Pixels(0.0));
        map.insert("rotate".to_string(), AnimationValue::Degrees(0.0));
        map
    };

    view! {
        <div>
            <div style="margin-bottom: 1rem;">
                <button on:click=move |_| set_mode.set(0)>"Fade"</button>
                <button on:click=move |_| set_mode.set(1)>"Slide"</button>
                <button on:click=move |_| set_mode.set(2)>"Rotate"</button>
            </div>

            <MotionDiv
                initial=Some(get_animation())
                animate=Some(get_target())
                _transition=Some(Transition {
                    duration: Some(0.5),
                    ease: Easing::EaseInOut,
                    delay: None,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                style="
                    padding: 2rem;
                    background: linear-gradient(45deg, #a8edea, #fed6e3);
                    border-radius: 10px;
                    color: #333;
                "
            >
                "Mode: {move || mode.get()}"
            </MotionDiv>
        </div>
    }
}
```

### Exit Animations with AnimatePresence

```rust
#[component]
fn ExitAnimation() -> impl IntoView {
    let (show, set_show) = signal(true);

    let enter = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(1.0));
        map.insert("scale".to_string(), AnimationValue::Number(1.0));
        map
    };

    let exit = {
        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(0.0));
        map.insert("scale".to_string(), AnimationValue::Number(0.5));
        map
    };

    view! {
        <div>
            <button on:click=move |_| set_show.set(!show.get())>
                {move || if show.get() { "Hide" } else { "Show" }}
            </button>

            <AnimatePresence mode=Some(PresenceMode::Sync)>
                {move || show.get().then(|| view! {
                    <MotionDiv
                        initial=Some(enter.clone())
                        animate=Some(enter.clone())
                        exit=Some(exit.clone())
                        _transition=Some(Transition {
                            duration: Some(0.3),
                            ease: Easing::EaseInOut,
                            delay: None,
                            repeat: RepeatConfig::Never,
                            stagger: None,
                        })
                        style="
                            margin-top: 1rem;
                            padding: 2rem;
                            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                            color: white;
                            border-radius: 10px;
                        "
                    >
                        "This element will exit with animation!"
                    </MotionDiv>
                })}
            </AnimatePresence>
        </div>
    }
}
```

## Step 6: Best Practices

### 1. Performance Optimization

```rust
// Use hardware-accelerated properties
let optimized_animation = {
    let mut map = HashMap::new();
    // Good: transform and opacity are GPU-accelerated
    map.insert("scale".to_string(), AnimationValue::Number(1.1));
    map.insert("opacity".to_string(), AnimationValue::Number(0.8));
    // Avoid: animating layout properties like width, height
    map
};
```

### 2. Reusable Animation Helpers

```rust
// Create reusable animation functions
fn create_fade_animation(visible: bool) -> AnimationTarget {
    let mut map = HashMap::new();
    map.insert("opacity".to_string(), AnimationValue::Number(if visible { 1.0 } else { 0.0 }));
    map
}

fn create_scale_animation(scale: f64) -> AnimationTarget {
    let mut map = HashMap::new();
    map.insert("scale".to_string(), AnimationValue::Number(scale));
    map
}

fn create_slide_animation(x: f64, y: f64) -> AnimationTarget {
    let mut map = HashMap::new();
    map.insert("x".to_string(), AnimationValue::Pixels(x));
    map.insert("y".to_string(), AnimationValue::Pixels(y));
    map
}
```

### 3. Consistent Transitions

```rust
// Define common transitions
const FAST_TRANSITION: Transition = Transition {
    duration: Some(0.2),
    ease: Easing::EaseOut,
    delay: None,
    repeat: RepeatConfig::Never,
    stagger: None,
};

const SMOOTH_TRANSITION: Transition = Transition {
    duration: Some(0.5),
    ease: Easing::EaseInOut,
    delay: None,
    repeat: RepeatConfig::Never,
    stagger: None,
};

const SPRING_TRANSITION: Transition = Transition {
    duration: None,
    ease: Easing::Spring(SpringConfig {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        velocity: 0.0,
        rest_delta: 0.01,
        rest_speed: 0.01,
    }),
    delay: None,
    repeat: RepeatConfig::Never,
    stagger: None,
};
```

## Step 7: Debugging Tips

### 1. Console Logging

```rust
use web_sys::console;

#[component]
fn DebugAnimation() -> impl IntoView {
    let (is_visible, set_is_visible) = signal(true);

    let animate = move || {
        let visible = is_visible.get();
        console::log_1(&format!("Animation state: {}", visible).into());

        let mut map = HashMap::new();
        map.insert("opacity".to_string(), AnimationValue::Number(if visible { 1.0 } else { 0.0 }));
        map
    };

    view! {
        <button on:click=move |_| set_is_visible.set(!is_visible.get())>
            "Toggle"
        </button>

        <MotionDiv
            animate=Some(animate())
            _transition=Some(SMOOTH_TRANSITION)
        >
            "Debug me!"
        </MotionDiv>
    }
}
```

### 2. CSS Validation

```css
/* Ensure your CSS supports the animations */
.animated-element {
  will-change: transform, opacity; /* Hint to browser for optimization */
  backface-visibility: hidden; /* Prevent flickering */
  transform-style: preserve-3d; /* Enable 3D transforms */
}
```

## Step 8: Next Steps

### Explore More Features

1. **Gestures**: Add drag, hover, and tap interactions
2. **Layout Animations**: Animate position and size changes
3. **Scroll Animations**: Trigger animations on scroll
4. **Complex Sequences**: Chain multiple animations together

### Resources

- [User Guide](./LEPTOS_MOTION_USER_GUIDE.md) - Comprehensive API reference
- [Examples](../examples/) - Working code examples
- [Architecture Docs](../docs/07-architecture/) - Technical deep-dive
- [Community](https://github.com/leptos-rs/leptos) - Get help and contribute

### Common Issues and Solutions

| Issue                  | Solution                                        |
| ---------------------- | ----------------------------------------------- |
| Animations not working | Check prop types and `Some()` wrappers          |
| Choppy animations      | Use hardware-accelerated properties             |
| Type errors            | Import correct types from `leptos_motion_core`  |
| Performance issues     | Optimize CSS and reduce simultaneous animations |

---

**Congratulations! 🎉** You've built your first Leptos Motion animations. Keep
experimenting and building amazing user experiences!

**Happy Coding! 🚀**
