//! Ambient lighting implementation for WebGL rendering

use super::light_types::{Light, LightType, Color};

/// Ambient light (affects all objects uniformly)
#[derive(Debug, Clone)]
pub struct AmbientLight {
    /// Base light
    pub light: Light,
}

impl AmbientLight {
    /// Create a new ambient light
    pub fn new(name: &str, color: Color, intensity: f32) -> Self {
        Self {
            light: Light::new(
                uuid::Uuid::new_v4().to_string(),
                name.to_string(),
                LightType::Ambient,
                color,
                intensity,
            ),
        }
    }

    /// Get the base light
    pub fn get_light(&self) -> &Light {
        &self.light
    }

    /// Get mutable base light
    pub fn get_light_mut(&mut self) -> &mut Light {
        &mut self.light
    }

    /// Calculate ambient contribution
    pub fn calculate_ambient_contribution(&self, material_ambient: [f32; 3]) -> [f32; 3] {
        if !self.light.enabled {
            return [0.0, 0.0, 0.0];
        }

        let light_color = self.light.color.as_rgb_array();
        let intensity = self.light.intensity;

        [
            light_color[0] * material_ambient[0] * intensity,
            light_color[1] * material_ambient[1] * intensity,
            light_color[2] * material_ambient[2] * intensity,
        ]
    }

    /// Get ambient light data for shader
    pub fn get_shader_data(&self) -> [f32; 4] {
        if !self.light.enabled {
            return [0.0, 0.0, 0.0, 0.0];
        }

        let color = self.light.color.as_rgb_array();
        [color[0], color[1], color[2], self.light.intensity]
    }
}
