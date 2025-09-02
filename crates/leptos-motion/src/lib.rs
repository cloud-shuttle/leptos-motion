//! # Leptos Motion
//! 
//! A comprehensive animation library for Leptos with Motion-inspired API.
//! 
//! ## Features
//! 
//! - **Declarative animations** with Motion-style API
//! - **Hardware acceleration** via Web Animations API with RAF fallback
//! - **Gesture support** for drag, hover, tap, and more
//! - **Layout animations** using FLIP technique
//! - **Scroll animations** with intersection observers
//! - **Type safety** with full Rust compile-time validation
//! - **High performance** targeting 60fps for 100+ concurrent animations
//! 
//! ## Quick Start
//! 
//! ```rust,no_run
//! use leptos::*;
//! use leptos_motion::*;
//! 
//! #[component]
//! fn App() -> impl IntoView {
//!     view! {
//!         <MotionDiv
//!             animate=motion_target!("x" => AnimationValue::Pixels(100.0))
//!             transition=Transition {
//!                 duration: Some(1.0),
//!                 ease: Easing::Spring(SpringConfig::default()),
//!                 ..Default::default()
//!             }
//!         >
//!             "Hello Leptos Motion!"
//!         </MotionDiv>
//!     }
//! }
//! ```
//! 
//! ## Core Concepts
//! 
//! ### Animation Values
//! 
//! Animation values represent different types of animatable properties:
//! 
//! ```rust,no_run
//! use leptos_motion::*;
//! 
//! let opacity = AnimationValue::Number(1.0);
//! let position = AnimationValue::Pixels(100.0);
//! let rotation = AnimationValue::Degrees(45.0);
//! let color = AnimationValue::Color("#ff0000".to_string());
//! ```
//! 
//! ### Transitions
//! 
//! Configure how animations behave:
//! 
//! ```rust,no_run
//! # use leptos_motion::*;
//! // Spring animation
//! let spring = Transition {
//!     ease: Easing::Spring(SpringConfig {
//!         stiffness: 100.0,
//!         damping: 10.0,
//!         mass: 1.0,
//!         ..Default::default()
//!     }),
//!     ..Default::default()
//! };
//! 
//! // Tween animation
//! let tween = Transition {
//!     duration: Some(0.3),
//!     ease: Easing::EaseInOut,
//!     delay: Some(0.1),
//!     ..Default::default()
//! };
//! ```
//! 
//! ### Gestures
//! 
//! Add interactivity to your animations:
//! 
//! ```rust,no_run
//! # use leptos::*;
//! # use leptos_motion::*;
//! view! {
//!     <MotionDiv
//!         while_hover=motion_target!("scale" => AnimationValue::Number(1.1))
//!         while_tap=motion_target!("scale" => AnimationValue::Number(0.9))
//!         drag=Some(DragConfig::new().axis(DragAxis::Both))
//!     >
//!         "Interactive element"
//!     </MotionDiv>
//! }
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

// Re-export all core functionality
pub use leptos_motion_core::*;
pub use leptos_motion_dom::*;

// Re-export the motion_target macro
pub use leptos_motion_dom::motion_target;

#[cfg(feature = "gestures")]
pub use leptos_motion_gestures::*;

#[cfg(feature = "layout")]
pub use leptos_motion_layout::*;

#[cfg(feature = "scroll")]
pub use leptos_motion_scroll::*;

pub use leptos_motion_macros::*;

/// Animation presets and common patterns
pub mod presets {
    // Core animation presets
    pub use leptos_motion_core::animation::presets::*;
    pub use leptos_motion_core::spring::presets::*;
    pub use leptos_motion_core::easing::presets::*;
}

/// Re-export commonly used external types
pub mod external {
    pub use leptos::*;
    pub use web_sys::{Element, HtmlElement, Event, MouseEvent, TouchEvent};
    pub use wasm_bindgen::prelude::*;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library information
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(!DESCRIPTION.is_empty());
    }
    
    #[test]
    fn test_animation_value_creation() {
        let opacity = AnimationValue::Number(1.0);
        let position = AnimationValue::Pixels(100.0);
        let rotation = AnimationValue::Degrees(45.0);
        
        assert_eq!(opacity, AnimationValue::Number(1.0));
        assert_eq!(position, AnimationValue::Pixels(100.0));
        assert_eq!(rotation, AnimationValue::Degrees(45.0));
    }
    
    #[test]
    fn test_transition_creation() {
        let transition = Transition {
            duration: Some(1.0),
            ease: Easing::Spring(SpringConfig::default()),
            ..Default::default()
        };
        
        assert_eq!(transition.duration, Some(1.0));
        assert!(matches!(transition.ease, Easing::Spring(_)));
    }
}