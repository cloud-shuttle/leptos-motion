//! Memory Management
//!
//! This module provides comprehensive memory management for the animation system,
//! broken down into focused, maintainable components.

pub mod memory_stats;
pub mod gc_strategy;
pub mod animation_memory_manager;
pub mod auto_memory_manager;
pub mod memory_pressure;

// Re-export main types for convenience
pub use memory_stats::{MemoryStats, MemoryPressure, MemoryTracker};
pub use gc_strategy::{GCStrategy, GarbageCollector};
pub use animation_memory_manager::AnimationMemoryManager;
pub use auto_memory_manager::AutoMemoryManager;
pub use memory_pressure::{MemoryPressureMonitor, PressureTrend, PressureStats};
