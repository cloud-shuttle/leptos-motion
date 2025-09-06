//! Minimal animation engine for bundle size optimization
//!
//! This engine only includes the essential animation functionality
//! without performance monitoring, complex scheduling, or advanced features.

use crate::{AnimationHandle, AnimationTarget, Result, Transition};
use std::collections::HashMap;
#[cfg(feature = "web-sys")]
use web_sys::window;

/// Minimal animation engine with only essential features
pub struct MinimalEngine {
    animations: HashMap<AnimationHandle, MinimalAnimation>,
    current_handle: u64,
}

struct MinimalAnimation {
    _target: AnimationTarget,
    _transition: Transition,
    _start_time: f64,
    _duration: f64,
}

impl MinimalEngine {
    /// Create a new minimal engine instance
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            current_handle: 0,
        }
    }

    /// Start a new animation
    pub fn animate(
        &mut self,
        target: AnimationTarget,
        transition: Transition,
    ) -> Result<AnimationHandle> {
        let handle = AnimationHandle(self.current_handle);
        self.current_handle += 1;

        let duration = transition.duration.unwrap_or(0.3);
        #[cfg(feature = "web-sys")]
        let start_time = window().unwrap().performance().unwrap().now();
        #[cfg(not(feature = "web-sys"))]
        let start_time = 0.0;

        let animation = MinimalAnimation {
            _target: target,
            _transition: transition,
            _start_time: start_time,
            _duration: duration,
        };

        self.animations.insert(handle, animation);
        Ok(handle)
    }

    /// Stop an animation
    pub fn stop(&mut self, handle: AnimationHandle) -> Result<()> {
        self.animations.remove(&handle);
        Ok(())
    }

    /// Check if an animation is running
    pub fn is_running(&self, handle: AnimationHandle) -> bool {
        self.animations.contains_key(&handle)
    }

    /// Get the number of active animations
    pub fn active_count(&self) -> usize {
        self.animations.len()
    }
}

impl Default for MinimalEngine {
    fn default() -> Self {
        Self::new()
    }
}
