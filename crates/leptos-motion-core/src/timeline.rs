//! Timeline Animation System - Green Phase Implementation
//!
//! Provides timeline-based animation sequencing with keyframes,
//! scrubbing, synchronization, and performance optimization.

use crate::{AnimationError, AnimationValue, Easing, Result};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Timeline system for complex animation sequences
pub struct Timeline {
    /// Keyframes sorted by time
    keyframes: Vec<TimelineKeyframe>,
    /// Total duration of the timeline
    duration: f64,
    /// Current playback time
    current_time: f64,
    /// Whether elements are synchronized
    is_synchronized: bool,
    /// Progress callback
    progress_callback: Option<Arc<Mutex<dyn Fn(f64) + Send + 'static>>>,
    /// Timeline creation time for performance tracking
    created_at: Instant,
    /// Cache for interpolated values
    value_cache: HashMap<String, f64>,
}

// Manual Clone implementation to handle the progress_callback
impl Clone for Timeline {
    fn clone(&self) -> Self {
        Self {
            keyframes: self.keyframes.clone(),
            duration: self.duration,
            current_time: self.current_time,
            is_synchronized: self.is_synchronized,
            progress_callback: None,     // Don't clone callbacks
            created_at: Instant::now(),  // Reset creation time
            value_cache: HashMap::new(), // Reset cache
        }
    }
}

impl std::fmt::Debug for Timeline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Timeline")
            .field("keyframes", &self.keyframes)
            .field("duration", &self.duration)
            .field("current_time", &self.current_time)
            .field("is_synchronized", &self.is_synchronized)
            .field("has_callback", &self.progress_callback.is_some())
            .field("created_at", &self.created_at)
            .field("value_cache", &self.value_cache)
            .finish()
    }
}

impl Timeline {
    /// Create a new empty timeline
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
            duration: 0.0,
            current_time: 0.0,
            is_synchronized: true,
            progress_callback: None,
            created_at: Instant::now(),
            value_cache: HashMap::new(),
        }
    }

    /// Add a keyframe at the specified time
    pub fn add_keyframe(&mut self, time: f64, keyframe: TimelineKeyframe) {
        // Validate time
        if !time.is_finite() || time < 0.0 {
            return; // Silently ignore invalid times for now
        }

        // Insert keyframe in sorted order by time
        let mut keyframe = keyframe;
        keyframe.time = time;

        let insert_pos = self
            .keyframes
            .binary_search_by(|k| {
                k.time
                    .partial_cmp(&time)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|pos| pos);

        self.keyframes.insert(insert_pos, keyframe);

        // Update timeline duration
        self.duration = self.duration.max(time);

        // Clear cache since structure changed
        self.value_cache.clear();
    }

    /// Add a keyframe with specific easing
    pub fn add_keyframe_with_easing(
        &mut self,
        time: f64,
        mut keyframe: TimelineKeyframe,
        easing: Easing,
    ) {
        keyframe.easing = easing;
        self.add_keyframe(time, keyframe);
    }

    /// Get the number of keyframes
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }

    /// Get the total duration of the timeline
    pub fn total_duration(&self) -> f64 {
        self.duration
    }

    /// Check if the timeline maintains synchronization
    pub fn is_synchronized(&self) -> bool {
        self.is_synchronized
    }

    /// Get list of elements that are synchronized in this timeline
    pub fn get_synchronized_elements(&self) -> Vec<String> {
        let mut elements = Vec::new();
        for keyframe in &self.keyframes {
            if let Some(ref element_id) = keyframe.element_id
                && !elements.contains(element_id) {
                    elements.push(element_id.clone());
                }
        }
        elements
    }

    /// Scrub to a specific time in the timeline
    pub fn scrub_to(&mut self, time: f64) -> Result<()> {
        if !time.is_finite() {
            return Err(AnimationError::InvalidValue(
                "Scrub time must be finite".to_string(),
            ));
        }

        if time < 0.0 {
            return Err(AnimationError::InvalidValue(
                "Scrub time cannot be negative".to_string(),
            ));
        }

        if time > self.duration {
            return Err(AnimationError::InvalidValue(format!(
                "Scrub time {} exceeds timeline duration {}",
                time, self.duration
            )));
        }

        self.current_time = time;

        // Clear cache to force recalculation
        self.value_cache.clear();

        // Trigger progress callback if set
        if let Some(ref callback) = self.progress_callback {
            let progress = if self.duration > 0.0 {
                time / self.duration
            } else {
                1.0
            };

            if let Ok(callback) = callback.lock() {
                callback(progress);
            }
        }

        Ok(())
    }

    /// Get the current playback time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Get interpolated value for a property at a specific time
    pub fn get_value_at_time(&mut self, property: &str, time: f64) -> f64 {
        // Check cache first
        let cache_key = format!("{}@{}", property, time);
        if let Some(&cached_value) = self.value_cache.get(&cache_key) {
            return cached_value;
        }

        // Find keyframes surrounding this time
        let (before, after) = self.find_surrounding_keyframes(time);

        let value = match (before, after) {
            (Some(before_kf), Some(after_kf)) if before_kf.time != after_kf.time => {
                // Interpolate between keyframes
                let before_val = self.get_keyframe_property_value(before_kf, property);
                let after_val = self.get_keyframe_property_value(after_kf, property);

                let progress = (time - before_kf.time) / (after_kf.time - before_kf.time);
                let eased_progress = after_kf.easing.evaluate(progress);

                before_val + (after_val - before_val) * eased_progress
            }
            (Some(keyframe), _) | (_, Some(keyframe)) => {
                // Use exact keyframe value
                self.get_keyframe_property_value(keyframe, property)
            }
            (None, None) => {
                // No keyframes, return default
                0.0
            }
        };

        // Cache for future lookups (but don't cache indefinitely to prevent memory leaks)
        if self.value_cache.len() < 1000 {
            self.value_cache.insert(cache_key, value);
        }

        value
    }

    /// Set progress callback
    pub fn on_progress(&mut self, callback: Box<dyn Fn(f64) + Send + 'static>) {
        self.progress_callback = Some(Arc::new(Mutex::new(callback)));
    }

    /// Stop the timeline
    pub fn stop(&mut self) {
        self.current_time = 0.0;
        self.value_cache.clear();
    }

    /// Get performance metrics for this timeline
    pub fn get_performance_metrics(&self) -> TimelinePerformanceMetrics {
        TimelinePerformanceMetrics {
            keyframe_count: self.keyframes.len(),
            duration: self.duration,
            cache_size: self.value_cache.len(),
            age: self.created_at.elapsed(),
            synchronized_elements: self.get_synchronized_elements().len(),
        }
    }

    // Internal helper methods

    fn find_surrounding_keyframes(
        &self,
        time: f64,
    ) -> (Option<&TimelineKeyframe>, Option<&TimelineKeyframe>) {
        if self.keyframes.is_empty() {
            return (None, None);
        }

        // Find the keyframe at or before this time
        let mut before_idx = None;
        let mut after_idx = None;

        for (i, keyframe) in self.keyframes.iter().enumerate() {
            if keyframe.time <= time {
                before_idx = Some(i);
            }
            if keyframe.time >= time && after_idx.is_none() {
                after_idx = Some(i);
            }
        }

        let before = before_idx.map(|i| &self.keyframes[i]);
        let after = after_idx.map(|i| &self.keyframes[i]);

        (before, after)
    }

    fn get_keyframe_property_value(&self, keyframe: &TimelineKeyframe, property: &str) -> f64 {
        keyframe
            .properties
            .get(property)
            .map(|value| {
                match value {
                    AnimationValue::Number(n) => *n,
                    AnimationValue::Pixels(p) => *p,
                    AnimationValue::Percentage(p) => *p,
                    AnimationValue::Degrees(d) => *d,
                    _ => 0.0, // Default for complex types
                }
            })
            .unwrap_or(0.0)
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Individual keyframe in a timeline
#[derive(Debug, Clone)]
pub struct TimelineKeyframe {
    /// Time position in the timeline (0.0 to 1.0 or absolute)
    pub time: f64,
    /// Animation properties at this keyframe
    pub properties: HashMap<String, AnimationValue>,
    /// Easing function for this segment
    pub easing: Easing,
    /// Optional element ID for multi-element animations
    pub element_id: Option<String>,
}

impl TimelineKeyframe {
    /// Create a new keyframe at the specified time
    pub fn new(time: f64) -> Self {
        Self {
            time,
            properties: HashMap::new(),
            easing: Easing::Linear,
            element_id: None,
        }
    }

    /// Add a property to this keyframe
    pub fn with_property(mut self, name: &str, value: AnimationValue) -> Self {
        self.properties.insert(name.to_string(), value);
        self
    }

    /// Set the target element for this keyframe
    pub fn with_element(mut self, element_id: String) -> Self {
        self.element_id = Some(element_id);
        self
    }

    /// Set the easing function for this segment
    pub fn with_easing(mut self, easing: Easing) -> Self {
        self.easing = easing;
        self
    }
}

/// Performance metrics for timeline operations
#[derive(Debug, Clone)]
pub struct TimelinePerformanceMetrics {
    /// Number of keyframes in the timeline
    pub keyframe_count: usize,
    /// Total timeline duration
    pub duration: f64,
    /// Size of the interpolation cache
    pub cache_size: usize,
    /// How long the timeline has existed
    pub age: Duration,
    /// Number of synchronized elements
    pub synchronized_elements: usize,
}

impl TimelinePerformanceMetrics {
    /// Check if timeline performance is acceptable
    pub fn is_performant(&self) -> bool {
        // Performance criteria:
        // - Cache size should not exceed 1000 entries
        // - Should handle reasonable number of keyframes efficiently
        self.cache_size < 1000 && (self.keyframe_count < 100 || self.age < Duration::from_secs(10))
    }

    /// Get memory usage estimate in bytes
    pub fn estimated_memory_bytes(&self) -> usize {
        // Rough estimate
        self.keyframe_count * 256 +  // Keyframes
        self.cache_size * 64 +       // Cache entries
        1024 // Timeline overhead
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AnimationValue;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new();
        assert_eq!(timeline.keyframe_count(), 0);
        assert_eq!(timeline.total_duration(), 0.0);
        assert_eq!(timeline.current_time(), 0.0);
        assert!(timeline.is_synchronized());
    }

    #[test]
    fn test_keyframe_addition() {
        let mut timeline = Timeline::new();
        let keyframe =
            TimelineKeyframe::new(0.5).with_property("opacity", AnimationValue::Number(1.0));

        timeline.add_keyframe(0.5, keyframe);

        assert_eq!(timeline.keyframe_count(), 1);
        assert_eq!(timeline.total_duration(), 0.5);
    }

    #[test]
    fn test_keyframe_sorting() {
        let mut timeline = Timeline::new();

        // Add keyframes out of order
        timeline.add_keyframe(
            1.0,
            TimelineKeyframe::new(1.0).with_property("opacity", AnimationValue::Number(1.0)),
        );
        timeline.add_keyframe(
            0.0,
            TimelineKeyframe::new(0.0).with_property("opacity", AnimationValue::Number(0.0)),
        );
        timeline.add_keyframe(
            0.5,
            TimelineKeyframe::new(0.5).with_property("opacity", AnimationValue::Number(0.5)),
        );

        // Should be sorted by time
        assert_eq!(timeline.keyframes[0].time, 0.0);
        assert_eq!(timeline.keyframes[1].time, 0.5);
        assert_eq!(timeline.keyframes[2].time, 1.0);
        assert_eq!(timeline.total_duration(), 1.0);
    }

    #[test]
    fn test_timeline_scrubbing() {
        let mut timeline = Timeline::new();
        timeline.add_keyframe(
            0.0,
            TimelineKeyframe::new(0.0).with_property("opacity", AnimationValue::Number(0.0)),
        );
        timeline.add_keyframe(
            1.0,
            TimelineKeyframe::new(1.0).with_property("opacity", AnimationValue::Number(1.0)),
        );

        assert!(timeline.scrub_to(0.5).is_ok());
        assert_eq!(timeline.current_time(), 0.5);

        // Test invalid scrub times
        assert!(timeline.scrub_to(-0.1).is_err());
        assert!(timeline.scrub_to(1.5).is_err());
        assert!(timeline.scrub_to(f64::NAN).is_err());
    }

    #[test]
    fn test_value_interpolation() {
        let mut timeline = Timeline::new();
        timeline.add_keyframe(
            0.0,
            TimelineKeyframe::new(0.0).with_property("opacity", AnimationValue::Number(0.0)),
        );
        timeline.add_keyframe(
            1.0,
            TimelineKeyframe::new(1.0).with_property("opacity", AnimationValue::Number(1.0)),
        );

        // Test linear interpolation
        assert_eq!(timeline.get_value_at_time("opacity", 0.0), 0.0);
        assert_eq!(timeline.get_value_at_time("opacity", 0.5), 0.5);
        assert_eq!(timeline.get_value_at_time("opacity", 1.0), 1.0);

        // Test property that doesn't exist
        assert_eq!(timeline.get_value_at_time("nonexistent", 0.5), 0.0);
    }

    #[test]
    fn test_easing_evaluation() {
        // Test different easing functions
        assert_eq!(Easing::Linear.evaluate(0.5), 0.5);

        let ease_in_result = Easing::EaseIn.evaluate(0.5);
        assert!(ease_in_result < 0.5); // EaseIn should be slower at start

        let ease_out_result = Easing::EaseOut.evaluate(0.5);
        assert!(ease_out_result > 0.5); // EaseOut should be faster at start
    }

    #[test]
    fn test_synchronized_elements() {
        let mut timeline = Timeline::new();
        timeline.add_keyframe(
            0.0,
            TimelineKeyframe::new(0.0).with_element("element1".to_string()),
        );
        timeline.add_keyframe(
            0.5,
            TimelineKeyframe::new(0.5).with_element("element2".to_string()),
        );
        timeline.add_keyframe(
            1.0,
            TimelineKeyframe::new(1.0).with_element("element1".to_string()),
        ); // Duplicate

        let elements = timeline.get_synchronized_elements();
        assert_eq!(elements.len(), 2);
        assert!(elements.contains(&"element1".to_string()));
        assert!(elements.contains(&"element2".to_string()));
    }

    #[test]
    fn test_performance_metrics() {
        let mut timeline = Timeline::new();
        timeline.add_keyframe(
            0.0,
            TimelineKeyframe::new(0.0).with_property("opacity", AnimationValue::Number(0.0)),
        );
        timeline.add_keyframe(
            1.0,
            TimelineKeyframe::new(1.0).with_property("opacity", AnimationValue::Number(1.0)),
        );

        let metrics = timeline.get_performance_metrics();
        assert_eq!(metrics.keyframe_count, 2);
        assert_eq!(metrics.duration, 1.0);
        assert!(metrics.is_performant());
        assert!(metrics.estimated_memory_bytes() > 0);
    }

    #[test]
    fn test_keyframe_builder_pattern() {
        let keyframe = TimelineKeyframe::new(0.5)
            .with_property("opacity", AnimationValue::Number(1.0))
            .with_property("scale", AnimationValue::Number(1.5))
            .with_element("test_element".to_string())
            .with_easing(Easing::EaseInOut);

        assert_eq!(keyframe.time, 0.5);
        assert_eq!(keyframe.properties.len(), 2);
        assert_eq!(keyframe.element_id, Some("test_element".to_string()));
        assert_eq!(keyframe.easing, Easing::EaseInOut);
    }

    #[test]
    fn test_complex_easing_with_keyframes() {
        let mut timeline = Timeline::new();
        timeline.add_keyframe_with_easing(
            0.0,
            TimelineKeyframe::new(0.0).with_property("opacity", AnimationValue::Number(0.0)),
            Easing::Linear,
        );
        timeline.add_keyframe_with_easing(
            1.0,
            TimelineKeyframe::new(1.0).with_property("opacity", AnimationValue::Number(1.0)),
            Easing::EaseIn,
        );

        // The interpolation should use the easing from the "to" keyframe
        let value_at_half = timeline.get_value_at_time("opacity", 0.5);
        // With EaseIn, 0.5 should map to 0.25 (0.5^2), so interpolated value should be 0.25
        assert!((value_at_half - 0.25).abs() < 0.01);
    }
}
