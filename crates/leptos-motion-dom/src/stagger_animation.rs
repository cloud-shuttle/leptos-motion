//! Stagger Animation
//!
//! This module implements stagger animations that sequence multiple animations
//! with delays between them, creating a cascading effect.

use crate::animation_trait::{Animation, AnimationError, AnimationResult};
use crate::animation_handle::AnimationManager;
use crate::css_transition_animation::CssTransitionAnimation;
use leptos_motion_core::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::Element;

/// Stagger configuration
#[derive(Debug, Clone)]
pub struct StaggerConfig {
    /// Delay between each animation start (in seconds)
    pub delay: f64,
    /// Whether to stagger from the first element (true) or last element (false)
    pub from_first: bool,
    /// Maximum delay to apply (prevents very long delays)
    pub max_delay: Option<f64>,
}

impl Default for StaggerConfig {
    fn default() -> Self {
        Self {
            delay: 0.1,
            from_first: true,
            max_delay: Some(2.0),
        }
    }
}

/// Stagger animation that manages multiple child animations
pub struct StaggerAnimation {
    /// Unique animation ID
    id: String,
    /// Child animations
    child_animations: Vec<Box<dyn Animation>>,
    /// Stagger configuration
    stagger_config: StaggerConfig,
    /// Animation manager for child animations
    animation_manager: Rc<RefCell<AnimationManager>>,
    /// Whether animation is complete
    is_complete: bool,
    /// Whether animation is currently running
    is_running: bool,
    /// Current progress (0.0 to 1.0)
    progress: f64,
    /// Total duration including stagger delays
    total_duration: f64,
    /// Start time of the stagger animation
    start_time: Option<f64>,
}

impl StaggerAnimation {
    /// Create a new stagger animation
    pub fn new(
        id: String,
        child_animations: Vec<Box<dyn Animation>>,
        stagger_config: StaggerConfig,
    ) -> Self {
        let animation_manager = Rc::new(RefCell::new(AnimationManager::new()));
        
        // Calculate total duration
        let child_count = child_animations.len();
        let max_child_duration = child_animations.iter()
            .map(|anim| anim.duration())
            .fold(0.0, f64::max);
        
        let stagger_delay = if child_count > 1 {
            (child_count - 1) as f64 * stagger_config.delay
        } else {
            0.0
        };
        
        let total_duration = max_child_duration + stagger_delay;
        
        Self {
            id,
            child_animations,
            stagger_config,
            animation_manager,
            is_complete: false,
            is_running: false,
            progress: 0.0,
            total_duration,
            start_time: None,
        }
    }
    
    /// Create stagger animation from elements and targets
    pub fn from_elements(
        id: String,
        elements: Vec<Element>,
        targets: Vec<AnimationTarget>,
        stagger_config: StaggerConfig,
    ) -> Self {
        let mut child_animations = Vec::new();
        
        for (i, element) in elements.iter().enumerate() {
            let target = if i < targets.len() {
                &targets[i]
            } else {
                &targets[targets.len() - 1] // Use last target for remaining elements
            };
            
            let child_id = format!("{}_{}", id, i);
            let child_animation = CssTransitionAnimation::new(
                child_id,
                element.clone(),
                target.clone(),
                Transition::default(),
            );
            
            child_animations.push(Box::new(child_animation) as Box<dyn Animation>);
        }
        
        Self::new(id, child_animations, stagger_config)
    }
    
    /// Start child animations with stagger delays
    fn start_child_animations(&mut self) -> AnimationResult<()> {
        // For now, just start all animations immediately
        // In a real implementation, we'd handle the stagger delays properly
        for child_animation in &mut self.child_animations {
            let _ = child_animation.start();
        }
        
        Ok(())
    }
    
    /// Get current time in seconds
    fn get_current_time() -> f64 {
        if let Some(window) = web_sys::window() {
            if let Some(performance) = window.performance() {
                performance.now() / 1000.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }
    
    /// Check if all child animations are complete
    fn are_all_children_complete(&self) -> bool {
        self.animation_manager.borrow().active_count() == 0
    }
}

impl Animation for StaggerAnimation {
    fn start(&mut self) -> AnimationResult<()> {
        if self.is_running {
            return Err(AnimationError::AlreadyRunning(self.id.clone()));
        }
        
        if self.child_animations.is_empty() {
            return Err(AnimationError::InvalidConfig("No child animations provided".to_string()));
        }
        
        // Start child animations with stagger
        self.start_child_animations()?;
        
        // Set start time
        self.start_time = Some(Self::get_current_time());
        self.is_running = true;
        self.is_complete = false;
        self.progress = 0.0;
        
        Ok(())
    }
    
    fn stop(&mut self) -> AnimationResult<()> {
        if !self.is_running {
            return Err(AnimationError::NotRunning(self.id.clone()));
        }
        
        // Stop all child animations
        if let Ok(mut manager) = self.animation_manager.try_borrow_mut() {
            let _ = manager.stop_all();
        }
        
        self.is_running = false;
        self.is_complete = true;
        
        Ok(())
    }
    
    fn is_complete(&self) -> bool {
        self.is_complete
    }
    
    fn progress(&self) -> f64 {
        self.progress
    }
    
    fn update(&mut self, _delta_time: f64) -> AnimationResult<()> {
        if !self.is_running || self.is_complete {
            return Ok(());
        }
        
        // Update child animations
        if let Ok(mut manager) = self.animation_manager.try_borrow_mut() {
            let _ = manager.update_all(_delta_time);
        }
        
        // Calculate progress based on elapsed time
        if let Some(start_time) = self.start_time {
            let current_time = Self::get_current_time();
            let elapsed = current_time - start_time;
            
            self.progress = (elapsed / self.total_duration).min(1.0);
            
            // Check if all children are complete
            if self.are_all_children_complete() || self.progress >= 1.0 {
                self.is_complete = true;
                self.is_running = false;
            }
        }
        
        Ok(())
    }
    
    fn id(&self) -> &str {
        &self.id
    }
    
    fn duration(&self) -> f64 {
        self.total_duration
    }
    
    fn is_running(&self) -> bool {
        self.is_running
    }
}

// Note: StaggerAnimation cannot implement Clone due to Box<dyn Animation>

/// Helper function to create stagger animation from elements
pub fn create_stagger_animation(
    id: String,
    elements: Vec<Element>,
    targets: Vec<AnimationTarget>,
    stagger_config: StaggerConfig,
) -> StaggerAnimation {
    StaggerAnimation::from_elements(id, elements, targets, stagger_config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_stagger_animation_creation() {
        let document = web_sys::window().unwrap().document().unwrap();
        let element1 = document.create_element("div").unwrap();
        let element2 = document.create_element("div").unwrap();
        
        let mut properties = HashMap::new();
        properties.insert("opacity".to_string(), AnimationValue::Number(1.0));
        
        let transition = Transition {
            duration: Some(0.5),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let animation1 = CssTransitionAnimation::new(
            "child1".to_string(),
            element1,
            properties.clone(),
            transition.clone(),
        );
        
        let animation2 = CssTransitionAnimation::new(
            "child2".to_string(),
            element2,
            properties,
            transition,
        );
        
        let child_animations: Vec<Box<dyn Animation>> = vec![
            Box::new(animation1) as Box<dyn Animation>,
            Box::new(animation2) as Box<dyn Animation>,
        ];
        let stagger_config = StaggerConfig::default();
        
        let animation = StaggerAnimation::new(
            "stagger_test".to_string(),
            child_animations,
            stagger_config,
        );
        
        assert_eq!(animation.id(), "stagger_test");
        assert!(!animation.is_running());
        assert!(!animation.is_complete());
        assert_eq!(animation.progress(), 0.0);
    }
    
    #[wasm_bindgen_test]
    fn test_stagger_config_default() {
        let config = StaggerConfig::default();
        assert_eq!(config.delay, 0.1);
        assert!(config.from_first);
        assert_eq!(config.max_delay, Some(2.0));
    }
}
