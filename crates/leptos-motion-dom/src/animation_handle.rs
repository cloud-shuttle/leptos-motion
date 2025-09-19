//! Animation Handle
//!
//! This module provides the DomAnimationHandle struct for controlling animations.
//! It acts as a safe wrapper around animation IDs and provides methods for
//! controlling animation lifecycle.

use crate::animation_trait::{Animation, AnimationError, AnimationResult};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

/// Handle for controlling DOM animations
#[derive(Debug, Clone)]
pub struct DomAnimationHandle {
    /// Unique animation ID
    pub id: String,
    /// Weak reference to animation manager
    manager: Weak<RefCell<AnimationManager>>,
}

/// Animation manager for tracking and controlling animations
pub struct AnimationManager {
    /// Map of animation ID to animation instance
    animations: HashMap<String, Rc<RefCell<Box<dyn Animation>>>>,
    /// Next available animation ID
    next_id: u64,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Register a new animation
    pub fn register(&mut self, mut animation: Box<dyn Animation>) -> AnimationResult<DomAnimationHandle> {
        let id = animation.id().to_string();
        
        // Check if animation already exists
        if self.animations.contains_key(&id) {
            return Err(AnimationError::AlreadyRunning(id));
        }
        
        // Start the animation
        animation.start()?;
        
        // Store the animation
        let animation_rc = Rc::new(RefCell::new(animation));
        self.animations.insert(id.clone(), animation_rc);
        
        // Create handle
        let handle = DomAnimationHandle {
            id,
            manager: Weak::new(), // Will be set by the caller
        };
        
        Ok(handle)
    }
    
    /// Unregister an animation
    pub fn unregister(&mut self, handle: DomAnimationHandle) -> AnimationResult<()> {
        if let Some(animation_rc) = self.animations.remove(&handle.id) {
            // Stop the animation
            if let Ok(mut animation) = animation_rc.try_borrow_mut() {
                let _ = animation.stop();
            }
            Ok(())
        } else {
            Err(AnimationError::NotFound(handle.id.clone()))
        }
    }
    
    /// Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<Weak<RefCell<Box<dyn Animation>>>> {
        self.animations.get(id).map(|rc| Rc::downgrade(rc))
    }
    
    /// Check if animation exists
    pub fn has_animation(&self, id: &str) -> bool {
        self.animations.contains_key(id)
    }
    
    /// Get all animation IDs
    pub fn get_animation_ids(&self) -> Vec<String> {
        self.animations.keys().cloned().collect()
    }
    
    /// Get number of active animations
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }
    
    /// Update all animations
    pub fn update_all(&mut self, delta_time: f64) -> AnimationResult<()> {
        let mut completed_animations = Vec::new();
        
        // Update all animations
        for (id, animation_rc) in &self.animations {
            if let Ok(mut animation) = animation_rc.try_borrow_mut() {
                if let Err(e) = animation.update(delta_time) {
                    eprintln!("Animation update error for {}: {:?}", id, e);
                }
                
                // Check if animation is complete
                if animation.is_complete() {
                    completed_animations.push(id.clone());
                }
            }
        }
        
        // Remove completed animations
        for id in completed_animations {
            if let Some(animation_rc) = self.animations.remove(&id) {
                if let Ok(mut animation) = animation_rc.try_borrow_mut() {
                    let _ = animation.stop();
                }
            }
        }
        
        Ok(())
    }
    
    /// Stop all animations
    pub fn stop_all(&mut self) -> AnimationResult<()> {
        let mut errors = Vec::new();
        
        for (id, animation_rc) in &self.animations {
            if let Ok(mut animation) = animation_rc.try_borrow_mut() {
                if let Err(e) = animation.stop() {
                    errors.push(format!("Failed to stop animation {}: {}", id, e));
                }
            }
        }
        
        self.animations.clear();
        
        if !errors.is_empty() {
            return Err(AnimationError::Generic(errors.join(", ")));
        }
        
        Ok(())
    }
    
    /// Generate next unique ID
    pub fn generate_id(&mut self) -> String {
        let id = format!("anim_{}", self.next_id);
        self.next_id += 1;
        id
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DomAnimationHandle {
    /// Create a new animation handle
    pub fn new(id: String, manager: Weak<RefCell<AnimationManager>>) -> Self {
        Self { id, manager }
    }
    
    /// Stop the animation
    pub fn stop(self) -> AnimationResult<()> {
        if let Some(manager) = self.manager.upgrade() {
            manager.borrow_mut().unregister(self)
        } else {
            Err(AnimationError::EngineUnavailable("Animation manager dropped".to_string()))
        }
    }
    
    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        if let Some(manager) = self.manager.upgrade() {
            if let Some(animation_weak) = manager.borrow().get_animation(&self.id) {
                if let Some(animation_rc) = animation_weak.upgrade() {
                    if let Ok(animation) = animation_rc.try_borrow() {
                        return animation.is_running() && !animation.is_complete();
                    }
                }
            }
        }
        false
    }
    
    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        if let Some(manager) = self.manager.upgrade() {
            if let Some(animation_weak) = manager.borrow().get_animation(&self.id) {
                if let Some(animation_rc) = animation_weak.upgrade() {
                    if let Ok(animation) = animation_rc.try_borrow() {
                        return animation.is_complete();
                    }
                }
            }
        }
        false
    }
    
    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if let Some(manager) = self.manager.upgrade() {
            if let Some(animation_weak) = manager.borrow().get_animation(&self.id) {
                if let Some(animation_rc) = animation_weak.upgrade() {
                    if let Ok(animation) = animation_rc.try_borrow() {
                        return animation.progress();
                    }
                }
            }
        }
        0.0
    }
    
    /// Get animation duration
    pub fn duration(&self) -> f64 {
        if let Some(manager) = self.manager.upgrade() {
            if let Some(animation_weak) = manager.borrow().get_animation(&self.id) {
                if let Some(animation_rc) = animation_weak.upgrade() {
                    if let Ok(animation) = animation_rc.try_borrow() {
                        return animation.duration();
                    }
                }
            }
        }
        0.0
    }
    
    /// Get animation ID
    pub fn id(&self) -> &str {
        &self.id
    }
}

impl Drop for DomAnimationHandle {
    fn drop(&mut self) {
        // Automatically stop animation when handle is dropped
        if let Some(manager) = self.manager.upgrade() {
            let _ = manager.borrow_mut().unregister(self.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation_trait::AnimationConfig;
    
    // Mock animation for testing
    struct MockAnimation {
        id: String,
        duration: f64,
        is_complete: bool,
        is_running: bool,
        progress: f64,
    }
    
    impl MockAnimation {
        fn new(id: String, duration: f64) -> Self {
            Self {
                id,
                duration,
                is_complete: false,
                is_running: false,
                progress: 0.0,
            }
        }
    }
    
    impl Animation for MockAnimation {
        fn start(&mut self) -> AnimationResult<()> {
            self.is_running = true;
            self.is_complete = false;
            self.progress = 0.0;
            Ok(())
        }
        
        fn stop(&mut self) -> AnimationResult<()> {
            self.is_running = false;
            Ok(())
        }
        
        fn is_complete(&self) -> bool {
            self.is_complete
        }
        
        fn progress(&self) -> f64 {
            self.progress
        }
        
        fn update(&mut self, delta_time: f64) -> AnimationResult<()> {
            if self.is_running && !self.is_complete {
                self.progress += delta_time / self.duration;
                if self.progress >= 1.0 {
                    self.progress = 1.0;
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
            self.duration
        }
        
        fn is_running(&self) -> bool {
            self.is_running
        }
    }
    
    #[test]
    fn test_animation_manager_creation() {
        let manager = AnimationManager::new();
        assert_eq!(manager.active_count(), 0);
        assert!(manager.get_animation_ids().is_empty());
    }
    
    #[test]
    fn test_animation_registration() {
        let mut manager = AnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let handle = manager.register(animation).unwrap();
        assert_eq!(handle.id(), "test");
        assert_eq!(manager.active_count(), 1);
        assert!(manager.has_animation("test"));
    }
    
    #[test]
    fn test_animation_unregistration() {
        let mut manager = AnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let handle = manager.register(animation).unwrap();
        assert_eq!(manager.active_count(), 1);
        
        manager.unregister(handle).unwrap();
        assert_eq!(manager.active_count(), 0);
        assert!(!manager.has_animation("test"));
    }
    
    #[test]
    fn test_animation_handle() {
        // Test that we can create an DomAnimationHandle without RefCell issues
        let handle = DomAnimationHandle::new("test_handle".to_string(), Weak::new());
        
        // Test that the handle has the correct ID
        assert_eq!(handle.id(), "test_handle");
        
        // Test that methods return reasonable defaults when manager is not available
        assert_eq!(handle.progress(), 0.0);
        assert_eq!(handle.duration(), 0.0);
        assert!(!handle.is_running());
        assert!(!handle.is_complete());
    }
    
    #[test]
    fn test_animation_update() {
        let mut manager = AnimationManager::new();
        let animation = Box::new(MockAnimation::new("test".to_string(), 1.0));
        
        let _handle = manager.register(animation).unwrap();
        
        // Update animation
        manager.update_all(0.5).unwrap();
        
        // Check progress
        if let Some(animation_weak) = manager.get_animation("test") {
            if let Some(animation_rc) = animation_weak.upgrade() {
                if let Ok(animation) = animation_rc.try_borrow() {
                    assert_eq!(animation.progress(), 0.5);
                }
            }
        }
    }
}
