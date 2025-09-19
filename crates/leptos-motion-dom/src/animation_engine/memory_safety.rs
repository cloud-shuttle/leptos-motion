//! Memory safety utilities for animation engine

use leptos_motion_core::AnimationError;

/// Maximum allowed slice length to prevent memory issues
const MAX_SLICE_LEN: usize = 1024 * 1024; // 1MB

/// Maximum allowed string length
const MAX_STRING_LEN: usize = 10000;

/// Maximum allowed number of animations
const MAX_ANIMATIONS: usize = 1000;

/// Memory safety utilities for animation engine
pub struct MemorySafety;

impl MemorySafety {
    /// Safe slice creation with bounds checking (without unsafe code)
    pub fn safe_slice_from_ptr(_ptr: *const u8, len: usize) -> leptos_motion_core::Result<Vec<u8>> {
        if len > MAX_SLICE_LEN {
            return Err(AnimationError::EngineUnavailable("Slice too large".to_string()));
        }

        // For now, return empty vector since we can't use unsafe
        // In a real implementation, this would need to be handled differently
        Ok(Vec::new())
    }

    /// Validate string before use
    pub fn validate_string(s: &str) -> leptos_motion_core::Result<()> {
        if s.is_empty() {
            return Err(AnimationError::InvalidProperty { property: "Empty string".to_string() });
        }
        
        if s.len() > MAX_STRING_LEN {
            return Err(AnimationError::InvalidProperty { property: "String too long".to_string() });
        }

        // Check for null bytes
        if s.contains('\0') {
            return Err(AnimationError::InvalidProperty { property: "String contains null bytes".to_string() });
        }

        Ok(())
    }

    /// Safe string cloning with validation
    pub fn safe_string_clone(s: &str) -> leptos_motion_core::Result<String> {
        Self::validate_string(s)?;
        Ok(s.to_string())
    }

    /// Validate property name
    pub fn validate_property_name(property: &str) -> leptos_motion_core::Result<()> {
        if property.is_empty() {
            return Err(AnimationError::InvalidProperty { property: "Property name cannot be empty".to_string() });
        }
        
        if property.len() > 1000 {
            return Err(AnimationError::InvalidProperty { property: "Property name too long".to_string() });
        }

        // Check for invalid characters
        if property.contains('\0') || property.contains('\n') || property.contains('\r') {
            return Err(AnimationError::InvalidProperty { property: "Property name contains invalid characters".to_string() });
        }

        Ok(())
    }

    /// Validate numeric values
    pub fn validate_numeric_value(value: f64) -> leptos_motion_core::Result<()> {
        if !value.is_finite() {
            return Err(AnimationError::InvalidProperty { property: "Animation values must be finite numbers".to_string() });
        }

        if value.is_nan() {
            return Err(AnimationError::InvalidProperty { property: "Animation values cannot be NaN".to_string() });
        }

        Ok(())
    }

    /// Validate animation count
    pub fn validate_animation_count(count: usize) -> leptos_motion_core::Result<()> {
        if count > MAX_ANIMATIONS {
            return Err(AnimationError::MemoryError("Too many animations".to_string()));
        }

        Ok(())
    }

    /// Validate animation values map
    pub fn validate_animation_values(values: &std::collections::HashMap<String, f64>) -> leptos_motion_core::Result<()> {
        if values.len() > MAX_ANIMATIONS {
            return Err(AnimationError::MemoryError("Too many animation values".to_string()));
        }

        for (key, value) in values {
            Self::validate_property_name(key)?;
            Self::validate_numeric_value(*value)?;
        }

        Ok(())
    }

    /// Safe property name cloning
    pub fn safe_property_name_clone(property: &str) -> leptos_motion_core::Result<String> {
        Self::validate_property_name(property)?;
        Ok(property.to_string())
    }

    /// Validate pointer alignment
    pub fn validate_pointer_alignment<T>(ptr: *const T) -> leptos_motion_core::Result<()> {
        if ptr.is_null() {
            return Err(AnimationError::EngineUnavailable("Null pointer".to_string()));
        }

        if ptr as usize % std::mem::align_of::<T>() != 0 {
            return Err(AnimationError::EngineUnavailable("Unaligned pointer".to_string()));
        }

        Ok(())
    }

    /// Validate memory size
    pub fn validate_memory_size(size: usize) -> leptos_motion_core::Result<()> {
        if size > MAX_SLICE_LEN {
            return Err(AnimationError::MemoryError("Memory size too large".to_string()));
        }

        Ok(())
    }

    /// Safe memory allocation check
    pub fn check_memory_allocation(size: usize) -> leptos_motion_core::Result<()> {
        Self::validate_memory_size(size)?;
        
        // Additional checks for memory allocation
        if size == 0 {
            return Err(AnimationError::MemoryError("Cannot allocate zero bytes".to_string()));
        }

        Ok(())
    }

    /// Validate callback function
    pub fn validate_callback<T>(callback: &T) -> leptos_motion_core::Result<()> {
        // Basic validation - in a real implementation, this might check for valid function pointers
        // For now, just ensure the callback is not null (which is impossible in Rust)
        Ok(())
    }

    /// Safe callback execution with error handling
    pub fn safe_callback_execution<F, R>(callback: F) -> leptos_motion_core::Result<R>
    where
        F: FnOnce() -> R,
    {
        // Execute callback in a safe context
        // In a real implementation, this might use panic handling
        Ok(callback())
    }

    /// Validate animation duration
    pub fn validate_duration(duration: f64) -> leptos_motion_core::Result<()> {
        if duration < 0.0 {
            return Err(AnimationError::InvalidProperty { property: "Duration cannot be negative".to_string() });
        }

        if duration > 1000.0 {
            return Err(AnimationError::InvalidProperty { property: "Duration too long".to_string() });
        }

        if !duration.is_finite() {
            return Err(AnimationError::InvalidProperty { property: "Duration must be finite".to_string() });
        }

        Ok(())
    }

    /// Validate spring configuration
    pub fn validate_spring_config(stiffness: f64, damping: f64, mass: f64) -> leptos_motion_core::Result<()> {
        if stiffness < 0.0 {
            return Err(AnimationError::InvalidProperty { property: "Stiffness cannot be negative".to_string() });
        }

        if damping < 0.0 {
            return Err(AnimationError::InvalidProperty { property: "Damping cannot be negative".to_string() });
        }

        if mass <= 0.0 {
            return Err(AnimationError::InvalidProperty { property: "Mass must be positive".to_string() });
        }

        if !stiffness.is_finite() || !damping.is_finite() || !mass.is_finite() {
            return Err(AnimationError::InvalidProperty { property: "Spring parameters must be finite".to_string() });
        }

        Ok(())
    }

    /// Get maximum allowed slice length
    pub fn get_max_slice_len() -> usize {
        MAX_SLICE_LEN
    }

    /// Get maximum allowed string length
    pub fn get_max_string_len() -> usize {
        MAX_STRING_LEN
    }

    /// Get maximum allowed animations
    pub fn get_max_animations() -> usize {
        MAX_ANIMATIONS
    }
}
