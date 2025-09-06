//! Layout animation presets for common transitions
//!
//! This module provides predefined animation configurations for
//! common layout changes like list reordering, grid resizing,
//! responsive breakpoints, modal expansion, and card flipping.

use crate::LayoutAnimationConfig;
use std::collections::HashMap;
use web_sys::DomRect;

/// Layout animation preset
#[derive(Debug, Clone)]
pub struct LayoutPreset {
    /// Preset name
    pub name: String,
    /// Preset description
    pub description: String,
    /// Preset type
    pub preset_type: PresetType,
    /// Default configuration
    pub default_config: LayoutAnimationConfig,
    /// Required parameters
    pub required_parameters: Vec<PresetParameter>,
    /// Optional parameters
    pub optional_parameters: Vec<PresetParameter>,
}

/// Preset parameter definition
#[derive(Debug, Clone)]
pub struct PresetParameter {
    /// Parameter name
    pub name: String,
    /// Parameter description
    pub description: String,
    /// Parameter type
    pub parameter_type: ParameterType,
    /// Default value
    pub default_value: Option<String>,
    /// Whether parameter is required
    pub required: bool,
}

/// Preset type enumeration
#[derive(Debug, Clone)]
pub enum PresetType {
    /// List reordering animation
    ListReorder,
    /// Grid resize animation
    GridResize,
    /// Responsive breakpoint animation
    ResponsiveBreakpoint,
    /// Modal expand/collapse animation
    ModalExpand,
    /// Card flip animation
    CardFlip,
    /// Custom preset
    Custom(String),
}

/// Parameter type enumeration
#[derive(Debug, Clone)]
pub enum ParameterType {
    /// Numeric parameter
    Number,
    /// String parameter
    String,
    /// Boolean parameter
    Boolean,
    /// Duration parameter (in milliseconds)
    Duration,
    /// Easing function parameter
    Easing,
}

/// Manager for layout animation presets
pub struct LayoutPresetManager {
    /// Built-in presets
    built_in_presets: HashMap<String, LayoutPreset>,
    /// Custom presets
    custom_presets: HashMap<String, LayoutPreset>,
}

impl LayoutPresetManager {
    /// Create a new preset manager
    pub fn new() -> Self {
        let mut manager = Self {
            built_in_presets: HashMap::new(),
            custom_presets: HashMap::new(),
        };

        // Initialize built-in presets
        manager.initialize_built_in_presets();

        manager
    }

    /// Initialize built-in presets
    fn initialize_built_in_presets(&mut self) {
        // List reorder preset
        let list_reorder = LayoutPreset {
            name: "list-reorder".to_string(),
            description: "Smooth animation for reordering list items".to_string(),
            preset_type: PresetType::ListReorder,
            default_config: LayoutAnimationConfig {
                duration: 300.0,
                easing: "ease-out".to_string(),
                delay: 0.0,
                stagger: 50.0,
                ..Default::default()
            },
            required_parameters: vec![PresetParameter {
                name: "direction".to_string(),
                description: "Reorder direction (horizontal/vertical)".to_string(),
                parameter_type: ParameterType::String,
                default_value: Some("vertical".to_string()),
                required: false,
            }],
            optional_parameters: vec![PresetParameter {
                name: "stagger".to_string(),
                description: "Delay between item animations".to_string(),
                parameter_type: ParameterType::Duration,
                default_value: Some("50".to_string()),
                required: false,
            }],
        };

        // Grid resize preset
        let grid_resize = LayoutPreset {
            name: "grid-resize".to_string(),
            description: "Animated grid column/row changes".to_string(),
            preset_type: PresetType::GridResize,
            default_config: LayoutAnimationConfig {
                duration: 400.0,
                easing: "ease-in-out".to_string(),
                delay: 0.0,
                stagger: 0.0,
                ..Default::default()
            },
            required_parameters: vec![PresetParameter {
                name: "columns".to_string(),
                description: "Target number of columns".to_string(),
                parameter_type: ParameterType::Number,
                default_value: None,
                required: true,
            }],
            optional_parameters: vec![PresetParameter {
                name: "maintain_aspect".to_string(),
                description: "Maintain item aspect ratios".to_string(),
                parameter_type: ParameterType::Boolean,
                default_value: Some("true".to_string()),
                required: false,
            }],
        };

        // Responsive breakpoint preset
        let responsive_breakpoint = LayoutPreset {
            name: "responsive-breakpoint".to_string(),
            description: "Smooth transition between responsive layouts".to_string(),
            preset_type: PresetType::ResponsiveBreakpoint,
            default_config: LayoutAnimationConfig {
                duration: 500.0,
                easing: "ease-in-out".to_string(),
                delay: 0.0,
                stagger: 0.0,
                ..Default::default()
            },
            required_parameters: vec![PresetParameter {
                name: "breakpoint".to_string(),
                description: "Target breakpoint (sm/md/lg/xl)".to_string(),
                parameter_type: ParameterType::String,
                default_value: None,
                required: true,
            }],
            optional_parameters: vec![PresetParameter {
                name: "preserve_content".to_string(),
                description: "Preserve content during transition".to_string(),
                parameter_type: ParameterType::Boolean,
                default_value: Some("true".to_string()),
                required: false,
            }],
        };

        // Modal expand preset
        let modal_expand = LayoutPreset {
            name: "modal-expand".to_string(),
            description: "Expand/collapse modal with backdrop".to_string(),
            preset_type: PresetType::ModalExpand,
            default_config: LayoutAnimationConfig {
                duration: 250.0,
                easing: "ease-out".to_string(),
                delay: 0.0,
                stagger: 0.0,
                ..Default::default()
            },
            required_parameters: vec![PresetParameter {
                name: "action".to_string(),
                description: "Action to perform (expand/collapse)".to_string(),
                parameter_type: ParameterType::String,
                default_value: None,
                required: true,
            }],
            optional_parameters: vec![PresetParameter {
                name: "backdrop".to_string(),
                description: "Animate backdrop separately".to_string(),
                parameter_type: ParameterType::Boolean,
                default_value: Some("true".to_string()),
                required: false,
            }],
        };

        // Card flip preset
        let card_flip = LayoutPreset {
            name: "card-flip".to_string(),
            description: "3D card flip animation".to_string(),
            preset_type: PresetType::CardFlip,
            default_config: LayoutAnimationConfig {
                duration: 600.0,
                easing: "ease-in-out".to_string(),
                delay: 0.0,
                stagger: 0.0,
                ..Default::default()
            },
            required_parameters: vec![PresetParameter {
                name: "axis".to_string(),
                description: "Flip axis (x/y)".to_string(),
                parameter_type: ParameterType::String,
                default_value: Some("y".to_string()),
                required: false,
            }],
            optional_parameters: vec![PresetParameter {
                name: "perspective".to_string(),
                description: "3D perspective distance".to_string(),
                parameter_type: ParameterType::Number,
                default_value: Some("1000".to_string()),
                required: false,
            }],
        };

        // Add built-in presets
        self.built_in_presets
            .insert("list-reorder".to_string(), list_reorder);
        self.built_in_presets
            .insert("grid-resize".to_string(), grid_resize);
        self.built_in_presets
            .insert("responsive-breakpoint".to_string(), responsive_breakpoint);
        self.built_in_presets
            .insert("modal-expand".to_string(), modal_expand);
        self.built_in_presets
            .insert("card-flip".to_string(), card_flip);
    }

    /// Get a built-in preset by name
    pub fn get_built_in_preset(&self, name: &str) -> Option<&LayoutPreset> {
        self.built_in_presets.get(name)
    }

    /// Get all built-in preset names
    pub fn get_built_in_preset_names(&self) -> Vec<&String> {
        self.built_in_presets.keys().collect()
    }

    /// Add a custom preset
    pub fn add_custom_preset(&mut self, preset: LayoutPreset) -> Result<(), String> {
        // Validate preset
        self.validate_preset(&preset)?;

        // Check for name conflicts
        if self.built_in_presets.contains_key(&preset.name) {
            return Err("Cannot override built-in preset".to_string());
        }

        if self.custom_presets.contains_key(&preset.name) {
            return Err("Custom preset with this name already exists".to_string());
        }

        // Add preset
        self.custom_presets.insert(preset.name.clone(), preset);

        Ok(())
    }

    /// Get a custom preset by name
    pub fn get_custom_preset(&self, name: &str) -> Option<&LayoutPreset> {
        self.custom_presets.get(name)
    }

    /// Remove a custom preset
    pub fn remove_custom_preset(&mut self, name: &str) -> Result<(), String> {
        if self.custom_presets.remove(name).is_some() {
            Ok(())
        } else {
            Err("Custom preset not found".to_string())
        }
    }

    /// Get all custom preset names
    pub fn get_custom_preset_names(&self) -> Vec<&String> {
        self.custom_presets.keys().collect()
    }

    /// Calculate target layout for a preset
    pub fn calculate_target_layout(
        &self,
        preset: &LayoutPreset,
        current_layout: &DomRect,
        parameters: &HashMap<String, String>,
    ) -> Result<DomRect, String> {
        match &preset.preset_type {
            PresetType::ListReorder => {
                // For list reorder, layout remains the same
                Ok(current_layout.clone())
            }
            PresetType::GridResize => {
                // For grid resize, calculate new dimensions
                let column_count = parameters
                    .get("columns")
                    .ok_or("Missing required parameter: columns")?
                    .parse::<f64>()
                    .map_err(|_| "Invalid column count")?;

                // For now, just return current layout as a placeholder
                // In a real implementation, this would calculate new grid dimensions
                let _new_width = current_layout.width() * column_count;
                Ok(current_layout.clone())
            }
            PresetType::ResponsiveBreakpoint => {
                // For responsive breakpoint, layout changes based on breakpoint
                let breakpoint = parameters
                    .get("breakpoint")
                    .ok_or("Missing required parameter: breakpoint")?;

                // For now, just return current layout as a placeholder
                // In a real implementation, this would calculate new responsive layout
                log::info!("Calculating layout for breakpoint: {}", breakpoint);
                Ok(current_layout.clone())
            }
            PresetType::ModalExpand => {
                // For modal expand, layout changes based on action
                let action = parameters
                    .get("action")
                    .ok_or("Missing required parameter: action")?;

                // For now, just return current layout as a placeholder
                // In a real implementation, this would calculate new modal dimensions
                log::info!("Calculating layout for modal action: {}", action);
                Ok(current_layout.clone())
            }
            PresetType::CardFlip => {
                // For card flip, layout remains the same
                Ok(current_layout.clone())
            }
            PresetType::Custom(name) => {
                // For custom presets, return current layout
                log::info!("Calculating layout for custom preset: {}", name);
                Ok(current_layout.clone())
            }
        }
    }

    /// Validate a preset configuration
    fn validate_preset(&self, preset: &LayoutPreset) -> Result<(), String> {
        // Check required parameters
        for param in &preset.required_parameters {
            if !param.required {
                return Err(format!(
                    "Required parameter '{}' is marked as not required",
                    param.name
                ));
            }
        }

        // Check parameter names are unique
        let mut param_names = std::collections::HashSet::new();
        for param in &preset.required_parameters {
            if !param_names.insert(&param.name) {
                return Err(format!("Duplicate parameter name: {}", param.name));
            }
        }
        for param in &preset.optional_parameters {
            if !param_names.insert(&param.name) {
                return Err(format!("Duplicate parameter name: {}", param.name));
            }
        }

        // Check for conflicts with required parameters
        for param in &preset.required_parameters {
            for opt_param in &preset.optional_parameters {
                if param.name == opt_param.name {
                    return Err(format!(
                        "Parameter '{}' appears in both required and optional lists",
                        param.name
                    ));
                }
            }
        }

        Ok(())
    }

    /// Get preset by name (built-in or custom)
    pub fn get_preset(&self, name: &str) -> Option<&LayoutPreset> {
        self.built_in_presets
            .get(name)
            .or_else(|| self.custom_presets.get(name))
    }

    /// Get all preset names
    pub fn get_all_preset_names(&self) -> Vec<&String> {
        let mut names: Vec<&String> = self.built_in_presets.keys().collect();
        names.extend(self.custom_presets.keys());
        names
    }
}

impl Default for LayoutPresetManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_preset_manager_creation() {
        let manager = LayoutPresetManager::new();
        assert!(!manager.get_built_in_preset_names().is_empty());
        assert!(manager.get_custom_preset_names().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_built_in_presets() {
        let manager = LayoutPresetManager::new();

        // Check that built-in presets exist
        assert!(manager.get_built_in_preset("list-reorder").is_some());
        assert!(manager.get_built_in_preset("grid-resize").is_some());
        assert!(
            manager
                .get_built_in_preset("responsive-breakpoint")
                .is_some()
        );
        assert!(manager.get_built_in_preset("modal-expand").is_some());
        assert!(manager.get_built_in_preset("card-flip").is_some());
    }

    #[wasm_bindgen_test]
    fn test_custom_preset_management() {
        let mut manager = LayoutPresetManager::new();

        // Create custom preset
        let custom_preset = LayoutPreset {
            name: "custom-test".to_string(),
            description: "Test custom preset".to_string(),
            preset_type: PresetType::Custom("test".to_string()),
            default_config: LayoutAnimationConfig::default(),
            required_parameters: vec![],
            optional_parameters: vec![],
        };

        // Add custom preset
        assert!(manager.add_custom_preset(custom_preset).is_ok());

        // Check it exists
        assert!(manager.get_custom_preset("custom-test").is_some());

        // Remove custom preset
        assert!(manager.remove_custom_preset("custom-test").is_ok());

        // Check it's gone
        assert!(manager.get_custom_preset("custom-test").is_none());
    }

    #[wasm_bindgen_test]
    fn test_target_layout_calculation() {
        let manager = LayoutPresetManager::new();
        let preset = manager.get_built_in_preset("list-reorder").unwrap();
        let current_layout = DomRect::new().unwrap();
        let parameters = HashMap::new();

        let result = manager.calculate_target_layout(preset, &current_layout, &parameters);
        assert!(result.is_ok());
    }
}
