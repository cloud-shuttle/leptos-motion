//! Stagger Animations - Sequential element animation effects
//!
//! This module provides sophisticated staggering capabilities for animating
//! multiple elements in sequence with configurable delays, patterns, and
//! timing functions. Perfect for creating cascading animations and list reveals.

use crate::AnimationValue;
use std::collections::HashMap;

/// Direction in which stagger animations progress
#[derive(Clone, Debug, PartialEq)]
pub enum StaggerDirection {
    /// Start from the first element and progress forward
    Forward,
    /// Start from the last element and progress backward
    Backward,
    /// Start from the center and progress outward
    Center,
    /// Start from both ends and progress toward the center
    Edges,
}

/// Pattern for calculating stagger delays
pub enum StaggerPattern {
    /// Fixed delay between each element
    Fixed(f64),
    /// Delay increases linearly with each element
    Linear { base_delay: f64, increment: f64 },
    /// Delay increases exponentially
    Exponential { base_delay: f64, multiplier: f64 },
    /// Custom function that takes element index and returns delay
    Custom(Box<dyn Fn(usize, usize) -> f64 + Send + Sync + 'static>),
}

impl Clone for StaggerPattern {
    fn clone(&self) -> Self {
        match self {
            StaggerPattern::Fixed(delay) => StaggerPattern::Fixed(*delay),
            StaggerPattern::Linear { base_delay, increment } => StaggerPattern::Linear {
                base_delay: *base_delay,
                increment: *increment,
            },
            StaggerPattern::Exponential { base_delay, multiplier } => StaggerPattern::Exponential {
                base_delay: *base_delay,
                multiplier: *multiplier,
            },
            StaggerPattern::Custom(_) => panic!("Custom stagger patterns cannot be cloned"),
        }
    }
}

impl std::fmt::Debug for StaggerPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StaggerPattern::Fixed(delay) => f.debug_tuple("Fixed").field(delay).finish(),
            StaggerPattern::Linear { base_delay, increment } => f.debug_struct("Linear")
                .field("base_delay", base_delay)
                .field("increment", increment)
                .finish(),
            StaggerPattern::Exponential { base_delay, multiplier } => f.debug_struct("Exponential")
                .field("base_delay", base_delay)
                .field("multiplier", multiplier)
                .finish(),
            StaggerPattern::Custom(_) => f.debug_tuple("Custom").field(&"<function>").finish(),
        }
    }
}

/// Configuration for stagger animations
#[derive(Clone, Debug)]
pub struct MotionStaggerConfig {
    /// Direction in which the stagger progresses
    pub direction: StaggerDirection,
    /// Pattern for calculating delays
    pub pattern: StaggerPattern,
    /// Whether to reverse the order after reaching the end
    pub reverse: bool,
    /// Maximum number of elements to animate (None for all)
    pub max_elements: Option<usize>,
    /// Starting delay before the first element
    pub start_delay: f64,
    /// Whether to restart the stagger when new elements are added
    pub restart_on_change: bool,
}

impl Default for MotionStaggerConfig {
    fn default() -> Self {
        Self {
            direction: StaggerDirection::Forward,
            pattern: StaggerPattern::Fixed(0.1),
            reverse: false,
            max_elements: None,
            start_delay: 0.0,
            restart_on_change: false,
        }
    }
}

/// Builder pattern for creating stagger configurations
pub struct MotionStaggerConfigBuilder {
    config: MotionStaggerConfig,
}

impl MotionStaggerConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: MotionStaggerConfig::default(),
        }
    }

    pub fn direction(mut self, direction: StaggerDirection) -> Self {
        self.config.direction = direction;
        self
    }

    pub fn fixed_delay(mut self, delay: f64) -> Self {
        self.config.pattern = StaggerPattern::Fixed(delay);
        self
    }

    pub fn linear_delay(mut self, base_delay: f64, increment: f64) -> Self {
        self.config.pattern = StaggerPattern::Linear { base_delay, increment };
        self
    }

    pub fn exponential_delay(mut self, base_delay: f64, multiplier: f64) -> Self {
        self.config.pattern = StaggerPattern::Exponential { base_delay, multiplier };
        self
    }

    pub fn custom_delay<F>(mut self, func: F) -> Self
    where
        F: Fn(usize, usize) -> f64 + Send + Sync + 'static,
    {
        self.config.pattern = StaggerPattern::Custom(Box::new(func));
        self
    }

    pub fn reverse(mut self, reverse: bool) -> Self {
        self.config.reverse = reverse;
        self
    }

    pub fn max_elements(mut self, max: usize) -> Self {
        self.config.max_elements = Some(max);
        self
    }

    pub fn start_delay(mut self, delay: f64) -> Self {
        self.config.start_delay = delay;
        self
    }

    pub fn restart_on_change(mut self, restart: bool) -> Self {
        self.config.restart_on_change = restart;
        self
    }

    pub fn build(self) -> MotionStaggerConfig {
        self.config
    }
}

impl Default for MotionStaggerConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages stagger animations for a collection of elements
#[derive(Clone)]
pub struct MotionStaggerAnimation {
    config: MotionStaggerConfig,
    element_count: usize,
    started_at: Option<f64>,
}

impl MotionStaggerAnimation {
    /// Create a new stagger animation manager
    pub fn new(config: MotionStaggerConfig, element_count: usize) -> Self {
        Self {
            config,
            element_count,
            started_at: None,
        }
    }

    /// Start the stagger animation
    pub fn start(&mut self, current_time: f64) {
        self.started_at = Some(current_time);
    }

    /// Get the delay for a specific element index
    pub fn delay_for_element(&self, index: usize) -> f64 {
        if self.started_at.is_none() {
            return 0.0;
        }

        let base_delay = self.config.start_delay;
        let pattern_delay = self.calculate_pattern_delay(index);

        base_delay + pattern_delay
    }

    /// Get the total duration of the stagger animation
    pub fn total_duration(&self) -> f64 {
        if self.element_count == 0 {
            return 0.0;
        }

        let max_delay = (0..self.element_count)
            .map(|i| self.calculate_pattern_delay(i))
            .fold(0.0, f64::max);

        self.config.start_delay + max_delay
    }

    /// Calculate delay based on the stagger pattern
    fn calculate_pattern_delay(&self, index: usize) -> f64 {
        let reordered_index = self.reorder_index(index);

        match &self.config.pattern {
            StaggerPattern::Fixed(delay) => reordered_index as f64 * *delay,
            StaggerPattern::Linear { base_delay, increment } => {
                base_delay + (reordered_index as f64 * increment)
            }
            StaggerPattern::Exponential { base_delay, multiplier } => {
                base_delay * multiplier.powi(reordered_index as i32)
            }
            StaggerPattern::Custom(func) => {
                func(reordered_index, self.element_count)
            }
        }
    }

    /// Reorder index based on stagger direction
    fn reorder_index(&self, index: usize) -> usize {
        let max_index = self.element_count.saturating_sub(1);

        match self.config.direction {
            StaggerDirection::Forward => index,
            StaggerDirection::Backward => max_index.saturating_sub(index),
            StaggerDirection::Center => {
                let center = max_index / 2;
                if index <= center {
                    index * 2
                } else {
                    (max_index - index) * 2 + 1
                }
            }
            StaggerDirection::Edges => {
                let center = max_index / 2;
                if index % 2 == 0 {
                    index / 2
                } else {
                    max_index - (index / 2)
                }
            }
        }
    }

    /// Check if the stagger animation is complete
    pub fn is_complete(&self, current_time: f64) -> bool {
        if let Some(started_at) = self.started_at {
            current_time >= started_at + self.total_duration()
        } else {
            false
        }
    }

    /// Get progress of the stagger animation (0.0 to 1.0)
    pub fn progress(&self, current_time: f64) -> f64 {
        if let Some(started_at) = self.started_at {
            let elapsed = current_time - started_at;
            let duration = self.total_duration();
            if duration > 0.0 {
                (elapsed / duration).clamp(0.0, 1.0)
            } else {
                1.0
            }
        } else {
            0.0
        }
    }
}

/// Configuration for stagger animations in MotionDiv
#[derive(Clone, Debug)]
pub struct ElementStaggerConfig {
    /// Base stagger configuration
    pub config: MotionStaggerConfig,
    /// Individual animation properties for each element
    pub element_props: Vec<HashMap<String, AnimationValue>>,
}

impl ElementStaggerConfig {
    /// Create a new stagger config with element properties
    pub fn new(config: MotionStaggerConfig, element_props: Vec<HashMap<String, AnimationValue>>) -> Self {
        Self {
            config,
            element_props,
        }
    }

    /// Create a stagger config with the same properties for all elements
    pub fn uniform(config: MotionStaggerConfig, element_count: usize, properties: HashMap<String, AnimationValue>) -> Self {
        let element_props = vec![properties; element_count];
        Self::new(config, element_props)
    }
}


/// Helper function to create a stagger animation
pub fn create_motion_stagger_animation(
    config: MotionStaggerConfig,
    element_props: Vec<HashMap<String, AnimationValue>>,
) -> ElementStaggerConfig {
    ElementStaggerConfig::new(config, element_props)
}

/// Helper function to create a uniform stagger animation
pub fn create_uniform_motion_stagger(
    config: MotionStaggerConfig,
    element_count: usize,
    properties: HashMap<String, AnimationValue>,
) -> ElementStaggerConfig {
    ElementStaggerConfig::uniform(config, element_count, properties)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stagger_config_builder() {
        let config = MotionStaggerConfigBuilder::new()
            .direction(StaggerDirection::Forward)
            .fixed_delay(0.1)
            .start_delay(0.2)
            .build();

        assert_eq!(config.direction, StaggerDirection::Forward);
        assert_eq!(config.start_delay, 0.2);
        match config.pattern {
            StaggerPattern::Fixed(delay) => assert_eq!(delay, 0.1),
            _ => panic!("Expected fixed pattern"),
        }
    }

    #[test]
    fn test_stagger_animation_delays() {
        let config = MotionStaggerConfig {
            direction: StaggerDirection::Forward,
            pattern: StaggerPattern::Fixed(0.1),
            start_delay: 0.0,
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.delay_for_element(0), 0.0);
        assert_eq!(animation.delay_for_element(1), 0.1);
        assert_eq!(animation.delay_for_element(2), 0.2);
    }

    #[test]
    fn test_stagger_backward_direction() {
        let config = MotionStaggerConfig {
            direction: StaggerDirection::Backward,
            pattern: StaggerPattern::Fixed(0.1),
            start_delay: 0.0,
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.delay_for_element(0), 0.2); // Last element first
        assert_eq!(animation.delay_for_element(1), 0.1);
        assert_eq!(animation.delay_for_element(2), 0.0); // First element last
    }

    #[test]
    fn test_stagger_center_direction() {
        let config = MotionStaggerConfig {
            direction: StaggerDirection::Center,
            pattern: StaggerPattern::Fixed(0.1),
            start_delay: 0.0,
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 4);
        animation.start(0.0);

        // Center direction: start from middle and work outward
        assert_eq!(animation.delay_for_element(0), 0.0); // center
        assert_eq!(animation.delay_for_element(1), 0.2); // right of center
        assert_eq!(animation.delay_for_element(2), 0.0); // left of center
        assert_eq!(animation.delay_for_element(3), 0.2); // far right
    }

    #[test]
    fn test_linear_pattern() {
        let config = MotionStaggerConfig {
            pattern: StaggerPattern::Linear { base_delay: 0.1, increment: 0.05 },
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.delay_for_element(0), 0.1);
        assert_eq!(animation.delay_for_element(1), 0.15);
        assert_eq!(animation.delay_for_element(2), 0.2);
    }

    #[test]
    fn test_exponential_pattern() {
        let config = MotionStaggerConfig {
            pattern: StaggerPattern::Exponential { base_delay: 0.1, multiplier: 2.0 },
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.delay_for_element(0), 0.1);
        assert_eq!(animation.delay_for_element(1), 0.2);
        assert_eq!(animation.delay_for_element(2), 0.4);
    }

    #[test]
    fn test_custom_pattern() {
        let config = MotionStaggerConfig {
            pattern: StaggerPattern::Custom(Box::new(|index, _total| index as f64 * 0.5)),
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.delay_for_element(0), 0.0);
        assert_eq!(animation.delay_for_element(1), 0.5);
        assert_eq!(animation.delay_for_element(2), 1.0);
    }

    #[test]
    fn test_stagger_progress() {
        let config = MotionStaggerConfig {
            pattern: StaggerPattern::Fixed(0.1),
            start_delay: 0.0,
            ..Default::default()
        };

        let mut animation = MotionStaggerAnimation::new(config, 3);
        animation.start(0.0);

        assert_eq!(animation.total_duration(), 0.2);

        assert_eq!(animation.progress(0.0), 0.0);
        assert_eq!(animation.progress(0.1), 0.5);
        assert_eq!(animation.progress(0.2), 1.0);
        assert_eq!(animation.progress(0.3), 1.0); // clamped
    }

    #[test]
    fn test_element_stagger_config() {
        let config = MotionStaggerConfig::default();
        let props1 = hashmap! { "opacity" => AnimationValue::Number(0.0) };
        let props2 = hashmap! { "opacity" => AnimationValue::Number(1.0) };

        let stagger_config = ElementStaggerConfig::new(config, vec![props1.clone(), props2.clone()]);

        assert_eq!(stagger_config.element_props.len(), 2);
        assert_eq!(stagger_config.element_props[0], props1);
        assert_eq!(stagger_config.element_props[1], props2);
    }

    #[test]
    fn test_uniform_element_stagger() {
        let config = MotionStaggerConfig::default();
        let props = hashmap! { "scale" => AnimationValue::Number(1.1) };

        let stagger_config = ElementStaggerConfig::uniform(config, 3, props.clone());

        assert_eq!(stagger_config.element_props.len(), 3);
        assert_eq!(stagger_config.element_props[0], props);
        assert_eq!(stagger_config.element_props[1], props);
        assert_eq!(stagger_config.element_props[2], props);
    }
}
