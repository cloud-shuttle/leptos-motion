//! Repeat Configuration for Leptos Motion
//!
//! This module provides comprehensive repeat configuration support for animations.
//! Includes count-based, infinite, and reverse repeat modes.

use leptos_motion_core::RepeatConfig;

/// Repeat state tracker
#[derive(Debug, Clone)]
pub struct RepeatState {
    /// Current repeat count
    pub current_count: u32,
    /// Total number of repeats (None for infinite)
    pub total_count: Option<u32>,
    /// Whether to reverse on each repeat
    pub reverse: bool,
    /// Whether the animation is currently reversed
    pub is_reversed: bool,
    /// Whether the repeat cycle is complete
    pub is_complete: bool,
}

impl RepeatState {
    /// Create a new repeat state
    pub fn new(config: &RepeatConfig) -> Self {
        let (total_count, reverse) = match config {
            RepeatConfig::Never => (Some(0), false),
            RepeatConfig::Count(count) => (Some(*count), false),
            RepeatConfig::Infinite => (None, false),
            RepeatConfig::InfiniteReverse => (None, true),
        };

        Self {
            current_count: 0,
            total_count,
            reverse,
            is_reversed: false,
            is_complete: false,
        }
    }

    /// Check if the animation should continue repeating
    pub fn should_continue(&self) -> bool {
        match self.total_count {
            Some(total) => self.current_count < total,
            None => true, // Infinite
        }
    }

    /// Advance to the next repeat cycle
    pub fn advance(&mut self) {
        self.current_count += 1;

        if self.reverse {
            self.is_reversed = !self.is_reversed;
        }

        // Check if we've reached the total count
        if let Some(total) = self.total_count && self.current_count >= total {
            self.is_complete = true;
        }
    }

    /// Get the current progress multiplier (1.0 for normal, -1.0 for reversed)
    pub fn progress_multiplier(&self) -> f64 {
        if self.is_reversed { -1.0 } else { 1.0 }
    }

    /// Reset the repeat state
    pub fn reset(&mut self) {
        self.current_count = 0;
        self.is_reversed = false;
        self.is_complete = false;
    }
}

/// Repeat configuration builder for easy setup
pub struct RepeatConfigBuilder {
    count: Option<u32>,
    reverse: bool,
}

impl RepeatConfigBuilder {
    /// Create a new repeat config builder
    pub fn new() -> Self {
        Self {
            count: None,
            reverse: false,
        }
    }

    /// Set the number of repeats
    pub fn count(mut self, count: u32) -> Self {
        self.count = Some(count);
        self
    }

    /// Set to infinite repeats
    pub fn infinite(mut self) -> Self {
        self.count = None;
        self
    }

    /// Enable reverse mode
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Build the repeat configuration
    pub fn build(self) -> RepeatConfig {
        match (self.count, self.reverse) {
            (None, false) => RepeatConfig::Infinite,
            (None, true) => RepeatConfig::InfiniteReverse,
            (Some(0), _) => RepeatConfig::Never,
            (Some(count), false) => RepeatConfig::Count(count),
            (Some(_count), true) => {
                // For count-based reverse, we'll use InfiniteReverse and track manually
                RepeatConfig::InfiniteReverse
            }
        }
    }
}

impl Default for RepeatConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Animation cycle manager for handling repeat logic
pub struct AnimationCycleManager {
    repeat_state: RepeatState,
    base_duration: f64,
    current_cycle_time: f64,
    total_elapsed_time: f64,
}

impl AnimationCycleManager {
    /// Create a new animation cycle manager
    pub fn new(config: &RepeatConfig, duration: f64) -> Self {
        Self {
            repeat_state: RepeatState::new(config),
            base_duration: duration,
            current_cycle_time: 0.0,
            total_elapsed_time: 0.0,
        }
    }

    /// Update the cycle with delta time
    pub fn update(&mut self, delta_time: f64) -> CycleUpdate {
        self.total_elapsed_time += delta_time;
        self.current_cycle_time += delta_time;

        // Check if current cycle is complete
        if self.current_cycle_time >= self.base_duration {
            self.current_cycle_time = 0.0;

            // Advance the repeat state first
            self.repeat_state.advance();
            
            // Check if we should continue repeating
            if self.repeat_state.should_continue() {
                CycleUpdate::CycleComplete
            } else {
                self.repeat_state.is_complete = true;
                CycleUpdate::AnimationComplete
            }
        } else {
            CycleUpdate::Continue
        }
    }

    /// Get the current progress within the cycle (0.0 to 1.0)
    pub fn cycle_progress(&self) -> f64 {
        (self.current_cycle_time / self.base_duration).min(1.0)
    }

    /// Get the effective progress considering reverse mode
    pub fn effective_progress(&self) -> f64 {
        let progress = self.cycle_progress();
        if self.repeat_state.is_reversed {
            1.0 - progress
        } else {
            progress
        }
    }

    /// Get the current repeat count
    pub fn current_count(&self) -> u32 {
        self.repeat_state.current_count
    }

    /// Check if the animation is complete
    pub fn is_complete(&self) -> bool {
        self.repeat_state.is_complete
    }

    /// Check if the current cycle is reversed
    pub fn is_reversed(&self) -> bool {
        self.repeat_state.is_reversed
    }

    /// Reset the cycle manager
    pub fn reset(&mut self) {
        self.repeat_state.reset();
        self.current_cycle_time = 0.0;
        self.total_elapsed_time = 0.0;
    }
}

/// Result of a cycle update
#[derive(Debug, Clone, PartialEq)]
pub enum CycleUpdate {
    /// Continue the current cycle
    Continue,
    /// Current cycle completed, starting next cycle
    CycleComplete,
    /// All cycles completed, animation finished
    AnimationComplete,
}

/// Stagger configuration for multiple animations
#[derive(Debug, Clone)]
pub struct StaggerConfig {
    /// Delay between each animation start
    pub delay: f64,
    /// Whether to stagger in reverse order
    pub reverse: bool,
    /// Maximum number of animations to stagger
    pub max_count: Option<usize>,
}

impl StaggerConfig {
    /// Create a new stagger configuration
    pub fn new(delay: f64) -> Self {
        Self {
            delay,
            reverse: false,
            max_count: None,
        }
    }

    /// Enable reverse staggering
    pub fn reverse(mut self) -> Self {
        self.reverse = true;
        self
    }

    /// Set maximum stagger count
    pub fn max_count(mut self, count: usize) -> Self {
        self.max_count = Some(count);
        self
    }

    /// Calculate stagger delay for a given index
    pub fn get_delay(&self, index: usize) -> f64 {
        let effective_index = if self.reverse {
            if let Some(max) = self.max_count {
                max - 1 - index
            } else {
                index // Fallback if no max count
            }
        } else {
            index
        };

        effective_index as f64 * self.delay
    }
}

/// Stagger manager for coordinating multiple animations
pub struct StaggerManager {
    config: StaggerConfig,
    active_animations: Vec<usize>,
    completed_animations: Vec<usize>,
    total_animations: usize,
}

impl StaggerManager {
    /// Create a new stagger manager
    pub fn new(config: StaggerConfig, total_animations: usize) -> Self {
        Self {
            config,
            active_animations: Vec::new(),
            completed_animations: Vec::new(),
            total_animations,
        }
    }

    /// Get animations that should start at the current time
    pub fn get_animations_to_start(&self, current_time: f64) -> Vec<usize> {
        let mut to_start = Vec::new();

        for i in 0..self.total_animations {
            let delay = self.config.get_delay(i);

            // Check if this animation should start now
            if current_time >= delay
                && !self.active_animations.contains(&i)
                && !self.completed_animations.contains(&i)
            {
                to_start.push(i);
            }
        }

        to_start
    }

    /// Mark an animation as started
    pub fn start_animation(&mut self, index: usize) {
        if !self.active_animations.contains(&index) {
            self.active_animations.push(index);
        }
    }

    /// Mark an animation as completed
    pub fn complete_animation(&mut self, index: usize) {
        if let Some(pos) = self.active_animations.iter().position(|&x| x == index) {
            self.active_animations.remove(pos);
        }
        if !self.completed_animations.contains(&index) {
            self.completed_animations.push(index);
        }
    }

    /// Check if all animations are complete
    pub fn all_complete(&self) -> bool {
        self.completed_animations.len() == self.total_animations
    }

    /// Get the total duration including stagger delays
    pub fn total_duration(&self, base_duration: f64) -> f64 {
        if self.total_animations == 0 {
            return 0.0;
        }

        let max_delay = self.config.get_delay(self.total_animations - 1);
        max_delay + base_duration
    }
}

/// Convenience functions for common repeat patterns
pub mod presets {
    use super::*;

    /// Create a repeat config for a single play
    pub fn once() -> RepeatConfig {
        RepeatConfig::Never
    }

    /// Create a repeat config for a specific count
    pub fn times(count: u32) -> RepeatConfig {
        RepeatConfig::Count(count)
    }

    /// Create a repeat config for infinite repeats
    pub fn forever() -> RepeatConfig {
        RepeatConfig::Infinite
    }

    /// Create a repeat config for infinite reverse repeats
    pub fn forever_reverse() -> RepeatConfig {
        RepeatConfig::InfiniteReverse
    }

    /// Create a stagger config for subtle delays
    pub fn subtle_stagger() -> StaggerConfig {
        StaggerConfig::new(0.1)
    }

    /// Create a stagger config for dramatic delays
    pub fn dramatic_stagger() -> StaggerConfig {
        StaggerConfig::new(0.3)
    }

    /// Create a stagger config for rapid succession
    pub fn rapid_stagger() -> StaggerConfig {
        StaggerConfig::new(0.05)
    }
}
