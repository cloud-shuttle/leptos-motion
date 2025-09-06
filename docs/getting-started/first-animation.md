# Your First Animation

This tutorial will guide you through creating your first animated component with Leptos Motion.

## What We'll Build

We'll create a simple animated button that:

- Fades in when the page loads
- Scales up on hover
- Bounces when clicked
- Slides in from the left

## Step 1: Basic Setup

Start with a simple Leptos component:

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn AnimatedButton() -> impl IntoView {
    view! {
        <button class="my-button">
            "Click me!"
        </button>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"My First Animation"</h1>
            <AnimatedButton />
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
```

## Step 2: Add Fade-In Animation

Let's make the button fade in when it first appears:

```rust
#[component]
fn AnimatedButton() -> impl IntoView {
    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Click me!"
        </MotionDiv>
    }
}
```

**What's happening:**

- `initial` sets the starting state (invisible)
- `animate` sets the target state (visible)
- `transition` controls how the animation happens

## Step 3: Add Hover Effect

Now let's add a hover animation that scales the button:

```rust
#[component]
fn AnimatedButton() -> impl IntoView {
    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Click me!"
        </MotionDiv>
    }
}
```

**What's new:**

- `while_hover` triggers when the mouse hovers over the element
- The button will scale to 110% of its original size

## Step 4: Add Click Animation

Let's add a bounce effect when the button is clicked:

```rust
#[component]
fn AnimatedButton() -> impl IntoView {
    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            while_tap=motion_target!(
                "scale" => AnimationValue::Number(0.95)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Click me!"
        </MotionDiv>
    }
}
```

**What's new:**

- `while_tap` triggers when the element is clicked/tapped
- The button will briefly scale down to 95% when clicked

## Step 5: Add Slide-In Effect

Let's make the button slide in from the left:

```rust
#[component]
fn AnimatedButton() -> impl IntoView {
    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-100.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(0.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            while_tap=motion_target!(
                "scale" => AnimationValue::Number(0.95)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Click me!"
        </MotionDiv>
    }
}
```

**What's new:**

- Added `x` property to both `initial` and `animate`
- The button starts 100px to the left and slides to its final position

## Step 6: Add Some Style

Let's add CSS to make it look good:

```rust
#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <style>
                {r#"
                .container {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    min-height: 100vh;
                    font-family: Arial, sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }

                .my-button {
                    padding: 15px 30px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                    color: white;
                    border: none;
                    border-radius: 25px;
                    font-size: 18px;
                    font-weight: bold;
                    cursor: pointer;
                    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
                    transition: box-shadow 0.3s ease;
                }

                .my-button:hover {
                    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                }

                h1 {
                    color: white;
                    margin-bottom: 2rem;
                    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }
                "#
            }
            </style>
            <h1>"My First Animation"</h1>
            <AnimatedButton />
        </div>
    }
}
```

## Step 7: Add State Management

Let's make the button actually do something when clicked:

```rust
#[component]
fn AnimatedButton() -> impl IntoView {
    let (click_count, set_click_count) = signal(0);
    let (is_clicked, set_is_clicked) = signal(false);

    let handle_click = move |_| {
        set_click_count.update(|count| *count += 1);
        set_is_clicked.set(true);

        // Reset the clicked state after animation
        set_timeout(move || {
            set_is_clicked.set(false);
        }, 200);
    };

    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-100.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(0.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            while_tap=motion_target!(
                "scale" => AnimationValue::Number(0.95)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
            on:click=handle_click
        >
            {move || if is_clicked.get() {
                "Clicked!".to_string()
            } else {
                format!("Click me! ({})", click_count.get())
            }}
        </MotionDiv>
    }
}
```

**What's new:**

- Added state management with signals
- Button text changes when clicked
- Click counter tracks interactions
- Temporary "Clicked!" message

## Complete Example

Here's the complete, working example:

```rust
use leptos::*;
use leptos_motion::*;

#[component]
fn AnimatedButton() -> impl IntoView {
    let (click_count, set_click_count) = signal(0);
    let (is_clicked, set_is_clicked) = signal(false);

    let handle_click = move |_| {
        set_click_count.update(|count| *count += 1);
        set_is_clicked.set(true);

        set_timeout(move || {
            set_is_clicked.set(false);
        }, 200);
    };

    view! {
        <MotionDiv
            class="my-button".to_string()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-100.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(0.0)
            )
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1)
            )
            while_tap=motion_target!(
                "scale" => AnimationValue::Number(0.95)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
            on:click=handle_click
        >
            {move || if is_clicked.get() {
                "Clicked!".to_string()
            } else {
                format!("Click me! ({})", click_count.get())
            }}
        </MotionDiv>
    }
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="container">
            <style>
                {r#"
                .container {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    min-height: 100vh;
                    font-family: Arial, sans-serif;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                }

                .my-button {
                    padding: 15px 30px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                    color: white;
                    border: none;
                    border-radius: 25px;
                    font-size: 18px;
                    font-weight: bold;
                    cursor: pointer;
                    box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
                    transition: box-shadow 0.3s ease;
                }

                .my-button:hover {
                    box-shadow: 0 6px 20px rgba(0, 0, 0, 0.3);
                }

                h1 {
                    color: white;
                    margin-bottom: 2rem;
                    text-shadow: 0 2px 4px rgba(0, 0, 0, 0.3);
                }
                "#
            }
            </style>
            <h1>"My First Animation"</h1>
            <AnimatedButton />
        </div>
    }
}

fn main() {
    mount_to_body(App)
}
```

## What You've Learned

In this tutorial, you've learned:

1. **Basic MotionDiv usage** - The core animation component
2. **Animation properties** - `initial`, `animate`, `while_hover`, `while_tap`
3. **Animation values** - `AnimationValue::Number` and `AnimationValue::Pixels`
4. **Transitions** - Controlling animation timing and easing
5. **State management** - Using signals with animations
6. **Event handling** - Combining animations with user interactions

## Next Steps

Now that you've created your first animation:

1. **Experiment** - Try different animation values and transitions
2. **Explore Examples** - Check out more examples in the [Examples](../examples/basic.md) section
3. **Read the API** - Learn about all available features in the [API Reference](../api/README.md)
4. **Try Advanced Features** - Explore [gestures](../guides/gestures.md) and [layout animations](../guides/layout-animations.md)

## Common Patterns

Here are some common animation patterns you can try:

### Fade In

```rust
initial=motion_target!("opacity" => AnimationValue::Number(0.0))
animate=motion_target!("opacity" => AnimationValue::Number(1.0))
```

### Slide In from Left

```rust
initial=motion_target!("x" => AnimationValue::Pixels(-100.0))
animate=motion_target!("x" => AnimationValue::Pixels(0.0))
```

### Scale In

```rust
initial=motion_target!("scale" => AnimationValue::Number(0.0))
animate=motion_target!("scale" => AnimationValue::Number(1.0))
```

### Rotate In

```rust
initial=motion_target!("rotate" => AnimationValue::Degrees(-180.0))
animate=motion_target!("rotate" => AnimationValue::Degrees(0.0))
```

---

**Next**: [Animation Types](../guides/animation-types.md) - Learn about different types of animations
