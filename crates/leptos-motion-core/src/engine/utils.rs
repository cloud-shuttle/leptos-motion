//! Utility functions for animation engines
//!
//! This module contains helper functions and utilities
//! used across different animation engines.

use super::traits::*;
use crate::{AnimationError, AnimationHandle, Result};

/// Utility functions for animation engines
pub struct AnimationUtils;

impl AnimationUtils {
    /// Validate an animation configuration
    pub fn validate_config(config: &AnimationConfig) -> Result<()> {
        // Check if target is valid
        if config.values.is_empty() {
            return Err(AnimationError::InvalidValue("No animation values provided".to_string()));
        }

        // Check if transition duration is valid
        if config.transition.duration.unwrap_or(0.0) < 0.0 {
            return Err(AnimationError::InvalidValue("Invalid transition duration".to_string()));
        }

        Ok(())
    }

    /// Calculate animation progress (0.0 to 1.0)
    pub fn calculate_progress(start_time: f64, current_time: f64, duration: f64) -> f64 {
        let elapsed = current_time - start_time;
        (elapsed / duration).clamp(0.0, 1.0)
    }

    /// Apply easing function to a progress value
    pub fn apply_easing(progress: f64, easing: &crate::Easing) -> f64 {
        match easing {
            crate::Easing::Linear => progress,
            crate::Easing::EaseIn => progress * progress,
            crate::Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            crate::Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            _ => progress, // Simplified for other easing types
        }
    }

    /// Interpolate between two values
    pub fn interpolate(from: f64, to: f64, progress: f64) -> f64 {
        from + (to - from) * progress
    }

    /// Check if an animation handle is valid
    pub fn is_valid_handle(handle: AnimationHandle) -> bool {
        handle.0 > 0
    }
}
