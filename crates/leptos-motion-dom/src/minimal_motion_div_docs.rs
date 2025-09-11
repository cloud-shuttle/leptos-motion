//! # MinimalMotionDiv API Documentation
//!
//! This module provides comprehensive documentation for the `MinimalMotionDiv` component,
//! a lightweight, high-performance alternative to `ReactiveMotionDiv` for scenarios
//! where maximum performance and minimal overhead are required.
//!
//! ## Overview
//!
//! `MinimalMotionDiv` is designed as a fallback component for problematic scenarios
//! where the full `ReactiveMotionDiv` might cause issues (such as right-click blocking
//! or memory leaks). It provides the core animation functionality with minimal overhead.
//!
//! ## Features
//!
//! - ✅ **Zero Event Handlers**: No mouse/touch event interference
//! - ✅ **Minimal Memory Usage**: Optimized for performance
//! - ✅ **Simple API**: Easy to use and understand
//! - ✅ **Right-Click Compatible**: No browser interaction blocking
//! - ✅ **Memory Leak Free**: Clean resource management
//!
//! ## Basic Usage
//!
//! ```rust
//! use leptos::prelude::*;
//! use leptos_motion_dom::MinimalMotionDiv;
//! use leptos_motion_core::{AnimationValue, Transition, Easing, RepeatConfig};
//! use std::collections::HashMap;
//!
//! #[component]
//! pub fn MyComponent() -> impl IntoView {
//!     // Create animation target
//!     let mut animate = HashMap::new();
//!     animate.insert("scale".to_string(), AnimationValue::Number(1.2));
//!     animate.insert("opacity".to_string(), AnimationValue::Number(0.8));
//!
//!     // Create transition configuration
//!     let transition = Transition {
//!         duration: Some(0.5),
//!         delay: None,
//!         ease: Easing::EaseInOut,
//!         repeat: RepeatConfig::Never,
//!         stagger: None,
//!     };
//!
//!     view! {
//!         <MinimalMotionDiv
//!             animate=animate
//!             transition=transition
//!             style="width: 100px; height: 100px; background: #ff6b6b; border-radius: 10px;".to_string()
//!         >
//!             "Animate!"
//!         </MinimalMotionDiv>
//!     }
//! }
//! ```
//!
//! ## API Reference
//!
//! ### Props
//!
//! #### `animate: AnimationTarget`
//! The target animation state. This is a `HashMap<String, AnimationValue>` that defines
//! the properties to animate and their target values.
//!
//! **Supported Properties:**
//! - `scale`: Scale transformation (number)
//! - `rotateZ`: Z-axis rotation in degrees (number)
//! - `opacity`: Opacity value 0.0-1.0 (number)
//! - `translateX`: X-axis translation in pixels (number)
//! - `translateY`: Y-axis translation in pixels (number)
//! - `skewX`: X-axis skew in degrees (number)
//! - `skewY`: Y-axis skew in degrees (number)
//! - `rotateX`: X-axis rotation in degrees (number)
//! - `rotateY`: Y-axis rotation in degrees (number)
//! - `translateZ`: Z-axis translation in pixels (number)
//!
//! **Example:**
//! ```rust
//! let mut animate = HashMap::new();
//! animate.insert("scale".to_string(), AnimationValue::Number(1.5));
//! animate.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
//! animate.insert("opacity".to_string(), AnimationValue::Number(0.7));
//! ```
//!
//! #### `transition: Transition`
//! Animation timing and easing configuration.
//!
//! **Properties:**
//! - `duration: Option<f64>` - Animation duration in seconds
//! - `delay: Option<f64>` - Delay before animation starts in seconds
//! - `ease: Easing` - Easing function for the animation
//! - `repeat: RepeatConfig` - Repeat behavior
//! - `stagger: Option<StaggerConfig>` - Stagger configuration for multiple elements
//!
//! **Example:**
//! ```rust
//! let transition = Transition {
//!     duration: Some(0.8),
//!     delay: Some(0.2),
//!     ease: Easing::EaseInOut,
//!     repeat: RepeatConfig::Count(3),
//!     stagger: None,
//! };
//! ```
//!
//! #### `style: String`
//! CSS styles to apply to the div element. This is a standard CSS string.
//!
//! **Example:**
//! ```rust
//! let style = "width: 200px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #ee5a24); border-radius: 15px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;".to_string();
//! ```
//!
//! #### `class: Option<String>`
//! Optional CSS class name to apply to the div element.
//!
//! **Example:**
//! ```rust
//! let class = Some("my-animation-class".to_string());
//! ```
//!
//! #### `node_ref: Option<NodeRef<leptos::html::Div>>`
//! Optional reference to the DOM element for advanced use cases.
//!
//! **Example:**
//! ```rust
//! let node_ref = NodeRef::<leptos::html::Div>::new();
//! ```
//!
//! ## Easing Functions
//!
//! The `Easing` enum provides various easing functions:
//!
//! ```rust
//! use leptos_motion_core::Easing;
//!
//! // Linear easing
//! Easing::Linear
//!
//! // Standard easing functions
//! Easing::EaseIn
//! Easing::EaseOut
//! Easing::EaseInOut
//!
//! // Circular easing
//! Easing::CircIn
//! Easing::CircOut
//! Easing::CircInOut
//!
//! // Back easing (overshoot effect)
//! Easing::BackIn
//! Easing::BackOut
//! Easing::BackInOut
//!
//! // Custom bezier curve
//! Easing::Bezier(0.25, 0.1, 0.25, 1.0)
//!
//! // Spring physics
//! Easing::Spring(SpringConfig {
//!     stiffness: 100.0,
//!     damping: 10.0,
//!     mass: 1.0,
//!     rest_delta: 0.01,
//!     rest_speed: 0.01,
//!     velocity: 0.0,
//! })
//! ```
//!
//! ## Repeat Configurations
//!
//! The `RepeatConfig` enum controls animation repetition:
//!
//! ```rust
//! use leptos_motion_core::RepeatConfig;
//!
//! // No repetition
//! RepeatConfig::Never
//!
//! // Repeat a specific number of times
//! RepeatConfig::Count(3)
//!
//! // Repeat infinitely
//! RepeatConfig::Infinite
//!
//! // Repeat infinitely with reverse direction
//! RepeatConfig::InfiniteReverse
//! ```
//!
//! ## Animation Values
//!
//! The `AnimationValue` enum supports various value types:
//!
//! ```rust
//! use leptos_motion_core::AnimationValue;
//!
//! // Numeric values
//! AnimationValue::Number(1.5)
//!
//! // String values
//! AnimationValue::String("100px".to_string())
//!
//! // Pixel values
//! AnimationValue::Pixels(100.0)
//!
//! // Percentage values
//! AnimationValue::Percentage(50.0)
//!
//! // Degree values
//! AnimationValue::Degrees(45.0)
//!
//! // Radian values
//! AnimationValue::Radians(0.785)
//!
//! // Color values
//! AnimationValue::Color("#ff6b6b".to_string())
//! ```
//!
//! ## Advanced Examples
//!
//! ### Complex 3D Animation
//!
//! ```rust
//! let mut animate = HashMap::new();
//! animate.insert("rotateX".to_string(), AnimationValue::Number(360.0));
//! animate.insert("rotateY".to_string(), AnimationValue::Number(180.0));
//! animate.insert("rotateZ".to_string(), AnimationValue::Number(90.0));
//! animate.insert("translateZ".to_string(), AnimationValue::Number(100.0));
//! animate.insert("scale".to_string(), AnimationValue::Number(1.2));
//!
//! let transition = Transition {
//!     duration: Some(2.0),
//!     delay: None,
//!     ease: Easing::EaseInOut,
//!     repeat: RepeatConfig::Infinite,
//!     stagger: None,
//! };
//! ```
//!
//! ### Spring Physics Animation
//!
//! ```rust
//! let mut animate = HashMap::new();
//! animate.insert("scale".to_string(), AnimationValue::Number(1.5));
//! animate.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
//!
//! let transition = Transition {
//!     duration: None, // Spring physics doesn't use duration
//!     delay: None,
//!     ease: Easing::Spring(SpringConfig {
//!         stiffness: 200.0,
//!         damping: 20.0,
//!         mass: 0.5,
//!         rest_delta: 0.005,
//!         rest_speed: 0.005,
//!         velocity: 0.0,
//!     }),
//!     repeat: RepeatConfig::Never,
//!     stagger: None,
//! };
//! ```
//!
//! ### Staggered Animation
//!
//! ```rust
//! let transition = Transition {
//!     duration: Some(0.5),
//!     delay: None,
//!     ease: Easing::EaseOut,
//!     repeat: RepeatConfig::Never,
//!     stagger: Some(StaggerConfig {
//!         amount: 0.1,
//!         from: StaggerFrom::First,
//!         direction: StaggerDirection::Forward,
//!     }),
//! };
//! ```
//!
//! ## Performance Considerations
//!
//! `MinimalMotionDiv` is optimized for performance:
//!
//! - **Memory Efficient**: Minimal memory allocation
//! - **CPU Optimized**: Efficient animation calculations
//! - **No Event Overhead**: No event handler processing
//! - **Clean Rendering**: Direct style application
//!
//! ## When to Use MinimalMotionDiv
//!
//! Use `MinimalMotionDiv` when:
//!
//! - ✅ You need maximum performance
//! - ✅ You're experiencing right-click issues with `ReactiveMotionDiv`
//! - ✅ You have memory leak concerns
//! - ✅ You need a simple, reliable animation component
//! - ✅ You don't need complex event handling
//!
//! Use `ReactiveMotionDiv` when:
//!
//! - ✅ You need hover/tap animations
//! - ✅ You need complex event handling
//! - ✅ You need drag functionality
//! - ✅ You need layout animations
//!
//! ## Migration from ReactiveMotionDiv
//!
//! To migrate from `ReactiveMotionDiv` to `MinimalMotionDiv`:
//!
//! 1. Replace the component name
//! 2. Remove event-related props (`while_hover`, `while_tap`, etc.)
//! 3. Keep the same `animate` and `transition` props
//! 4. Update any event handlers to use standard Leptos event handling
//!
//! **Before:**
//! ```rust
//! <ReactiveMotionDiv
//!     animate=animate
//!     while_hover=hover_animate
//!     while_tap=tap_animate
//!     transition=transition
//! >
//!     "Content"
//! </ReactiveMotionDiv>
//! ```
//!
//! **After:**
//! ```rust
//! <MinimalMotionDiv
//!     animate=animate
//!     transition=transition
//!     on:mouseenter=move |_| { /* handle hover */ }
//!     on:click=move |_| { /* handle tap */ }
//! >
//!     "Content"
//! </MinimalMotionDiv>
//! ```
//!
//! ## Testing
//!
//! `MinimalMotionDiv` includes comprehensive tests:
//!
//! - ✅ Animation target creation
//! - ✅ Transition configuration
//! - ✅ Easing function validation
//! - ✅ Repeat configuration testing
//! - ✅ Edge case handling
//! - ✅ Performance validation
//!
//! Run tests with:
//! ```bash
//! cargo test minimal_motion_div_tests --lib
//! ```
//!
//! ## Troubleshooting
//!
//! **Animation not working:**
//! - Check that `animate` prop contains valid properties
//! - Verify `transition` configuration is correct
//! - Ensure CSS styles don't conflict with animations
//!
//! **Performance issues:**
//! - Use `MinimalMotionDiv` instead of `ReactiveMotionDiv`
//! - Reduce the number of animated properties
//! - Use simpler easing functions
//! - Avoid complex spring configurations
//!
//! **Right-click not working:**
//! - This should not happen with `MinimalMotionDiv`
//! - If it does, check for CSS `pointer-events` conflicts
//! - Verify no parent elements are blocking events
//!
//! ## Changelog
//!
//! ### v0.8.1
//! - ✅ Initial release of `MinimalMotionDiv`
//! - ✅ Comprehensive test suite
//! - ✅ Performance optimizations
//! - ✅ Right-click compatibility
//! - ✅ Memory leak prevention
//!
//! ## License
//!
//! This component is part of the leptos-motion library and is licensed under the same terms.
