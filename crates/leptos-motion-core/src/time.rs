//! Time and scheduling utilities

use web_sys::{window, Performance};


/// High-precision timer for animations
pub struct Timer {
    performance: Performance,
    start_time: Option<f64>,
}

impl Timer {
    /// Create a new timer
    pub fn new() -> Option<Self> {
        let performance = window()?.performance()?;
        Some(Self {
            performance,
            start_time: None,
        })
    }
    
    /// Start the timer
    pub fn start(&mut self) {
        self.start_time = Some(self.performance.now());
    }
    
    /// Get elapsed time in milliseconds
    pub fn elapsed(&self) -> Option<f64> {
        let start = self.start_time?;
        Some(self.performance.now() - start)
    }
    
    /// Get current timestamp
    pub fn now(&self) -> f64 {
        self.performance.now()
    }
    
    /// Reset the timer
    pub fn reset(&mut self) {
        self.start_time = None;
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new().expect("Failed to create timer")
    }
}

/// Frame rate utilities
pub mod fps {
    /// Target frame rate for smooth animations
    pub const TARGET_FPS: f64 = 60.0;
    
    /// Target frame time in milliseconds
    pub const FRAME_TIME: f64 = 1000.0 / TARGET_FPS; // ~16.67ms
    
    /// Check if frame time is within acceptable range
    pub fn is_frame_time_acceptable(frame_time: f64) -> bool {
        frame_time <= FRAME_TIME * 1.5 // Allow up to 25ms (40fps)
    }
    
    /// Calculate FPS from frame time
    pub fn calculate_fps(frame_time: f64) -> f64 {
        if frame_time > 0.0 {
            1000.0 / frame_time
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fps_calculations() {
        use fps::*;
        
        assert!((calculate_fps(16.67) - 60.0).abs() < 1.0);
        assert!((calculate_fps(33.33) - 30.0).abs() < 1.0);
        
        assert!(is_frame_time_acceptable(16.0));
        assert!(is_frame_time_acceptable(20.0));
        assert!(!is_frame_time_acceptable(30.0));
    }
}