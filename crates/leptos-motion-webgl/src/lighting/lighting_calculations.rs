//! Lighting calculations and management for WebGL rendering

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use super::{
    light_types::{Light, LightType, Color},
    ambient_lighting::AmbientLight,
    directional_lighting::DirectionalLight,
    point_lighting::PointLight,
    spot_lighting::SpotLight,
};

/// Lighting manager for handling multiple lights
#[derive(Debug, Clone)]
pub struct LightingManager {
    /// Ambient lights
    pub ambient_lights: HashMap<String, AmbientLight>,
    /// Directional lights
    pub directional_lights: HashMap<String, DirectionalLight>,
    /// Point lights
    pub point_lights: HashMap<String, PointLight>,
    /// Spot lights
    pub spot_lights: HashMap<String, SpotLight>,
    /// Maximum number of lights per type
    pub max_ambient_lights: usize,
    pub max_directional_lights: usize,
    pub max_point_lights: usize,
    pub max_spot_lights: usize,
}

impl LightingManager {
    /// Create a new lighting manager
    pub fn new() -> Self {
        Self {
            ambient_lights: HashMap::new(),
            directional_lights: HashMap::new(),
            point_lights: HashMap::new(),
            spot_lights: HashMap::new(),
            max_ambient_lights: 8,
            max_directional_lights: 4,
            max_point_lights: 16,
            max_spot_lights: 8,
        }
    }

    /// Add an ambient light
    pub fn add_ambient_light(&mut self, light: AmbientLight) -> Result<()> {
        if self.ambient_lights.len() >= self.max_ambient_lights {
            return Err(WebGLError::LightingError("Maximum number of ambient lights reached".to_string()));
        }

        let id = light.light.id.clone();
        self.ambient_lights.insert(id, light);
        Ok(())
    }

    /// Add a directional light
    pub fn add_directional_light(&mut self, light: DirectionalLight) -> Result<()> {
        if self.directional_lights.len() >= self.max_directional_lights {
            return Err(WebGLError::LightingError("Maximum number of directional lights reached".to_string()));
        }

        let id = light.light.id.clone();
        self.directional_lights.insert(id, light);
        Ok(())
    }

    /// Add a point light
    pub fn add_point_light(&mut self, light: PointLight) -> Result<()> {
        if self.point_lights.len() >= self.max_point_lights {
            return Err(WebGLError::LightingError("Maximum number of point lights reached".to_string()));
        }

        let id = light.light.id.clone();
        self.point_lights.insert(id, light);
        Ok(())
    }

    /// Add a spot light
    pub fn add_spot_light(&mut self, light: SpotLight) -> Result<()> {
        if self.spot_lights.len() >= self.max_spot_lights {
            return Err(WebGLError::LightingError("Maximum number of spot lights reached".to_string()));
        }

        let id = light.light.id.clone();
        self.spot_lights.insert(id, light);
        Ok(())
    }

    /// Remove an ambient light
    pub fn remove_ambient_light(&mut self, id: &str) -> Option<AmbientLight> {
        self.ambient_lights.remove(id)
    }

    /// Remove a directional light
    pub fn remove_directional_light(&mut self, id: &str) -> Option<DirectionalLight> {
        self.directional_lights.remove(id)
    }

    /// Remove a point light
    pub fn remove_point_light(&mut self, id: &str) -> Option<PointLight> {
        self.point_lights.remove(id)
    }

    /// Remove a spot light
    pub fn remove_spot_light(&mut self, id: &str) -> Option<SpotLight> {
        self.spot_lights.remove(id)
    }

    /// Get an ambient light
    pub fn get_ambient_light(&self, id: &str) -> Option<&AmbientLight> {
        self.ambient_lights.get(id)
    }

    /// Get a directional light
    pub fn get_directional_light(&self, id: &str) -> Option<&DirectionalLight> {
        self.directional_lights.get(id)
    }

    /// Get a point light
    pub fn get_point_light(&self, id: &str) -> Option<&PointLight> {
        self.point_lights.get(id)
    }

    /// Get a spot light
    pub fn get_spot_light(&self, id: &str) -> Option<&SpotLight> {
        self.spot_lights.get(id)
    }

    /// Get a mutable ambient light
    pub fn get_ambient_light_mut(&mut self, id: &str) -> Option<&mut AmbientLight> {
        self.ambient_lights.get_mut(id)
    }

    /// Get a mutable directional light
    pub fn get_directional_light_mut(&mut self, id: &str) -> Option<&mut DirectionalLight> {
        self.directional_lights.get_mut(id)
    }

    /// Get a mutable point light
    pub fn get_point_light_mut(&mut self, id: &str) -> Option<&mut PointLight> {
        self.point_lights.get_mut(id)
    }

    /// Get a mutable spot light
    pub fn get_spot_light_mut(&mut self, id: &str) -> Option<&mut SpotLight> {
        self.spot_lights.get_mut(id)
    }

    /// Get all ambient lights
    pub fn get_ambient_lights(&self) -> &HashMap<String, AmbientLight> {
        &self.ambient_lights
    }

    /// Get all directional lights
    pub fn get_directional_lights(&self) -> &HashMap<String, DirectionalLight> {
        &self.directional_lights
    }

    /// Get all point lights
    pub fn get_point_lights(&self) -> &HashMap<String, PointLight> {
        &self.point_lights
    }

    /// Get all spot lights
    pub fn get_spot_lights(&self) -> &HashMap<String, SpotLight> {
        &self.spot_lights
    }

    /// Get total light count
    pub fn get_total_light_count(&self) -> usize {
        self.ambient_lights.len()
            + self.directional_lights.len()
            + self.point_lights.len()
            + self.spot_lights.len()
    }

    /// Get enabled light count
    pub fn get_enabled_light_count(&self) -> usize {
        let mut count = 0;

        for light in self.ambient_lights.values() {
            if light.light.enabled {
                count += 1;
            }
        }

        for light in self.directional_lights.values() {
            if light.light.enabled {
                count += 1;
            }
        }

        for light in self.point_lights.values() {
            if light.light.enabled {
                count += 1;
            }
        }

        for light in self.spot_lights.values() {
            if light.light.enabled {
                count += 1;
            }
        }

        count
    }

    /// Clear all lights
    pub fn clear(&mut self) {
        self.ambient_lights.clear();
        self.directional_lights.clear();
        self.point_lights.clear();
        self.spot_lights.clear();
    }

    /// Set maximum number of lights per type
    pub fn set_max_lights(
        &mut self,
        ambient: usize,
        directional: usize,
        point: usize,
        spot: usize,
    ) {
        self.max_ambient_lights = ambient;
        self.max_directional_lights = directional;
        self.max_point_lights = point;
        self.max_spot_lights = spot;
    }

    /// Get maximum number of lights per type
    pub fn get_max_lights(&self) -> (usize, usize, usize, usize) {
        (
            self.max_ambient_lights,
            self.max_directional_lights,
            self.max_point_lights,
            self.max_spot_lights,
        )
    }

    /// Calculate total lighting contribution for a surface
    pub fn calculate_total_lighting(
        &self,
        surface_position: [f32; 3],
        surface_normal: [f32; 3],
        material_ambient: [f32; 3],
        material_diffuse: [f32; 3],
        material_specular: [f32; 3],
        view_direction: [f32; 3],
        shininess: f32,
    ) -> [f32; 3] {
        let mut total_ambient = [0.0, 0.0, 0.0];
        let mut total_diffuse = [0.0, 0.0, 0.0];
        let mut total_specular = [0.0, 0.0, 0.0];

        // Calculate ambient contribution
        for ambient_light in self.ambient_lights.values() {
            let ambient_contrib = ambient_light.calculate_ambient_contribution(material_ambient);
            total_ambient[0] += ambient_contrib[0];
            total_ambient[1] += ambient_contrib[1];
            total_ambient[2] += ambient_contrib[2];
        }

        // Calculate directional light contribution
        for directional_light in self.directional_lights.values() {
            let (diffuse_contrib, specular_contrib) = directional_light
                .calculate_directional_contribution(
                    surface_normal,
                    material_diffuse,
                    material_specular,
                    view_direction,
                    shininess,
                );
            total_diffuse[0] += diffuse_contrib[0];
            total_diffuse[1] += diffuse_contrib[1];
            total_diffuse[2] += diffuse_contrib[2];
            total_specular[0] += specular_contrib[0];
            total_specular[1] += specular_contrib[1];
            total_specular[2] += specular_contrib[2];
        }

        // Calculate point light contribution
        for point_light in self.point_lights.values() {
            let (diffuse_contrib, specular_contrib) = point_light
                .calculate_point_contribution(
                    surface_position,
                    surface_normal,
                    material_diffuse,
                    material_specular,
                    view_direction,
                    shininess,
                );
            total_diffuse[0] += diffuse_contrib[0];
            total_diffuse[1] += diffuse_contrib[1];
            total_diffuse[2] += diffuse_contrib[2];
            total_specular[0] += specular_contrib[0];
            total_specular[1] += specular_contrib[1];
            total_specular[2] += specular_contrib[2];
        }

        // Calculate spot light contribution
        for spot_light in self.spot_lights.values() {
            let (diffuse_contrib, specular_contrib) = spot_light
                .calculate_spot_contribution(
                    surface_position,
                    surface_normal,
                    material_diffuse,
                    material_specular,
                    view_direction,
                    shininess,
                );
            total_diffuse[0] += diffuse_contrib[0];
            total_diffuse[1] += diffuse_contrib[1];
            total_diffuse[2] += diffuse_contrib[2];
            total_specular[0] += specular_contrib[0];
            total_specular[1] += specular_contrib[1];
            total_specular[2] += specular_contrib[2];
        }

        // Combine all contributions
        [
            total_ambient[0] + total_diffuse[0] + total_specular[0],
            total_ambient[1] + total_diffuse[1] + total_specular[1],
            total_ambient[2] + total_diffuse[2] + total_specular[2],
        ]
    }
}
