//! Core motion components for Leptos

use leptos::prelude::*;
use leptos_motion_core::*;

/// Props for motion components
#[derive(Clone, Debug)]
pub struct MotionProps {
    /// Initial animation state
    pub initial: Option<AnimationTarget>,
    /// Target animation state
    pub animate: Option<AnimationTarget>,
    /// Exit animation state  
    pub exit: Option<AnimationTarget>,
    /// Transition configuration
    pub transition: Option<Transition>,
    /// Animation variants
    pub variants: Option<Variants>,
    /// Layout animation enabled
    pub layout: Option<bool>,
    /// Drag configuration
    pub drag: Option<DragConfig>,
    /// Hover animation state
    pub while_hover: Option<AnimationTarget>,
    /// Tap animation state
    pub while_tap: Option<AnimationTarget>,
    /// Focus animation state
    pub while_focus: Option<AnimationTarget>,
    /// In-view animation state
    pub while_in_view: Option<AnimationTarget>,
}

impl Default for MotionProps {
    fn default() -> Self {
        Self {
            initial: None,
            animate: None,
            exit: None,
            transition: None,
            variants: None,
            layout: None,
            drag: None,
            while_hover: None,
            while_tap: None,
            while_focus: None,
            while_in_view: None,
        }
    }
}

/// Drag configuration
#[derive(Clone, Debug)]
pub struct DragConfig {
    /// Drag axis constraint
    pub axis: Option<DragAxis>,
    /// Drag constraints
    pub constraints: Option<DragConstraints>,
    /// Elastic behavior
    pub elastic: f64,
    /// Momentum enabled
    pub momentum: bool,
}

/// Drag axis constraint
#[derive(Clone, Debug, PartialEq)]
pub enum DragAxis {
    /// Horizontal only
    X,
    /// Vertical only  
    Y,
    /// Both axes
    Both,
}

/// Drag constraints
#[derive(Clone, Debug)]
pub struct DragConstraints {
    /// Left boundary
    pub left: Option<f64>,
    /// Right boundary
    pub right: Option<f64>,
    /// Top boundary
    pub top: Option<f64>,
    /// Bottom boundary
    pub bottom: Option<f64>,
}

impl DragConfig {
    /// Create a new drag config
    pub fn new() -> Self {
        Self {
            axis: Some(DragAxis::Both),
            constraints: None,
            elastic: 0.5,
            momentum: true,
        }
    }
    
    /// Set drag axis
    pub fn axis(mut self, axis: DragAxis) -> Self {
        self.axis = Some(axis);
        self
    }
    
    /// Set drag constraints
    pub fn constraints(mut self, constraints: DragConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }
}

impl Default for DragConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Core motion div component
#[component]
pub fn MotionDiv(
    #[prop(optional)] initial: Option<AnimationTarget>,
    #[prop(optional)] animate: Option<AnimationTarget>, 
    #[prop(optional)] exit: Option<AnimationTarget>,
    #[prop(optional)] transition: Option<Transition>,
    #[prop(optional)] variants: Option<Variants>,
    #[prop(optional)] layout: Option<bool>,
    #[prop(optional)] drag: Option<DragConfig>,
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    #[prop(optional)] while_focus: Option<AnimationTarget>,
    #[prop(optional)] while_in_view: Option<AnimationTarget>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    let motion_props = MotionProps {
        initial,
        animate,
        exit,
        transition,
        variants,
        layout,
        drag,
        while_hover,
        while_tap,
        while_focus,
        while_in_view,
    };
    
    create_motion_div(motion_props, class, style, id, children)
}

/// Core motion span component
#[component]
pub fn MotionSpan(
    #[prop(optional)] initial: Option<AnimationTarget>,
    #[prop(optional)] animate: Option<AnimationTarget>,
    #[prop(optional)] exit: Option<AnimationTarget>,
    #[prop(optional)] transition: Option<Transition>,
    #[prop(optional)] variants: Option<Variants>,
    #[prop(optional)] layout: Option<bool>,
    #[prop(optional)] drag: Option<DragConfig>,
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    #[prop(optional)] while_focus: Option<AnimationTarget>,
    #[prop(optional)] while_in_view: Option<AnimationTarget>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] id: Option<String>,
    children: Children,
) -> impl IntoView {
    let motion_props = MotionProps {
        initial,
        animate,
        exit,
        transition,
        variants,
        layout,
        drag,
        while_hover,
        while_tap,
        while_focus,
        while_in_view,
    };
    
    create_motion_span(motion_props, class, style, id, children)
}

/// Create a motion div element
fn create_motion_div(
    motion_props: MotionProps,
    class: Option<String>,
    style: Option<String>,
    _id: Option<String>,
    children: Children,
) -> impl IntoView {
    let element_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    
    // Create motion state
    let motion_state = RwSignal::new(MotionState::new(motion_props.clone()));
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            let state = motion_state.get();
            apply_animation_styles(&element, &state.current_values);
        }
    });
    
    // Handle animations
    Effect::new(move |_| {
        if let Some(animate) = &motion_props.animate {
            if let Some(element) = element_ref.get() {
                start_animation(&element, animate, &motion_props.transition);
            }
        }
    });
    
    view! {
        <div
            node_ref=element_ref
            class=class
            style=style
        >
            {children()}
        </div>
    }.into_view()
}

/// Create a motion span element
fn create_motion_span(
    motion_props: MotionProps,
    class: Option<String>,
    style: Option<String>,
    _id: Option<String>,
    children: Children,
) -> impl IntoView {
    let element_ref: NodeRef<leptos::html::Span> = NodeRef::new();
    
    // Create motion state
    let motion_state = RwSignal::new(MotionState::new(motion_props.clone()));
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = element_ref.get() {
            let state = motion_state.get();
            apply_animation_styles(&element, &state.current_values);
        }
    });
    
    // Handle animations
    Effect::new(move |_| {
        if let Some(animate) = &motion_props.animate {
            if let Some(element) = element_ref.get() {
                start_animation(&element, animate, &motion_props.transition);
            }
        }
    });
    
    view! {
        <span
            class=class
            style=style
        >
            {children()}
        </span>
    }.into_view()
}

/// Internal motion state
#[derive(Clone, Debug)]
struct MotionState {
    props: MotionProps,
    current_values: AnimationTarget,
    is_animating: bool,
}

impl MotionState {
    fn new(props: MotionProps) -> Self {
        let current_values = props.initial.clone().unwrap_or_default();
        
        Self {
            props,
            current_values,
            is_animating: false,
        }
    }
}

/// Apply animation styles to element
fn apply_animation_styles(_element: &web_sys::Element, _values: &AnimationTarget) {
    // Placeholder implementation
    // log::info!("Applying animation styles: {:?}", values);
}

/// Start animation on element (placeholder implementation)
fn start_animation(
    element: &web_sys::Element,
    target: &AnimationTarget,
    _transition: &Option<Transition>,
) {
    // This is a simplified implementation
    // A full implementation would use the animation engine
    // log::info!("Starting animation on element: {:?}", target);
    
    // For now, just apply the target styles directly
    apply_animation_styles(element, target);
}

/// Convenience macro for creating animation targets
#[macro_export]
macro_rules! motion_target {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut target = std::collections::HashMap::new();
            $(
                target.insert($key.to_string(), $value);
            )*
            target
        }
    };
}

/// Helper functions for common animation patterns
pub mod helpers {
    use super::*;
    
    /// Create fade in animation target
    pub fn fade_in() -> AnimationTarget {
        motion_target!("opacity" => AnimationValue::Number(1.0))
    }
    
    /// Create fade out animation target
    pub fn fade_out() -> AnimationTarget {
        motion_target!("opacity" => AnimationValue::Number(0.0))
    }
    
    /// Create slide up animation target
    pub fn slide_up(distance: f64) -> AnimationTarget {
        motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "y" => AnimationValue::Pixels(-distance)
        )
    }
    
    /// Create scale animation target
    pub fn scale(factor: f64) -> AnimationTarget {
        motion_target!("scale" => AnimationValue::Number(factor))
    }
    
    /// Create rotation animation target
    pub fn rotate(degrees: f64) -> AnimationTarget {
        motion_target!("rotate" => AnimationValue::Degrees(degrees))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_motion_props_default() {
        let props = MotionProps::default();
        assert!(props.initial.is_none());
        assert!(props.animate.is_none());
        assert!(props.layout.is_none());
    }
    
    #[test] 
    fn test_drag_config() {
        let drag = DragConfig::new()
            .axis(DragAxis::X)
            .constraints(DragConstraints {
                left: Some(-100.0),
                right: Some(100.0),
                top: None,
                bottom: None,
            });
            
        assert_eq!(drag.axis, Some(DragAxis::X));
        assert!(drag.constraints.is_some());
    }
    
    #[test]
    fn test_helper_functions() {
        let fade = helpers::fade_in();
        assert_eq!(fade.get("opacity"), Some(&AnimationValue::Number(1.0)));
        
        let slide = helpers::slide_up(20.0);
        assert_eq!(slide.get("y"), Some(&AnimationValue::Pixels(-20.0)));
        
        let scale = helpers::scale(2.0);
        assert_eq!(scale.get("scale"), Some(&AnimationValue::Number(2.0)));
    }
}