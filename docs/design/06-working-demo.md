# Working Demo Creation

## Goal
Create a minimal working demo to prove the system works.

## Requirements
- Compiles without errors
- Runs in browser
- Shows basic animation
- Uses MotionDiv component

## Demo Structure
```rust
// examples/simple-demo/src/lib.rs
use leptos::prelude::*;
use leptos_motion_dom::MotionDiv;

#[component]
pub fn SimpleDemo() -> impl IntoView {
    let (is_active, set_is_active) = signal(false);
    
    view! {
        <MotionDiv
            initial=HashMap::from([("opacity".to_string(), AnimationValue::Number(0.0))])
            animate=HashMap::from([("opacity".to_string(), AnimationValue::Number(1.0))])
        >
            "Hello Motion!"
        </MotionDiv>
        <button on:click=move |_| set_is_active.set(!is_active.get())>
            "Toggle"
        </button>
    }
}
```

## Build Configuration
```toml
# examples/simple-demo/Cargo.toml
[package]
name = "simple-demo"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.8.6", features = ["csr"] }
leptos-motion-dom = { path = "../../crates/leptos-motion-dom" }
```

## Status
⏳ **PENDING** - Need to implement after build fixes
