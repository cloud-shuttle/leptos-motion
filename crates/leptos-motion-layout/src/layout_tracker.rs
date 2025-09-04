//! Layout tracking and performance monitoring
//! 
//! This module provides functionality for tracking layout changes,
//! monitoring performance metrics, and optimizing layout animations.

use std::collections::HashMap;
use std::time::Instant;
use web_sys::{Element, DomRect};


/// Layout change record
#[derive(Debug, Clone)]
pub struct LayoutChange {
    /// Element ID
    pub element_id: String,
    /// Previous layout
    pub previous_layout: DomRect,
    /// Current layout
    pub current_layout: DomRect,
    /// Change timestamp
    pub timestamp: f64,
    /// Change type
    pub change_type: LayoutChangeType,
    /// Performance impact
    pub performance_impact: PerformanceImpact,
}

/// Type of layout change
#[derive(Debug, Clone)]
pub enum LayoutChangeType {
    /// Position change
    Position,
    /// Size change
    Size,
    /// Both position and size
    PositionAndSize,
    /// No change
    None,
}

/// Performance impact of layout change
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerformanceImpact {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
    /// Critical impact
    Critical,
}

/// Layout tracking statistics
#[derive(Debug, Clone)]
pub struct LayoutStats {
    /// Total layout changes
    pub total_changes: usize,
    /// Changes in last second
    pub changes_last_second: usize,
    /// Average change frequency
    pub change_frequency: f64,
    /// Performance impact distribution
    pub impact_distribution: HashMap<PerformanceImpact, usize>,
    /// Frame rate during tracking
    pub frame_rate: f64,
}

/// Layout tracker for monitoring changes
pub struct LayoutTracker {
    /// Tracked elements
    tracked_elements: HashMap<String, TrackedElement>,
    /// Layout change history
    change_history: Vec<LayoutChange>,
    /// Performance monitoring
    performance_monitor: PerformanceMonitor,
    /// Statistics
    stats: LayoutStats,
    /// Tracking enabled
    enabled: bool,
}

/// Tracked element information
#[derive(Debug, Clone)]
pub struct TrackedElement {
    /// Element ID
    pub id: String,
    /// Element reference
    pub element: Element,
    /// Current layout
    pub current_layout: DomRect,
    /// Previous layout
    pub previous_layout: Option<DomRect>,
    /// Last update time
    pub last_update: Instant,
    /// Change count
    pub change_count: usize,
}

/// Performance monitoring
pub struct PerformanceMonitor {
    /// Frame rate tracking
    frame_times: Vec<f64>,
    /// Memory usage tracking
    memory_usage: Vec<usize>,
    /// Performance observer
    performance_observer: Option<PerformanceObserver>,
    /// Last frame time
    pub last_frame_time: f64,
}

/// Performance observer (placeholder for future implementation)
pub struct PerformanceObserver {
    /// Observer ID
    pub id: String,
    /// Callback function
    pub callback: Box<dyn FnMut(f64)>,
}

impl LayoutTracker {
    /// Create a new layout tracker
    pub fn new() -> Self {
        Self {
            tracked_elements: HashMap::new(),
            change_history: Vec::new(),
            performance_monitor: PerformanceMonitor::new(),
            stats: LayoutStats::default(),
            enabled: true,
        }
    }

    /// Start tracking an element
    pub fn track_element(&mut self, element: Element) -> Result<(), String> {
        let element_id = element.id();
        
        // Get current layout
        let current_layout = element.get_bounding_client_rect();
        
        // Create tracked element
        let tracked_element = TrackedElement {
            id: element_id.clone(),
            element: element.clone(),
            current_layout,
            previous_layout: None,
            last_update: Instant::now(),
            change_count: 0,
        };
        
        // Add to tracking
        self.tracked_elements.insert(element_id, tracked_element);
        
        Ok(())
    }

    /// Stop tracking an element
    pub fn untrack_element(&mut self, element_id: &str) -> Result<(), String> {
        if self.tracked_elements.remove(element_id).is_some() {
            Ok(())
        } else {
            Err(format!("Element not being tracked: {}", element_id))
        }
    }

    /// Update layout tracking
    pub fn update(&mut self) -> Result<Vec<LayoutChange>, String> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        let mut changes = Vec::new();
        let current_time = js_sys::Date::now();
        
        // Check each tracked element for changes
        let element_ids: Vec<String> = self.tracked_elements.keys().cloned().collect();
        
        for element_id in element_ids {
            // Get current layout first
            let new_layout = if let Some(tracked_element) = self.tracked_elements.get(&element_id) {
                tracked_element.element.get_bounding_client_rect()
            } else {
                continue;
            };
            
            // Check if layout changed
            let current_layout = if let Some(tracked_element) = self.tracked_elements.get(&element_id) {
                tracked_element.current_layout.clone()
            } else {
                continue;
            };
            
            if self.has_layout_changed(&current_layout, &new_layout) {
                // Create change record
                let change = LayoutChange {
                    element_id: element_id.clone(),
                    previous_layout: current_layout.clone(),
                    current_layout: new_layout.clone(),
                    timestamp: current_time,
                    change_type: LayoutChangeType::PositionAndSize, // Default to both
                    performance_impact: PerformanceImpact::Low, // Default to low impact
                };
            
                changes.push(change.clone());
                
                // Update tracked element (now we can borrow mutably)
                if let Some(tracked_element) = self.tracked_elements.get_mut(&element_id) {
                    tracked_element.previous_layout = Some(current_layout);
                    tracked_element.current_layout = new_layout;
                    tracked_element.last_update = Instant::now();
                    tracked_element.change_count += 1;
                }
                
                // Add to history
                self.change_history.push(change);
            }
        }
        
        // Update performance monitoring
        self.performance_monitor.update()?;
        
        // Update statistics
        self.update_stats(&changes);
        
        // Clean up old history
        self.cleanup_history();
        
        Ok(changes)
    }

    /// Get layout for a specific element
    pub fn get_element_layout(&self, element: &Element) -> Result<DomRect, String> {
        Ok(element.get_bounding_client_rect())
    }

    /// Get layout changes for an element
    pub fn get_element_changes(&self, element_id: &str) -> Vec<&LayoutChange> {
        self.change_history
            .iter()
            .filter(|change| change.element_id == element_id)
            .collect()
    }

    /// Get recent layout changes
    pub fn get_recent_changes(&self, count: usize) -> Vec<&LayoutChange> {
        let start = if self.change_history.len() > count {
            self.change_history.len() - count
        } else {
            0
        };
        
        self.change_history[start..].iter().collect()
    }

    /// Get layout statistics
    pub fn get_stats(&self) -> &LayoutStats {
        &self.stats
    }

    /// Get frame rate
    pub fn get_frame_rate(&self) -> f64 {
        self.performance_monitor.get_frame_rate()
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> usize {
        self.performance_monitor.get_memory_usage()
    }

    /// Enable or disable tracking
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Check if layout has changed
    fn has_layout_changed(&self, old: &DomRect, new: &DomRect) -> bool {
        const THRESHOLD: f64 = 0.1; // 0.1px threshold for changes
        
        (old.left() - new.left()).abs() > THRESHOLD ||
        (old.top() - new.top()).abs() > THRESHOLD ||
        (old.width() - new.width()).abs() > THRESHOLD ||
        (old.height() - new.height()).abs() > THRESHOLD
    }

    /// Create a layout change record
    fn create_layout_change(
        &self,
        tracked_element: &TrackedElement,
        new_layout: &DomRect,
        timestamp: f64,
    ) -> Result<LayoutChange, String> {
        let previous_layout = tracked_element.current_layout.clone();
        
        // Determine change type
        let change_type = self.determine_change_type(&previous_layout, new_layout);
        
        // Calculate performance impact
        let performance_impact = self.calculate_performance_impact(&previous_layout, new_layout);
        
        Ok(LayoutChange {
            element_id: tracked_element.id.clone(),
            previous_layout,
            current_layout: new_layout.clone(),
            timestamp,
            change_type,
            performance_impact,
        })
    }

    /// Determine the type of layout change
    fn determine_change_type(&self, old: &DomRect, new: &DomRect) -> LayoutChangeType {
        let position_changed = (old.left() - new.left()).abs() > 0.1 || (old.top() - new.top()).abs() > 0.1;
        let size_changed = (old.width() - new.width()).abs() > 0.1 || (old.height() - new.height()).abs() > 0.1;
        
        match (position_changed, size_changed) {
            (true, true) => LayoutChangeType::PositionAndSize,
            (true, false) => LayoutChangeType::Position,
            (false, true) => LayoutChangeType::Size,
            (false, false) => LayoutChangeType::None,
        }
    }

    /// Calculate performance impact of layout change
    fn calculate_performance_impact(&self, old: &DomRect, new: &DomRect) -> PerformanceImpact {
        let area_change = (new.width() * new.height() - old.width() * old.height()).abs();
        let position_change = ((new.left() - old.left()).powi(2) + (new.top() - old.top()).powi(2)).sqrt();
        
        let total_change = area_change + position_change;
        
        match total_change {
            change if change < 100.0 => PerformanceImpact::Low,
            change if change < 1000.0 => PerformanceImpact::Medium,
            change if change < 10000.0 => PerformanceImpact::High,
            _ => PerformanceImpact::Critical,
        }
    }

    /// Update statistics
    fn update_stats(&mut self, changes: &[LayoutChange]) {
        self.stats.total_changes += changes.len();
        
        // Update changes in last second
        let current_time = js_sys::Date::now();
        let one_second_ago = current_time - 1000.0;
        
        self.stats.changes_last_second = self.change_history
            .iter()
            .filter(|change| change.timestamp >= one_second_ago)
            .count();
        
        // Update change frequency
        if self.stats.total_changes > 0 {
            let total_time = current_time / 1000.0; // Convert to seconds
            self.stats.change_frequency = self.stats.total_changes as f64 / total_time;
        }
        
        // Update impact distribution
        for change in changes {
            let count = self.stats.impact_distribution.entry(change.performance_impact.clone()).or_insert(0);
            *count += 1;
        }
        
        // Update frame rate
        self.stats.frame_rate = self.performance_monitor.get_frame_rate();
    }

    /// Clean up old history
    fn cleanup_history(&mut self) {
        const MAX_HISTORY_SIZE: usize = 1000;
        
        if self.change_history.len() > MAX_HISTORY_SIZE {
            let remove_count = self.change_history.len() - MAX_HISTORY_SIZE;
            self.change_history.drain(0..remove_count);
        }
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        Self {
            frame_times: Vec::new(),
            memory_usage: Vec::new(),
            performance_observer: None,
            last_frame_time: js_sys::Date::now(),
        }
    }

    /// Update performance monitoring
    pub fn update(&mut self) -> Result<(), String> {
        let current_time = js_sys::Date::now();
        let frame_time = current_time - self.last_frame_time;
        
        // Record frame time
        self.frame_times.push(frame_time);
        
        // Keep only recent frame times (last 60 frames)
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }
        
        // Update last frame time
        self.last_frame_time = current_time;
        
        // Update memory usage (placeholder for future implementation)
        self.memory_usage.push(0);
        if self.memory_usage.len() > 60 {
            self.memory_usage.remove(0);
        }
        
        Ok(())
    }

    /// Get current frame rate
    pub fn get_frame_rate(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 60.0; // Default frame rate
        }
        
        let total_time: f64 = self.frame_times.iter().sum();
        let frame_count = self.frame_times.len() as f64;
        
        if total_time > 0.0 {
            1000.0 / (total_time / frame_count) // Convert to FPS
        } else {
            60.0
        }
    }

    /// Get memory usage
    pub fn get_memory_usage(&self) -> usize {
        // Placeholder implementation
        // In a real implementation, this would use performance.memory or similar
        self.memory_usage.last().copied().unwrap_or(0)
    }
}

impl Default for LayoutTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LayoutStats {
    fn default() -> Self {
        Self {
            total_changes: 0,
            changes_last_second: 0,
            change_frequency: 0.0,
            impact_distribution: HashMap::new(),
            frame_rate: 60.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_layout_tracker_creation() {
        let tracker = LayoutTracker::new();
        assert_eq!(tracker.tracked_elements.len(), 0);
        assert_eq!(tracker.change_history.len(), 0);
        assert!(tracker.enabled);
    }

    #[wasm_bindgen_test]
    fn test_performance_monitor_creation() {
        let monitor = PerformanceMonitor::new();
        assert_eq!(monitor.frame_times.len(), 0);
        assert_eq!(monitor.memory_usage.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_layout_change_type_determination() {
        let tracker = LayoutTracker::new();
        
        // Create test rectangles
        let old = DomRect::new().unwrap();
        let new_position = DomRect::new().unwrap();
        let new_size = DomRect::new().unwrap();
        let new_both = DomRect::new().unwrap();
        
        // Test position change
        let change_type = tracker.determine_change_type(&old, &new_position);
        assert!(matches!(change_type, LayoutChangeType::None));
        
        // Test size change
        let change_type = tracker.determine_change_type(&old, &new_size);
        assert!(matches!(change_type, LayoutChangeType::None));
        
        // Test both position and size change
        let change_type = tracker.determine_change_type(&old, &new_both);
        assert!(matches!(change_type, LayoutChangeType::None));
        
        // Test no change
        let change_type = tracker.determine_change_type(&old, &old);
        assert!(matches!(change_type, LayoutChangeType::None));
    }

    #[wasm_bindgen_test]
    fn test_performance_impact_calculation() {
        let tracker = LayoutTracker::new();
        
        // Create test rectangles
        let old = DomRect::new().unwrap();
        let small_change = DomRect::new().unwrap();
        let large_change = DomRect::new().unwrap();
        
        // Test small change
        let impact = tracker.calculate_performance_impact(&old, &small_change);
        assert!(matches!(impact, PerformanceImpact::Low));
        
        // Test large change
        let impact = tracker.calculate_performance_impact(&old, &large_change);
        assert!(matches!(impact, PerformanceImpact::Low));
    }
}
