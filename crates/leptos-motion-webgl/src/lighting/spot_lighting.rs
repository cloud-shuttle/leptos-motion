//! Spot lighting implementation for WebGL rendering

use crate::error::{Result, WebGLError};
use gl_matrix::vec3;
use super::light_types::{Light, LightType, Color};

/// Spot light (like a flashlight)
#[derive(Debug, Clone)]
pub struct SpotLight {
    /// Base light
    pub light: Light,
    /// Light position
    pub position: [f32; 3],
    /// Light direction (normalized)
    pub direction: [f32; 3],
    /// Inner cone angle (in radians)
    pub inner_cone_angle: f32,
    /// Outer cone angle (in radians)
    pub outer_cone_angle: f32,
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
            outer_cone_angle: outer_cone_angle
                .max(inner_cone_angle)
                .min(std::f32::consts::PI),
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

    /// Get attenuation parameters
    pub fn get_attenuation(&self) -> (f32, f32, f32) {
        (self.constant_attenuation, self.linear_attenuation, self.quadratic_attenuation)
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
        let smooth_factor = (cos_angle - self.outer_cone_angle.cos())
            / (self.inner_cone_angle.cos() - self.outer_cone_angle.cos());
        smooth_factor * smooth_factor
    }

    /// Calculate attenuation factor for a given distance
    pub fn calculate_attenuation(&self, distance: f32) -> f32 {
        if distance >= self.range {
            return 0.0;
        }

        1.0 / (self.constant_attenuation
            + self.linear_attenuation * distance
            + self.quadratic_attenuation * distance * distance)
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

    /// Calculate spot light contribution
    pub fn calculate_spot_contribution(
        &self,
        surface_position: [f32; 3],
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

        // Calculate light direction and distance
        let light_direction = [
            self.position[0] - surface_position[0],
            self.position[1] - surface_position[1],
            self.position[2] - surface_position[2],
        ];

        let distance = (light_direction[0] * light_direction[0]
            + light_direction[1] * light_direction[1]
            + light_direction[2] * light_direction[2]).sqrt();

        if distance == 0.0 {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        let mut normalized_light_dir = [
            light_direction[0] / distance,
            light_direction[1] / distance,
            light_direction[2] / distance,
        ];

        // Calculate spot factor
        let spot_factor = self.calculate_spot_factor(light_direction);
        if spot_factor == 0.0 {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        // Calculate attenuation
        let attenuation = self.calculate_attenuation(distance);

        // Calculate diffuse contribution
        let mut normalized_normal = surface_normal;
        let normal_length = (surface_normal[0] * surface_normal[0]
            + surface_normal[1] * surface_normal[1]
            + surface_normal[2] * surface_normal[2]).sqrt();

        if normal_length > 0.0 {
            normalized_normal[0] /= normal_length;
            normalized_normal[1] /= normal_length;
            normalized_normal[2] /= normal_length;
        }

        let diffuse_factor = (normalized_normal[0] * normalized_light_dir[0]
            + normalized_normal[1] * normalized_light_dir[1]
            + normalized_normal[2] * normalized_light_dir[2]).max(0.0);

        let diffuse_contribution = [
            light_color[0] * material_diffuse[0] * diffuse_factor * intensity * attenuation * spot_factor,
            light_color[1] * material_diffuse[1] * diffuse_factor * intensity * attenuation * spot_factor,
            light_color[2] * material_diffuse[2] * diffuse_factor * intensity * attenuation * spot_factor,
        ];

        // Calculate specular contribution
        let mut normalized_view_dir = view_direction;
        let view_length = (view_direction[0] * view_direction[0]
            + view_direction[1] * view_direction[1]
            + view_direction[2] * view_direction[2]).sqrt();

        if view_length > 0.0 {
            normalized_view_dir[0] /= view_length;
            normalized_view_dir[1] /= view_length;
            normalized_view_dir[2] /= view_length;
        }

        let reflect_direction = [
            2.0 * diffuse_factor * normalized_normal[0] - normalized_light_dir[0],
            2.0 * diffuse_factor * normalized_normal[1] - normalized_light_dir[1],
            2.0 * diffuse_factor * normalized_normal[2] - normalized_light_dir[2],
        ];

        let specular_factor = (normalized_view_dir[0] * reflect_direction[0]
            + normalized_view_dir[1] * reflect_direction[1]
            + normalized_view_dir[2] * reflect_direction[2]).max(0.0).powf(shininess);

        let specular_contribution = [
            light_color[0] * material_specular[0] * specular_factor * intensity * attenuation * spot_factor,
            light_color[1] * material_specular[1] * specular_factor * intensity * attenuation * spot_factor,
            light_color[2] * material_specular[2] * specular_factor * intensity * attenuation * spot_factor,
        ];

        (diffuse_contribution, specular_contribution)
    }

    /// Get spot light data for shader
    pub fn get_shader_data(&self) -> [f32; 16] {
        if !self.light.enabled {
            return [0.0; 16];
        }

        let color = self.light.color.as_rgb_array();
        [
            color[0], color[1], color[2], self.light.intensity,
            self.position[0], self.position[1], self.position[2], self.range,
            self.direction[0], self.direction[1], self.direction[2], self.inner_cone_angle,
            self.outer_cone_angle, self.constant_attenuation, self.linear_attenuation,
            if self.cast_shadow { 1.0 } else { 0.0 },
        ]
    }
}
