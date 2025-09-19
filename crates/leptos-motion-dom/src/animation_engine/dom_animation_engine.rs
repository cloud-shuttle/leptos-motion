//! DOM-specific AnimationEngine implementation

use std::collections::HashMap;
use leptos_motion_core::{AnimationEngine, AnimationValue, Result, Transition, PlaybackState, AnimationConfig};
use leptos_motion_core::AnimationHandle as CoreAnimationHandle;
use crate::animation_trait::{Animation, AnimationError, AnimationResult};

/// DOM-specific implementation of AnimationEngine
pub struct DomAnimationEngine {
    /// Active animations
    animations: HashMap<String, AnimationValue>,
    /// Animation handles
    handles: HashMap<CoreAnimationHandle, String>,
    /// Next handle ID
    next_handle_id: u64,
}

impl DomAnimationEngine {
    /// Create a new DOM animation engine
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            handles: HashMap::new(),
            next_handle_id: 1,
        }
    }

    /// Get all animation values
    pub fn get_all_values(&self) -> &HashMap<String, AnimationValue> {
        &self.animations
    }

    /// Animate a property
    pub fn animate_property(&mut self, property: String, from: f64, to: f64, transition: Transition) -> AnimationResult<()> {
        if property.is_empty() {
            return Err(AnimationError::InvalidConfig("Property name cannot be empty".to_string()));
        }

        let value = AnimationValue::Number(to);
        self.animations.insert(property, value);
        Ok(())
    }

    /// Get a property value
    pub fn get_property_value(&self, property: &str) -> Option<&AnimationValue> {
        self.animations.get(property)
    }

    /// Set a property value
    pub fn set_property_value(&mut self, property: String, value: AnimationValue) {
        self.animations.insert(property, value);
    }

    /// Remove a property
    pub fn remove_property(&mut self, property: &str) -> Option<AnimationValue> {
        self.animations.remove(property)
    }

    /// Clear all properties
    pub fn clear_all(&mut self) {
        self.animations.clear();
        self.handles.clear();
    }

    /// Get animation count
    pub fn animation_count(&self) -> usize {
        self.animations.len()
    }

    /// Check if has active animations
    pub fn has_active_animations(&self) -> bool {
        !self.animations.is_empty()
    }
}

impl Default for DomAnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationEngine for DomAnimationEngine {
    fn is_available(&self) -> bool {
        true // DOM is always available in browser context
    }

    fn animate(&mut self, _animation: &AnimationConfig) -> Result<CoreAnimationHandle> {
        let handle = CoreAnimationHandle::new(self.next_handle_id);
        self.next_handle_id += 1;
        self.handles.insert(handle, "dom_animation".to_string());
        Ok(handle)
    }

    fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        self.handles.remove(&handle);
        Ok(())
    }

    fn pause(&mut self, _handle: AnimationHandle) -> Result<()> {
        // DOM animations pause automatically when element is hidden
        Ok(())
    }

    fn resume(&mut self, _handle: AnimationHandle) -> Result<()> {
        // DOM animations resume automatically when element is visible
        Ok(())
    }

    fn tick(&mut self, _timestamp: f64) -> Result<()> {
        // DOM animations are handled by the browser
        Ok(())
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if self.handles.contains_key(&handle) {
            Ok(PlaybackState::Running)
        } else {
            Ok(PlaybackState::Idle)
        }
    }

    fn is_running(&self, handle: AnimationHandle) -> bool {
        self.handles.contains_key(&handle)
    }

    fn get_performance_metrics(&self) -> Option<()> {
        None // Not implemented yet
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dom_animation_engine_creation() {
        let engine = DomAnimationEngine::new();
        assert_eq!(engine.animation_count(), 0);
        assert!(!engine.has_active_animations());
    }

    #[test]
    fn test_animate_property() {
        let mut engine = DomAnimationEngine::new();
        let result = engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_ok());
        assert_eq!(engine.animation_count(), 1);
        assert!(engine.has_active_animations());
    }

    #[test]
    fn test_empty_property_name() {
        let mut engine = DomAnimationEngine::new();
        let result = engine.animate_property("".to_string(), 0.0, 1.0, Transition::default());
        assert!(result.is_err());
    }

    #[test]
    fn test_get_property_value() {
        let mut engine = DomAnimationEngine::new();
        engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default()).unwrap();
        
        let value = engine.get_property_value("opacity");
        assert!(value.is_some());
        assert_eq!(value.unwrap(), &AnimationValue::Number(1.0));
    }

    #[test]
    fn test_clear_all() {
        let mut engine = DomAnimationEngine::new();
        engine.animate_property("opacity".to_string(), 0.0, 1.0, Transition::default()).unwrap();
        engine.animate_property("scale".to_string(), 1.0, 2.0, Transition::default()).unwrap();
        
        assert_eq!(engine.animation_count(), 2);
        engine.clear_all();
        assert_eq!(engine.animation_count(), 0);
        assert!(!engine.has_active_animations());
    }
}
