//! Motion values for reactive animation state

use leptos::prelude::*;
use std::collections::HashMap;
use crate::AnimationValue;

/// Reactive motion value that tracks animation state
pub struct MotionValue<T: Clone + Send + Sync + 'static> {
    value: RwSignal<T>,
    velocity: RwSignal<f64>,
    subscribers: std::sync::Mutex<Vec<Box<dyn Fn(&T) + Send + Sync + 'static>>>,
}

impl<T: Clone + Send + Sync + 'static> MotionValue<T> {
    /// Create a new motion value
    pub fn new(initial: T) -> Self {
        Self {
            value: RwSignal::new(initial),
            velocity: RwSignal::new(0.0),
            subscribers: std::sync::Mutex::new(Vec::new()),
        }
    }
    
    /// Get current value
    pub fn get(&self) -> T {
        self.value.get()
    }
    
    /// Set value (triggers subscribers)
    pub fn set(&self, value: T) {
        self.value.set(value.clone());
        self.notify_subscribers(&value);
    }
    
    /// Update value with a function
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.value.update(f);
        let value = self.value.get();
        self.notify_subscribers(&value);
    }
    
    /// Get current velocity
    pub fn get_velocity(&self) -> f64 {
        self.velocity.get()
    }
    
    /// Set velocity
    pub fn set_velocity(&self, velocity: f64) {
        self.velocity.set(velocity);
    }
    
    /// Set value with velocity
    pub fn set_with_velocity(&self, value: T, velocity: f64) {
        self.set_velocity(velocity);
        self.set(value);
    }
    
    /// Subscribe to value changes
    pub fn subscribe(&self, callback: impl Fn(&T) + Send + Sync + 'static) {
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push(Box::new(callback));
        }
    }
    
    fn notify_subscribers(&self, value: &T) {
        if let Ok(subs) = self.subscribers.lock() {
            for callback in subs.iter() {
                callback(value);
            }
        }
    }
}

/// Specialized motion value for f64 values
pub type MotionNumber = MotionValue<f64>;

impl MotionValue<f64> {
    /// Create a numeric motion value starting at 0
    pub fn zero() -> Self {
        Self::new(0.0)
    }
    
    /// Increment the value
    pub fn increment(&self, amount: f64) {
        self.update(|v: &mut f64| *v += amount);
    }
    
    /// Decrement the value
    pub fn decrement(&self, amount: f64) {
        self.update(|v: &mut f64| *v -= amount);
    }
    
    /// Animate to a target value
    pub fn animate_to(&self, target: f64, _duration: f64) {
        // This would trigger an animation to the target value
        // For now, just set the value directly
        self.set(target);
    }
}

/// Specialized motion value for transform values
pub type MotionTransform = MotionValue<crate::Transform>;

impl MotionValue<crate::Transform> {
    /// Create a transform motion value at identity
    pub fn identity() -> Self {
        Self::new(crate::Transform::default())
    }
    
    /// Set translation
    pub fn set_translate(&self, x: f64, y: f64) {
        self.update(|t: &mut crate::Transform| {
            t.x = Some(x);
            t.y = Some(y);
        });
    }
    
    /// Set rotation
    pub fn set_rotation(&self, degrees: f64) {
        self.update(|t: &mut crate::Transform| {
            t.rotate_z = Some(degrees);
        });
    }
    
    /// Set scale
    pub fn set_scale(&self, scale: f64) {
        self.update(|t: &mut crate::Transform| {
            t.scale = Some(scale);
        });
    }
}

/// Collection of motion values for complex animations
pub struct MotionValues {
    values: HashMap<String, MotionValue<AnimationValue>>,
}

impl MotionValues {
    /// Create a new motion values collection
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    
    /// Add a motion value
    pub fn add(&mut self, key: impl Into<String>, initial: AnimationValue) {
        self.values.insert(key.into(), MotionValue::new(initial));
    }
    
    /// Get a motion value by key
    pub fn get(&self, key: &str) -> Option<&MotionValue<AnimationValue>> {
        self.values.get(key)
    }
    
    /// Set value for a key
    pub fn set(&self, key: &str, value: AnimationValue) {
        if let Some(motion_value) = self.values.get(key) {
            motion_value.set(value);
        }
    }
    
    /// Get all current values as AnimationTarget
    pub fn get_all(&self) -> crate::AnimationTarget {
        let mut result = HashMap::new();
        for (key, motion_value) in &self.values {
            result.insert(key.clone(), motion_value.get());
        }
        result
    }
    
    /// Set all values from AnimationTarget
    pub fn set_all(&self, target: &crate::AnimationTarget) {
        for (key, value) in target {
            self.set(key, value.clone());
        }
    }
}

impl Default for MotionValues {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_motion_value() {
        let motion_value = MotionValue::new(42.0);
        assert_eq!(motion_value.get(), 42.0);
        
        motion_value.set(100.0);
        assert_eq!(motion_value.get(), 100.0);
        
        motion_value.update(|v| *v *= 2.0);
        assert_eq!(motion_value.get(), 200.0);
    }
    
    #[test]
    fn test_motion_number() {
        let motion_num = MotionNumber::zero();
        assert_eq!(motion_num.get(), 0.0);
        
        motion_num.increment(5.0);
        assert_eq!(motion_num.get(), 5.0);
        
        motion_num.decrement(2.0);
        assert_eq!(motion_num.get(), 3.0);
    }
    
    #[test]
    fn test_motion_transform() {
        let motion_transform = MotionTransform::identity();
        assert!(motion_transform.get().is_identity());
        
        motion_transform.set_translate(10.0, 20.0);
        let transform = motion_transform.get();
        assert_eq!(transform.x, Some(10.0));
        assert_eq!(transform.y, Some(20.0));
    }
    
    #[test]
    fn test_motion_values_collection() {
        let mut values = MotionValues::new();
        values.add("opacity", AnimationValue::Number(0.5));
        values.add("x", AnimationValue::Pixels(100.0));
        
        assert!(values.get("opacity").is_some());
        assert!(values.get("x").is_some());
        assert!(values.get("nonexistent").is_none());
        
        values.set("opacity", AnimationValue::Number(1.0));
        if let Some(opacity) = values.get("opacity") {
            assert_eq!(opacity.get(), AnimationValue::Number(1.0));
        }
        
        let all_values = values.get_all();
        assert_eq!(all_values.len(), 2);
    }
}