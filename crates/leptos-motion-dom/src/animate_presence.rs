//! AnimatePresence Component
//!
//! A component for handling enter and exit animations of conditionally rendered elements

use leptos::prelude::*;
use leptos::reactive::signal::signal;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;

/// Type alias for animate presence hook return type
type UseAnimatePresenceReturn = (
    ReadSignal<bool>,
    WriteSignal<bool>,
    ReadSignal<Vec<String>>,
    WriteSignal<Vec<String>>,
);

/// Mode for handling presence animations
#[derive(Debug, Clone, PartialEq, Default)]
pub enum PresenceMode {
    /// Synchronous mode - all animations complete before next state
    #[default]
    Sync,
    /// Wait mode - wait for exit animations before entering new elements
    Wait,
    /// Immediate mode - start enter animations immediately
    Immediate,
    /// Pop layout mode - handle layout changes
    PopLayout,
}

/// Configuration for AnimatePresence
#[derive(Debug, Clone, Default)]
pub struct AnimatePresenceConfig {
    /// Presence mode
    pub mode: PresenceMode,
    /// Default exit transition
    pub exit_transition: Option<Transition>,
    /// Default enter transition
    pub enter_transition: Option<Transition>,
}

/// AnimatePresence component for handling enter/exit animations
#[component]
pub fn AnimatePresence(
    /// Presence mode
    #[prop(optional)]
    mode: Option<PresenceMode>,
    /// Default exit transition
    #[prop(optional)]
    exit_transition: Option<Transition>,
    /// Default enter transition
    #[prop(optional)]
    enter_transition: Option<Transition>,
    /// Children to animate
    children: Children,
) -> impl IntoView {
    let _config = AnimatePresenceConfig {
        mode: mode.unwrap_or_default(),
        exit_transition,
        enter_transition,
    };

    view! {
        <div class="animate-presence">
            {children()}
        </div>
    }
}

/// Hook for managing presence state
pub fn use_animate_presence() -> UseAnimatePresenceReturn {
    let (is_present, set_is_present) = signal(false);
    let (children, set_children) = signal(Vec::<String>::new());

    (is_present, set_is_present, children, set_children)
}

/// Hook for managing exiting children
pub fn use_exiting_children() -> (ReadSignal<Vec<String>>, WriteSignal<Vec<String>>) {
    signal(Vec::<String>::new())
}

/// Presence animation manager
#[derive(Debug, Clone)]
pub struct PresenceManager {
    #[allow(dead_code)]
    config: AnimatePresenceConfig,
    children: Vec<String>,
    exiting_children: Vec<String>,
    animations: HashMap<String, AnimationTarget>,
}

impl PresenceManager {
    /// Create a new presence manager
    pub fn new(config: AnimatePresenceConfig) -> Self {
        Self {
            config,
            children: Vec::new(),
            exiting_children: Vec::new(),
            animations: HashMap::new(),
        }
    }

    /// Add a child to be animated
    pub fn add_child(&mut self, id: String, animation: Option<AnimationTarget>) {
        if !self.children.contains(&id) {
            self.children.push(id.clone());
            if let Some(anim) = animation {
                self.animations.insert(id, anim);
            }
        }
    }

    /// Remove a child (start exit animation)
    pub fn remove_child(&mut self, id: &str) {
        if let Some(pos) = self.children.iter().position(|x| x == id) {
            self.children.remove(pos);
            self.exiting_children.push(id.to_string());
        }
    }

    /// Update exiting children
    pub fn update_exiting_children(&mut self) {
        // In a real implementation, this would handle exit animation completion
        self.exiting_children.clear();
    }

    /// Get current children
    pub fn current_children(&self) -> &Vec<String> {
        &self.children
    }

    /// Get exiting children
    pub fn current_exiting_children(&self) -> &Vec<String> {
        &self.exiting_children
    }

    /// Check if a child is present
    pub fn is_child_present(&self, id: &str) -> bool {
        self.children.contains(&id.to_string())
    }

    /// Check if a child is exiting
    pub fn is_child_exiting(&self, id: &str) -> bool {
        self.exiting_children.contains(&id.to_string())
    }

    /// Get animation for a child
    pub fn get_child_animation(&self, id: &str) -> Option<&AnimationTarget> {
        self.animations.get(id)
    }

    /// Set animation for a child
    pub fn set_child_animation(&mut self, id: String, animation: AnimationTarget) {
        self.animations.insert(id, animation);
    }

    /// Clear all children
    pub fn clear(&mut self) {
        self.children.clear();
        self.exiting_children.clear();
        self.animations.clear();
    }

    /// Get child count
    pub fn child_count(&self) -> usize {
        self.children.len()
    }

    /// Get exiting child count
    pub fn exiting_child_count(&self) -> usize {
        self.exiting_children.len()
    }
}

impl Default for PresenceManager {
    fn default() -> Self {
        Self::new(AnimatePresenceConfig::default())
    }
}

/// Utility function to create exit animation
pub fn create_exit_animation() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );
    target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(0.8),
    );
    target
}

/// Utility function to create enter animation
pub fn create_enter_animation() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(1.0),
    );
    target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(1.0),
    );
    target
}

/// Utility function to create initial animation
pub fn create_initial_animation() -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(
        "opacity".to_string(),
        leptos_motion_core::AnimationValue::Number(0.0),
    );
    target.insert(
        "scale".to_string(),
        leptos_motion_core::AnimationValue::Number(0.8),
    );
    target
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_presence_manager_basic() {
        let mut manager = PresenceManager::default();

        assert_eq!(manager.child_count(), 0);
        assert_eq!(manager.exiting_child_count(), 0);

        manager.add_child("child1".to_string(), None);
        assert_eq!(manager.child_count(), 1);
        assert!(manager.is_child_present("child1"));

        manager.remove_child("child1");
        assert_eq!(manager.child_count(), 0);
        assert_eq!(manager.exiting_child_count(), 1);
        assert!(manager.is_child_exiting("child1"));
    }

    #[test]
    fn test_presence_manager_animations() {
        let mut manager = PresenceManager::default();

        let animation = create_enter_animation();
        manager.add_child("child1".to_string(), Some(animation.clone()));

        assert!(manager.get_child_animation("child1").is_some());
        assert_eq!(manager.get_child_animation("child1").unwrap(), &animation);

        let new_animation = create_exit_animation();
        manager.set_child_animation("child1".to_string(), new_animation.clone());
        assert_eq!(
            manager.get_child_animation("child1").unwrap(),
            &new_animation
        );
    }

    #[test]
    fn test_presence_manager_clear() {
        let mut manager = PresenceManager::default();

        manager.add_child("child1".to_string(), None);
        manager.add_child("child2".to_string(), None);
        manager.remove_child("child1");

        assert_eq!(manager.child_count(), 1);
        assert_eq!(manager.exiting_child_count(), 1);

        manager.clear();

        assert_eq!(manager.child_count(), 0);
        assert_eq!(manager.exiting_child_count(), 0);
    }

    #[test]
    fn test_animation_utilities() {
        let exit_anim = create_exit_animation();
        let enter_anim = create_enter_animation();
        let initial_anim = create_initial_animation();

        // Exit animation should have opacity 0
        assert_eq!(
            exit_anim.get("opacity").unwrap(),
            &leptos_motion_core::AnimationValue::Number(0.0)
        );

        // Enter animation should have opacity 1
        assert_eq!(
            enter_anim.get("opacity").unwrap(),
            &leptos_motion_core::AnimationValue::Number(1.0)
        );

        // Initial animation should have opacity 0
        assert_eq!(
            initial_anim.get("opacity").unwrap(),
            &leptos_motion_core::AnimationValue::Number(0.0)
        );
    }
}
