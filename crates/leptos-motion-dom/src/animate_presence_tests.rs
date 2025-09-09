// AnimatePresence Tests
//
// These tests define the expected behavior of the AnimatePresence component
// for exit animations using Test-Driven Development.

use leptos::prelude::*;
use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Presence mode for AnimatePresence component
#[derive(Debug, Clone, PartialEq)]
pub enum PresenceMode {
    /// Wait for exit animation to complete before removing element
    Wait,
    /// Immediately remove element (no exit animation)
    Immediate,
    /// Allow multiple elements with same key
    PopLayout,
}

impl Default for PresenceMode {
    fn default() -> Self {
        Self::Wait
    }
}

/// Exit animation configuration
#[derive(Debug, Clone, PartialEq)]
pub struct ExitAnimation {
    /// Animation target for exit state
    pub target: HashMap<String, AnimationValue>,
    /// Transition configuration
    pub transition: Transition,
}

impl Default for ExitAnimation {
    fn default() -> Self {
        let mut target = HashMap::new();
        target.insert("opacity".to_string(), AnimationValue::Number(0.0));
        target.insert("scale".to_string(), AnimationValue::Number(0.8));

        Self {
            target,
            transition: Transition {
                duration: Some(0.3),
                ease: Easing::EaseOut,
                delay: None,
                repeat: RepeatConfig::Never,
                stagger: None,
            },
        }
    }
}

/// AnimatePresence component for exit animations
pub struct AnimatePresence {
    mode: PresenceMode,
    exit_animation: ExitAnimation,
    children: Vec<AnimatePresenceChild>,
}

/// Child component with exit animation support
pub struct AnimatePresenceChild {
    key: String,
    visible: bool,
    exit_animation: Option<ExitAnimation>,
}

impl AnimatePresenceChild {
    pub fn new(key: String, visible: bool) -> Self {
        Self {
            key,
            visible,
            exit_animation: None,
        }
    }

    pub fn with_exit_animation(mut self, animation: ExitAnimation) -> Self {
        self.exit_animation = Some(animation);
        self
    }
}

impl AnimatePresence {
    pub fn new(mode: PresenceMode) -> Self {
        Self {
            mode,
            exit_animation: ExitAnimation::default(),
            children: Vec::new(),
        }
    }

    pub fn with_exit_animation(mut self, animation: ExitAnimation) -> Self {
        self.exit_animation = animation;
        self
    }

    pub fn add_child(&mut self, child: AnimatePresenceChild) {
        self.children.push(child);
    }

    /// Update visibility of a child by key
    pub fn set_child_visible(&mut self, key: &str, visible: bool) {
        for child in &mut self.children {
            if child.key == key {
                child.visible = visible;
                break;
            }
        }
    }

    /// Get all visible children
    pub fn visible_children(&self) -> Vec<&AnimatePresenceChild> {
        self.children.iter().filter(|child| child.visible).collect()
    }

    /// Get all children (including exiting)
    pub fn all_children(&self) -> &[AnimatePresenceChild] {
        &self.children
    }

    /// Check if any child is currently exiting
    pub fn has_exiting_children(&self) -> bool {
        self.children.iter().any(|child| !child.visible)
    }
}

/// Test basic AnimatePresence functionality
#[test]
fn test_animate_presence_basic() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Add a child
    let child = AnimatePresenceChild::new("item1".to_string(), true);
    presence.add_child(child);

    // Should have one visible child
    assert_eq!(presence.visible_children().len(), 1);
    assert_eq!(presence.visible_children()[0].key, "item1");

    // Hide the child
    presence.set_child_visible("item1", false);

    // Should have no visible children
    assert_eq!(presence.visible_children().len(), 0);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence with multiple children
#[test]
fn test_animate_presence_multiple_children() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Add multiple children
    presence.add_child(AnimatePresenceChild::new("item1".to_string(), true));
    presence.add_child(AnimatePresenceChild::new("item2".to_string(), true));
    presence.add_child(AnimatePresenceChild::new("item3".to_string(), false));

    // Should have two visible children
    assert_eq!(presence.visible_children().len(), 2);

    // Hide one more
    presence.set_child_visible("item2", false);

    // Should have one visible child
    assert_eq!(presence.visible_children().len(), 1);
    assert_eq!(presence.visible_children()[0].key, "item1");
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence with custom exit animation
#[test]
fn test_animate_presence_custom_exit_animation() {
    let mut exit_animation = ExitAnimation::default();
    exit_animation
        .target
        .insert("rotateZ".to_string(), AnimationValue::Number(180.0));
    exit_animation.transition.duration = Some(0.5);

    let mut presence =
        AnimatePresence::new(PresenceMode::Wait).with_exit_animation(exit_animation.clone());

    let child =
        AnimatePresenceChild::new("item1".to_string(), true).with_exit_animation(exit_animation);
    presence.add_child(child);

    // Should have one visible child
    assert_eq!(presence.visible_children().len(), 1);

    // Hide the child
    presence.set_child_visible("item1", false);

    // Should have no visible children but child should still exist
    assert_eq!(presence.visible_children().len(), 0);
    assert_eq!(presence.all_children().len(), 1);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence with Immediate mode
#[test]
fn test_animate_presence_immediate_mode() {
    let mut presence = AnimatePresence::new(PresenceMode::Immediate);

    let child = AnimatePresenceChild::new("item1".to_string(), true);
    presence.add_child(child);

    // Should have one visible child
    assert_eq!(presence.visible_children().len(), 1);

    // Hide the child
    presence.set_child_visible("item1", false);

    // In immediate mode, child should be removed immediately
    assert_eq!(presence.visible_children().len(), 0);
    assert_eq!(presence.all_children().len(), 1); // Still exists in the list
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence with PopLayout mode
#[test]
fn test_animate_presence_pop_layout_mode() {
    let mut presence = AnimatePresence::new(PresenceMode::PopLayout);

    // Add multiple children with same key (allowed in PopLayout mode)
    presence.add_child(AnimatePresenceChild::new("item1".to_string(), true));
    presence.add_child(AnimatePresenceChild::new("item1".to_string(), true));

    // Should have two visible children
    assert_eq!(presence.visible_children().len(), 2);

    // Hide one
    presence.set_child_visible("item1", false);

    // Should still have one visible child
    assert_eq!(presence.visible_children().len(), 1);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence exit animation timing
#[test]
fn test_animate_presence_exit_animation_timing() {
    let mut exit_animation = ExitAnimation::default();
    exit_animation.transition.duration = Some(0.5);
    exit_animation.transition.delay = Some(0.1);

    let mut presence = AnimatePresence::new(PresenceMode::Wait).with_exit_animation(exit_animation);

    let child = AnimatePresenceChild::new("item1".to_string(), true);
    presence.add_child(child);

    // Hide the child
    presence.set_child_visible("item1", false);

    // Should have no visible children
    assert_eq!(presence.visible_children().len(), 0);
    assert!(presence.has_exiting_children());

    // Child should still exist for exit animation
    assert_eq!(presence.all_children().len(), 1);
}

/// Test AnimatePresence with different exit animations per child
#[test]
fn test_animate_presence_per_child_exit_animation() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Create different exit animations
    let mut fade_animation = ExitAnimation::default();
    fade_animation
        .target
        .insert("opacity".to_string(), AnimationValue::Number(0.0));

    let mut slide_animation = ExitAnimation::default();
    slide_animation
        .target
        .insert("y".to_string(), AnimationValue::Number(-100.0));

    // Add children with different exit animations
    presence.add_child(
        AnimatePresenceChild::new("fade_item".to_string(), true)
            .with_exit_animation(fade_animation),
    );
    presence.add_child(
        AnimatePresenceChild::new("slide_item".to_string(), true)
            .with_exit_animation(slide_animation),
    );

    // Should have two visible children
    assert_eq!(presence.visible_children().len(), 2);

    // Hide both
    presence.set_child_visible("fade_item", false);
    presence.set_child_visible("slide_item", false);

    // Should have no visible children
    assert_eq!(presence.visible_children().len(), 0);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence performance with many children
#[test]
fn test_animate_presence_performance() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Add many children
    for i in 0..1000 {
        presence.add_child(AnimatePresenceChild::new(format!("item{}", i), true));
    }

    // Should have 1000 visible children
    assert_eq!(presence.visible_children().len(), 1000);

    // Hide half of them
    for i in 0..500 {
        presence.set_child_visible(&format!("item{}", i), false);
    }

    // Should have 500 visible children
    assert_eq!(presence.visible_children().len(), 500);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence edge cases
#[test]
fn test_animate_presence_edge_cases() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Test with empty children
    assert_eq!(presence.visible_children().len(), 0);
    assert!(!presence.has_exiting_children());

    // Test hiding non-existent child
    presence.set_child_visible("nonexistent", false);
    assert_eq!(presence.visible_children().len(), 0);

    // Test showing non-existent child
    presence.set_child_visible("nonexistent", true);
    assert_eq!(presence.visible_children().len(), 0);

    // Add child and test toggling
    presence.add_child(AnimatePresenceChild::new("item1".to_string(), false));
    assert_eq!(presence.visible_children().len(), 0);

    presence.set_child_visible("item1", true);
    assert_eq!(presence.visible_children().len(), 1);

    presence.set_child_visible("item1", false);
    assert_eq!(presence.visible_children().len(), 0);
    assert!(presence.has_exiting_children());
}

/// Test AnimatePresence integration with MotionDiv
#[test]
fn test_animate_presence_motion_div_integration() {
    let mut presence = AnimatePresence::new(PresenceMode::Wait);

    // Create exit animation that works with MotionDiv
    let mut exit_animation = ExitAnimation::default();
    exit_animation
        .target
        .insert("opacity".to_string(), AnimationValue::Number(0.0));
    exit_animation
        .target
        .insert("scale".to_string(), AnimationValue::Number(0.5));
    exit_animation
        .target
        .insert("rotateZ".to_string(), AnimationValue::Number(180.0));

    let child = AnimatePresenceChild::new("motion_item".to_string(), true)
        .with_exit_animation(exit_animation);
    presence.add_child(child);

    // Should have one visible child
    assert_eq!(presence.visible_children().len(), 1);

    // Hide the child
    presence.set_child_visible("motion_item", false);

    // Should have no visible children
    assert_eq!(presence.visible_children().len(), 0);
    assert!(presence.has_exiting_children());

    // Child should still exist for exit animation
    assert_eq!(presence.all_children().len(), 1);
}
