//! 3D Animation Implementation
//!
//! This module implements 3D animation capabilities for the Leptos Motion library.
//! It provides support for 3D transforms, perspective, and advanced 3D animations.

use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// 3D Transform properties for animations
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Transform3D {
    /// X translation
    pub translate_x: Option<f64>,
    /// Y translation
    pub translate_y: Option<f64>,
    /// Z translation
    pub translate_z: Option<f64>,
    /// Rotation around X axis (degrees)
    pub rotate_x: Option<f64>,
    /// Rotation around Y axis (degrees)
    pub rotate_y: Option<f64>,
    /// Rotation around Z axis (degrees)
    pub rotate_z: Option<f64>,
    /// X scale
    pub scale_x: Option<f64>,
    /// Y scale
    pub scale_y: Option<f64>,
    /// Z scale
    pub scale_z: Option<f64>,
}

/// 3D Perspective configuration
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Perspective3D {
    /// Perspective distance
    pub perspective: Option<String>,
    /// Perspective origin
    pub perspective_origin: Option<String>,
    /// Transform style (flat or preserve-3d)
    pub transform_style: Option<String>,
    /// Backface visibility
    pub backface_visibility: Option<String>,
    /// Transform origin for 3D
    pub transform_origin: Option<String>,
}

/// 3D Animation configuration
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Animation3D {
    /// 3D transform properties
    pub transform: Transform3D,
    /// 3D perspective configuration
    pub perspective: Perspective3D,
    /// Animation transition
    pub transition: Transition,
}

impl Transform3D {
    /// Create a new 3D transform
    pub fn new() -> Self {
        Self::default()
    }

    /// Set X translation
    pub fn translate_x(mut self, x: f64) -> Self {
        self.translate_x = Some(x);
        self
    }

    /// Set Y translation
    pub fn translate_y(mut self, y: f64) -> Self {
        self.translate_y = Some(y);
        self
    }

    /// Set Z translation
    pub fn translate_z(mut self, z: f64) -> Self {
        self.translate_z = Some(z);
        self
    }

    /// Set X rotation
    pub fn rotate_x(mut self, x: f64) -> Self {
        self.rotate_x = Some(x);
        self
    }

    /// Set Y rotation
    pub fn rotate_y(mut self, y: f64) -> Self {
        self.rotate_y = Some(y);
        self
    }

    /// Set Z rotation
    pub fn rotate_z(mut self, z: f64) -> Self {
        self.rotate_z = Some(z);
        self
    }

    /// Set X scale
    pub fn scale_x(mut self, x: f64) -> Self {
        self.scale_x = Some(x);
        self
    }

    /// Set Y scale
    pub fn scale_y(mut self, y: f64) -> Self {
        self.scale_y = Some(y);
        self
    }

    /// Set Z scale
    pub fn scale_z(mut self, z: f64) -> Self {
        self.scale_z = Some(z);
        self
    }

    /// Convert to AnimationTarget for use with ReactiveMotionDiv
    pub fn to_animation_target(&self) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();

        if let Some(x) = self.translate_x {
            target.insert("translateX".to_string(), AnimationValue::Number(x));
        }
        if let Some(y) = self.translate_y {
            target.insert("translateY".to_string(), AnimationValue::Number(y));
        }
        if let Some(z) = self.translate_z {
            target.insert("translateZ".to_string(), AnimationValue::Number(z));
        }
        if let Some(x) = self.rotate_x {
            target.insert("rotateX".to_string(), AnimationValue::Number(x));
        }
        if let Some(y) = self.rotate_y {
            target.insert("rotateY".to_string(), AnimationValue::Number(y));
        }
        if let Some(z) = self.rotate_z {
            target.insert("rotateZ".to_string(), AnimationValue::Number(z));
        }
        if let Some(x) = self.scale_x {
            target.insert("scaleX".to_string(), AnimationValue::Number(x));
        }
        if let Some(y) = self.scale_y {
            target.insert("scaleY".to_string(), AnimationValue::Number(y));
        }
        if let Some(z) = self.scale_z {
            target.insert("scaleZ".to_string(), AnimationValue::Number(z));
        }

        target
    }
}

impl Perspective3D {
    /// Create a new 3D perspective configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set perspective distance
    pub fn perspective(mut self, distance: &str) -> Self {
        self.perspective = Some(distance.to_string());
        self
    }

    /// Set perspective origin
    pub fn perspective_origin(mut self, origin: &str) -> Self {
        self.perspective_origin = Some(origin.to_string());
        self
    }

    /// Set transform style
    pub fn transform_style(mut self, style: &str) -> Self {
        self.transform_style = Some(style.to_string());
        self
    }

    /// Set backface visibility
    pub fn backface_visibility(mut self, visibility: &str) -> Self {
        self.backface_visibility = Some(visibility.to_string());
        self
    }

    /// Set transform origin
    pub fn transform_origin(mut self, origin: &str) -> Self {
        self.transform_origin = Some(origin.to_string());
        self
    }

    /// Convert to AnimationTarget for use with ReactiveMotionDiv
    pub fn to_animation_target(&self) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();

        if let Some(perspective) = &self.perspective {
            target.insert(
                "perspective".to_string(),
                AnimationValue::String(perspective.clone()),
            );
        }
        if let Some(origin) = &self.perspective_origin {
            target.insert(
                "perspective-origin".to_string(),
                AnimationValue::String(origin.clone()),
            );
        }
        if let Some(style) = &self.transform_style {
            target.insert(
                "transform-style".to_string(),
                AnimationValue::String(style.clone()),
            );
        }
        if let Some(visibility) = &self.backface_visibility {
            target.insert(
                "backface-visibility".to_string(),
                AnimationValue::String(visibility.clone()),
            );
        }
        if let Some(origin) = &self.transform_origin {
            target.insert(
                "transform-origin".to_string(),
                AnimationValue::String(origin.clone()),
            );
        }

        target
    }
}

impl Animation3D {
    /// Create a new 3D animation
    pub fn new() -> Self {
        Self::default()
    }

    /// Set 3D transform
    pub fn transform(mut self, transform: Transform3D) -> Self {
        self.transform = transform;
        self
    }

    /// Set 3D perspective
    pub fn perspective(mut self, perspective: Perspective3D) -> Self {
        self.perspective = perspective;
        self
    }

    /// Set transition
    pub fn transition(mut self, transition: Transition) -> Self {
        self.transition = transition;
        self
    }

    /// Convert to AnimationTarget for use with ReactiveMotionDiv
    pub fn to_animation_target(&self) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();

        // Add transform properties
        target.extend(self.transform.to_animation_target());

        // Add perspective properties
        target.extend(self.perspective.to_animation_target());

        target
    }

    /// Create a 3D rotation animation
    pub fn rotate_3d(x: f64, y: f64, z: f64) -> Self {
        Self::new().transform(Transform3D::new().rotate_x(x).rotate_y(y).rotate_z(z))
    }

    /// Create a 3D translation animation
    pub fn translate_3d(x: f64, y: f64, z: f64) -> Self {
        Self::new().transform(
            Transform3D::new()
                .translate_x(x)
                .translate_y(y)
                .translate_z(z),
        )
    }

    /// Create a 3D scale animation
    pub fn scale_3d(x: f64, y: f64, z: f64) -> Self {
        Self::new().transform(Transform3D::new().scale_x(x).scale_y(y).scale_z(z))
    }

    /// Create a 3D perspective animation
    pub fn with_perspective(distance: &str) -> Self {
        Self::new().perspective(
            Perspective3D::new()
                .perspective(distance)
                .transform_style("preserve-3d"),
        )
    }
}

/// Utility functions for 3D animations
pub mod utils {
    use super::*;

    /// Create a matrix3d transform string
    pub fn matrix3d(
        m11: f64,
        m12: f64,
        m13: f64,
        m14: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m24: f64,
        m31: f64,
        m32: f64,
        m33: f64,
        m34: f64,
        m41: f64,
        m42: f64,
        m43: f64,
        m44: f64,
    ) -> String {
        format!(
            "matrix3d({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})",
            m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44
        )
    }

    /// Create a rotate3d transform string
    pub fn rotate3d(x: f64, y: f64, z: f64, angle: f64) -> String {
        format!("rotate3d({}, {}, {}, {}deg)", x, y, z, angle)
    }

    /// Create a translate3d transform string
    pub fn translate3d(x: f64, y: f64, z: f64) -> String {
        format!("translate3d({}px, {}px, {}px)", x, y, z)
    }

    /// Create a scale3d transform string
    pub fn scale3d(x: f64, y: f64, z: f64) -> String {
        format!("scale3d({}, {}, {})", x, y, z)
    }

    /// Create a 3D animation target with matrix3d
    pub fn matrix3d_animation_target(
        m11: f64,
        m12: f64,
        m13: f64,
        m14: f64,
        m21: f64,
        m22: f64,
        m23: f64,
        m24: f64,
        m31: f64,
        m32: f64,
        m33: f64,
        m34: f64,
        m41: f64,
        m42: f64,
        m43: f64,
        m44: f64,
    ) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();
        target.insert(
            "matrix3d".to_string(),
            AnimationValue::String(matrix3d(
                m11, m12, m13, m14, m21, m22, m23, m24, m31, m32, m33, m34, m41, m42, m43, m44,
            )),
        );
        target
    }

    /// Create a 3D animation target with rotate3d
    pub fn rotate3d_animation_target(
        x: f64,
        y: f64,
        z: f64,
        angle: f64,
    ) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();
        target.insert(
            "rotate3d".to_string(),
            AnimationValue::String(rotate3d(x, y, z, angle)),
        );
        target
    }

    /// Create a 3D animation target with translate3d
    pub fn translate3d_animation_target(x: f64, y: f64, z: f64) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();
        target.insert(
            "translate3d".to_string(),
            AnimationValue::String(translate3d(x, y, z)),
        );
        target
    }

    /// Create a 3D animation target with scale3d
    pub fn scale3d_animation_target(x: f64, y: f64, z: f64) -> HashMap<String, AnimationValue> {
        let mut target = HashMap::new();
        target.insert(
            "scale3d".to_string(),
            AnimationValue::String(scale3d(x, y, z)),
        );
        target
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform3d_creation() {
        let transform = Transform3D::new()
            .translate_x(100.0)
            .translate_y(200.0)
            .translate_z(300.0)
            .rotate_x(45.0)
            .rotate_y(90.0)
            .rotate_z(180.0)
            .scale_x(1.5)
            .scale_y(2.0)
            .scale_z(0.5);

        assert_eq!(transform.translate_x, Some(100.0));
        assert_eq!(transform.translate_y, Some(200.0));
        assert_eq!(transform.translate_z, Some(300.0));
        assert_eq!(transform.rotate_x, Some(45.0));
        assert_eq!(transform.rotate_y, Some(90.0));
        assert_eq!(transform.rotate_z, Some(180.0));
        assert_eq!(transform.scale_x, Some(1.5));
        assert_eq!(transform.scale_y, Some(2.0));
        assert_eq!(transform.scale_z, Some(0.5));
    }

    #[test]
    fn test_transform3d_to_animation_target() {
        let transform = Transform3D::new()
            .translate_x(100.0)
            .translate_y(200.0)
            .translate_z(300.0)
            .rotate_x(45.0)
            .rotate_y(90.0)
            .rotate_z(180.0)
            .scale_x(1.5)
            .scale_y(2.0)
            .scale_z(0.5);

        let target = transform.to_animation_target();

        assert_eq!(
            target.get("translateX"),
            Some(&AnimationValue::Number(100.0))
        );
        assert_eq!(
            target.get("translateY"),
            Some(&AnimationValue::Number(200.0))
        );
        assert_eq!(
            target.get("translateZ"),
            Some(&AnimationValue::Number(300.0))
        );
        assert_eq!(target.get("rotateX"), Some(&AnimationValue::Number(45.0)));
        assert_eq!(target.get("rotateY"), Some(&AnimationValue::Number(90.0)));
        assert_eq!(target.get("rotateZ"), Some(&AnimationValue::Number(180.0)));
        assert_eq!(target.get("scaleX"), Some(&AnimationValue::Number(1.5)));
        assert_eq!(target.get("scaleY"), Some(&AnimationValue::Number(2.0)));
        assert_eq!(target.get("scaleZ"), Some(&AnimationValue::Number(0.5)));
    }

    #[test]
    fn test_perspective3d_creation() {
        let perspective = Perspective3D::new()
            .perspective("1000px")
            .perspective_origin("50% 50%")
            .transform_style("preserve-3d")
            .backface_visibility("hidden")
            .transform_origin("center center 0px");

        assert_eq!(perspective.perspective, Some("1000px".to_string()));
        assert_eq!(perspective.perspective_origin, Some("50% 50%".to_string()));
        assert_eq!(perspective.transform_style, Some("preserve-3d".to_string()));
        assert_eq!(perspective.backface_visibility, Some("hidden".to_string()));
        assert_eq!(
            perspective.transform_origin,
            Some("center center 0px".to_string())
        );
    }

    #[test]
    fn test_perspective3d_to_animation_target() {
        let perspective = Perspective3D::new()
            .perspective("1000px")
            .perspective_origin("50% 50%")
            .transform_style("preserve-3d")
            .backface_visibility("hidden")
            .transform_origin("center center 0px");

        let target = perspective.to_animation_target();

        assert_eq!(
            target.get("perspective"),
            Some(&AnimationValue::String("1000px".to_string()))
        );
        assert_eq!(
            target.get("perspective-origin"),
            Some(&AnimationValue::String("50% 50%".to_string()))
        );
        assert_eq!(
            target.get("transform-style"),
            Some(&AnimationValue::String("preserve-3d".to_string()))
        );
        assert_eq!(
            target.get("backface-visibility"),
            Some(&AnimationValue::String("hidden".to_string()))
        );
        assert_eq!(
            target.get("transform-origin"),
            Some(&AnimationValue::String("center center 0px".to_string()))
        );
    }

    #[test]
    fn test_animation3d_creation() {
        let animation = Animation3D::new()
            .transform(Transform3D::new().translate_x(100.0).rotate_x(45.0))
            .perspective(Perspective3D::new().perspective("1000px"))
            .transition(Transition {
                duration: Some(1.0),
                delay: Some(0.0),
                ease: Easing::EaseInOut,
                repeat: RepeatConfig::Never,
                stagger: None,
            });

        assert_eq!(animation.transform.translate_x, Some(100.0));
        assert_eq!(animation.transform.rotate_x, Some(45.0));
        assert_eq!(
            animation.perspective.perspective,
            Some("1000px".to_string())
        );
        assert_eq!(animation.transition.duration, Some(1.0));
    }

    #[test]
    fn test_animation3d_to_animation_target() {
        let animation = Animation3D::new()
            .transform(Transform3D::new().translate_x(100.0).rotate_x(45.0))
            .perspective(Perspective3D::new().perspective("1000px"));

        let target = animation.to_animation_target();

        assert_eq!(
            target.get("translateX"),
            Some(&AnimationValue::Number(100.0))
        );
        assert_eq!(target.get("rotateX"), Some(&AnimationValue::Number(45.0)));
        assert_eq!(
            target.get("perspective"),
            Some(&AnimationValue::String("1000px".to_string()))
        );
    }

    #[test]
    fn test_animation3d_convenience_methods() {
        let rotate_animation = Animation3D::rotate_3d(45.0, 90.0, 180.0);
        assert_eq!(rotate_animation.transform.rotate_x, Some(45.0));
        assert_eq!(rotate_animation.transform.rotate_y, Some(90.0));
        assert_eq!(rotate_animation.transform.rotate_z, Some(180.0));

        let translate_animation = Animation3D::translate_3d(100.0, 200.0, 300.0);
        assert_eq!(translate_animation.transform.translate_x, Some(100.0));
        assert_eq!(translate_animation.transform.translate_y, Some(200.0));
        assert_eq!(translate_animation.transform.translate_z, Some(300.0));

        let scale_animation = Animation3D::scale_3d(1.5, 2.0, 0.5);
        assert_eq!(scale_animation.transform.scale_x, Some(1.5));
        assert_eq!(scale_animation.transform.scale_y, Some(2.0));
        assert_eq!(scale_animation.transform.scale_z, Some(0.5));

        let perspective_animation = Animation3D::with_perspective("1000px");
        assert_eq!(
            perspective_animation.perspective.perspective,
            Some("1000px".to_string())
        );
        assert_eq!(
            perspective_animation.perspective.transform_style,
            Some("preserve-3d".to_string())
        );
    }

    #[test]
    fn test_3d_utility_functions() {
        let matrix3d = utils::matrix3d(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        assert_eq!(
            matrix3d,
            "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)"
        );

        let rotate3d = utils::rotate3d(1.0, 1.0, 1.0, 45.0);
        assert_eq!(rotate3d, "rotate3d(1, 1, 1, 45deg)");

        let translate3d = utils::translate3d(100.0, 200.0, 300.0);
        assert_eq!(translate3d, "translate3d(100px, 200px, 300px)");

        let scale3d = utils::scale3d(1.5, 2.0, 0.5);
        assert_eq!(scale3d, "scale3d(1.5, 2, 0.5)");
    }

    #[test]
    fn test_3d_utility_animation_targets() {
        let matrix3d_target = utils::matrix3d_animation_target(
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        );
        assert_eq!(
            matrix3d_target.get("matrix3d"),
            Some(&AnimationValue::String(
                "matrix3d(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)".to_string()
            ))
        );

        let rotate3d_target = utils::rotate3d_animation_target(1.0, 1.0, 1.0, 45.0);
        assert_eq!(
            rotate3d_target.get("rotate3d"),
            Some(&AnimationValue::String(
                "rotate3d(1, 1, 1, 45deg)".to_string()
            ))
        );

        let translate3d_target = utils::translate3d_animation_target(100.0, 200.0, 300.0);
        assert_eq!(
            translate3d_target.get("translate3d"),
            Some(&AnimationValue::String(
                "translate3d(100px, 200px, 300px)".to_string()
            ))
        );

        let scale3d_target = utils::scale3d_animation_target(1.5, 2.0, 0.5);
        assert_eq!(
            scale3d_target.get("scale3d"),
            Some(&AnimationValue::String("scale3d(1.5, 2, 0.5)".to_string()))
        );
    }
}
