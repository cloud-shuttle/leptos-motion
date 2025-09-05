//! Simplified Event Handling System
//! 
//! This module provides a simplified, user-friendly event handling API
//! that removes the complex event system and provides a clean interface.

use crate::*;

/// Simplified motion props that removes complex event handling
/// 
/// This is the main public API for motion component props. It provides
/// a clean, simple interface while hiding the complexity of the
/// underlying event system.
#[derive(Clone, Debug)]
pub struct SimplifiedMotionProps {
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
    pub drag: Option<SimplifiedDragConfig>,
    /// Hover animation state
    pub while_hover: Option<AnimationTarget>,
    /// Tap animation state
    pub while_tap: Option<AnimationTarget>,
    /// Focus animation state
    pub while_focus: Option<AnimationTarget>,
    /// In-view animation state
    pub while_in_view: Option<AnimationTarget>,
}

/// Simplified drag configuration
#[derive(Clone, Debug)]
pub struct SimplifiedDragConfig {
    /// Drag axis constraint
    pub axis: DragAxis,
    /// Drag constraints
    pub constraints: Option<DragConstraints>,
    /// Elastic behavior
    pub elastic: f64,
    /// Momentum enabled
    pub momentum: bool,
}

impl SimplifiedMotionProps {
    /// Create new simplified motion props
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set initial animation state
    pub fn initial(mut self, initial: AnimationTarget) -> Self {
        self.initial = Some(initial);
        self
    }
    
    /// Set target animation state
    pub fn animate(mut self, animate: AnimationTarget) -> Self {
        self.animate = Some(animate);
        self
    }
    
    /// Set exit animation state
    pub fn exit(mut self, exit: AnimationTarget) -> Self {
        self.exit = Some(exit);
        self
    }
    
    /// Set transition configuration
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }
    
    /// Set animation variants
    pub fn variants(mut self, variants: Variants) -> Self {
        self.variants = Some(variants);
        self
    }
    
    /// Set layout animation
    pub fn layout(mut self, layout: bool) -> Self {
        self.layout = Some(layout);
        self
    }
    
    /// Set drag configuration
    pub fn drag(mut self, drag: SimplifiedDragConfig) -> Self {
        self.drag = Some(drag);
        self
    }
    
    /// Set hover animation state
    pub fn while_hover(mut self, while_hover: AnimationTarget) -> Self {
        self.while_hover = Some(while_hover);
        self
    }
    
    /// Set tap animation state
    pub fn while_tap(mut self, while_tap: AnimationTarget) -> Self {
        self.while_tap = Some(while_tap);
        self
    }
    
    /// Set focus animation state
    pub fn while_focus(mut self, while_focus: AnimationTarget) -> Self {
        self.while_focus = Some(while_focus);
        self
    }
    
    /// Set in-view animation state
    pub fn while_in_view(mut self, while_in_view: AnimationTarget) -> Self {
        self.while_in_view = Some(while_in_view);
        self
    }
    
    /// Check if any animation is configured
    pub fn has_animations(&self) -> bool {
        self.animate.is_some() || 
        self.while_hover.is_some() || 
        self.while_tap.is_some() || 
        self.while_focus.is_some() || 
        self.while_in_view.is_some()
    }
    
    /// Check if drag is configured
    pub fn has_drag(&self) -> bool {
        self.drag.is_some()
    }
    
    /// Check if layout animation is enabled
    pub fn has_layout(&self) -> bool {
        self.layout.unwrap_or(false)
    }
    
    /// Get the number of configured animations
    pub fn animation_count(&self) -> usize {
        let mut count = 0;
        if self.animate.is_some() { count += 1; }
        if self.while_hover.is_some() { count += 1; }
        if self.while_tap.is_some() { count += 1; }
        if self.while_focus.is_some() { count += 1; }
        if self.while_in_view.is_some() { count += 1; }
        count
    }
}

impl SimplifiedDragConfig {
    /// Create new simplified drag config
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set drag axis
    pub fn axis(mut self, axis: DragAxis) -> Self {
        self.axis = axis;
        self
    }
    
    /// Set drag constraints
    pub fn constraints(mut self, constraints: DragConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }
    
    /// Set elastic behavior
    pub fn elastic(mut self, elastic: f64) -> Self {
        self.elastic = elastic;
        self
    }
    
    /// Set momentum enabled
    pub fn momentum(mut self, momentum: bool) -> Self {
        self.momentum = momentum;
        self
    }
    
    /// Check if constraints are configured
    pub fn has_constraints(&self) -> bool {
        self.constraints.is_some()
    }
    
    /// Check if elastic behavior is enabled
    pub fn has_elastic(&self) -> bool {
        self.elastic > 0.0
    }
}

impl Default for SimplifiedMotionProps {
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

impl Default for SimplifiedDragConfig {
    fn default() -> Self {
        Self {
            axis: DragAxis::Both,
            constraints: None,
            elastic: 0.0,
            momentum: false,
        }
    }
}

/// Conversion from complex MotionProps to simplified MotionProps
impl From<MotionProps> for SimplifiedMotionProps {
    fn from(props: MotionProps) -> Self {
        Self {
            initial: props.initial,
            animate: props.animate,
            exit: props.exit,
            transition: props.transition,
            variants: props.variants,
            layout: props.layout,
            drag: props.drag.map(|d| SimplifiedDragConfig {
                axis: d.axis.unwrap_or(DragAxis::Both),
                constraints: d.constraints,
                elastic: d.elastic,
                momentum: d.momentum,
            }),
            while_hover: props.while_hover,
            while_tap: props.while_tap,
            while_focus: props.while_focus,
            while_in_view: props.while_in_view,
        }
    }
}

/// Conversion from complex DragConfig to simplified DragConfig
impl From<DragConfig> for SimplifiedDragConfig {
    fn from(config: DragConfig) -> Self {
        Self {
            axis: config.axis.unwrap_or(DragAxis::Both),
            constraints: config.constraints,
            elastic: config.elastic,
            momentum: config.momentum,
        }
    }
}

/// Conversion from simplified MotionProps to complex MotionProps
impl From<SimplifiedMotionProps> for MotionProps {
    fn from(props: SimplifiedMotionProps) -> Self {
        Self {
            initial: props.initial,
            animate: props.animate,
            exit: props.exit,
            transition: props.transition,
            variants: props.variants,
            layout: props.layout,
            drag: props.drag.map(|d| DragConfig {
                axis: Some(d.axis),
                constraints: d.constraints,
                elastic: d.elastic,
                momentum: d.momentum,
            }),
            while_hover: props.while_hover,
            while_tap: props.while_tap,
            while_focus: props.while_focus,
            while_in_view: props.while_in_view,
            event_handlers: None, // Simplified version doesn't use complex event handlers
        }
    }
}

/// Conversion from simplified DragConfig to complex DragConfig
impl From<SimplifiedDragConfig> for DragConfig {
    fn from(config: SimplifiedDragConfig) -> Self {
        Self {
            axis: Some(config.axis),
            constraints: config.constraints,
            elastic: config.elastic,
            momentum: config.momentum,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simplified_motion_props_creation() {
        let props = SimplifiedMotionProps::new();
        assert!(props.initial.is_none());
        assert!(props.animate.is_none());
        assert!(!props.has_animations());
        assert!(!props.has_drag());
        assert!(!props.has_layout());
        assert_eq!(props.animation_count(), 0);
    }
    
    #[test]
    fn test_simplified_motion_props_fluent_api() {
        let target = std::collections::HashMap::new();
        let props = SimplifiedMotionProps::new()
            .animate(target.clone())
            .layout(true);
        
        assert!(props.animate.is_some());
        assert!(props.has_animations());
        assert!(props.has_layout());
        assert_eq!(props.animation_count(), 1);
    }
    
    #[test]
    fn test_simplified_drag_config_creation() {
        let drag_config = SimplifiedDragConfig::new();
        assert_eq!(drag_config.axis, DragAxis::Both);
        assert!(!drag_config.has_constraints());
        assert!(!drag_config.has_elastic());
    }
    
    #[test]
    fn test_simplified_drag_config_fluent_api() {
        let drag_config = SimplifiedDragConfig::new()
            .axis(DragAxis::X)
            .elastic(0.3)
            .momentum(true);
        
        assert_eq!(drag_config.axis, DragAxis::X);
        assert!(drag_config.has_elastic());
        assert!(drag_config.momentum);
    }
    
    #[test]
    fn test_conversion_from_complex_motion_props() {
        let complex_props = MotionProps {
            initial: None,
            animate: None,
            exit: None,
            transition: None,
            variants: None,
            layout: Some(true),
            drag: Some(DragConfig {
                axis: Some(DragAxis::X),
                constraints: None,
                elastic: 0.2,
                momentum: true,
            }),
            while_hover: None,
            while_tap: None,
            while_focus: None,
            while_in_view: None,
            event_handlers: None,
        };
        
        let simplified_props = SimplifiedMotionProps::from(complex_props);
        assert!(simplified_props.has_layout());
        assert!(simplified_props.has_drag());
        
        let drag = simplified_props.drag.unwrap();
        assert_eq!(drag.axis, DragAxis::X);
        assert_eq!(drag.elastic, 0.2);
        assert!(drag.momentum);
    }
    
    #[test]
    fn test_conversion_to_complex_motion_props() {
        let simplified_props = SimplifiedMotionProps::new()
            .layout(true)
            .drag(SimplifiedDragConfig::new().axis(DragAxis::Y));
        
        let complex_props = MotionProps::from(simplified_props);
        assert!(complex_props.layout.unwrap());
        assert!(complex_props.drag.is_some());
        assert!(complex_props.event_handlers.is_none()); // Simplified version doesn't use complex event handlers
    }
}
