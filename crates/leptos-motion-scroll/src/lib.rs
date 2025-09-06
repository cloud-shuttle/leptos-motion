//! Scroll animations for Leptos Motion

#![warn(missing_docs)]

use std::collections::HashMap;

/// Scroll direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    /// Scrolling up
    Up,
    /// Scrolling down
    Down,
    /// Scrolling left
    Left,
    /// Scrolling right
    Right,
}

/// Scroll trigger configuration
#[derive(Debug, Clone)]
pub struct ScrollTrigger {
    /// Trigger threshold (0.0 to 1.0)
    pub threshold: f64,
    /// Whether trigger should repeat
    pub repeat: bool,
    /// Element ID to observe
    pub element_id: String,
}

impl Default for ScrollTrigger {
    fn default() -> Self {
        Self {
            threshold: 0.5,
            repeat: false,
            element_id: String::new(),
        }
    }
}

impl ScrollTrigger {
    /// Create new scroll trigger
    pub fn new(element_id: String) -> Self {
        Self {
            element_id,
            ..Self::default()
        }
    }

    /// Set threshold
    pub fn with_threshold(mut self, threshold: f64) -> Self {
        self.threshold = threshold.clamp(0.0, 1.0);
        self
    }

    /// Set repeat
    pub fn with_repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }
}

/// Scroll animation state
#[derive(Debug, Clone)]
pub struct ScrollAnimationState {
    /// Current scroll position
    pub scroll_y: f64,
    /// Previous scroll position
    pub previous_scroll_y: f64,
    /// Scroll direction
    pub direction: ScrollDirection,
    /// Scroll velocity
    pub velocity: f64,
}

impl Default for ScrollAnimationState {
    fn default() -> Self {
        Self {
            scroll_y: 0.0,
            previous_scroll_y: 0.0,
            direction: ScrollDirection::Down,
            velocity: 0.0,
        }
    }
}

impl ScrollAnimationState {
    /// Create new scroll state
    pub fn new() -> Self {
        Self::default()
    }

    /// Update scroll position
    pub fn update_position(&mut self, new_y: f64) {
        self.previous_scroll_y = self.scroll_y;
        self.scroll_y = new_y;

        let delta = new_y - self.previous_scroll_y;
        self.velocity = delta;

        self.direction = if delta > 0.0 {
            ScrollDirection::Down
        } else if delta < 0.0 {
            ScrollDirection::Up
        } else {
            self.direction // Keep previous direction if no change
        };
    }

    /// Get scroll progress (0.0 to 1.0)
    pub fn get_progress(&self, viewport_height: f64, document_height: f64) -> f64 {
        if document_height <= viewport_height {
            return 0.0;
        }
        let max_scroll = document_height - viewport_height;
        (self.scroll_y / max_scroll).clamp(0.0, 1.0)
    }

    /// Check if scrolling in direction
    pub fn is_scrolling(&self, direction: ScrollDirection) -> bool {
        self.direction == direction && self.velocity.abs() > 0.1
    }
}

/// Scroll animator
#[derive(Default)]
pub struct ScrollAnimator {
    /// Active animations
    pub active: bool,
    /// Scroll triggers
    triggers: HashMap<String, ScrollTrigger>,
    /// Current scroll state
    scroll_state: ScrollAnimationState,
    /// Triggered elements
    triggered: HashMap<String, bool>,
}

impl ScrollAnimator {
    /// Create new scroll animator
    pub fn new() -> Self {
        Self {
            active: false,
            triggers: HashMap::new(),
            scroll_state: ScrollAnimationState::new(),
            triggered: HashMap::new(),
        }
    }

    /// Add scroll trigger
    pub fn add_trigger(&mut self, trigger: ScrollTrigger) {
        let id = trigger.element_id.clone();
        self.triggers.insert(id.clone(), trigger);
        self.triggered.insert(id, false);
    }

    /// Remove scroll trigger
    pub fn remove_trigger(&mut self, element_id: &str) -> bool {
        let removed = self.triggers.remove(element_id).is_some();
        self.triggered.remove(element_id);
        removed
    }

    /// Update scroll position
    pub fn update_scroll(&mut self, scroll_y: f64) {
        self.scroll_state.update_position(scroll_y);
        self.active = self.scroll_state.velocity.abs() > 0.1;
    }

    /// Check triggers and return activated ones
    pub fn check_triggers(&mut self, viewport_height: f64, document_height: f64) -> Vec<String> {
        let mut activated = Vec::new();
        let progress = self
            .scroll_state
            .get_progress(viewport_height, document_height);

        for (id, trigger) in &self.triggers {
            let was_triggered = self.triggered.get(id).copied().unwrap_or(false);
            let should_trigger = progress >= trigger.threshold;

            if should_trigger && (!was_triggered || trigger.repeat) {
                activated.push(id.clone());
                self.triggered.insert(id.clone(), true);
            } else if !should_trigger && trigger.repeat {
                self.triggered.insert(id.clone(), false);
            }
        }

        activated
    }

    /// Get current scroll state
    pub fn get_scroll_state(&self) -> &ScrollAnimationState {
        &self.scroll_state
    }

    /// Get trigger count
    pub fn trigger_count(&self) -> usize {
        self.triggers.len()
    }

    /// Clear all triggers
    pub fn clear_triggers(&mut self) {
        self.triggers.clear();
        self.triggered.clear();
    }

    /// Check if element was triggered
    pub fn was_triggered(&self, element_id: &str) -> bool {
        self.triggered.get(element_id).copied().unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_direction_values() {
        assert_eq!(ScrollDirection::Up, ScrollDirection::Up);
        assert_eq!(ScrollDirection::Down, ScrollDirection::Down);
        assert_eq!(ScrollDirection::Left, ScrollDirection::Left);
        assert_eq!(ScrollDirection::Right, ScrollDirection::Right);
    }

    #[test]
    fn test_scroll_trigger_new() {
        let trigger = ScrollTrigger::new("element1".to_string());
        assert_eq!(trigger.element_id, "element1");
        assert_eq!(trigger.threshold, 0.5);
        assert!(!trigger.repeat);
    }

    #[test]
    fn test_scroll_trigger_default() {
        let trigger = ScrollTrigger::default();
        assert_eq!(trigger.element_id, "");
        assert_eq!(trigger.threshold, 0.5);
        assert!(!trigger.repeat);
    }

    #[test]
    fn test_scroll_trigger_builder() {
        let trigger = ScrollTrigger::new("element1".to_string())
            .with_threshold(0.8)
            .with_repeat(true);

        assert_eq!(trigger.element_id, "element1");
        assert_eq!(trigger.threshold, 0.8);
        assert!(trigger.repeat);
    }

    #[test]
    fn test_scroll_trigger_threshold_clamping() {
        let trigger1 = ScrollTrigger::new("element1".to_string()).with_threshold(-0.5);
        assert_eq!(trigger1.threshold, 0.0);

        let trigger2 = ScrollTrigger::new("element2".to_string()).with_threshold(1.5);
        assert_eq!(trigger2.threshold, 1.0);
    }

    #[test]
    fn test_scroll_animation_state_new() {
        let state = ScrollAnimationState::new();
        assert_eq!(state.scroll_y, 0.0);
        assert_eq!(state.previous_scroll_y, 0.0);
        assert_eq!(state.direction, ScrollDirection::Down);
        assert_eq!(state.velocity, 0.0);
    }

    #[test]
    fn test_scroll_animation_state_default() {
        let state = ScrollAnimationState::default();
        assert_eq!(state.scroll_y, 0.0);
        assert_eq!(state.previous_scroll_y, 0.0);
        assert_eq!(state.direction, ScrollDirection::Down);
        assert_eq!(state.velocity, 0.0);
    }

    #[test]
    fn test_scroll_animation_state_update_position_down() {
        let mut state = ScrollAnimationState::new();
        state.update_position(100.0);

        assert_eq!(state.scroll_y, 100.0);
        assert_eq!(state.previous_scroll_y, 0.0);
        assert_eq!(state.direction, ScrollDirection::Down);
        assert_eq!(state.velocity, 100.0);
    }

    #[test]
    fn test_scroll_animation_state_update_position_up() {
        let mut state = ScrollAnimationState::new();
        state.update_position(100.0);
        state.update_position(50.0);

        assert_eq!(state.scroll_y, 50.0);
        assert_eq!(state.previous_scroll_y, 100.0);
        assert_eq!(state.direction, ScrollDirection::Up);
        assert_eq!(state.velocity, -50.0);
    }

    #[test]
    fn test_scroll_animation_state_get_progress() {
        let mut state = ScrollAnimationState::new();
        state.update_position(200.0);

        // Viewport: 600px, Document: 1200px, Scroll: 200px
        // Max scroll: 1200 - 600 = 600px
        // Progress: 200 / 600 = 0.33...
        let progress = state.get_progress(600.0, 1200.0);
        assert!((progress - 0.3333333333333333).abs() < 0.0001);
    }

    #[test]
    fn test_scroll_animation_state_get_progress_no_scroll() {
        let state = ScrollAnimationState::new();

        // Document fits in viewport
        let progress = state.get_progress(1200.0, 800.0);
        assert_eq!(progress, 0.0);
    }

    #[test]
    fn test_scroll_animation_state_is_scrolling() {
        let mut state = ScrollAnimationState::new();

        // Not scrolling initially
        assert!(!state.is_scrolling(ScrollDirection::Down));
        assert!(!state.is_scrolling(ScrollDirection::Up));

        // Scroll down with significant velocity
        state.update_position(10.0);
        assert!(state.is_scrolling(ScrollDirection::Down));
        assert!(!state.is_scrolling(ScrollDirection::Up));

        // Scroll up
        state.update_position(0.0);
        assert!(!state.is_scrolling(ScrollDirection::Down));
        assert!(state.is_scrolling(ScrollDirection::Up));
    }

    #[test]
    fn test_scroll_animator_new() {
        let animator = ScrollAnimator::new();
        assert!(!animator.active);
        assert_eq!(animator.trigger_count(), 0);
    }

    #[test]
    fn test_scroll_animator_default() {
        let animator = ScrollAnimator::default();
        assert!(!animator.active);
        assert_eq!(animator.trigger_count(), 0);
    }

    #[test]
    fn test_scroll_animator_add_trigger() {
        let mut animator = ScrollAnimator::new();
        let trigger = ScrollTrigger::new("element1".to_string());

        animator.add_trigger(trigger);
        assert_eq!(animator.trigger_count(), 1);
        assert!(!animator.was_triggered("element1"));
    }

    #[test]
    fn test_scroll_animator_remove_trigger() {
        let mut animator = ScrollAnimator::new();
        let trigger = ScrollTrigger::new("element1".to_string());

        animator.add_trigger(trigger);
        assert_eq!(animator.trigger_count(), 1);

        let removed = animator.remove_trigger("element1");
        assert!(removed);
        assert_eq!(animator.trigger_count(), 0);

        let not_removed = animator.remove_trigger("nonexistent");
        assert!(!not_removed);
    }

    #[test]
    fn test_scroll_animator_update_scroll() {
        let mut animator = ScrollAnimator::new();

        animator.update_scroll(100.0);
        assert!(animator.active); // Should be active due to velocity
        assert_eq!(animator.get_scroll_state().scroll_y, 100.0);
    }

    #[test]
    fn test_scroll_animator_check_triggers() {
        let mut animator = ScrollAnimator::new();

        // Add trigger at 50% threshold
        let trigger = ScrollTrigger::new("element1".to_string()).with_threshold(0.5);
        animator.add_trigger(trigger);

        // Scroll to 30% - should not trigger
        animator.update_scroll(180.0); // 30% of 600px max scroll (1200-600)
        let activated = animator.check_triggers(600.0, 1200.0);
        assert!(activated.is_empty());
        assert!(!animator.was_triggered("element1"));

        // Scroll to 60% - should trigger
        animator.update_scroll(360.0); // 60% of 600px max scroll
        let activated = animator.check_triggers(600.0, 1200.0);
        assert_eq!(activated.len(), 1);
        assert_eq!(activated[0], "element1");
        assert!(animator.was_triggered("element1"));
    }

    #[test]
    fn test_scroll_animator_check_triggers_repeat() {
        let mut animator = ScrollAnimator::new();

        // Add repeating trigger at 50% threshold
        let trigger = ScrollTrigger::new("element1".to_string())
            .with_threshold(0.5)
            .with_repeat(true);
        animator.add_trigger(trigger);

        // Scroll past threshold
        animator.update_scroll(360.0); // 60% of 600px max scroll
        let activated = animator.check_triggers(600.0, 1200.0);
        assert_eq!(activated.len(), 1);

        // Scroll back below threshold
        animator.update_scroll(240.0); // 40% of 600px max scroll
        let activated = animator.check_triggers(600.0, 1200.0);
        assert!(activated.is_empty());

        // Scroll past threshold again - should retrigger due to repeat
        animator.update_scroll(360.0); // 60% of 600px max scroll
        let activated = animator.check_triggers(600.0, 1200.0);
        assert_eq!(activated.len(), 1);
    }

    #[test]
    fn test_scroll_animator_clear_triggers() {
        let mut animator = ScrollAnimator::new();

        let trigger1 = ScrollTrigger::new("element1".to_string());
        let trigger2 = ScrollTrigger::new("element2".to_string());

        animator.add_trigger(trigger1);
        animator.add_trigger(trigger2);
        assert_eq!(animator.trigger_count(), 2);

        animator.clear_triggers();
        assert_eq!(animator.trigger_count(), 0);
    }

    #[test]
    fn test_scroll_animator_get_scroll_state() {
        let mut animator = ScrollAnimator::new();
        animator.update_scroll(150.0);

        let state = animator.get_scroll_state();
        assert_eq!(state.scroll_y, 150.0);
        assert_eq!(state.direction, ScrollDirection::Down);
    }
}
