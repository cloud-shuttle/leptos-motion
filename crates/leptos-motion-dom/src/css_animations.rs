//! CSS Class-Based Animations
//! 
//! This module provides CSS class-based animation support, bridging the simplicity
//! gap with leptos-animate while maintaining our advanced features.

use std::collections::HashMap;
use web_sys::Element;
use wasm_bindgen::prelude::*;
// use leptos::prelude::*; // Not used in current implementation

/// CSS animation configuration
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

/// CSS animation state
#[derive(Debug, Clone)]
pub struct CSSAnimationState {
    pub element_id: String,
    pub class_name: String,
    pub is_active: bool,
    pub start_time: u64,
    pub duration: u32,
}

/// CSS class animation manager
pub struct CSSAnimationManager {
    config: CSSAnimationConfig,
    active_animations: HashMap<String, CSSAnimationState>,
    css_rules: Vec<String>,
    style_element: Option<Element>,
}

impl CSSAnimationManager {
    /// Create a new CSS animation manager
    pub fn new(config: CSSAnimationConfig) -> Self {
        Self {
            config,
            active_animations: HashMap::new(),
            css_rules: Vec::new(),
            style_element: None,
        }
    }

    /// Initialize the CSS animation manager with DOM integration
    pub fn initialize(&mut self) -> Result<(), JsValue> {
        // Create a style element for our CSS rules
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;
        
        let style_element = document
            .create_element("style")
            .map_err(|_| "Failed to create style element")?;
        
        style_element.set_attribute("id", "leptos-motion-css-animations")?;
        document.head()
            .ok_or("No head element")?
            .append_child(&style_element)?;
        
        self.style_element = Some(style_element);
        
        // Generate and inject CSS classes
        let css = self.generate_css_classes();
        self.inject_css(&css)?;
        
        Ok(())
    }

    /// Generate CSS classes for common animations
    pub fn generate_css_classes(&mut self) -> String {
        let mut css = String::new();
        
        // Base animation class
        css.push_str(&format!(
            ".{} {{\n\
            \ttransition: all {}ms {};\n\
            }}\n\n",
            self.config.base_class,
            self.config.duration,
            self.config.easing
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
        
        css.push_str(&format!(
            ".{}-slide-in-right {{\n\
            \ttransform: translateX(0);\n\
            }}\n\n",
            self.config.base_class
        ));
        
        css.push_str(&format!(
            ".{}-slide-out-right {{\n\
            \ttransform: translateX(100%);\n\
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
        
        // Initial states (for entering animations)
        css.push_str(&format!(
            ".{}-fade-in-enter {{\n\
            \topacity: 0;\n\
            }}\n\n",
            self.config.base_class
        ));
        
        css.push_str(&format!(
            ".{}-slide-in-left-enter {{\n\
            \ttransform: translateX(-100%);\n\
            }}\n\n",
            self.config.base_class
        ));
        
        css.push_str(&format!(
            ".{}-scale-in-enter {{\n\
            \ttransform: scale(0);\n\
            }}\n\n",
            self.config.base_class
        ));
        
        self.css_rules.push(css.clone());
        css
    }

    /// Inject CSS into the document
    fn inject_css(&self, css: &str) -> Result<(), JsValue> {
        if let Some(style_element) = &self.style_element {
            style_element.set_text_content(Some(css));
        }
        Ok(())
    }

    /// Start a CSS class animation
    pub fn start_animation(&mut self, element: &Element, animation_type: &str) -> Result<(), JsValue> {
        let element_id = element.id();
        if element_id.is_empty() {
            return Err("Element must have an ID".into());
        }
        
        let class_name = format!("{}-{}", self.config.base_class, animation_type);
        
        if self.active_animations.contains_key(&element_id) {
            return Err(format!("Animation already active for element {}", element_id).into());
        }
        
        // Add the base class and animation class to the element
        element.class_list().add_1(&self.config.base_class)?;
        element.class_list().add_1(&class_name)?;
        
        let state = CSSAnimationState {
            element_id: element_id.clone(),
            class_name: class_name.clone(),
            is_active: true,
            start_time: 0, // Would be current timestamp in real implementation
            duration: self.config.duration,
        };
        
        self.active_animations.insert(element_id, state);
        Ok(())
    }

    /// Stop a CSS class animation
    pub fn stop_animation(&mut self, element: &Element) -> Result<(), JsValue> {
        let element_id = element.id();
        if element_id.is_empty() {
            return Err("Element must have an ID".into());
        }
        
        if let Some(state) = self.active_animations.get_mut(&element_id) {
            // Remove animation classes from element
            element.class_list().remove_1(&self.config.base_class)?;
            element.class_list().remove_1(&state.class_name)?;
            
            state.is_active = false;
            Ok(())
        } else {
            Err(format!("No active animation found for element {}", element_id).into())
        }
    }

    /// Check if an animation is active
    pub fn is_animation_active(&self, element: &Element) -> bool {
        let element_id = element.id();
        self.active_animations.get(&element_id)
            .map(|state| state.is_active)
            .unwrap_or(false)
    }

    /// Get the CSS class for an element
    pub fn get_element_class(&self, element: &Element) -> Option<String> {
        let element_id = element.id();
        self.active_animations.get(&element_id)
            .map(|state| state.class_name.clone())
    }

    /// Get all active animations
    pub fn get_active_animations(&self) -> Vec<&CSSAnimationState> {
        self.active_animations.values()
            .filter(|state| state.is_active)
            .collect()
    }

    /// Clear all animations
    pub fn clear_all(&mut self) -> Result<(), JsValue> {
        for (_, state) in &self.active_animations {
            if let Some(style_element) = &self.style_element {
                if let Some(element) = style_element.owner_document()
                    .and_then(|doc| doc.get_element_by_id(&state.element_id)) {
                    element.class_list().remove_1(&self.config.base_class)?;
                    element.class_list().remove_1(&state.class_name)?;
                }
            }
        }
        
        self.active_animations.clear();
        Ok(())
    }

    /// Get CSS rules
    pub fn get_css_rules(&self) -> &[String] {
        &self.css_rules
    }

    /// Update configuration
    pub fn update_config(&mut self, config: CSSAnimationConfig) -> Result<(), JsValue> {
        self.config = config;
        
        // Regenerate and inject CSS
        let css = self.generate_css_classes();
        self.inject_css(&css)?;
        
        Ok(())
    }
}

/// Create a new CSS animation manager
pub fn create_css_animation_manager() -> CSSAnimationManager {
    CSSAnimationManager::new(CSSAnimationConfig::default())
}

/// Initialize CSS animations with a manager
pub fn initialize_css_animations(manager: &mut CSSAnimationManager) -> Result<(), JsValue> {
    manager.initialize()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_css_animation_manager_creation() {
        let config = CSSAnimationConfig::default();
        let manager = CSSAnimationManager::new(config);
        
        assert_eq!(manager.get_active_animations().len(), 0);
        assert!(manager.get_css_rules().is_empty());
    }

    #[wasm_bindgen_test]
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

    #[wasm_bindgen_test]
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
}
