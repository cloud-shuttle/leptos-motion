//! WebGL acceleration utilities

use crate::{Result, StudioError};

/// WebGL acceleration manager
pub struct WebGLAcceleration {
    /// Acceleration capabilities
    capabilities: AccelerationCapabilities,
    /// Performance settings
    performance_settings: PerformanceSettings,
    /// Acceleration state
    state: AccelerationState,
}

/// Acceleration capabilities
#[derive(Debug, Clone)]
pub struct AccelerationCapabilities {
    /// Whether hardware acceleration is available
    pub hardware_acceleration: bool,
    /// Whether GPU memory is available
    pub gpu_memory_available: bool,
    /// Maximum texture size
    pub max_texture_size: u32,
    /// Maximum vertex attributes
    pub max_vertex_attributes: u32,
    /// Whether instanced rendering is supported
    pub instanced_rendering: bool,
    /// Whether transform feedback is supported
    pub transform_feedback: bool,
}

/// Performance settings
#[derive(Debug, Clone)]
pub struct PerformanceSettings {
    /// Enable hardware acceleration
    pub enable_hardware_acceleration: bool,
    /// Enable GPU memory optimization
    pub enable_gpu_memory_optimization: bool,
    /// Enable instanced rendering
    pub enable_instanced_rendering: bool,
    /// Enable transform feedback
    pub enable_transform_feedback: bool,
    /// Maximum number of animations to render per frame
    pub max_animations_per_frame: usize,
    /// Enable performance monitoring
    pub enable_performance_monitoring: bool,
}

/// Acceleration state
#[derive(Debug, Clone)]
pub struct AccelerationState {
    /// Whether acceleration is currently active
    pub acceleration_active: bool,
    /// Current GPU memory usage
    pub gpu_memory_usage: usize,
    /// Current performance level
    pub performance_level: PerformanceLevel,
    /// Number of active accelerated animations
    pub active_animations: usize,
}

/// Performance levels
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceLevel {
    /// Low performance
    Low,
    /// Medium performance
    Medium,
    /// High performance
    High,
    /// Maximum performance
    Maximum,
}

impl WebGLAcceleration {
    /// Create a new WebGL acceleration manager
    pub fn new() -> Result<Self> {
        let capabilities = Self::detect_capabilities()?;
        let performance_settings = PerformanceSettings::default();
        let state = AccelerationState {
            acceleration_active: false,
            gpu_memory_usage: 0,
            performance_level: PerformanceLevel::Medium,
            active_animations: 0,
        };

        Ok(Self {
            capabilities,
            performance_settings,
            state,
        })
    }

    /// Detect acceleration capabilities
    fn detect_capabilities() -> Result<AccelerationCapabilities> {
        // In a real implementation, this would detect actual WebGL capabilities
        Ok(AccelerationCapabilities {
            hardware_acceleration: true,
            gpu_memory_available: true,
            max_texture_size: 4096,
            max_vertex_attributes: 16,
            instanced_rendering: true,
            transform_feedback: false,
        })
    }

    /// Enable acceleration
    pub fn enable_acceleration(&mut self) -> Result<()> {
        if !self.capabilities.hardware_acceleration {
            return Err(StudioError::InvalidState("Hardware acceleration not available".to_string()));
        }

        self.state.acceleration_active = true;
        self.state.performance_level = PerformanceLevel::High;
        Ok(())
    }

    /// Disable acceleration
    pub fn disable_acceleration(&mut self) {
        self.state.acceleration_active = false;
        self.state.performance_level = PerformanceLevel::Low;
    }

    /// Check if acceleration is enabled
    pub fn is_acceleration_enabled(&self) -> bool {
        self.state.acceleration_active
    }

    /// Get acceleration capabilities
    pub fn capabilities(&self) -> &AccelerationCapabilities {
        &self.capabilities
    }

    /// Get performance settings
    pub fn performance_settings(&self) -> &PerformanceSettings {
        &self.performance_settings
    }

    /// Get mutable performance settings
    pub fn performance_settings_mut(&mut self) -> &mut PerformanceSettings {
        &mut self.performance_settings
    }

    /// Get acceleration state
    pub fn state(&self) -> &AccelerationState {
        &self.state
    }

    /// Update GPU memory usage
    pub fn update_gpu_memory_usage(&mut self, usage: usize) {
        self.state.gpu_memory_usage = usage;
    }

    /// Update active animations count
    pub fn update_active_animations(&mut self, count: usize) {
        self.state.active_animations = count;
    }

    /// Get performance level
    pub fn performance_level(&self) -> &PerformanceLevel {
        &self.state.performance_level
    }

    /// Set performance level
    pub fn set_performance_level(&mut self, level: PerformanceLevel) {
        self.state.performance_level = level;
    }

    /// Check if performance is optimal
    pub fn is_performance_optimal(&self) -> bool {
        self.state.acceleration_active && 
        self.state.performance_level == PerformanceLevel::Maximum
    }

    /// Get performance recommendations
    pub fn get_performance_recommendations(&self) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !self.state.acceleration_active {
            recommendations.push("Enable hardware acceleration for better performance".to_string());
        }

        if self.state.gpu_memory_usage > 100 * 1024 * 1024 { // 100MB
            recommendations.push("High GPU memory usage detected - consider optimization".to_string());
        }

        if self.state.active_animations > self.performance_settings.max_animations_per_frame {
            recommendations.push("Too many active animations - consider reducing count".to_string());
        }

        if self.state.performance_level == PerformanceLevel::Low {
            recommendations.push("Performance level is low - consider enabling acceleration".to_string());
        }

        recommendations
    }
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self {
            enable_hardware_acceleration: true,
            enable_gpu_memory_optimization: true,
            enable_instanced_rendering: true,
            enable_transform_feedback: false,
            max_animations_per_frame: 100,
            enable_performance_monitoring: true,
        }
    }
}
