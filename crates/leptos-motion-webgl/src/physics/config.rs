//! Physics world configuration

use crate::{Result, WebGLError};

/// Physics world configuration
#[derive(Debug, Clone)]
pub struct PhysicsWorldConfig {
    /// Gravity vector (x, y, z)
    pub gravity: (f32, f32, f32),
    /// Time step for physics simulation
    pub time_step: f32,
    /// Maximum number of iterations for constraint solver
    pub max_iterations: u32,
    /// Whether to enable continuous collision detection
    pub continuous_collision_detection: bool,
    /// Whether to enable sleeping for inactive bodies
    pub enable_sleeping: bool,
    /// Sleep threshold for linear velocity
    pub sleep_linear_threshold: f32,
    /// Sleep threshold for angular velocity
    pub sleep_angular_threshold: f32,
    /// Whether to enable debug rendering
    pub enable_debug_rendering: bool,
    /// Whether to enable performance monitoring
    pub enable_performance_monitoring: bool,
}

impl Default for PhysicsWorldConfig {
    fn default() -> Self {
        Self {
            gravity: (0.0, -9.81, 0.0),
            time_step: 1.0 / 60.0, // 60 FPS
            max_iterations: 10,
            continuous_collision_detection: true,
            enable_sleeping: true,
            sleep_linear_threshold: 0.1,
            sleep_angular_threshold: 0.1,
            enable_debug_rendering: false,
            enable_performance_monitoring: false,
        }
    }
}

impl PhysicsWorldConfig {
    /// Create a new physics world configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set gravity
    pub fn with_gravity(mut self, gravity: (f32, f32, f32)) -> Self {
        self.gravity = gravity;
        self
    }

    /// Set time step
    pub fn with_time_step(mut self, time_step: f32) -> Self {
        self.time_step = time_step;
        self
    }

    /// Set maximum iterations
    pub fn with_max_iterations(mut self, max_iterations: u32) -> Self {
        self.max_iterations = max_iterations;
        self
    }

    /// Enable continuous collision detection
    pub fn with_continuous_collision_detection(mut self, enable: bool) -> Self {
        self.continuous_collision_detection = enable;
        self
    }

    /// Enable sleeping
    pub fn with_sleeping(mut self, enable: bool) -> Self {
        self.enable_sleeping = enable;
        self
    }

    /// Set sleep thresholds
    pub fn with_sleep_thresholds(mut self, linear: f32, angular: f32) -> Self {
        self.sleep_linear_threshold = linear;
        self.sleep_angular_threshold = angular;
        self
    }

    /// Enable debug rendering
    pub fn with_debug_rendering(mut self, enable: bool) -> Self {
        self.enable_debug_rendering = enable;
        self
    }

    /// Enable performance monitoring
    pub fn with_performance_monitoring(mut self, enable: bool) -> Self {
        self.enable_performance_monitoring = enable;
        self
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        if self.time_step <= 0.0 {
            return Err(WebGLError::invalid_state("Time step must be positive"));
        }

        if self.max_iterations == 0 {
            return Err(WebGLError::invalid_state("Max iterations must be greater than 0"));
        }

        if self.sleep_linear_threshold < 0.0 {
            return Err(WebGLError::invalid_state("Sleep linear threshold must be non-negative"));
        }

        if self.sleep_angular_threshold < 0.0 {
            return Err(WebGLError::invalid_state("Sleep angular threshold must be non-negative"));
        }

        Ok(())
    }

    /// Get gravity magnitude
    pub fn gravity_magnitude(&self) -> f32 {
        let (x, y, z) = self.gravity;
        (x * x + y * y + z * z).sqrt()
    }

    /// Check if gravity is enabled
    pub fn has_gravity(&self) -> bool {
        self.gravity_magnitude() > 0.0
    }

    /// Get effective time step (clamped to reasonable range)
    pub fn effective_time_step(&self) -> f32 {
        self.time_step.clamp(1.0 / 1000.0, 1.0 / 10.0) // Between 10 FPS and 1000 FPS
    }

    /// Get effective max iterations (clamped to reasonable range)
    pub fn effective_max_iterations(&self) -> u32 {
        self.max_iterations.clamp(1, 100)
    }
}
