//! FLIP (First, Last, Invert, Play) animation implementation
//! 
//! FLIP is a technique for creating smooth layout animations by:
//! 1. First: Record the initial position
//! 2. Last: Record the final position  
//! 3. Invert: Apply transforms to make it appear in the initial position
//! 4. Play: Animate the transforms to their natural values

use crate::{LayoutAnimationConfig};
use std::collections::HashMap;
use web_sys::{Element, DomRect};
use wasm_bindgen::prelude::*;

/// FLIP animation state
#[derive(Debug, Clone)]
pub struct FLIPState {
    /// Initial position and dimensions
    pub first: DomRect,
    /// Final position and dimensions
    pub last: DomRect,
    /// Inverted transform values
    pub inverted: TransformValues,
    /// Current animation progress (0.0 to 1.0)
    pub progress: f64,
    /// Whether the animation is active
    pub active: bool,
}

/// Transform values for FLIP animations
#[derive(Debug, Clone)]
pub struct TransformValues {
    /// X translation
    pub translate_x: f64,
    /// Y translation
    pub translate_y: f64,
    /// Scale X
    pub scale_x: f64,
    /// Scale Y
    pub scale_y: f64,
    /// Rotation in degrees
    pub rotation: f64,
}

impl Default for TransformValues {
    fn default() -> Self {
        Self {
            translate_x: 0.0,
            translate_y: 0.0,
            scale_x: 1.0,
            scale_y: 1.0,
            rotation: 0.0,
        }
    }
}

impl TransformValues {
    /// Create new transform values
    pub fn new(translate_x: f64, translate_y: f64, scale_x: f64, scale_y: f64, rotation: f64) -> Self {
        Self {
            translate_x,
            translate_y,
            scale_x,
            scale_y,
            rotation,
        }
    }

    /// Create translation-only transform
    pub fn translation(x: f64, y: f64) -> Self {
        Self::new(x, y, 1.0, 1.0, 0.0)
    }

    /// Create scale-only transform
    pub fn scale(x: f64, y: f64) -> Self {
        Self::new(0.0, 0.0, x, y, 0.0)
    }

    /// Create rotation-only transform
    pub fn rotation(degrees: f64) -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0, degrees)
    }
}

/// FLIP animation instance
#[derive(Debug)]
pub struct FLIPAnimation {
    /// Unique animation ID
    pub id: String,
    /// Target element
    pub element: Element,
    /// FLIP state
    pub state: FLIPState,
    /// Animation configuration
    pub config: LayoutAnimationConfig,
    /// Start time
    pub start_time: f64,
    /// Duration
    pub duration: f64,
    /// Easing function
    pub easing: EasingFunction,
}

/// Easing function for FLIP animations
#[derive(Debug, Clone)]
pub enum EasingFunction {
    /// Linear easing
    Linear,
    /// Ease-in
    EaseIn,
    /// Ease-out
    EaseOut,
    /// Ease-in-out
    EaseInOut,
    /// Custom cubic-bezier
    CubicBezier(f64, f64, f64, f64),
    /// Spring physics
    Spring { 
        /// Spring tension
        tension: f64, 
        /// Spring friction
        friction: f64 
    },
}

impl Default for EasingFunction {
    fn default() -> Self {
        EasingFunction::EaseOut
    }
}

impl EasingFunction {
    /// Evaluate the easing function at given progress (0.0 to 1.0)
    pub fn evaluate(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            EasingFunction::Linear => t,
            EasingFunction::EaseIn => t * t,
            EasingFunction::EaseOut => 1.0 - (1.0 - t).powi(2),
            EasingFunction::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t).powi(2)
                }
            }
            EasingFunction::CubicBezier(p1, _p2, p3, _p4) => {
                // Simplified cubic bezier evaluation
                let u = 1.0 - t;
                u * u * u * 0.0 + 3.0 * u * u * t * p1 + 3.0 * u * t * t * p3 + t * t * t * 1.0
            }
            EasingFunction::Spring { tension, friction } => {
                // Simplified spring physics
                let damping = friction / (2.0 * (tension).sqrt());
                let frequency = (tension).sqrt();
                
                if damping < 1.0 {
                    let damped_freq = frequency * (1.0 - damping * damping).sqrt();
                    let decay = (-damping * frequency * t).exp();
                    1.0 - decay * (damped_freq * t + damping * frequency * t / damped_freq).cos()
                } else {
                    1.0 - (-frequency * t).exp()
                }
            }
        }
    }
}

/// FLIP animator for managing layout transitions
pub struct FLIPAnimator {
    /// Active FLIP animations
    active_animations: HashMap<String, FLIPAnimation>,
    /// Animation frame callback
    animation_frame: Option<i32>,
    /// Performance tracking
    performance_metrics: FLIPPerformanceMetrics,
}

/// FLIP performance metrics
#[derive(Debug, Clone)]
pub struct FLIPPerformanceMetrics {
    /// Total animations created
    pub total_animations: usize,
    /// Average animation duration
    pub average_duration: f64,
    /// Failed animations
    pub failed_animations: usize,
    /// Performance score (0.0 to 1.0)
    pub performance_score: f64,
}

impl Default for FLIPPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_animations: 0,
            average_duration: 0.0,
            failed_animations: 0,
            performance_score: 1.0,
        }
    }
}

impl FLIPAnimator {
    /// Create a new FLIP animator
    pub fn new() -> Self {
        Self {
            active_animations: HashMap::new(),
            animation_frame: None,
            performance_metrics: FLIPPerformanceMetrics::default(),
        }
    }

    /// Start a FLIP animation
    pub fn animate(
        &mut self,
        id: String,
        element: Element,
        first: DomRect,
        last: DomRect,
        config: LayoutAnimationConfig,
    ) -> Result<(), String> {
        let inverted = self.calculate_transform_values(&first, &last);
        
        let state = FLIPState {
            first,
            last,
            inverted,
            progress: 0.0,
            active: true,
        };

        let animation = FLIPAnimation {
            id: id.clone(),
            element,
            state,
            config: config.clone(),
            start_time: self.get_current_time(),
            duration: config.duration,
            easing: config.easing,
        };

        self.active_animations.insert(id, animation);
        self.performance_metrics.total_animations += 1;
        
        Ok(())
    }

    /// Calculate transform values from first to last positions
    fn calculate_transform_values(&self, first: &DomRect, last: &DomRect) -> TransformValues {
        let translate_x = first.x() - last.x();
        let translate_y = first.y() - last.y();
        let scale_x = if last.width() > 0.0 { first.width() / last.width() } else { 1.0 };
        let scale_y = if last.height() > 0.0 { first.height() / last.height() } else { 1.0 };

        TransformValues::new(translate_x, translate_y, scale_x, scale_y, 0.0)
    }

    /// Get current time in milliseconds
    fn get_current_time(&self) -> f64 {
        js_sys::Date::now()
    }

    /// Update all active animations
    pub fn update(&mut self) {
        let current_time = self.get_current_time();
        let mut completed_ids = Vec::new();

        for (id, animation) in &mut self.active_animations {
            let elapsed = (current_time - animation.start_time) / 1000.0; // Convert to seconds
            let progress = (elapsed / animation.duration).clamp(0.0, 1.0);
            let eased_progress = animation.easing.evaluate(progress);

            animation.state.progress = eased_progress;

            if progress >= 1.0 {
                animation.state.active = false;
                completed_ids.push(id.clone());
            }

            // Apply transform inline to avoid borrow checker issues
            let inverted = &animation.state.inverted;
            let current_x = inverted.translate_x * (1.0 - eased_progress);
            let current_y = inverted.translate_y * (1.0 - eased_progress);
            let current_scale_x = 1.0 + (inverted.scale_x - 1.0) * (1.0 - eased_progress);
            let current_scale_y = 1.0 + (inverted.scale_y - 1.0) * (1.0 - eased_progress);

            let transform = format!(
                "translateX({}px) translateY({}px) scaleX({}) scaleY({})",
                current_x, current_y, current_scale_x, current_scale_y
            );

            if let Ok(_) = animation.element.dyn_ref::<web_sys::HtmlElement>().unwrap().style().set_property("transform", &transform) {
                // Transform applied successfully
            }
        }

        // Remove completed animations
        for id in completed_ids {
            self.active_animations.remove(&id);
        }
    }

    /// Apply transform to element based on progress
    fn apply_transform(&self, animation: &FLIPAnimation, progress: f64) {
        let inverted = &animation.state.inverted;
        let current_x = inverted.translate_x * (1.0 - progress);
        let current_y = inverted.translate_y * (1.0 - progress);
        let current_scale_x = 1.0 + (inverted.scale_x - 1.0) * (1.0 - progress);
        let current_scale_y = 1.0 + (inverted.scale_y - 1.0) * (1.0 - progress);

        let transform = format!(
            "translateX({}px) translateY({}px) scaleX({}) scaleY({})",
            current_x, current_y, current_scale_x, current_scale_y
        );

        if let Ok(style) = animation.element.dyn_ref::<web_sys::HtmlElement>().unwrap().style().set_property("transform", &transform) {
            // Transform applied successfully
        }
    }

    /// Get active animation count
    pub fn active_count(&self) -> usize {
        self.active_animations.len()
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &FLIPPerformanceMetrics {
        &self.performance_metrics
    }

    /// Cancel all animations
    pub fn cancel_all(&mut self) {
        self.active_animations.clear();
    }

    /// Cancel specific animation
    pub fn cancel(&mut self, id: &str) -> bool {
        self.active_animations.remove(id).is_some()
    }

    fn parse_easing_function(&self, easing: &str) -> Result<EasingFunction, String> {
        match easing {
            "linear" => Ok(EasingFunction::Linear),
            "ease-in" => Ok(EasingFunction::EaseIn),
            "ease-out" => Ok(EasingFunction::EaseOut),
            "ease-in-out" => Ok(EasingFunction::EaseInOut),
            _ => {
                if easing.starts_with("cubic-bezier(") && easing.ends_with(')') {
                    // Parse cubic-bezier values
                    let values_str = &easing[13..easing.len()-1];
                    let values: Result<Vec<f64>, _> = values_str
                        .split(',')
                        .map(|s| s.trim().parse())
                        .collect();
                    
                    match values {
                        Ok(v) if v.len() == 4 => Ok(EasingFunction::CubicBezier(v[0], v[1], v[2], v[3])),
                        _ => Err("Invalid cubic-bezier format".to_string()),
                    }
                } else if easing.starts_with("spring(") && easing.ends_with(')') {
                    // Parse spring values
                    let values_str = &easing[7..easing.len()-1];
                    let values: Result<Vec<f64>, _> = values_str
                        .split(',')
                        .map(|s| s.trim().parse())
                        .collect();
                    
                    match values {
                        Ok(v) if v.len() == 2 => Ok(EasingFunction::Spring { tension: v[0], friction: v[1] }),
                        _ => Err("Invalid spring format".to_string()),
                    }
                } else {
                    Err(format!("Unknown easing function: {}", easing))
                }
            }
        }
    }

    fn update_performance_metrics(&mut self, duration: f64) {
        let total = self.performance_metrics.total_animations as f64;
        let current_avg = self.performance_metrics.average_duration;
        self.performance_metrics.average_duration = (current_avg * (total - 1.0) + duration) / total;
        
        // Update performance score based on success rate
        let success_rate = 1.0 - (self.performance_metrics.failed_animations as f64 / total);
        self.performance_metrics.performance_score = success_rate;
    }
}

impl Default for FLIPAnimator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_transform_values_new() {
        let transform = TransformValues::new(10.0, 20.0, 1.5, 2.0, 45.0);
        assert_eq!(transform.translate_x, 10.0);
        assert_eq!(transform.translate_y, 20.0);
        assert_eq!(transform.scale_x, 1.5);
        assert_eq!(transform.scale_y, 2.0);
        assert_eq!(transform.rotation, 45.0);
    }

    #[test]
    fn test_transform_values_translation() {
        let transform = TransformValues::translation(10.0, 20.0);
        assert_eq!(transform.translate_x, 10.0);
        assert_eq!(transform.translate_y, 20.0);
        assert_eq!(transform.scale_x, 1.0);
        assert_eq!(transform.scale_y, 1.0);
        assert_eq!(transform.rotation, 0.0);
    }

    #[test]
    fn test_transform_values_scale() {
        let transform = TransformValues::scale(1.5, 2.0);
        assert_eq!(transform.translate_x, 0.0);
        assert_eq!(transform.translate_y, 0.0);
        assert_eq!(transform.scale_x, 1.5);
        assert_eq!(transform.scale_y, 2.0);
        assert_eq!(transform.rotation, 0.0);
    }

    #[test]
    fn test_transform_values_rotation() {
        let transform = TransformValues::rotation(45.0);
        assert_eq!(transform.translate_x, 0.0);
        assert_eq!(transform.translate_y, 0.0);
        assert_eq!(transform.scale_x, 1.0);
        assert_eq!(transform.scale_y, 1.0);
        assert_eq!(transform.rotation, 45.0);
    }

    #[test]
    fn test_transform_values_default() {
        let transform = TransformValues::default();
        assert_eq!(transform.translate_x, 0.0);
        assert_eq!(transform.translate_y, 0.0);
        assert_eq!(transform.scale_x, 1.0);
        assert_eq!(transform.scale_y, 1.0);
        assert_eq!(transform.rotation, 0.0);
    }

    #[test]
    fn test_easing_function_linear() {
        let easing = EasingFunction::Linear;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(0.5), 0.5);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_ease_in() {
        let easing = EasingFunction::EaseIn;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(0.5), 0.25);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_ease_out() {
        let easing = EasingFunction::EaseOut;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(0.5), 0.75);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_ease_in_out() {
        let easing = EasingFunction::EaseInOut;
        assert_eq!(easing.evaluate(0.0), 0.0);
        assert_eq!(easing.evaluate(0.25), 0.125);
        assert_eq!(easing.evaluate(0.75), 0.875);
        assert_eq!(easing.evaluate(1.0), 1.0);
    }

    #[test]
    fn test_easing_function_cubic_bezier() {
        let easing = EasingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0);
        let result = easing.evaluate(0.5);
        assert!(result > 0.0 && result < 1.0);
    }

    #[test]
    fn test_easing_function_spring() {
        let easing = EasingFunction::Spring { tension: 120.0, friction: 14.0 };
        let result = easing.evaluate(0.5);
        // Spring animations can temporarily exceed bounds due to oscillation
        assert!(result >= -0.5 && result <= 1.5, "Spring result was: {}", result);
    }

    #[test]
    fn test_easing_function_default() {
        let easing = EasingFunction::default();
        match easing {
            EasingFunction::EaseOut => {},
            _ => panic!("Default should be EaseOut"),
        }
    }

    #[test]
    fn test_flip_animator_creation() {
        let animator = FLIPAnimator::new();
        assert_eq!(animator.active_count(), 0);
        assert_eq!(animator.performance_metrics().total_animations, 0);
    }

    #[test]
    fn test_flip_animator_default() {
        let animator = FLIPAnimator::default();
        assert_eq!(animator.active_count(), 0);
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = FLIPPerformanceMetrics::default();
        assert_eq!(metrics.total_animations, 0);
        assert_eq!(metrics.average_duration, 0.0);
        assert_eq!(metrics.failed_animations, 0);
        assert_eq!(metrics.performance_score, 1.0);
    }

    #[wasm_bindgen_test]
    fn test_flip_animator_cancel_all() {
        let mut animator = FLIPAnimator::new();
        animator.cancel_all();
        assert_eq!(animator.active_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_flip_animator_cancel_nonexistent() {
        let mut animator = FLIPAnimator::new();
        assert!(!animator.cancel("nonexistent"));
    }

    #[test]
    fn test_easing_function_parsing() {
        let animator = FLIPAnimator::new();
        
        assert!(matches!(animator.parse_easing_function("linear"), Ok(EasingFunction::Linear)));
        assert!(matches!(animator.parse_easing_function("ease-in"), Ok(EasingFunction::EaseIn)));
        assert!(matches!(animator.parse_easing_function("ease-out"), Ok(EasingFunction::EaseOut)));
        assert!(matches!(animator.parse_easing_function("ease-in-out"), Ok(EasingFunction::EaseInOut)));
        
        assert!(matches!(
            animator.parse_easing_function("cubic-bezier(0.25, 0.1, 0.25, 1.0)"),
            Ok(EasingFunction::CubicBezier(0.25, 0.1, 0.25, 1.0))
        ));
        
        assert!(matches!(
            animator.parse_easing_function("spring(120, 14)"),
            Ok(EasingFunction::Spring { tension: 120.0, friction: 14.0 })
        ));
        
        assert!(animator.parse_easing_function("invalid").is_err());
    }

    #[wasm_bindgen_test]
    fn test_transform_values_calculation() {
        let animator = FLIPAnimator::new();
        
        // Create mock DomRect objects
        let first = web_sys::DomRect::new_with_x_and_y_and_width_and_height(0.0, 0.0, 100.0, 100.0).unwrap();
        let last = web_sys::DomRect::new_with_x_and_y_and_width_and_height(50.0, 25.0, 200.0, 150.0).unwrap();
        
        let transform = animator.calculate_transform_values(&first, &last);
        
        assert_eq!(transform.translate_x, -50.0); // 0 - 50
        assert_eq!(transform.translate_y, -25.0); // 0 - 25
        assert_eq!(transform.scale_x, 0.5); // 100 / 200
        assert_eq!(transform.scale_y, 100.0 / 150.0); // 100 / 150
    }
}