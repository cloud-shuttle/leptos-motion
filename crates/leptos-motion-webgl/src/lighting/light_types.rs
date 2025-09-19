//! Light types and color definitions for WebGL rendering

use crate::error::{Result, WebGLError};

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
            intensity: intensity.max(0.0),
            enabled: true,
        }
    }

    /// Set light intensity
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity.max(0.0);
    }

    /// Get light intensity
    pub fn get_intensity(&self) -> f32 {
        self.intensity
    }

    /// Set light color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Get light color
    pub fn get_color(&self) -> Color {
        self.color
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

    /// Get light type
    pub fn get_light_type(&self) -> LightType {
        self.light_type
    }

    /// Get light name
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get light ID
    pub fn get_id(&self) -> &str {
        &self.id
    }
}
