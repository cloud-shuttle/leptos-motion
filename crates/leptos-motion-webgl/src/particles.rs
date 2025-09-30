//! Particle system for WebGL rendering

use crate::error::{Result, WebGLError};
use std::collections::HashMap;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

/// Particle properties
#[derive(Debug, Clone)]
pub struct Particle {
    /// Position
    pub position: [f32; 3],
    /// Velocity
    pub velocity: [f32; 3],
    /// Acceleration
    pub acceleration: [f32; 3],
    /// Color
    pub color: [f32; 4],
    /// Size
    pub size: f32,
    /// Life
    pub life: f32,
    /// Maximum life
    pub max_life: f32,
    /// Age
    pub age: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self::new()
    }
}

impl Particle {
    /// Create a new particle
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            color: [1.0, 1.0, 1.0, 1.0],
            size: 1.0,
            life: 1.0,
            max_life: 1.0,
            age: 0.0,
        }
    }

    /// Update particle
    pub fn update(&mut self, delta_time: f32) {
        // Update velocity with acceleration
        self.velocity[0] += self.acceleration[0] * delta_time;
        self.velocity[1] += self.acceleration[1] * delta_time;
        self.velocity[2] += self.acceleration[2] * delta_time;

        // Update position with velocity
        self.position[0] += self.velocity[0] * delta_time;
        self.position[1] += self.velocity[1] * delta_time;
        self.position[2] += self.velocity[2] * delta_time;

        // Update age and life
        self.age += delta_time;
        self.life = (self.max_life - self.age) / self.max_life;
    }

    /// Check if particle is alive
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}

/// Particle emitter configuration
#[derive(Debug, Clone)]
pub struct ParticleEmitter {
    /// Emitter ID
    pub id: String,
    /// Emitter position
    pub position: [f32; 3],
    /// Emitter direction
    pub direction: [f32; 3],
    /// Emission rate (particles per second)
    pub emission_rate: f32,
    /// Particle lifetime
    pub particle_lifetime: f32,
    /// Particle size range
    pub size_range: (f32, f32),
    /// Velocity range
    pub velocity_range: (f32, f32),
    /// Color range
    pub color_range: ([f32; 4], [f32; 4]),
    /// Enabled flag
    pub enabled: bool,
    /// Burst mode
    pub burst_mode: bool,
    /// Burst count
    pub burst_count: u32,
}

impl ParticleEmitter {
    /// Create a new particle emitter
    pub fn new(id: String) -> Self {
        Self {
            id,
            position: [0.0, 0.0, 0.0],
            direction: [0.0, 1.0, 0.0],
            emission_rate: 10.0,
            particle_lifetime: 2.0,
            size_range: (0.5, 2.0),
            velocity_range: (1.0, 5.0),
            color_range: ([1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0, 1.0]),
            enabled: true,
            burst_mode: false,
            burst_count: 100,
        }
    }

    /// Set position
    pub fn set_position(&mut self, position: [f32; 3]) {
        self.position = position;
    }

    /// Set direction
    pub fn set_direction(&mut self, direction: [f32; 3]) {
        self.direction = direction;
    }

    /// Set emission rate
    pub fn set_emission_rate(&mut self, rate: f32) {
        self.emission_rate = rate.max(0.0);
    }

    /// Set particle lifetime
    pub fn set_particle_lifetime(&mut self, lifetime: f32) {
        self.particle_lifetime = lifetime.max(0.1);
    }

    /// Enable burst mode
    pub fn enable_burst_mode(&mut self, count: u32) {
        self.burst_mode = true;
        self.burst_count = count;
    }

    /// Disable burst mode
    pub fn disable_burst_mode(&mut self) {
        self.burst_mode = false;
    }
}

/// Particle system manager
pub struct ParticleSystem {
    /// WebGL context
    context: WebGl2RenderingContext,
    /// Particles
    particles: Vec<Particle>,
    /// Emitters
    emitters: HashMap<String, ParticleEmitter>,
    /// Maximum particles
    max_particles: usize,
    /// Vertex buffer
    vertex_buffer: Option<WebGlBuffer>,
    /// Vertex array object
    vao: Option<WebGlVertexArrayObject>,
    /// Last update time
    last_update_time: f64,
}

impl ParticleSystem {
    /// Create a new particle system
    pub fn new(context: WebGl2RenderingContext, max_particles: usize) -> Result<Self> {
        Ok(Self {
            context,
            particles: Vec::with_capacity(max_particles),
            emitters: HashMap::new(),
            max_particles,
            vertex_buffer: None,
            vao: None,
            last_update_time: 0.0,
        })
    }

    /// Initialize the particle system
    pub fn initialize(&mut self) -> Result<()> {
        // Create vertex buffer
        self.vertex_buffer = Some(
            self.context
                .create_buffer()
                .ok_or_else(|| WebGLError::buffer_error("Failed to create vertex buffer"))?,
        );

        // Create vertex array object
        self.vao = Some(
            self.context
                .create_vertex_array()
                .ok_or_else(|| WebGLError::buffer_error("Failed to create VAO"))?,
        );

        // Set up vertex attributes
        if let (Some(vao), Some(vertex_buffer)) = (&self.vao, &self.vertex_buffer) {
            self.context.bind_vertex_array(Some(vao));
            self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vertex_buffer));

            // Position attribute
            self.context.enable_vertex_attrib_array(0);
            self.context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 28, 0);

            // Color attribute
            self.context.enable_vertex_attrib_array(1);
            self.context.vertex_attrib_pointer_with_i32(1, 4, WebGl2RenderingContext::FLOAT, false, 28, 12);

            // Size attribute
            self.context.enable_vertex_attrib_array(2);
            self.context.vertex_attrib_pointer_with_i32(2, 1, WebGl2RenderingContext::FLOAT, false, 28, 28);
        }

        Ok(())
    }

    /// Add an emitter
    pub fn add_emitter(&mut self, emitter: ParticleEmitter) {
        self.emitters.insert(emitter.id.clone(), emitter);
    }

    /// Remove an emitter
    pub fn remove_emitter(&mut self, id: &str) {
        self.emitters.remove(id);
    }

    /// Get an emitter
    pub fn get_emitter(&self, id: &str) -> Option<&ParticleEmitter> {
        self.emitters.get(id)
    }

    /// Get a mutable emitter
    pub fn get_emitter_mut(&mut self, id: &str) -> Option<&mut ParticleEmitter> {
        self.emitters.get_mut(id)
    }

    /// Update the particle system
    pub fn update(&mut self, current_time: f64) -> Result<()> {
        let delta_time = if self.last_update_time == 0.0 {
            0.016 // Assume 60 FPS for first frame
        } else {
            (current_time - self.last_update_time) as f32 / 1000.0
        };

        self.last_update_time = current_time;

        // Update existing particles
        self.particles.retain_mut(|particle| {
            particle.update(delta_time);
            particle.is_alive()
        });

        // Emit new particles
        // Collect emitter data first to avoid borrowing conflicts
        let mut emission_requests = Vec::new();
        for (id, emitter) in &self.emitters {
            if !emitter.enabled {
                continue;
            }

            if emitter.burst_mode {
                // Emit burst of particles
                for _ in 0..emitter.burst_count {
                    if self.particles.len() < self.max_particles {
                        emission_requests.push((id.clone(), emitter.clone()));
                    }
                }
            } else {
                // Emit particles based on emission rate
                let particles_to_emit = (emitter.emission_rate * delta_time) as u32;
                for _ in 0..particles_to_emit {
                    if self.particles.len() < self.max_particles {
                        emission_requests.push((id.clone(), emitter.clone()));
                    }
                }
            }
        }

        // Now emit particles without borrowing conflicts
        for (id, emitter_data) in emission_requests {
            self.emit_particle(&emitter_data);
            
            // Update emitter state if needed
            if let Some(emitter) = self.emitters.get_mut(&id) {
                if emitter.burst_mode {
                    emitter.disable_burst_mode();
                }
            }
        }

        // Update vertex buffer
        self.update_vertex_buffer()?;

        Ok(())
    }

    /// Emit a new particle
    fn emit_particle(&mut self, emitter: &ParticleEmitter) {
        let mut particle = Particle::new();

        // Set position
        particle.position = emitter.position;

        // Set velocity with random direction
        let speed = emitter.velocity_range.0 + (emitter.velocity_range.1 - emitter.velocity_range.0) * 0.5; // Simplified random
        let direction = emitter.direction;
        particle.velocity = [
            direction[0] * speed,
            direction[1] * speed,
            direction[2] * speed,
        ];

        // Set color with random interpolation
        let t = 0.5; // Simplified random
        particle.color = [
            emitter.color_range.0[0] + (emitter.color_range.1[0] - emitter.color_range.0[0]) * t,
            emitter.color_range.0[1] + (emitter.color_range.1[1] - emitter.color_range.0[1]) * t,
            emitter.color_range.0[2] + (emitter.color_range.1[2] - emitter.color_range.0[2]) * t,
            emitter.color_range.0[3] + (emitter.color_range.1[3] - emitter.color_range.0[3]) * t,
        ];

        // Set size with random range
        particle.size = emitter.size_range.0 + (emitter.size_range.1 - emitter.size_range.0) * 0.5; // Simplified random

        // Set lifetime
        particle.max_life = emitter.particle_lifetime;
        particle.life = 1.0;
        particle.age = 0.0;

        self.particles.push(particle);
    }

    /// Update vertex buffer with particle data
    fn update_vertex_buffer(&mut self) -> Result<()> {
        if self.particles.is_empty() {
            return Ok(());
        }

        // Create vertex data
        let mut vertex_data = Vec::with_capacity(self.particles.len() * 7); // 3 pos + 4 color + 1 size

        for particle in &self.particles {
            // Position
            vertex_data.push(particle.position[0]);
            vertex_data.push(particle.position[1]);
            vertex_data.push(particle.position[2]);

            // Color
            vertex_data.push(particle.color[0]);
            vertex_data.push(particle.color[1]);
            vertex_data.push(particle.color[2]);
            vertex_data.push(particle.color[3]);

            // Size
            vertex_data.push(particle.size);
        }

        // Upload to GPU
        if let Some(vertex_buffer) = &self.vertex_buffer {
            self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(vertex_buffer));
            
            unsafe {
                let array = js_sys::Float32Array::view(&vertex_data);
                self.context.buffer_data_with_array_buffer_view(
                    WebGl2RenderingContext::ARRAY_BUFFER,
                    &array,
                    WebGl2RenderingContext::DYNAMIC_DRAW,
                );
            }
        }

        Ok(())
    }

    /// Render the particle system
    pub fn render(&self) -> Result<()> {
        if self.particles.is_empty() {
            return Ok(());
        }

        // Bind VAO
        if let Some(vao) = &self.vao {
            self.context.bind_vertex_array(Some(vao));
        }

        // Enable blending for particles
        self.context.enable(WebGl2RenderingContext::BLEND);
        self.context.blend_func(
            WebGl2RenderingContext::SRC_ALPHA,
            WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA,
        );

        // Draw particles
        self.context.draw_arrays(
            WebGl2RenderingContext::POINTS,
            0,
            self.particles.len() as i32,
        );

        // Disable blending
        self.context.disable(WebGl2RenderingContext::BLEND);

        Ok(())
    }

    /// Get particle count
    pub fn get_particle_count(&self) -> usize {
        self.particles.len()
    }

    /// Get maximum particles
    pub fn get_max_particles(&self) -> usize {
        self.max_particles
    }

    /// Clear all particles
    pub fn clear_particles(&mut self) {
        self.particles.clear();
    }

    /// Clear all emitters
    pub fn clear_emitters(&mut self) {
        self.emitters.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_particle_creation() {
        let particle = Particle::new();
        assert_eq!(particle.position, [0.0, 0.0, 0.0]);
        assert_eq!(particle.life, 1.0);
        assert!(particle.is_alive());
    }

    #[test]
    fn test_particle_update() {
        let mut particle = Particle::new();
        particle.velocity = [1.0, 0.0, 0.0];
        particle.max_life = 2.0;
        particle.life = 1.0;

        particle.update(1.0);
        assert_eq!(particle.position[0], 1.0);
        assert_eq!(particle.age, 1.0);
        assert_eq!(particle.life, 0.5);
    }

    #[test]
    fn test_emitter_creation() {
        let emitter = ParticleEmitter::new("test".to_string());
        assert_eq!(emitter.id, "test");
        assert_eq!(emitter.emission_rate, 10.0);
        assert!(emitter.enabled);
    }
}
