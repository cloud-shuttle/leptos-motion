//! Pooled animation data structure

use super::animation_types::*;
use crate::{Result, StudioError, timeline::AnimationValue};

/// Pooled animation that can be reused
#[derive(Debug)]
pub struct PooledAnimation {
    /// Unique identifier
    pub id: u64,
    /// Animation type
    pub animation_type: AnimationType,
    /// Current state
    pub state: AnimationState,
    /// Animation values
    pub values: std::collections::HashMap<String, AnimationValue>,
    /// Duration in seconds
    pub duration: f64,
    /// Start time
    pub start_time: Option<f64>,
    /// End time
    pub end_time: Option<f64>,
    /// Whether this animation is currently in use
    pub in_use: bool,
    /// Custom data (not cloneable)
    pub custom_data: Option<std::rc::Rc<dyn std::any::Any + Send + Sync>>,
}

impl Clone for PooledAnimation {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            animation_type: self.animation_type.clone(),
            state: self.state.clone(),
            values: self.values.clone(),
            duration: self.duration,
            start_time: self.start_time,
            end_time: self.end_time,
            in_use: self.in_use,
            custom_data: None, // Don't clone custom data
        }
    }
}

impl PooledAnimation {
    /// Create a new pooled animation
    pub fn new(id: u64, animation_type: AnimationType) -> Self {
        Self {
            id,
            animation_type,
            state: AnimationState::Ready,
            values: std::collections::HashMap::new(),
            duration: 0.0,
            start_time: None,
            end_time: None,
            in_use: false,
            custom_data: None,
        }
    }

    /// Reset the animation for reuse
    pub fn reset(&mut self) {
        self.state = AnimationState::Ready;
        self.values.clear();
        self.duration = 0.0;
        self.start_time = None;
        self.end_time = None;
        self.in_use = false;
        self.custom_data = None;
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        matches!(self.state, AnimationState::Completed)
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        matches!(self.state, AnimationState::Playing)
    }

    /// Start the animation
    pub fn start(&mut self, current_time: f64) -> Result<()> {
        if self.in_use {
            return Err(StudioError::InvalidState("Animation already in use".to_string()));
        }

        self.state = AnimationState::Playing;
        self.start_time = Some(current_time);
        self.end_time = Some(current_time + self.duration);
        self.in_use = true;

        Ok(())
    }

    /// Stop the animation
    pub fn stop(&mut self) {
        self.state = AnimationState::Completed;
        self.in_use = false;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if matches!(self.state, AnimationState::Playing) {
            self.state = AnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if matches!(self.state, AnimationState::Paused) {
            self.state = AnimationState::Playing;
        }
    }

    /// Update animation progress
    pub fn update(&mut self, current_time: f64) -> Result<()> {
        if !self.is_running() {
            return Ok(());
        }

        if let Some(end_time) = self.end_time
            && current_time >= end_time {
                self.state = AnimationState::Completed;
                self.in_use = false;
            }

        Ok(())
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self, current_time: f64) -> f64 {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            if end <= start {
                return 1.0;
            }
            ((current_time - start) / (end - start)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}
