//! Phase 2 Minimal Demo
//!
//! This demo shows CSS-based animations using Leptos signals and CSS classes.
//! This approach avoids the complex animation engine API conflicts.

use leptos::prelude::*;

/// Main app component
#[component]
fn App() -> impl IntoView {
    view! {
        <div>
            <h1>"🚀 Leptos Motion - Phase 2 Minimal Demo"</h1>
            <p>"This demo shows CSS-based animations using Leptos signals and CSS classes."</p>
            
            <OpacityDemo/>
            <ScaleDemo/>
            <HoverDemo/>
            <MultiPropertyDemo/>
        </div>
    }
}

/// Opacity animation demo using CSS classes
#[component]
fn OpacityDemo() -> impl IntoView {
    let (is_clicked, set_clicked) = signal(false);
    
    let class = move || {
        if is_clicked.get() {
            "demo-box opacity-low"
        } else {
            "demo-box"
        }
    };
    
    view! {
        <div class="demo-section">
            <h2>"Basic Opacity Animation"</h2>
            <div 
                class=class
                on:click=move |_| set_clicked.update(|x| *x = !*x)
            >
                "Click me!"
            </div>
            <p>"Click the box to animate opacity from 1.0 to 0.5 and back."</p>
        </div>
    }
}

/// Scale animation demo using CSS classes
#[component]
fn ScaleDemo() -> impl IntoView {
    let (is_clicked, set_clicked) = signal(false);
    
    let class = move || {
        if is_clicked.get() {
            "demo-box scale-large"
        } else {
            "demo-box"
        }
    };
    
    view! {
        <div class="demo-section">
            <h2>"Scale Animation"</h2>
            <div 
                class=class
                on:click=move |_| set_clicked.update(|x| *x = !*x)
            >
                "Scale me!"
            </div>
            <p>"Click the box to animate scale from 1.0 to 1.5 and back."</p>
        </div>
    }
}

/// Hover animation demo using CSS hover
#[component]
fn HoverDemo() -> impl IntoView {
    view! {
        <div class="demo-section">
            <h2>"Hover Animation"</h2>
            <div class="demo-box">
                "Hover me!"
            </div>
            <p>"Hover over the box to see the animation."</p>
        </div>
    }
}

/// Multiple properties animation demo using CSS classes
#[component]
fn MultiPropertyDemo() -> impl IntoView {
    let (is_clicked, set_clicked) = signal(false);
    
    let class = move || {
        if is_clicked.get() {
            "demo-box multi-animated"
        } else {
            "demo-box"
        }
    };
    
    view! {
        <div class="demo-section">
            <h2>"Multiple Properties"</h2>
            <div 
                class=class
                on:click=move |_| set_clicked.update(|x| *x = !*x)
            >
                "Multi!"
            </div>
            <p>"Click to animate opacity, scale, and rotation simultaneously."</p>
        </div>
    }
}

/// Main function
fn main() {
    console_error_panic_hook::set_once();
    
    mount_to_body(|| view! { <App/> })
}
