//! Lighting system for WebGL rendering

use crate::error::{Result, WebGLError};
use gl_matrix::vec3;
use std::collections::HashMap;

/// Light type enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LightType {
    Ambient,
    Directional,
    Point,
    Spot,
}

/// Color representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Create a new color
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from RGB values (0-255)
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Create a color from RGBA values (0-255)
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }

    /// Create a white color
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    /// Create a black color
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    /// Create a red color
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 1.0)
    }

    /// Create a green color
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 1.0)
    }

    /// Create a blue color
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 1.0)
    }

    /// Get color as array
    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Get RGB as array
    pub fn as_rgb_array(&self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

/// Base light structure
#[derive(Debug, Clone)]
pub struct Light {
    /// Light ID
    pub id: String,
    /// Light name
    pub name: String,
    /// Light type
    pub light_type: LightType,
    /// Light color
    pub color: Color,
    /// Light intensity
    pub intensity: f32,
    /// Enabled flag
    pub enabled: bool,
}

impl Light {
    /// Create a new light
    pub fn new(id: String, name: String, light_type: LightType, color: Color, intensity: f32) -> Self {
        Self {
            id,
            name,
            light_type,
            color,
            intensity,
            enabled: true,
        }
    }

    /// Set light intensity
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.max(0.0);
    }

    /// Set light color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Enable the light
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the light
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if light is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Ambient light
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
}

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
}

/// Point light (like a light bulb)
#[derive(Debug, Clone)]
pub struct PointLight {
    /// Base light
    pub light: Light,
    /// Light position
    pub position: [f32; 3],
    /// Light range (distance at which light intensity becomes 0)
    pub range: f32,
    /// Constant attenuation
    pub constant_attenuation: f32,
    /// Linear attenuation
    pub linear_attenuation: f32,
    /// Quadratic attenuation
    pub quadratic_attenuation: f32,
    /// Shadow casting flag
    pub cast_shadow: bool,
}

impl PointLight {
    /// Create a new point light
    pub fn new(name: &str, color: Color, intensity: f32, position: [f32; 3]) -> Self {
        Self {
            light: Light::new(
                uuid::Uuid::new_v4().to_string(),
                name.to_string(),
                LightType::Point,
                color,
                intensity,
            ),
            position,
            range: 100.0,
            constant_attenuation: 1.0,
            linear_attenuation: 0.09,
            quadratic_attenuation: 0.032,
            cast_shadow: false,
        }
    }

    /// Set light position
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    /// Get light position
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set light range
    pub fn set_range(&mut self, range: f32) {
        self.range = range.max(0.0);
    }

    /// Get light range
    pub fn get_range(&self) -> f32 {
        self.range
    }

    /// Set attenuation parameters
    pub fn set_attenuation(&mut self, constant: f32, linear: f32, quadratic: f32) {
        self.constant_attenuation = constant.max(0.0);
        self.linear_attenuation = linear.max(0.0);
        self.quadratic_attenuation = quadratic.max(0.0);
    }

    /// Calculate attenuation factor for a given distance
    pub fn calculate_attenuation(&self, distance: f32) -> f32 {
        if distance >= self.range {
            return 0.0;
        }
        
        1.0 / (self.constant_attenuation + 
               self.linear_attenuation * distance + 
               self.quadratic_attenuation * distance * distance)
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
}

/// Spot light (like a flashlight)
#[derive(Debug, Clone)]
pub struct SpotLight {
    /// Base light
    pub light: Light,
    /// Light position
    pub position: [f32; 3],
    /// Light direction (normalized)
    pub direction: [f32; 3],
    /// Inner cone angle in radians
    pub inner_cone_angle: f32,
    /// Outer cone angle in radians
    pub outer_cone_angle: f32,
    /// Light range
    pub range: f32,
    /// Constant attenuation
    pub constant_attenuation: f32,
    /// Linear attenuation
    pub linear_attenuation: f32,
    /// Quadratic attenuation
    pub quadratic_attenuation: f32,
    /// Shadow casting flag
    pub cast_shadow: bool,
}

impl SpotLight {
    /// Create a new spot light
    pub fn new(
        name: &str,
        color: Color,
        intensity: f32,
        position: [f32; 3],
        direction: [f32; 3],
        inner_cone_angle: f32,
        outer_cone_angle: f32,
    ) -> Self {
        let mut normalized_direction = direction;
        vec3::normalize(&mut normalized_direction, &direction);
        
        Self {
            light: Light::new(
                uuid::Uuid::new_v4().to_string(),
                name.to_string(),
                LightType::Spot,
                color,
                intensity,
            ),
            position,
            direction: normalized_direction,
            inner_cone_angle: inner_cone_angle.max(0.0).min(std::f32::consts::PI),
            outer_cone_angle: outer_cone_angle.max(inner_cone_angle).min(std::f32::consts::PI),
            range: 100.0,
            constant_attenuation: 1.0,
            linear_attenuation: 0.09,
            quadratic_attenuation: 0.032,
            cast_shadow: false,
        }
    }

    /// Set light position
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    /// Get light position
    pub fn get_position(&self) -> [f32; 3] {
        self.position
    }

    /// Set light direction
    pub fn set_direction(&mut self, direction: [f32; 3]) {
        vec3::normalize(&mut self.direction, &direction);
    }

    /// Get light direction
    pub fn get_direction(&self) -> [f32; 3] {
        self.direction
    }

    /// Set cone angles
    pub fn set_cone_angles(&mut self, inner: f32, outer: f32) {
        self.inner_cone_angle = inner.max(0.0).min(std::f32::consts::PI);
        self.outer_cone_angle = outer.max(self.inner_cone_angle).min(std::f32::consts::PI);
    }

    /// Get cone angles
    pub fn get_cone_angles(&self) -> (f32, f32) {
        (self.inner_cone_angle, self.outer_cone_angle)
    }

    /// Set light range
    pub fn set_range(&mut self, range: f32) {
        self.range = range.max(0.0);
    }

    /// Get light range
    pub fn get_range(&self) -> f32 {
        self.range
    }

    /// Set attenuation parameters
    pub fn set_attenuation(&mut self, constant: f32, linear: f32, quadratic: f32) {
        self.constant_attenuation = constant.max(0.0);
        self.linear_attenuation = linear.max(0.0);
        self.quadratic_attenuation = quadratic.max(0.0);
    }

    /// Calculate spot factor for a given direction
    pub fn calculate_spot_factor(&self, light_direction: [f32; 3]) -> f32 {
        let mut normalized_light_dir = light_direction;
        vec3::normalize(&mut normalized_light_dir, &light_direction);
        
        let cos_angle = -vec3::dot(&normalized_light_dir, &self.direction);
        
        if cos_angle < self.outer_cone_angle.cos() {
            return 0.0;
        }
        
        if cos_angle >= self.inner_cone_angle.cos() {
            return 1.0;
        }
        
        // Smooth transition between inner and outer cone
        let smooth_factor = (cos_angle - self.outer_cone_angle.cos()) / 
                           (self.inner_cone_angle.cos() - self.outer_cone_angle.cos());
        smooth_factor * smooth_factor
    }

    /// Calculate attenuation factor for a given distance
    pub fn calculate_attenuation(&self, distance: f32) -> f32 {
        if distance >= self.range {
            return 0.0;
        }
        
        1.0 / (self.constant_attenuation + 
               self.linear_attenuation * distance + 
               self.quadratic_attenuation * distance * distance)
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
}

/// Light manager for managing all lights in a scene
pub struct LightManager {
    /// Ambient lights
    ambient_lights: HashMap<String, AmbientLight>,
    /// Directional lights
    directional_lights: HashMap<String, DirectionalLight>,
    /// Point lights
    point_lights: HashMap<String, PointLight>,
    /// Spot lights
    spot_lights: HashMap<String, SpotLight>,
    /// Maximum number of lights per type
    max_ambient_lights: usize,
    max_directional_lights: usize,
    max_point_lights: usize,
    max_spot_lights: usize,
}

impl LightManager {
    /// Create a new light manager
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
    pub fn add_ambient_light(&mut self, name: &str, color: Color, intensity: f32) -> Result<()> {
        if self.ambient_lights.len() >= self.max_ambient_lights {
            return Err(WebGLError::lighting_error("Maximum number of ambient lights reached"));
        }
        
        let light = AmbientLight::new(name, color, intensity);
        self.ambient_lights.insert(name.to_string(), light);
        Ok(())
    }

    /// Add a directional light
    pub fn add_directional_light(
        &mut self,
        name: &str,
        color: Color,
        intensity: f32,
        direction: [f32; 3],
    ) -> Result<()> {
        if self.directional_lights.len() >= self.max_directional_lights {
            return Err(WebGLError::lighting_error("Maximum number of directional lights reached"));
        }
        
        let light = DirectionalLight::new(name, color, intensity, direction);
        self.directional_lights.insert(name.to_string(), light);
        Ok(())
    }

    /// Add a point light
    pub fn add_point_light(
        &mut self,
        name: &str,
        color: Color,
        intensity: f32,
        position: [f32; 3],
    ) -> Result<()> {
        if self.point_lights.len() >= self.max_point_lights {
            return Err(WebGLError::lighting_error("Maximum number of point lights reached"));
        }
        
        let light = PointLight::new(name, color, intensity, position);
        self.point_lights.insert(name.to_string(), light);
        Ok(())
    }

    /// Add a spot light
    pub fn add_spot_light(
        &mut self,
        name: &str,
        color: Color,
        intensity: f32,
        position: [f32; 3],
        direction: [f32; 3],
        inner_cone_angle: f32,
        outer_cone_angle: f32,
    ) -> Result<()> {
        if self.spot_lights.len() >= self.max_spot_lights {
            return Err(WebGLError::lighting_error("Maximum number of spot lights reached"));
        }
        
        let light = SpotLight::new(name, color, intensity, position, direction, inner_cone_angle, outer_cone_angle);
        self.spot_lights.insert(name.to_string(), light);
        Ok(())
    }

    /// Get an ambient light
    pub fn get_ambient_light(&self, name: &str) -> Option<&AmbientLight> {
        self.ambient_lights.get(name)
    }

    /// Get a mutable ambient light
    pub fn get_ambient_light_mut(&mut self, name: &str) -> Option<&mut AmbientLight> {
        self.ambient_lights.get_mut(name)
    }

    /// Get a directional light
    pub fn get_directional_light(&self, name: &str) -> Option<&DirectionalLight> {
        self.directional_lights.get(name)
    }

    /// Get a mutable directional light
    pub fn get_directional_light_mut(&mut self, name: &str) -> Option<&mut DirectionalLight> {
        self.directional_lights.get_mut(name)
    }

    /// Get a point light
    pub fn get_point_light(&self, name: &str) -> Option<&PointLight> {
        self.point_lights.get(name)
    }

    /// Get a mutable point light
    pub fn get_point_light_mut(&mut self, name: &str) -> Option<&mut PointLight> {
        self.point_lights.get_mut(name)
    }

    /// Get a spot light
    pub fn get_spot_light(&self, name: &str) -> Option<&SpotLight> {
        self.spot_lights.get(name)
    }

    /// Get a mutable spot light
    pub fn get_spot_light_mut(&mut self, name: &str) -> Option<&mut SpotLight> {
        self.spot_lights.get_mut(name)
    }

    /// Remove an ambient light
    pub fn remove_ambient_light(&mut self, name: &str) -> Option<AmbientLight> {
        self.ambient_lights.remove(name)
    }

    /// Remove a directional light
    pub fn remove_directional_light(&mut self, name: &str) -> Option<DirectionalLight> {
        self.directional_lights.remove(name)
    }

    /// Remove a point light
    pub fn remove_point_light(&mut self, name: &str) -> Option<PointLight> {
        self.point_lights.remove(name)
    }

    /// Remove a spot light
    pub fn remove_spot_light(&mut self, name: &str) -> Option<SpotLight> {
        self.spot_lights.remove(name)
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
        self.ambient_lights.len() + 
        self.directional_lights.len() + 
        self.point_lights.len() + 
        self.spot_lights.len()
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
    pub fn set_max_lights(&mut self, ambient: usize, directional: usize, point: usize, spot: usize) {
        self.max_ambient_lights = ambient;
        self.max_directional_lights = directional;
        self.max_point_lights = point;
        self.max_spot_lights = spot;
    }

    /// Get maximum number of lights per type
    pub fn get_max_lights(&self) -> (usize, usize, usize, usize) {
        (self.max_ambient_lights, self.max_directional_lights, self.max_point_lights, self.max_spot_lights)
    }
}
