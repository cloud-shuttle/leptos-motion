//! Transform Animations for Leptos Motion
//!
//! This module provides comprehensive transform animation support including:
//! - 2D and 3D transforms
//! - Independent property animation
//! - Transform composition
//! - CSS transform string generation

use leptos_motion_core::*;
use std::collections::HashMap;

/// 2D Transform properties
#[derive(Debug, Clone, PartialEq)]
pub struct Transform2D {
    /// X translation
    pub translate_x: f64,
    /// Y translation
    pub translate_y: f64,
    /// Rotation in degrees
    pub rotate: f64,
    /// X scale
    pub scale_x: f64,
    /// Y scale
    pub scale_y: f64,
    /// X skew in degrees
    pub skew_x: f64,
    /// Y skew in degrees
    pub skew_y: f64,
}

impl Default for Transform2D {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            rotate: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            skew_x: 0.0,
            skew_y: 0.0,
        }
    }
}

impl Transform2D {
    /// Create a new 2D transform
    pub fn new() -> Self {
        Self::default()
    }

    /// Set translation
    pub fn translate(mut self, x: f64, y: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self
    }

    /// Set rotation
    pub fn rotate(mut self, degrees: f64) -> Self {
        self.rotate = degrees;
        self
    }

    /// Set scale
    pub fn scale(mut self, x: f64, y: f64) -> Self {
        self.scale_x = x;
        self.scale_y = y;
        self
    }

    /// Set uniform scale
    pub fn scale_uniform(mut self, scale: f64) -> Self {
        self.scale_x = scale;
        self.scale_y = scale;
        self
    }

    /// Set skew
    pub fn skew(mut self, x: f64, y: f64) -> Self {
        self.skew_x = x;
        self.skew_y = y;
        self
    }

    /// Convert to CSS transform string
    pub fn to_css(&self) -> String {
        let mut transforms = Vec::new();

        if self.translate_x != 0.0 || self.translate_y != 0.0 {
            transforms.push(format!(
                "translate({}px, {}px)",
                self.translate_x, self.translate_y
            ));
        }

        if self.rotate != 0.0 {
            transforms.push(format!("rotate({}deg)", self.rotate));
        }

        if self.scale_x != 1.0 || self.scale_y != 1.0 {
            transforms.push(format!("scale({}, {})", self.scale_x, self.scale_y));
        }

        if self.skew_x != 0.0 || self.skew_y != 0.0 {
            transforms.push(format!("skew({}deg, {}deg)", self.skew_x, self.skew_y));
        }

        transforms.join(" ")
    }

    /// Interpolate between two transforms
    pub fn interpolate(&self, other: &Transform2D, progress: f64) -> Transform2D {
        Transform2D {
            translate_x: self.translate_x + (other.translate_x - self.translate_x) * progress,
            translate_y: self.translate_y + (other.translate_y - self.translate_y) * progress,
            rotate: self.rotate + (other.rotate - self.rotate) * progress,
            scale_x: self.scale_x + (other.scale_x - self.scale_x) * progress,
            scale_y: self.scale_y + (other.scale_y - self.scale_y) * progress,
            skew_x: self.skew_x + (other.skew_x - self.skew_x) * progress,
            skew_y: self.skew_y + (other.skew_y - self.skew_y) * progress,
        }
    }
}

/// 3D Transform properties
#[derive(Debug, Clone, PartialEq)]
pub struct Transform3D {
    /// X translation
    pub translate_x: f64,
    /// Y translation
    pub translate_y: f64,
    /// Z translation
    pub translate_z: f64,
    /// X rotation in degrees
    pub rotate_x: f64,
    /// Y rotation in degrees
    pub rotate_y: f64,
    /// Z rotation in degrees
    pub rotate_z: f64,
    /// X scale
    pub scale_x: f64,
    /// Y scale
    pub scale_y: f64,
    /// Z scale
    pub scale_z: f64,
    /// Perspective
    pub perspective: f64,
}

impl Default for Transform3D {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            translate_z: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
            rotate_z: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            scale_z: 1.0,
            perspective: 0.0,
        }
    }
}

impl Transform3D {
    /// Create a new 3D transform
    pub fn new() -> Self {
        Self::default()
    }

    /// Set 3D translation
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.translate_x = x;
        self.translate_y = y;
        self.translate_z = z;
        self
    }

    /// Set 3D rotation
    pub fn rotate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rotate_x = x;
        self.rotate_y = y;
        self.rotate_z = z;
        self
    }

    /// Set 3D scale
    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.scale_x = x;
        self.scale_y = y;
        self.scale_z = z;
        self
    }

    /// Set uniform 3D scale
    pub fn scale_uniform(mut self, scale: f64) -> Self {
        self.scale_x = scale;
        self.scale_y = scale;
        self.scale_z = scale;
        self
    }

    /// Set perspective
    pub fn perspective(mut self, value: f64) -> Self {
        self.perspective = value;
        self
    }

    /// Convert to CSS transform string
    pub fn to_css(&self) -> String {
        let mut transforms = Vec::new();

        if self.perspective != 0.0 {
            transforms.push(format!("perspective({}px)", self.perspective));
        }

        if self.translate_x != 0.0 || self.translate_y != 0.0 || self.translate_z != 0.0 {
            transforms.push(format!(
                "translate3d({}px, {}px, {}px)",
                self.translate_x, self.translate_y, self.translate_z
            ));
        }

        if self.rotate_x != 0.0 {
            transforms.push(format!("rotateX({}deg)", self.rotate_x));
        }

        if self.rotate_y != 0.0 {
            transforms.push(format!("rotateY({}deg)", self.rotate_y));
        }

        if self.rotate_z != 0.0 {
            transforms.push(format!("rotateZ({}deg)", self.rotate_z));
        }

        if self.scale_x != 1.0 || self.scale_y != 1.0 || self.scale_z != 1.0 {
            transforms.push(format!(
                "scale3d({}, {}, {})",
                self.scale_x, self.scale_y, self.scale_z
            ));
        }

        transforms.join(" ")
    }

    /// Interpolate between two 3D transforms
    pub fn interpolate(&self, other: &Transform3D, progress: f64) -> Transform3D {
        Transform3D {
            translate_x: self.translate_x + (other.translate_x - self.translate_x) * progress,
            translate_y: self.translate_y + (other.translate_y - self.translate_y) * progress,
            translate_z: self.translate_z + (other.translate_z - self.translate_z) * progress,
            rotate_x: self.rotate_x + (other.rotate_x - self.rotate_x) * progress,
            rotate_y: self.rotate_y + (other.rotate_y - self.rotate_y) * progress,
            rotate_z: self.rotate_z + (other.rotate_z - self.rotate_z) * progress,
            scale_x: self.scale_x + (other.scale_x - self.scale_x) * progress,
            scale_y: self.scale_y + (other.scale_y - self.scale_y) * progress,
            scale_z: self.scale_z + (other.scale_z - self.scale_z) * progress,
            perspective: self.perspective + (other.perspective - self.perspective) * progress,
        }
    }
}

/// Transform animation manager
pub struct TransformAnimationManager {
    /// 2D transform state
    pub transform_2d: Transform2D,
    /// 3D transform state
    pub transform_3d: Transform3D,
    /// Individual property animations
    property_animations: HashMap<String, PropertyAnimation>,
    /// Whether to use 3D transforms
    use_3d: bool,
}

/// Individual property animation state
#[derive(Debug, Clone)]
struct PropertyAnimation {
    current: f64,
    target: f64,
    #[allow(dead_code)]
    initial: f64,
    #[allow(dead_code)]
    velocity: f64,
    is_complete: bool,
}

impl TransformAnimationManager {
    /// Create a new transform animation manager
    pub fn new() -> Self {
        Self {
            transform_2d: Transform2D::default(),
            transform_3d: Transform3D::default(),
            property_animations: HashMap::new(),
            use_3d: false,
        }
    }

    /// Enable 3D transforms
    pub fn enable_3d(mut self) -> Self {
        self.use_3d = true;
        self
    }

    /// Animate a specific transform property
    pub fn animate_property(&mut self, property: &str, target: f64, _transition: &Transition) {
        let current = self.get_property_value(property);
        let animation = PropertyAnimation {
            current,
            target,
            initial: current,
            velocity: 0.0,
            is_complete: false,
        };

        self.property_animations
            .insert(property.to_string(), animation);
    }

    /// Get current value of a property
    pub fn get_property_value(&self, property: &str) -> f64 {
        match property {
            "translateX" => self.transform_2d.translate_x,
            "translateY" => self.transform_2d.translate_y,
            "translateZ" => self.transform_3d.translate_z,
            "rotate" | "rotateZ" => self.transform_2d.rotate,
            "rotateX" => self.transform_3d.rotate_x,
            "rotateY" => self.transform_3d.rotate_y,
            "scaleX" => self.transform_2d.scale_x,
            "scaleY" => self.transform_2d.scale_y,
            "scaleZ" => self.transform_3d.scale_z,
            "skewX" => self.transform_2d.skew_x,
            "skewY" => self.transform_2d.skew_y,
            "perspective" => self.transform_3d.perspective,
            _ => 0.0,
        }
    }

    /// Set a property value directly
    pub fn set_property_value(&mut self, property: &str, value: f64) {
        match property {
            "translateX" => self.transform_2d.translate_x = value,
            "translateY" => self.transform_2d.translate_y = value,
            "translateZ" => self.transform_3d.translate_z = value,
            "rotate" | "rotateZ" => self.transform_2d.rotate = value,
            "rotateX" => self.transform_3d.rotate_x = value,
            "rotateY" => self.transform_3d.rotate_y = value,
            "scaleX" => self.transform_2d.scale_x = value,
            "scaleY" => self.transform_2d.scale_y = value,
            "scaleZ" => self.transform_3d.scale_z = value,
            "skewX" => self.transform_2d.skew_x = value,
            "skewY" => self.transform_2d.skew_y = value,
            "perspective" => self.transform_3d.perspective = value,
            _ => {}
        }
    }

    /// Update animations with delta time
    pub fn update(&mut self, delta_time: f64) {
        let mut completed_properties = Vec::new();
        let mut updates = Vec::new();

        // First pass: update animations and collect updates
        for (property, animation) in &mut self.property_animations {
            if !animation.is_complete {
                // Simple linear interpolation for now
                // In a full implementation, this would use the easing functions
                let progress = (delta_time / 0.3).min(1.0); // 0.3s default duration
                animation.current += (animation.target - animation.current) * progress;

                if (animation.current - animation.target).abs() < 0.01 {
                    animation.current = animation.target;
                    animation.is_complete = true;
                }

                updates.push((property.clone(), animation.current));
            }

            if animation.is_complete {
                completed_properties.push(property.clone());
            }
        }

        // Second pass: apply updates
        for (property, value) in updates {
            self.set_property_value(&property, value);
        }

        // Remove completed animations
        for property in completed_properties {
            self.property_animations.remove(&property);
        }
    }

    /// Get the current CSS transform string
    pub fn get_css_transform(&self) -> String {
        if self.use_3d {
            self.transform_3d.to_css()
        } else {
            self.transform_2d.to_css()
        }
    }

    /// Check if any animations are active
    pub fn has_active_animations(&self) -> bool {
        !self.property_animations.is_empty()
    }

    /// Stop all animations
    pub fn stop_all(&mut self) {
        self.property_animations.clear();
    }

    /// Stop animation for a specific property
    pub fn stop_property(&mut self, property: &str) {
        self.property_animations.remove(property);
    }
}

impl Default for TransformAnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Transform animation builder for easy setup
pub struct TransformAnimationBuilder {
    manager: TransformAnimationManager,
}

impl TransformAnimationBuilder {
    /// Create a new transform animation builder
    pub fn new() -> Self {
        Self {
            manager: TransformAnimationManager::new(),
        }
    }

    /// Enable 3D transforms
    pub fn enable_3d(mut self) -> Self {
        self.manager.use_3d = true;
        self
    }

    /// Animate translation
    pub fn translate(mut self, x: f64, y: f64, transition: &Transition) -> Self {
        self.manager.animate_property("translateX", x, transition);
        self.manager.animate_property("translateY", y, transition);
        self
    }

    /// Animate 3D translation
    pub fn translate_3d(mut self, x: f64, y: f64, z: f64, transition: &Transition) -> Self {
        self.manager.animate_property("translateX", x, transition);
        self.manager.animate_property("translateY", y, transition);
        self.manager.animate_property("translateZ", z, transition);
        self
    }

    /// Animate rotation
    pub fn rotate(mut self, degrees: f64, transition: &Transition) -> Self {
        self.manager.animate_property("rotate", degrees, transition);
        self
    }

    /// Animate 3D rotation
    pub fn rotate_3d(mut self, x: f64, y: f64, z: f64, transition: &Transition) -> Self {
        self.manager.animate_property("rotateX", x, transition);
        self.manager.animate_property("rotateY", y, transition);
        self.manager.animate_property("rotateZ", z, transition);
        self
    }

    /// Animate scale
    pub fn scale(mut self, x: f64, y: f64, transition: &Transition) -> Self {
        self.manager.animate_property("scaleX", x, transition);
        self.manager.animate_property("scaleY", y, transition);
        self
    }

    /// Animate uniform scale
    pub fn scale_uniform(mut self, scale: f64, transition: &Transition) -> Self {
        self.manager.animate_property("scaleX", scale, transition);
        self.manager.animate_property("scaleY", scale, transition);
        self
    }

    /// Animate 3D scale
    pub fn scale_3d(mut self, x: f64, y: f64, z: f64, transition: &Transition) -> Self {
        self.manager.animate_property("scaleX", x, transition);
        self.manager.animate_property("scaleY", y, transition);
        self.manager.animate_property("scaleZ", z, transition);
        self
    }

    /// Animate skew
    pub fn skew(mut self, x: f64, y: f64, transition: &Transition) -> Self {
        self.manager.animate_property("skewX", x, transition);
        self.manager.animate_property("skewY", y, transition);
        self
    }

    /// Build the transform animation manager
    pub fn build(self) -> TransformAnimationManager {
        self.manager
    }
}

impl Default for TransformAnimationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common transform animations
pub mod presets {
    use super::*;
    use leptos_motion_core::Transition;

    /// Create a fade in animation
    pub fn fade_in() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().scale_uniform(0.8, &Transition::default())
    }

    /// Create a fade out animation
    pub fn fade_out() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().scale_uniform(0.0, &Transition::default())
    }

    /// Create a slide in from left animation
    pub fn slide_in_left() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().translate(-100.0, 0.0, &Transition::default())
    }

    /// Create a slide in from right animation
    pub fn slide_in_right() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().translate(100.0, 0.0, &Transition::default())
    }

    /// Create a slide in from top animation
    pub fn slide_in_top() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().translate(0.0, -100.0, &Transition::default())
    }

    /// Create a slide in from bottom animation
    pub fn slide_in_bottom() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().translate(0.0, 100.0, &Transition::default())
    }

    /// Create a bounce in animation
    pub fn bounce_in() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().scale_uniform(0.3, &Transition::default())
    }

    /// Create a zoom in animation
    pub fn zoom_in() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().scale_uniform(0.0, &Transition::default())
    }

    /// Create a zoom out animation
    pub fn zoom_out() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().scale_uniform(2.0, &Transition::default())
    }

    /// Create a flip animation
    pub fn flip() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().enable_3d().rotate_3d(
            0.0,
            180.0,
            0.0,
            &Transition::default(),
        )
    }

    /// Create a 3D rotate animation
    pub fn rotate_3d() -> TransformAnimationBuilder {
        TransformAnimationBuilder::new().enable_3d().rotate_3d(
            360.0,
            360.0,
            360.0,
            &Transition::default(),
        )
    }
}
