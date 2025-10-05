//! MotionPath Component
//!
//! Specialized component for animating SVG path elements with path drawing effects.
//! Extends EventDrivenMotionDiv functionality with automatic path length calculation and
//! stroke-dashoffset animation support.

use leptos::prelude::*;
use leptos_motion_core::*;
use crate::{
    AnimationType,
    Keyframe,
    EventStaggerConfig,
    EventSpringConfig,
    AnimationValue,
    Transition,
    AnimateProp,
    resolve_animate_prop,
    event_driven_motion_div::{DragConstraints, DragConfig},
};
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{SvgPathElement, Element};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Thread-local cache for SVG path lengths to avoid recalculating
thread_local! {
    static PATH_LENGTH_CACHE: std::cell::RefCell<HashMap<u64, f64>> = std::cell::RefCell::new(HashMap::new());
}

/// Calculate hash for path data to use as cache key
fn calculate_path_hash(path_data: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    path_data.hash(&mut hasher);
    hasher.finish()
}

/// Calculate path length with caching and error handling
fn calculate_path_length_with_cache(path_data: &str) -> Option<f64> {
    let hash = calculate_path_hash(path_data);

    // Check cache first
    let cached_result = PATH_LENGTH_CACHE.with(|cache| {
        cache.borrow().get(&hash).copied()
    });

    if let Some(length) = cached_result {
        return Some(length);
    }

    // Calculate length using a temporary SVG path element
    if let Some(length) = calculate_path_length_from_data(path_data) {
        // Cache the result
        PATH_LENGTH_CACHE.with(|cache| {
            cache.borrow_mut().insert(hash, length);
        });
        Some(length)
    } else {
        None
    }
}

/// Calculate path length from SVG path data using web_sys
fn calculate_path_length_from_data(path_data: &str) -> Option<f64> {
    // Create a temporary SVG element to calculate path length
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| {
            // Create a temporary SVG element (must be in DOM for getTotalLength to work)
            document.create_element_ns(Some("http://www.w3.org/2000/svg"), "svg").ok()
        })
        .and_then(|svg_elem| {
            svg_elem.dyn_into::<web_sys::SvgsvgElement>().ok()
        })
        .and_then(|svg| {
            // Set SVG attributes to ensure proper rendering context
            let _ = svg.set_attribute("width", "100");
            let _ = svg.set_attribute("height", "100");

            // Create a path element
            web_sys::window()
                .and_then(|window| window.document())
                .and_then(|document| {
                    document.create_element_ns(Some("http://www.w3.org/2000/svg"), "path").ok()
                })
                .and_then(|path_elem| {
                    path_elem.dyn_into::<web_sys::SvgPathElement>().ok()
                })
                .and_then(|path| {
                    // Set the path data
                    path.set_attribute("d", path_data).ok()?;

                    // Append path to SVG (required for getTotalLength to work)
                    svg.append_child(&path).ok()?;

                    // Append SVG to document body temporarily (required for getTotalLength)
                    web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| {
                            let body = document.body()?;
                            body.append_child(&svg).ok()
                        })?;

                    // Calculate the length
                    let length = path.get_total_length() as f64;

                    // Clean up - remove the temporary SVG from DOM
                    web_sys::window()
                        .and_then(|window| window.document())
                        .and_then(|document| {
                            let body = document.body()?;
                            body.remove_child(&svg).ok()
                        })?;

                    Some(length)
                })
        })
}

/// MotionPath component for SVG path drawing animations
#[component]
pub fn MotionPath(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,

    /// Target animation values (reactive support)
    #[prop(optional)]
    animate: Option<AnimateProp>,

    /// Animation while hovering
    #[prop(optional)]
    while_hover: Option<HashMap<String, AnimationValue>>,

    /// Animation while tapping
    #[prop(optional)]
    while_tap: Option<HashMap<String, AnimationValue>>,

    /// Animation while dragging
    #[prop(optional)]
    while_drag: Option<HashMap<String, AnimationValue>>,

    /// Transition configuration
    #[prop(optional)]
    _transition: Option<Transition>,

    /// Animation type (css, keyframe, stagger, spring)
    #[prop(optional, default = AnimationType::Css)]
    animation_type: AnimationType,

    /// Keyframes for keyframe animations
    #[prop(optional)]
    keyframes: Option<Vec<Keyframe>>,

    /// Stagger configuration
    #[prop(optional)]
    stagger_config: Option<EventStaggerConfig>,

    /// Spring configuration
    #[prop(optional)]
    spring_config: Option<EventSpringConfig>,

    /// Drag constraints
    #[prop(optional)]
    drag_constraints: Option<DragConstraints>,

    /// Whether element is draggable
    #[prop(optional)]
    drag: Option<DragConfig>,

    /// Layout animation
    #[prop(optional, default = false)]
    layout: bool,

    /// CSS classes
    #[prop(optional)]
    class: Option<String>,

    /// Inline styles
    #[prop(optional)]
    style: Option<String>,

    /// Path data (d attribute)
    #[prop(optional)]
    d: Option<String>,

    /// Stroke color
    #[prop(optional)]
    stroke: Option<String>,

    /// Stroke width
    #[prop(optional)]
    stroke_width: Option<String>,

    /// Fill color
    #[prop(optional)]
    fill: Option<String>,

    /// Stroke linecap
    #[prop(optional)]
    stroke_linecap: Option<String>,

    /// Stroke dash array (for manual control)
    #[prop(optional)]
    stroke_dasharray: Option<String>,

    /// Stroke dash offset (for manual control)
    #[prop(optional)]
    stroke_dashoffset: Option<String>,

    /// Additional attributes (not implemented yet)
    // #[prop(attrs)]
    // attrs: Vec<(&'static str, Attribute)>,

    /// Child elements
    children: Children,
) -> impl IntoView {
    // Node reference for the path element to calculate path length
    let path_ref = NodeRef::new();

    // Calculate path length and set up initial stroke-dashoffset
    let path_length = RwSignal::new(None::<f64>);
    let dash_offset = RwSignal::new(None::<f64>);

    // Create a signal for path data to avoid moving the prop
    let path_data_signal = RwSignal::new(d.clone());

    // Calculate path length when path data is available
    Effect::new(move |_| {
        if let Some(ref path_data) = path_data_signal.get() {
            if let Some(length) = calculate_path_length_with_cache(path_data) {
                path_length.set(Some(length));
                dash_offset.set(Some(length)); // Start with full dash offset (invisible)
            } else {
                // Fallback to default length if calculation fails
                leptos::logging::warn!("Failed to calculate path length for: {}", path_data);
                path_length.set(Some(1000.0));
                dash_offset.set(Some(1000.0));
            }
        }
    });

    // Prepare initial values with path drawing setup
    let initial_values = initial.unwrap_or_default();

    // Create the SVG path element with motion capabilities
    view! {
        <path
            node_ref=path_ref
            class=class
            style=move || {
                let mut base_style = style.clone().unwrap_or_default();

                // For path drawing animation, set up stroke-dasharray if calculated
                // Let the animation system handle stroke-dashoffset through initial/animate props
                if let Some(length) = path_length.get() {
                    if !base_style.contains("stroke-dasharray") {
                        if !base_style.is_empty() { base_style.push(';'); }
                        base_style.push_str(&format!("stroke-dasharray: {}", length));
                    }
                }

                base_style
            }
            d=d
            stroke=stroke
            stroke-width=stroke_width
            fill=fill
            stroke-linecap=stroke_linecap
            stroke-dasharray=stroke_dasharray
            stroke-dashoffset=stroke_dashoffset
        >
            {children()}
        </path>
    }
}
