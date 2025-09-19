//! Object pooling system for animations
//!
//! This module provides efficient object pooling for animations,
//! broken down into focused, maintainable modules.

pub mod pooled_animation;
pub mod animation_types;
pub mod pool_config;
pub mod memory_stats;
pub mod performance_metrics;
pub mod animation_pool;
pub mod memory_manager;
pub mod pool_monitor;

// Re-export main types for convenience
pub use pooled_animation::*;
pub use animation_types::*;
pub use pool_config::*;
pub use memory_stats::*;
pub use performance_metrics::*;
pub use animation_pool::*;
pub use memory_manager::*;
pub use pool_monitor::*;
