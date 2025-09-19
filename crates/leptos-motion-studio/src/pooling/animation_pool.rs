//! Animation pool implementation

use super::*;
use crate::{Result, StudioError, timeline::AnimationValue};
use std::collections::HashMap;

/// Pool of reusable animations
#[derive(Debug, Clone)]
pub struct AnimationPool {
    /// Pool configuration
    config: PoolConfig,
    /// Available animations by type
    available_animations: HashMap<AnimationType, Vec<PooledAnimation>>,
    /// Active animations
    active_animations: HashMap<u64, PooledAnimation>,
    /// Next animation ID
    next_id: u64,
    /// Memory statistics
    memory_stats: MemoryStats,
    /// Performance metrics
    performance_metrics: PerformanceMetrics,
}

impl AnimationPool {
    /// Create a new animation pool
    pub fn new(config: PoolConfig) -> Result<Self> {
        config.validate().map_err(|e| StudioError::InvalidInput(e.to_string()))?;

        let mut pool = Self {
            config,
            available_animations: HashMap::new(),
            active_animations: HashMap::new(),
            next_id: 1,
            memory_stats: MemoryStats::new(),
            performance_metrics: PerformanceMetrics::new(),
        };

        if pool.config.pre_allocate {
            pool.pre_allocate_animations()?;
        }

        Ok(pool)
    }

    /// Pre-allocate animations
    fn pre_allocate_animations(&mut self) -> Result<()> {
        let animation_types = vec![
            AnimationType::Transform,
            AnimationType::Opacity,
            AnimationType::Color,
            AnimationType::Path,
        ];

        for animation_type in animation_types {
            let mut animations = Vec::new();
            for _ in 0..self.config.initial_size {
                let animation = PooledAnimation::new(self.next_id, animation_type.clone());
                self.next_id += 1;
                animations.push(animation);
            }
            self.available_animations.insert(animation_type, animations);
        }

        Ok(())
    }

    /// Get an animation from the pool
    pub fn get_animation(&mut self, animation_type: AnimationType) -> Result<&mut PooledAnimation> {
        let start_time = std::time::Instant::now();

        // Try to get from available pool
        if let Some(animations) = self.available_animations.get_mut(&animation_type) {
            if let Some(mut animation) = animations.pop() {
                animation.reset();
                animation.id = self.next_id;
                self.next_id += 1;
                
                let animation_id = animation.id;
                self.active_animations.insert(animation_id, animation);
                
                let elapsed = start_time.elapsed().as_micros() as u64;
                self.performance_metrics.record_reuse(elapsed);
                
                return Ok(self.active_animations.get_mut(&animation_id).unwrap());
            }
        }

        // Create new animation if pool is empty
        let mut animation = PooledAnimation::new(self.next_id, animation_type);
        self.next_id += 1;
        let animation_id = animation.id;
        self.active_animations.insert(animation_id, animation);

        let elapsed = start_time.elapsed().as_micros() as u64;
        self.performance_metrics.record_creation(elapsed);

        Ok(self.active_animations.get_mut(&animation_id).unwrap())
    }

    /// Return an animation to the pool
    pub fn return_animation(&mut self, animation_id: u64) -> Result<()> {
        if let Some(animation) = self.active_animations.remove(&animation_id) {
            let animation_type = animation.animation_type.clone();
            let mut reset_animation = animation;
            reset_animation.reset();
            
            if let Some(animations) = self.available_animations.get_mut(&animation_type) {
                animations.push(reset_animation);
            } else {
                let mut animations = Vec::new();
                animations.push(reset_animation);
                self.available_animations.insert(animation_type, animations);
            }

            self.update_memory_stats();
            Ok(())
        } else {
            Err(StudioError::NotFound("Animation not found".to_string()))
        }
    }

    /// Update memory statistics
    fn update_memory_stats(&mut self) {
        let total_allocated = self.active_animations.len() + 
            self.available_animations.values().map(|v| v.len()).sum::<usize>();
        let memory_in_use = self.active_animations.len();
        let active_count = self.active_animations.len();
        let pooled_count = self.available_animations.values().map(|v| v.len()).sum::<usize>();

        self.memory_stats.update(total_allocated, memory_in_use, active_count, pooled_count);
    }

    /// Get memory statistics
    pub fn memory_stats(&self) -> &MemoryStats {
        &self.memory_stats
    }

    /// Get performance metrics
    pub fn performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    /// Clean up unused animations
    pub fn cleanup(&mut self) -> Result<()> {
        let target_size = (self.config.max_capacity as f32 * 0.8) as usize;
        
        for (_, animations) in self.available_animations.iter_mut() {
            if animations.len() > target_size {
                let excess = animations.len() - target_size;
                animations.truncate(animations.len() - excess);
                self.performance_metrics.record_contraction();
            }
        }

        self.update_memory_stats();
        Ok(())
    }

    /// Get pool status
    pub fn status(&self) -> PoolStatus {
        PoolStatus {
            total_capacity: self.config.max_capacity,
            active_animations: self.active_animations.len(),
            available_animations: self.available_animations.values().map(|v| v.len()).sum(),
            memory_usage: self.memory_stats.usage_percentage(),
            cache_hit_rate: self.performance_metrics.cache_hit_rate,
        }
    }
}

/// Pool status information
#[derive(Debug, Clone)]
pub struct PoolStatus {
    /// Total pool capacity
    pub total_capacity: usize,
    /// Number of active animations
    pub active_animations: usize,
    /// Number of available animations
    pub available_animations: usize,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Cache hit rate
    pub cache_hit_rate: f64,
}
