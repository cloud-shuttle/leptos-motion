//! 3D transforms and perspective support for Motion Studio

use crate::{Result, StudioError};
use glam::{Mat4, Quat, Vec3, Vec4};
use leptos::prelude::*;
use leptos::html::ElementChild;
use leptos::attr::global::ClassAttribute;
use leptos::prelude::{NodeRefAttribute, StyleAttribute, OnAttribute};
use serde::{Deserialize, Serialize};

/// 3D transform component with translation, rotation, and scale
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform3D {
    /// Translation in 3D space
    pub translation: Vec3,
    /// Rotation quaternion
    pub rotation: Quat,
    /// Scale factors for each axis
    pub scale: Vec3,
    /// Transform origin point
    pub origin: Vec3,
}

impl Transform3D {
    /// Create new identity transform
    pub fn new() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            origin: Vec3::ZERO,
        }
    }

    /// Create transform from translation
    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            ..Self::new()
        }
    }

    /// Create transform from rotation
    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            rotation,
            ..Self::new()
        }
    }

    /// Create transform from scale
    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            scale,
            ..Self::new()
        }
    }

    /// Create transform from Euler angles (radians)
    pub fn from_euler(x: f32, y: f32, z: f32) -> Self {
        Self {
            rotation: Quat::from_euler(glam::EulerRot::XYZ, x, y, z),
            ..Self::new()
        }
    }

    /// Get translation
    pub fn translation(&self) -> Vec3 {
        self.translation
    }

    /// Get rotation
    pub fn rotation(&self) -> Quat {
        self.rotation
    }

    /// Get scale
    pub fn scale(&self) -> Vec3 {
        self.scale
    }

    /// Convert to 4x4 transformation matrix
    pub fn to_matrix(&self) -> Mat4 {
        // Create transform matrix: T * R * S
        let translation_matrix = Mat4::from_translation(self.translation);
        let rotation_matrix = Mat4::from_quat(self.rotation);
        let scale_matrix = Mat4::from_scale(self.scale);

        // Apply origin offset
        let origin_offset = Mat4::from_translation(-self.origin);
        let origin_restore = Mat4::from_translation(self.origin);

        origin_restore * translation_matrix * rotation_matrix * scale_matrix * origin_offset
    }

    /// Linear interpolation between two transforms
    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);

        Self {
            translation: self.translation.lerp(other.translation, t),
            rotation: self.rotation.slerp(other.rotation, t),
            scale: self.scale.lerp(other.scale, t),
            origin: self.origin.lerp(other.origin, t),
        }
    }

    /// Compose with another transform
    pub fn compose(&self, other: &Self) -> Self {
        let matrix = self.to_matrix() * other.to_matrix();
        Self::from_matrix(matrix)
    }

    /// Create transform from 4x4 matrix
    pub fn from_matrix(matrix: Mat4) -> Self {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();

        Self {
            translation,
            rotation,
            scale,
            origin: Vec3::ZERO,
        }
    }

    /// Get CSS transform string for 3D transforms
    pub fn to_css(&self) -> String {
        let euler = self.rotation.to_euler(glam::EulerRot::XYZ);

        format!(
            "translate3d({:.2}px, {:.2}px, {:.2}px) rotateX({:.2}rad) rotateY({:.2}rad) rotateZ({:.2}rad) scale3d({:.2}, {:.2}, {:.2})",
            self.translation.x,
            self.translation.y,
            self.translation.z,
            euler.0,
            euler.1,
            euler.2,
            self.scale.x,
            self.scale.y,
            self.scale.z
        )
    }

    /// Get CSS transform-origin string
    pub fn to_css_origin(&self) -> String {
        format!(
            "{:.2}px {:.2}px {:.2}px",
            self.origin.x, self.origin.y, self.origin.z
        )
    }
}

impl Default for Transform3D {
    fn default() -> Self {
        Self::new()
    }
}

/// Perspective projection parameters
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Perspective {
    /// Field of view in degrees
    pub fov: f32,
    /// Aspect ratio (width/height)
    pub aspect: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
}

impl Perspective {
    /// Create new perspective projection
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
        }
    }

    /// Convert to projection matrix
    pub fn to_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov.to_radians(), self.aspect, self.near, self.far)
    }

    /// Convert to CSS perspective value
    pub fn to_css(&self) -> String {
        // CSS perspective is 1 / tan(fov/2)
        let perspective_value = 1.0 / (self.fov.to_radians() / 2.0).tan();
        format!("{}px", perspective_value * 500.0) // Scale for reasonable CSS values
    }
}

impl Default for Perspective {
    fn default() -> Self {
        Self::new(45.0, 1.0, 0.1, 100.0)
    }
}

/// 3D transform matrix utilities
pub struct TransformMatrix3D;

impl TransformMatrix3D {
    /// Create look-at matrix
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
        Mat4::look_at_rh(eye, target, up)
    }

    /// Create orthographic projection matrix
    pub fn orthographic(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Mat4 {
        Mat4::orthographic_rh_gl(left, right, bottom, top, near, far)
    }

    /// Extract transform components from matrix
    pub fn decompose(matrix: Mat4) -> (Vec3, Quat, Vec3) {
        matrix.to_scale_rotation_translation()
    }

    /// Check if matrix is invertible
    pub fn is_invertible(matrix: Mat4) -> bool {
        matrix.determinant().abs() > f32::EPSILON
    }

    /// Safe matrix inverse
    pub fn inverse(matrix: Mat4) -> Option<Mat4> {
        if Self::is_invertible(matrix) {
            Some(matrix.inverse())
        } else {
            None
        }
    }
}

/// 3D Transform animation system
#[derive(Debug, Clone)]
pub struct Transform3DAnimation {
    /// Animation ID
    pub id: uuid::Uuid,
    /// Start transform
    pub from: Transform3D,
    /// End transform
    pub to: Transform3D,
    /// Animation duration in seconds
    pub duration: f32,
    /// Easing function
    pub easing: EasingFunction,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Is animation playing
    pub is_playing: bool,
}

impl Transform3DAnimation {
    /// Create new transform animation
    pub fn new(from: Transform3D, to: Transform3D, duration: f32) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            from,
            to,
            duration,
            easing: EasingFunction::EaseInOut,
            progress: 0.0,
            is_playing: false,
        }
    }

    /// Update animation progress
    pub fn update(&mut self, delta_time: f32) -> Transform3D {
        if !self.is_playing {
            return self.current_transform();
        }

        self.progress += delta_time / self.duration;
        self.progress = self.progress.clamp(0.0, 1.0);

        if self.progress >= 1.0 {
            self.is_playing = false;
        }

        self.current_transform()
    }

    /// Get current interpolated transform
    pub fn current_transform(&self) -> Transform3D {
        let t = self.easing.apply(self.progress);
        self.from.lerp(&self.to, t)
    }

    /// Start animation
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause animation
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Reset animation
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.is_playing = false;
    }

    /// Seek to specific time
    pub fn seek(&mut self, time: f32) {
        self.progress = (time / self.duration).clamp(0.0, 1.0);
    }
}

/// Easing functions for 3D animations
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
}

impl EasingFunction {
    /// Apply easing function to input value
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            Self::EaseInBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Self::EaseOutBack => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Self::EaseInOutBack => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    (2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (t * 2.0 - 2.0) + c2) + 2.0) / 2.0
                }
            }
            Self::EaseInElastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    -2_f32.powf(10.0 * (t - 1.0))
                        * ((t - 1.0) * c4 - std::f32::consts::PI / 2.0).sin()
                }
            }
            Self::EaseOutElastic => {
                let c4 = (2.0 * std::f32::consts::PI) / 3.0;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else {
                    2_f32.powf(-10.0 * t) * (t * c4 - std::f32::consts::PI / 2.0).sin() + 1.0
                }
            }
            Self::EaseInOutElastic => {
                let c5 = (2.0 * std::f32::consts::PI) / 4.5;
                if t == 0.0 {
                    0.0
                } else if t == 1.0 {
                    1.0
                } else if t < 0.5 {
                    -(2_f32.powf(20.0 * t - 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0
                } else {
                    (2_f32.powf(-20.0 * t + 10.0) * ((20.0 * t - 11.125) * c5).sin()) / 2.0 + 1.0
                }
            }
            Self::EaseInBounce => 1.0 - Self::EaseOutBounce.apply(1.0 - t),
            Self::EaseOutBounce => {
                let n1 = 7.5625;
                let d1 = 2.75;
                if t < 1.0 / d1 {
                    n1 * t * t
                } else if t < 2.0 / d1 {
                    n1 * (t - 1.5 / d1) * (t - 1.5 / d1) + 0.75
                } else if t < 2.5 / d1 {
                    n1 * (t - 2.25 / d1) * (t - 2.25 / d1) + 0.9375
                } else {
                    n1 * (t - 2.625 / d1) * (t - 2.625 / d1) + 0.984375
                }
            }
            Self::EaseInOutBounce => {
                if t < 0.5 {
                    (1.0 - Self::EaseOutBounce.apply(1.0 - 2.0 * t)) / 2.0
                } else {
                    (1.0 + Self::EaseOutBounce.apply(2.0 * t - 1.0)) / 2.0
                }
            }
        }
    }
}

/// 3D Transform editor component
#[component]
pub fn Transform3DEditor(
    /// Current transform
    #[prop(default = Transform3D::default())]
    transform: Transform3D,

    /// Callback when transform changes
    #[prop(optional)]
    on_change: Option<Callback<Transform3D>>,

    /// Show perspective controls
    #[prop(default = true)]
    show_perspective: bool,
) -> impl IntoView {
    let (current_transform, set_transform) = create_signal(transform);
    let (perspective, set_perspective) = create_signal(Perspective::default());

    // Handle transform updates
    let handle_transform_change = move |new_transform: Transform3D| {
        set_transform.set(new_transform);
        // Temporarily disabled until callback API is clarified
        // if let Some(callback) = on_change {
        //     callback(new_transform);
        // }
    };

    view! {
        <div class="transform3d-editor">
            <div class="transform3d-section">
                <h4>"Translation"</h4>
                <TranslationControls
                    translation=move || current_transform.get().translation
                    on_change=move |t: Vec3| {
                        let mut transform = current_transform.get();
                        transform.translation = t;
                        handle_transform_change(transform);
                    }
                />
            </div>

            <div class="transform3d-section">
                <h4>"Rotation"</h4>
                <RotationControls
                    rotation=move || current_transform.get().rotation
                    on_change=move |r: Quat| {
                        let mut transform = current_transform.get();
                        transform.rotation = r;
                        handle_transform_change(transform);
                    }
                />
            </div>

            <div class="transform3d-section">
                <h4>"Scale"</h4>
                <ScaleControls
                    scale=move || current_transform.get().scale
                    on_change=move |s: Vec3| {
                        let mut transform = current_transform.get();
                        transform.scale = s;
                        handle_transform_change(transform);
                    }
                />
            </div>

            {move || show_perspective.then(|| view! {
                <div class="transform3d-section">
                    <h4>"Perspective"</h4>
                    <PerspectiveControls
                        perspective=perspective
                        on_change=set_perspective
                    />
                </div>
            })}

            <div class="transform3d-preview">
                <Transform3DPreview
                    transform=current_transform
                    perspective=perspective
                />
            </div>
        </div>
    }
}

// Control components (placeholders for now)
#[component]
fn TranslationControls(
    translation: impl Fn() -> Vec3 + 'static,
    on_change: impl Fn(Vec3) + 'static,
) -> impl IntoView {
    view! {
        <div class="vec3-controls">
            <label>"X: " <input type="number" step="0.1" /></label>
            <label>"Y: " <input type="number" step="0.1" /></label>
            <label>"Z: " <input type="number" step="0.1" /></label>
        </div>
    }
}

#[component]
fn RotationControls(
    rotation: impl Fn() -> Quat + 'static,
    on_change: impl Fn(Quat) + 'static,
) -> impl IntoView {
    view! {
        <div class="rotation-controls">
            <label>"X: " <input type="number" step="0.01" /></label>
            <label>"Y: " <input type="number" step="0.01" /></label>
            <label>"Z: " <input type="number" step="0.01" /></label>
        </div>
    }
}

#[component]
fn ScaleControls(
    scale: impl Fn() -> Vec3 + 'static,
    on_change: impl Fn(Vec3) + 'static,
) -> impl IntoView {
    view! {
        <div class="vec3-controls">
            <label>"X: " <input type="number" step="0.1" min="0.01" /></label>
            <label>"Y: " <input type="number" step="0.1" min="0.01" /></label>
            <label>"Z: " <input type="number" step="0.1" min="0.01" /></label>
        </div>
    }
}

#[component]
fn PerspectiveControls(
    perspective: ReadSignal<Perspective>,
    on_change: WriteSignal<Perspective>,
) -> impl IntoView {
    view! {
        <div class="perspective-controls">
            <label>"FOV: " <input type="number" step="1" min="10" max="170" /></label>
            <label>"Near: " <input type="number" step="0.1" min="0.1" /></label>
            <label>"Far: " <input type="number" step="1" min="1" /></label>
        </div>
    }
}

#[component]
fn Transform3DPreview(
    transform: ReadSignal<Transform3D>,
    perspective: ReadSignal<Perspective>,
) -> impl IntoView {
    view! {
        <div class="transform3d-preview__viewport"
             style:transform=move || transform.get().to_css()
             style:perspective=move || perspective.get().to_css()
        >
            <div class="transform3d-preview__cube">
                <div class="cube-face cube-face--front">"Front"</div>
                <div class="cube-face cube-face--back">"Back"</div>
                <div class="cube-face cube-face--right">"Right"</div>
                <div class="cube-face cube-face--left">"Left"</div>
                <div class="cube-face cube-face--top">"Top"</div>
                <div class="cube-face cube-face--bottom">"Bottom"</div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_transform3d_identity() {
        let transform = Transform3D::new();
        assert_eq!(transform.translation, Vec3::ZERO);
        assert_eq!(transform.rotation, Quat::IDENTITY);
        assert_eq!(transform.scale, Vec3::ONE);
        assert_eq!(transform.origin, Vec3::ZERO);
    }

    #[test]
    fn test_transform3d_matrix_identity() {
        let transform = Transform3D::new();
        let matrix = transform.to_matrix();
        assert_relative_eq!(matrix, Mat4::IDENTITY, epsilon = 1e-6);
    }

    #[test]
    fn test_transform3d_from_translation() {
        let translation = Vec3::new(1.0, 2.0, 3.0);
        let transform = Transform3D::from_translation(translation);
        assert_eq!(transform.translation, translation);

        let matrix = transform.to_matrix();
        let expected = Mat4::from_translation(translation);
        assert_relative_eq!(matrix, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_transform3d_from_rotation() {
        let rotation = Quat::from_rotation_z(std::f32::consts::PI / 4.0);
        let transform = Transform3D::from_rotation(rotation);
        assert_relative_eq!(transform.rotation, rotation, epsilon = 1e-6);

        let matrix = transform.to_matrix();
        let expected = Mat4::from_quat(rotation);
        assert_relative_eq!(matrix, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_transform3d_lerp() {
        let transform1 = Transform3D::from_translation(Vec3::new(0.0, 0.0, 0.0));
        let transform2 = Transform3D::from_translation(Vec3::new(10.0, 20.0, 30.0));

        let lerped = transform1.lerp(&transform2, 0.5);
        assert_relative_eq!(
            lerped.translation,
            Vec3::new(5.0, 10.0, 15.0),
            epsilon = 1e-6
        );
    }

    #[test]
    fn test_transform3d_css() {
        let transform = Transform3D::from_translation(Vec3::new(10.0, 20.0, 30.0));
        let css = transform.to_css();
        assert!(css.contains("translate3d(10.00px, 20.00px, 30.00px)"));
    }

    #[test]
    fn test_perspective_matrix() {
        let perspective = Perspective::new(45.0, 1.0, 0.1, 100.0);
        let matrix = perspective.to_matrix();

        // Matrix should not be identity for non-zero FOV
        assert_ne!(matrix, Mat4::IDENTITY);

        // Check that it's a valid perspective matrix
        assert!(matrix.w_axis.w != 0.0);
    }

    #[test]
    fn test_easing_functions() {
        for easing in [
            EasingFunction::Linear,
            EasingFunction::EaseIn,
            EasingFunction::EaseOut,
            EasingFunction::EaseInOut,
        ] {
            // Test boundary conditions
            assert_relative_eq!(easing.apply(0.0), 0.0, epsilon = 1e-6);
            assert_relative_eq!(easing.apply(1.0), 1.0, epsilon = 1e-6);

            // Test monotonicity for simple functions
            let t1 = 0.3;
            let t2 = 0.7;
            assert!(easing.apply(t1) <= easing.apply(t2));
        }
    }

    #[test]
    fn test_transform3d_animation() {
        let from = Transform3D::from_translation(Vec3::ZERO);
        let to = Transform3D::from_translation(Vec3::new(10.0, 0.0, 0.0));

        let mut animation = Transform3DAnimation::new(from, to, 1.0);
        assert_eq!(animation.progress, 0.0);
        assert!(!animation.is_playing);

        animation.play();
        assert!(animation.is_playing);

        let transform = animation.update(0.5);
        assert_eq!(animation.progress, 0.5);

        // Should be halfway between start and end
        assert_relative_eq!(transform.translation.x, 5.0, epsilon = 1e-6);
    }

    #[test]
    fn test_matrix_utilities() {
        let matrix = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));

        assert!(TransformMatrix3D::is_invertible(matrix));

        let inverse = TransformMatrix3D::inverse(matrix);
        assert!(inverse.is_some());

        let (scale, rotation, translation) = TransformMatrix3D::decompose(matrix);
        assert_relative_eq!(translation, Vec3::new(1.0, 2.0, 3.0), epsilon = 1e-6);
        assert_relative_eq!(scale, Vec3::ONE, epsilon = 1e-6);
        assert_relative_eq!(rotation, Quat::IDENTITY, epsilon = 1e-6);
    }
}
