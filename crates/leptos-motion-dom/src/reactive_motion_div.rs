//! Reactive MotionDiv Component - WORKING VERSION
//!
//! This module provides a working ReactiveMotionDiv component that properly
//! integrates with the animation engine.

use leptos::prelude::*;
use leptos_motion_core::{AnimationTarget, Transition, AnimationValue, Easing, RepeatConfig};
use crate::animation_engine::AnimationEngine;
use std::collections::HashMap;
use std::rc::Rc;

/// Reactive MotionDiv component - WORKING with animation engine
#[component]
pub fn ReactiveMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for animation engine integration
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Initial animation state
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    /// Target animation state (reactive)
    #[prop(optional)]
    animate: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Function-based target animation state
    #[prop(optional)]
    animate_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Hover animation state
    #[prop(optional)]
    _while_hover: Option<AnimationTarget>,
    /// Function-based hover animation state
    #[prop(optional)]
    _while_hover_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Tap animation state
    #[prop(optional)]
    _while_tap: Option<AnimationTarget>,
    /// Function-based tap animation state
    #[prop(optional)]
    _while_tap_fn: Option<Box<dyn Fn() -> AnimationTarget>>,
    /// Layout animation enabled
    #[prop(optional)]
    _layout: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    // Create node reference if not provided
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());

    // Create animation engine
    let animation_engine = Rc::new(std::cell::RefCell::new(AnimationEngine::new()));
    
    // Create reactive styles signal
    let (current_styles, set_styles) = signal(HashMap::<String, String>::new());
    
    // Set up animation engine callbacks
    let set_styles_clone = set_styles;
    let on_update = Rc::new(move |values: &HashMap<String, f64>| {
        let mut styles = HashMap::new();
        let mut transform_parts = Vec::new();
        
        for (key, value) in values {
            // Convert numeric values to appropriate CSS properties
            match key.as_str() {
                "opacity" => {
                    styles.insert("opacity".to_string(), format!("{}", value));
                }
                "scale" => {
                    transform_parts.push(format!("scale({})", value));
                }
                "x" => {
                    transform_parts.push(format!("translateX({}px)", value));
                }
                "y" => {
                    transform_parts.push(format!("translateY({}px)", value));
                }
                "rotation" => {
                    transform_parts.push(format!("rotate({}deg)", value));
                }
                _ => {
                    styles.insert(key.clone(), format!("{}", value));
                }
            }
        }
        
        // Combine all transform properties into a single transform
        if !transform_parts.is_empty() {
            let transform_string = transform_parts.join(" ");
            styles.insert("transform".to_string(), transform_string);
        }
        
        // Remove CSS transition to avoid conflicts with animation engine
        // The animation engine handles the smooth transitions
        
        set_styles_clone.set(styles);
    });
    
    // Use the animation engine for smooth animations
    animation_engine.borrow_mut().on_update(move |values| {
        on_update(values);
    });
    
    // Apply initial styles and store initial values for animation
    let mut initial_values = HashMap::new();
    if let Some(initial_target) = initial {
        let mut styles = HashMap::new();
        for (key, value) in initial_target.iter() {
            match value {
                AnimationValue::Number(num) => {
                    initial_values.insert(key.clone(), *num);
                    styles.insert(key.clone(), format!("{}", num));
                }
                AnimationValue::Pixels(num) => {
                    initial_values.insert(key.clone(), *num);
                    styles.insert(key.clone(), format!("{}px", num));
                }
                AnimationValue::Percentage(num) => {
                    initial_values.insert(key.clone(), *num);
                    styles.insert(key.clone(), format!("{}%", num));
                }
                AnimationValue::Degrees(num) => {
                    initial_values.insert(key.clone(), *num);
                    styles.insert(key.clone(), format!("{}deg", num));
                }
                AnimationValue::Radians(num) => {
                    initial_values.insert(key.clone(), *num);
                    styles.insert(key.clone(), format!("{}rad", num));
                }
                _ => {
                    styles.insert(key.clone(), value.to_string_value());
                }
            }
        }
        set_styles.set(styles);
    }
    
    // Store transition config for later use
    let _transition_config = transition.clone();
    let transition_config_clone = transition.clone();
    let animation_engine_clone = animation_engine.clone();
    let initial_values_clone = initial_values.clone();
    
    // Set up reactive animation
    Effect::new(move |_| {
        if let Some(animate_fn) = &animate {
            let animate_target = animate_fn();
            let mut animations = HashMap::new();
            for (key, value) in animate_target.iter() {
                match value {
                    AnimationValue::Number(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Pixels(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Percentage(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Degrees(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Radians(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::String(_) => {
                        // For string values, we'll need to parse them
                        // For now, skip complex string animations
                    }
                    AnimationValue::Color(_) => {
                        // For color values, we'll need to parse them
                        // For now, skip complex color animations
                    }
                    AnimationValue::Transform(_) => {
                        // For transform values, we'll need to parse them
                        // For now, skip complex transform animations
                    }
                    AnimationValue::Complex(_) => {
                        // For complex values, we'll need to parse them
                        // For now, skip complex animations
                    }
                }
            }
            
            if !animations.is_empty() {
                // Use the animation engine for smooth animations
                let default_transition = Transition {
                    duration: Some(0.3),
                    delay: None,
                    ease: Easing::EaseOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                };
                
                for (property, value) in animations {
                    let transition = transition_config_clone.clone().unwrap_or(default_transition.clone());
                    let initial_value = initial_values_clone.get(&property).copied().unwrap_or(0.0);
                    animation_engine_clone.borrow_mut().animate_property(
                        property,
                        initial_value,
                        value,
                        transition,
                    );
                }
            }
        }
        
        // Apply function-based animate styles
        if let Some(animate_function) = &animate_fn {
            let animate_values = animate_function();
            let mut animations = HashMap::new();
            for (key, value) in animate_values.iter() {
                match value {
                    AnimationValue::Number(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Pixels(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Percentage(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Degrees(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::Radians(num) => {
                        animations.insert(key.clone(), *num);
                    }
                    AnimationValue::String(_) => {
                        // Skip complex string animations for now
                    }
                    AnimationValue::Color(_) => {
                        // Skip complex color animations for now
                    }
                    AnimationValue::Transform(_) => {
                        // Skip complex transform animations for now
                    }
                    AnimationValue::Complex(_) => {
                        // Skip complex animations for now
                    }
                }
            }
            
            if !animations.is_empty() {
                let default_transition = Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseInOut,
                    delay: Some(0.0),
                    repeat: RepeatConfig::Never,
                    stagger: None,
                };
                
                for (property, value) in animations {
                    let transition = transition_config_clone.clone().unwrap_or(default_transition.clone());
                    let initial_value = initial_values_clone.get(&property).copied().unwrap_or(0.0);
                    animation_engine.borrow_mut().animate_property(
                        property,
                        initial_value, // Use stored initial value
                        value,
                        transition,
                    );
                }
            }
        }
    });
    
    // Build final style string as a reactive signal
    let final_style_signal = move || {
        let mut style_parts = Vec::new();
        let styles = current_styles.get();
        
        
        for (key, value) in styles.iter() {
            style_parts.push(format!("{}: {}", key, value));
        }
        
        // Add custom styles
        if let Some(custom_style) = &style {
            style_parts.push(custom_style.clone());
        }

        let final_style_string = style_parts.join("; ");
        
        
        final_style_string
    };

    view! {
        <div
            node_ref=node_ref
            class=class
            style=final_style_signal
        >
            {children()}
        </div>
    }
}