//! Directional lighting implementation for WebGL rendering

use crate::error::{Result, WebGLError};
use gl_matrix::vec3;
use super::light_types::{Light, LightType, Color};

/// Directional light (like sunlight)
#[derive(Debug, Clone)]
pub struct DirectionalLight {
    /// Base light
    pub light: Light,
    /// Light direction (normalized)
    pub direction: [f32; 3],
    /// Shadow casting flag
    pub cast_shadow: bool,
}

impl DirectionalLight {
    /// Create a new directional light
    pub fn new(name: &str, color: Color, intensity: f32, direction: [f32; 3]) -> Self {
        let mut normalized_direction = direction;
        vec3::normalize(&mut normalized_direction, &direction);

        Self {
            light: Light::new(
                uuid::Uuid::new_v4().to_string(),
                name.to_string(),
                LightType::Directional,
                color,
                intensity,
            ),
            direction: normalized_direction,
            cast_shadow: false,
        }
    }

    /// Set light direction
    pub fn set_direction(&mut self, direction: [f32; 3]) {
        vec3::normalize(&mut self.direction, &direction);
    }

    /// Get light direction
    pub fn get_direction(&self) -> [f32; 3] {
        self.direction
    }

    /// Enable shadow casting
    pub fn enable_shadow(&mut self) {
        self.cast_shadow = true;
    }

    /// Disable shadow casting
    pub fn disable_shadow(&mut self) {
        self.cast_shadow = false;
    }

    /// Check if casting shadow
    pub fn is_casting_shadow(&self) -> bool {
        self.cast_shadow
    }

    /// Get the base light
    pub fn get_light(&self) -> &Light {
        &self.light
    }

    /// Get mutable base light
    pub fn get_light_mut(&mut self) -> &mut Light {
        &mut self.light
    }

    /// Calculate directional light contribution
    pub fn calculate_directional_contribution(
        &self,
        surface_normal: [f32; 3],
        material_diffuse: [f32; 3],
        material_specular: [f32; 3],
        view_direction: [f32; 3],
        shininess: f32,
    ) -> ([f32; 3], [f32; 3]) {
        if !self.light.enabled {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        let light_color = self.light.color.as_rgb_array();
        let intensity = self.light.intensity;

        // Calculate diffuse contribution
        let mut normalized_normal = surface_normal;
        vec3::normalize(&mut normalized_normal, &surface_normal);

        let light_direction = [-self.direction[0], -self.direction[1], -self.direction[2]];
        let diffuse_factor = vec3::dot(&normalized_normal, &light_direction).max(0.0);

        let diffuse_contribution = [
            light_color[0] * material_diffuse[0] * diffuse_factor * intensity,
            light_color[1] * material_diffuse[1] * diffuse_factor * intensity,
            light_color[2] * material_diffuse[2] * diffuse_factor * intensity,
        ];

        // Calculate specular contribution
        let mut normalized_view_dir = view_direction;
        vec3::normalize(&mut normalized_view_dir, &view_direction);

        // Calculate reflection manually: reflect = incident - 2 * dot(incident, normal) * normal
        let incident = [-light_direction[0], -light_direction[1], -light_direction[2]];
        let dot_product = vec3::dot(&incident, &normalized_normal);
        let mut scaled_normal = [0.0; 3];
        vec3::scale(&mut scaled_normal, &normalized_normal, 2.0 * dot_product);
        let mut reflect_direction = [0.0; 3];
        vec3::subtract(&mut reflect_direction, &incident, &scaled_normal);
        let specular_factor = vec3::dot(&normalized_view_dir, &reflect_direction).max(0.0).powf(shininess);

        let specular_contribution = [
            light_color[0] * material_specular[0] * specular_factor * intensity,
            light_color[1] * material_specular[1] * specular_factor * intensity,
            light_color[2] * material_specular[2] * specular_factor * intensity,
        ];

        (diffuse_contribution, specular_contribution)
    }

    /// Get directional light data for shader
    pub fn get_shader_data(&self) -> [f32; 8] {
        if !self.light.enabled {
            return [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        }

        let color = self.light.color.as_rgb_array();
        [
            color[0], color[1], color[2], self.light.intensity,
            self.direction[0], self.direction[1], self.direction[2],
            if self.cast_shadow { 1.0 } else { 0.0 },
        ]
    }
}
