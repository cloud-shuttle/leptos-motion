//! GPU-accelerated animation system

use crate::{Result, StudioError, timeline::AnimationValue};
use std::collections::HashMap;

/// GPU-accelerated animation
pub struct GPUAnimation {
    /// Animation ID
    pub id: u64,
    /// Animation values
    pub values: HashMap<String, AnimationValue>,
    /// Animation duration
    pub duration: f64,
    /// Start time
    pub start_time: Option<f64>,
    /// End time
    pub end_time: Option<f64>,
    /// Animation state
    pub state: GPUAnimationState,
    /// GPU buffer handles
    pub buffers: GPUAnimationBuffers,
    /// Shader program
    pub shader_program: Option<web_sys::WebGlProgram>,
}

/// GPU animation state
#[derive(Debug, Clone, PartialEq)]
pub enum GPUAnimationState {
    /// Animation is ready
    Ready,
    /// Animation is running
    Running,
    /// Animation is paused
    Paused,
    /// Animation is completed
    Completed,
}

/// GPU animation buffers
#[derive(Debug, Clone)]
pub struct GPUAnimationBuffers {
    /// Vertex buffer
    pub vertex_buffer: Option<web_sys::WebGlBuffer>,
    /// Index buffer
    pub index_buffer: Option<web_sys::WebGlBuffer>,
    /// Uniform buffer
    pub uniform_buffer: Option<web_sys::WebGlBuffer>,
    /// Texture buffer
    pub texture_buffer: Option<web_sys::WebGlBuffer>,
}

impl GPUAnimation {
    /// Create a new GPU animation
    pub fn new(id: u64) -> Self {
        Self {
            id,
            values: HashMap::new(),
            duration: 0.0,
            start_time: None,
            end_time: None,
            state: GPUAnimationState::Ready,
            buffers: GPUAnimationBuffers {
                vertex_buffer: None,
                index_buffer: None,
                uniform_buffer: None,
                texture_buffer: None,
            },
            shader_program: None,
        }
    }

    /// Start the animation
    pub fn start(&mut self, current_time: f64) -> Result<()> {
        if self.state != GPUAnimationState::Ready {
            return Err(StudioError::InvalidState("Animation not ready".to_string()));
        }

        self.state = GPUAnimationState::Running;
        self.start_time = Some(current_time);
        self.end_time = Some(current_time + self.duration);

        Ok(())
    }

    /// Stop the animation
    pub fn stop(&mut self) {
        self.state = GPUAnimationState::Completed;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if self.state == GPUAnimationState::Running {
            self.state = GPUAnimationState::Paused;
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.state == GPUAnimationState::Paused {
            self.state = GPUAnimationState::Running;
        }
    }

    /// Update the animation
    pub fn update(&mut self, current_time: f64) -> Result<()> {
        if self.state != GPUAnimationState::Running {
            return Ok(());
        }

        if let Some(end_time) = self.end_time {
            if current_time >= end_time {
                self.state = GPUAnimationState::Completed;
            }
        }

        Ok(())
    }

    /// Get animation progress (0.0 to 1.0)
    pub fn progress(&self, current_time: f64) -> f64 {
        if let (Some(start), Some(end)) = (self.start_time, self.end_time) {
            if end <= start {
                return 1.0;
            }
            ((current_time - start) / (end - start)).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Set shader program
    pub fn set_shader_program(&mut self, program: web_sys::WebGlProgram) {
        self.shader_program = Some(program);
    }

    /// Get shader program
    pub fn shader_program(&self) -> Option<&web_sys::WebGlProgram> {
        self.shader_program.as_ref()
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.state == GPUAnimationState::Completed
    }

    /// Check if animation is running
    pub fn is_running(&self) -> bool {
        self.state == GPUAnimationState::Running
    }

    /// Get animation state
    pub fn state(&self) -> &GPUAnimationState {
        &self.state
    }

    /// Set animation values
    pub fn set_values(&mut self, values: HashMap<String, AnimationValue>) {
        self.values = values;
    }

    /// Get animation values
    pub fn values(&self) -> &HashMap<String, AnimationValue> {
        &self.values
    }

    /// Set animation duration
    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    /// Get animation duration
    pub fn duration(&self) -> f64 {
        self.duration
    }
}

impl GPUAnimationBuffers {
    /// Create new GPU animation buffers
    pub fn new() -> Self {
        Self {
            vertex_buffer: None,
            index_buffer: None,
            uniform_buffer: None,
            texture_buffer: None,
        }
    }

    /// Set vertex buffer
    pub fn set_vertex_buffer(&mut self, buffer: web_sys::WebGlBuffer) {
        self.vertex_buffer = Some(buffer);
    }

    /// Set index buffer
    pub fn set_index_buffer(&mut self, buffer: web_sys::WebGlBuffer) {
        self.index_buffer = Some(buffer);
    }

    /// Set uniform buffer
    pub fn set_uniform_buffer(&mut self, buffer: web_sys::WebGlBuffer) {
        self.uniform_buffer = Some(buffer);
    }

    /// Set texture buffer
    pub fn set_texture_buffer(&mut self, buffer: web_sys::WebGlBuffer) {
        self.texture_buffer = Some(buffer);
    }
}
