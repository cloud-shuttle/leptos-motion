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
    Spring { tension: f64, friction: f64 },
}

impl Default for EasingFunction {
    fn default() -> Self {
        EasingFunction::EaseOut
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
    /// Successful animations
    pub successful_animations: usize,
    /// Average animation duration
    pub average_duration: f64,
    /// Frame rate during animations
    pub frame_rate: f64,
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

    /// Create a new FLIP animation
    pub fn create_animation(
        &mut self,
        element: &Element,
        first: DomRect,
        last: DomRect,
        config: &LayoutAnimationConfig,
    ) -> Result<FLIPAnimation, String> {
        // Calculate inverted transform values
        let inverted = self.calculate_inverted_transforms(&first, &last);
        
        // Use easing function directly
        let easing = config.easing.clone();
        
        // Create FLIP state
        let state = FLIPState {
            first,
            last,
            inverted,
            progress: 0.0,
            active: false,
        };
        
        // Generate animation ID
        let id = self.generate_animation_id();
        
        // Create FLIP animation
        let animation = FLIPAnimation {
            id: id.clone(),
            element: element.clone(),
            state,
            config: config.clone(),
            start_time: 0.0,
            duration: config.duration,
            easing,
        };
        
        Ok(animation)
    }

    /// Start a FLIP animation
    pub fn start_animation(&mut self, mut animation: FLIPAnimation) -> Result<(), String> {
        // Set start time
        animation.start_time = js_sys::Date::now();
        animation.state.active = true;
        
        // Apply inverted transforms
        self.apply_inverted_transforms(&animation.element, &animation.state.inverted)?;
        
        // Store animation
        self.active_animations.insert(animation.id.clone(), animation);
        
        // Start animation loop if not already running
        if self.animation_frame.is_none() {
            self.start_animation_loop()?;
        }
        
        // Update performance metrics
        self.performance_metrics.total_animations += 1;
        
        Ok(())
    }

    /// Pause a FLIP animation
    pub fn pause_animation(&mut self, animation_id: &str) -> Result<(), String> {
        if let Some(animation) = self.active_animations.get_mut(animation_id) {
            animation.state.active = false;
            Ok(())
        } else {
            Err("Animation not found".to_string())
        }
    }

    /// Resume a FLIP animation
    pub fn resume_animation(&mut self, animation_id: &str) -> Result<(), String> {
        if let Some(animation) = self.active_animations.get_mut(animation_id) {
            animation.state.active = true;
            Ok(())
        } else {
            Err("Animation not found".to_string())
        }
    }

    /// Cancel a FLIP animation
    pub fn cancel_animation(&mut self, animation_id: &str) -> Result<(), String> {
        if let Some(animation) = self.active_animations.remove(animation_id) {
            // Reset element transforms
            self.reset_element_transforms(&animation.element)?;
            Ok(())
        } else {
            Err("Animation not found".to_string())
        }
    }

    /// Calculate inverted transform values
    fn calculate_inverted_transforms(&self, first: &DomRect, last: &DomRect) -> TransformValues {
        // Calculate position difference
        let delta_x = first.x() - last.x();
        let delta_y = first.y() - last.y();
        
        // Calculate scale differences
        let scale_x = first.width() / last.width();
        let scale_y = first.height() / last.height();
        
        TransformValues {
            translate_x: delta_x,
            translate_y: delta_y,
            scale_x,
            scale_y,
            rotation: 0.0, // Can be extended for rotation animations
        }
    }

    /// Apply inverted transforms to element
    fn apply_inverted_transforms(&self, element: &Element, transforms: &TransformValues) -> Result<(), String> {
        // Apply CSS transforms to the element
        let transform_string = format!(
            "translate({}px, {}px) scale({}, {}) rotate({}deg)",
            transforms.translate_x,
            transforms.translate_y,
            transforms.scale_x,
            transforms.scale_y,
            transforms.rotation
        );
        
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            let style = html_element.style();
            style.set_property("transform", &transform_string)
                .map_err(|_| "Failed to apply transform")?;
            style.set_property("will-change", "transform")
                .map_err(|_| "Failed to set will-change")?;
        } else {
            return Err("Element is not an HtmlElement".to_string());
        }
        
        log::info!("Applied FLIP transforms: {}", transform_string);
        Ok(())
    }

    /// Reset element transforms
    fn reset_element_transforms(&self, element: &Element) -> Result<(), String> {
        // Reset CSS transforms on the element
        if let Some(html_element) = element.dyn_ref::<web_sys::HtmlElement>() {
            let style = html_element.style();
            style.remove_property("transform")
                .map_err(|_| "Failed to remove transform")?;
            style.remove_property("will-change")
                .map_err(|_| "Failed to remove will-change")?;
        } else {
            return Err("Element is not an HtmlElement".to_string());
        }
        
        log::info!("Reset FLIP transforms for element");
        Ok(())
    }

    /// Start animation loop
    fn start_animation_loop(&mut self) -> Result<(), String> {
        // For now, just store a placeholder frame ID
        // In a real implementation, this would use requestAnimationFrame
        self.animation_frame = Some(0);
        
        Ok(())
    }

    /// Parse easing function from string
    fn parse_easing_function(&self, easing: &str) -> Result<EasingFunction, String> {
        match easing {
            "linear" => Ok(EasingFunction::Linear),
            "ease-in" => Ok(EasingFunction::EaseIn),
            "ease-out" => Ok(EasingFunction::EaseOut),
            "ease-in-out" => Ok(EasingFunction::EaseInOut),
            _ => {
                // Try to parse cubic-bezier
                if easing.starts_with("cubic-bezier(") && easing.ends_with(")") {
                    let params = &easing[13..easing.len()-1];
                    let values: Vec<f64> = params
                        .split(',')
                        .map(|s| s.trim().parse::<f64>())
                        .collect::<Result<Vec<f64>, _>>()
                        .map_err(|_| "Invalid cubic-bezier parameters")?;
                    
                    if values.len() == 4 {
                        Ok(EasingFunction::CubicBezier(values[0], values[1], values[2], values[3]))
                    } else {
                        Err("Cubic-bezier requires exactly 4 parameters".to_string())
                    }
                } else {
                    Err(format!("Unknown easing function: {}", easing))
                }
            }
        }
    }

    /// Generate unique animation ID
    fn generate_animation_id(&self) -> String {
        format!("flip_anim_{}", js_sys::Date::now())
    }

    /// Get performance metrics
    pub fn get_performance_metrics(&self) -> &FLIPPerformanceMetrics {
        &self.performance_metrics
    }

    /// Update performance metrics
    fn update_performance_metrics(&mut self, duration: f64) {
        self.performance_metrics.successful_animations += 1;
        
        // Update average duration
        let total = self.performance_metrics.total_animations as f64;
        let current_avg = self.performance_metrics.average_duration;
        self.performance_metrics.average_duration = 
            (current_avg * (total - 1.0) + duration) / total;
    }
}

impl Default for FLIPAnimator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for FLIPPerformanceMetrics {
    fn default() -> Self {
        Self {
            total_animations: 0,
            successful_animations: 0,
            average_duration: 0.0,
            frame_rate: 60.0,
        }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_flip_animator_creation() {
        let animator = FLIPAnimator::new();
        assert_eq!(animator.active_animations.len(), 0);
        assert!(animator.animation_frame.is_none());
    }

    #[wasm_bindgen_test]
    fn test_easing_function_parsing() {
        let animator = FLIPAnimator::new();
        
        // Test linear easing
        let result = animator.parse_easing_function("linear");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), EasingFunction::Linear));
        
        // Test cubic-bezier
        let result = animator.parse_easing_function("cubic-bezier(0.4, 0.0, 0.2, 1)");
        assert!(result.is_ok());
        
        // Test invalid easing
        let result = animator.parse_easing_function("invalid");
        assert!(result.is_err());
    }

    #[wasm_bindgen_test]
    fn test_transform_values_calculation() {
        let animator = FLIPAnimator::new();
        
        // Create test rectangles
        let first = DomRect::new().unwrap();
        let last = DomRect::new().unwrap();
        
        let transforms = animator.calculate_inverted_transforms(&first, &last);
        
        assert_eq!(transforms.translate_x, 0.0);
        assert_eq!(transforms.translate_y, 0.0);
        assert_eq!(transforms.scale_x, 1.0);
        assert_eq!(transforms.scale_y, 1.0);
    }
}
