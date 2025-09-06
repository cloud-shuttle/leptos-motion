//! CSS Class-Based Animation Tests
//!
//! These tests ensure our CSS class-based animation system works correctly
//! and provides the simplicity that makes leptos-animate appealing.

use std::collections::HashMap;

/// CSS class animation configuration
#[derive(Debug, Clone)]
pub struct CSSAnimationConfig {
    /// Base CSS class name
    pub base_class: String,
    /// Animation duration in milliseconds
    pub duration: u32,
    /// Animation easing function
    pub easing: String,
    /// Animation delay in milliseconds
    pub delay: u32,
    /// Whether to use CSS transitions
    pub use_transitions: bool,
    /// Whether to use CSS animations
    pub use_animations: bool,
}

impl Default for CSSAnimationConfig {
    fn default() -> Self {
        Self {
            base_class: "animate".to_string(),
            duration: 300,
            easing: "ease-in-out".to_string(),
            delay: 0,
            use_transitions: true,
            use_animations: false,
        }
    }
}

/// CSS class animation manager
pub struct CSSAnimationManager {
    config: CSSAnimationConfig,
    active_animations: HashMap<String, CSSAnimationState>,
    css_rules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CSSAnimationState {
    pub element_id: String,
    pub class_name: String,
    pub is_active: bool,
    pub start_time: u64,
    pub duration: u32,
}

impl CSSAnimationManager {
    /// Create a new CSS animation manager
    pub fn new(config: CSSAnimationConfig) -> Self {
        Self {
            config,
            active_animations: HashMap::new(),
            css_rules: Vec::new(),
        }
    }

    /// Generate CSS classes for common animations
    pub fn generate_css_classes(&mut self) -> String {
        let mut css = String::new();

        // Base animation class
        css.push_str(&format!(
            ".{} {{\n\
            \ttransition: all {}ms {};\n\
            }}\n\n",
            self.config.base_class, self.config.duration, self.config.easing
        ));

        // Fade animations
        css.push_str(&format!(
            ".{}-fade-in {{\n\
            \topacity: 1;\n\
            }}\n\n",
            self.config.base_class
        ));

        css.push_str(&format!(
            ".{}-fade-out {{\n\
            \topacity: 0;\n\
            }}\n\n",
            self.config.base_class
        ));

        // Slide animations
        css.push_str(&format!(
            ".{}-slide-in-left {{\n\
            \ttransform: translateX(0);\n\
            }}\n\n",
            self.config.base_class
        ));

        css.push_str(&format!(
            ".{}-slide-out-left {{\n\
            \ttransform: translateX(-100%);\n\
            }}\n\n",
            self.config.base_class
        ));

        // Scale animations
        css.push_str(&format!(
            ".{}-scale-in {{\n\
            \ttransform: scale(1);\n\
            }}\n\n",
            self.config.base_class
        ));

        css.push_str(&format!(
            ".{}-scale-out {{\n\
            \ttransform: scale(0);\n\
            }}\n\n",
            self.config.base_class
        ));

        self.css_rules.push(css.clone());
        css
    }

    /// Start a CSS class animation
    pub fn start_animation(
        &mut self,
        element_id: &str,
        animation_type: &str,
    ) -> Result<(), String> {
        let class_name = format!("{}-{}", self.config.base_class, animation_type);

        if self.active_animations.contains_key(element_id) {
            return Err(format!(
                "Animation already active for element {}",
                element_id
            ));
        }

        let state = CSSAnimationState {
            element_id: element_id.to_string(),
            class_name: class_name.clone(),
            is_active: true,
            start_time: 0, // Would be current timestamp in real implementation
            duration: self.config.duration,
        };

        self.active_animations.insert(element_id.to_string(), state);
        Ok(())
    }

    /// Stop a CSS class animation
    pub fn stop_animation(&mut self, element_id: &str) -> Result<(), String> {
        if let Some(state) = self.active_animations.get_mut(element_id) {
            state.is_active = false;
            Ok(())
        } else {
            Err(format!(
                "No active animation found for element {}",
                element_id
            ))
        }
    }

    /// Check if an animation is active
    pub fn is_animation_active(&self, element_id: &str) -> bool {
        self.active_animations
            .get(element_id)
            .map(|state| state.is_active)
            .unwrap_or(false)
    }

    /// Get the CSS class for an element
    pub fn get_element_class(&self, element_id: &str) -> Option<String> {
        self.active_animations
            .get(element_id)
            .map(|state| state.class_name.clone())
    }

    /// Get all active animations
    pub fn get_active_animations(&self) -> Vec<&CSSAnimationState> {
        self.active_animations
            .values()
            .filter(|state| state.is_active)
            .collect()
    }

    /// Clear all animations
    pub fn clear_all(&mut self) {
        self.active_animations.clear();
    }

    /// Get CSS rules
    pub fn get_css_rules(&self) -> &[String] {
        &self.css_rules
    }
}

/// Test CSS animation manager creation
#[test]
fn test_css_animation_manager_creation() {
    let config = CSSAnimationConfig::default();
    let manager = CSSAnimationManager::new(config);

    assert_eq!(manager.get_active_animations().len(), 0);
    assert!(manager.get_css_rules().is_empty());
}

/// Test CSS class generation
#[test]
fn test_css_class_generation() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    let css = manager.generate_css_classes();

    assert!(css.contains(".animate"));
    assert!(css.contains("transition: all 300ms ease-in-out"));
    assert!(css.contains(".animate-fade-in"));
    assert!(css.contains(".animate-fade-out"));
    assert!(css.contains(".animate-slide-in-left"));
    assert!(css.contains(".animate-slide-out-left"));
    assert!(css.contains(".animate-scale-in"));
    assert!(css.contains(".animate-scale-out"));

    assert_eq!(manager.get_css_rules().len(), 1);
}

/// Test starting animations
#[test]
fn test_start_animation() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start a fade-in animation
    let result = manager.start_animation("element1", "fade-in");
    assert!(result.is_ok());

    assert!(manager.is_animation_active("element1"));
    assert_eq!(
        manager.get_element_class("element1"),
        Some("animate-fade-in".to_string())
    );
    assert_eq!(manager.get_active_animations().len(), 1);
}

/// Test stopping animations
#[test]
fn test_stop_animation() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start and then stop an animation
    manager.start_animation("element1", "fade-in").unwrap();
    assert!(manager.is_animation_active("element1"));

    let result = manager.stop_animation("element1");
    assert!(result.is_ok());
    assert!(!manager.is_animation_active("element1"));
    assert_eq!(manager.get_active_animations().len(), 0);
}

/// Test multiple animations
#[test]
fn test_multiple_animations() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start multiple animations
    manager.start_animation("element1", "fade-in").unwrap();
    manager
        .start_animation("element2", "slide-in-left")
        .unwrap();
    manager.start_animation("element3", "scale-in").unwrap();

    assert_eq!(manager.get_active_animations().len(), 3);
    assert!(manager.is_animation_active("element1"));
    assert!(manager.is_animation_active("element2"));
    assert!(manager.is_animation_active("element3"));

    // Check individual classes
    assert_eq!(
        manager.get_element_class("element1"),
        Some("animate-fade-in".to_string())
    );
    assert_eq!(
        manager.get_element_class("element2"),
        Some("animate-slide-in-left".to_string())
    );
    assert_eq!(
        manager.get_element_class("element3"),
        Some("animate-scale-in".to_string())
    );
}

/// Test animation error handling
#[test]
fn test_animation_error_handling() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Try to start animation on non-existent element (should work)
    let result = manager.start_animation("element1", "fade-in");
    assert!(result.is_ok());

    // Try to start animation on same element again (should fail)
    let result = manager.start_animation("element1", "fade-out");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("already active"));

    // Try to stop animation on non-existent element (should fail)
    let result = manager.stop_animation("nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("No active animation"));
}

/// Test custom configuration
#[test]
fn test_custom_configuration() {
    let config = CSSAnimationConfig {
        base_class: "custom-animate".to_string(),
        duration: 500,
        easing: "ease".to_string(),
        delay: 100,
        use_transitions: true,
        use_animations: false,
    };

    let mut manager = CSSAnimationManager::new(config);
    let css = manager.generate_css_classes();

    assert!(css.contains(".custom-animate"));
    assert!(css.contains("transition: all 500ms ease"));
    assert!(css.contains(".custom-animate-fade-in"));
}

/// Test clear all animations
#[test]
fn test_clear_all_animations() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start multiple animations
    manager.start_animation("element1", "fade-in").unwrap();
    manager
        .start_animation("element2", "slide-in-left")
        .unwrap();
    manager.start_animation("element3", "scale-in").unwrap();

    assert_eq!(manager.get_active_animations().len(), 3);

    // Clear all animations
    manager.clear_all();

    assert_eq!(manager.get_active_animations().len(), 0);
    assert!(!manager.is_animation_active("element1"));
    assert!(!manager.is_animation_active("element2"));
    assert!(!manager.is_animation_active("element3"));
}

/// Test animation state management
#[test]
fn test_animation_state_management() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start an animation
    manager.start_animation("element1", "fade-in").unwrap();

    let active_animations = manager.get_active_animations();
    assert_eq!(active_animations.len(), 1);

    let state = active_animations[0];
    assert_eq!(state.element_id, "element1");
    assert_eq!(state.class_name, "animate-fade-in");
    assert!(state.is_active);
    assert_eq!(state.duration, 300);
}

/// Test CSS rule management
#[test]
fn test_css_rule_management() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Initially no CSS rules
    assert_eq!(manager.get_css_rules().len(), 0);

    // Generate CSS classes
    manager.generate_css_classes();
    assert_eq!(manager.get_css_rules().len(), 1);

    // Generate again (should add another rule)
    manager.generate_css_classes();
    assert_eq!(manager.get_css_rules().len(), 2);
}

/// Test integration with Tailwind CSS classes
#[test]
fn test_tailwind_integration() {
    let config = CSSAnimationConfig {
        base_class: "transition".to_string(),
        duration: 300,
        easing: "ease-in-out".to_string(),
        delay: 0,
        use_transitions: true,
        use_animations: false,
    };

    let mut manager = CSSAnimationManager::new(config);
    let css = manager.generate_css_classes();

    // Should generate Tailwind-compatible classes
    assert!(css.contains(".transition"));
    assert!(css.contains("transition: all 300ms ease-in-out"));

    // Start animations with Tailwind-style classes
    manager.start_animation("element1", "fade-in").unwrap();
    assert_eq!(
        manager.get_element_class("element1"),
        Some("transition-fade-in".to_string())
    );
}

/// Test performance with many animations
#[test]
fn test_performance_many_animations() {
    let config = CSSAnimationConfig::default();
    let mut manager = CSSAnimationManager::new(config);

    // Start many animations
    for i in 0..100 {
        let element_id = format!("element{}", i);
        let animation_type = if i % 2 == 0 {
            "fade-in"
        } else {
            "slide-in-left"
        };
        manager
            .start_animation(&element_id, animation_type)
            .unwrap();
    }

    assert_eq!(manager.get_active_animations().len(), 100);

    // Check a few specific elements
    assert!(manager.is_animation_active("element0"));
    assert!(manager.is_animation_active("element50"));
    assert!(manager.is_animation_active("element99"));

    // Clear all
    manager.clear_all();
    assert_eq!(manager.get_active_animations().len(), 0);
}
