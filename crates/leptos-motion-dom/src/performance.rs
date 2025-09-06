//! DOM-specific performance optimizations

use std::collections::HashMap;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::Element;

/// CSS property optimization manager
pub struct CSSOptimizer {
    gpu_accelerated_properties: std::collections::HashSet<String>,
    layout_triggering_properties: std::collections::HashSet<String>,
}

impl CSSOptimizer {
    /// Create a new CSS optimizer
    pub fn new() -> Self {
        let mut gpu_accelerated = std::collections::HashSet::new();
        gpu_accelerated.insert("transform".to_string());
        gpu_accelerated.insert("opacity".to_string());
        gpu_accelerated.insert("filter".to_string());
        gpu_accelerated.insert("backdrop-filter".to_string());

        let mut layout_triggering = std::collections::HashSet::new();
        layout_triggering.insert("width".to_string());
        layout_triggering.insert("height".to_string());
        layout_triggering.insert("top".to_string());
        layout_triggering.insert("left".to_string());
        layout_triggering.insert("right".to_string());
        layout_triggering.insert("bottom".to_string());
        layout_triggering.insert("margin".to_string());
        layout_triggering.insert("padding".to_string());
        layout_triggering.insert("border".to_string());

        Self {
            gpu_accelerated_properties: gpu_accelerated,
            layout_triggering_properties: layout_triggering,
        }
    }

    /// Check if a property is GPU accelerated
    pub fn is_gpu_accelerated(&self, property: &str) -> bool {
        self.gpu_accelerated_properties.contains(property)
    }

    /// Check if a property triggers layout
    pub fn triggers_layout(&self, property: &str) -> bool {
        self.layout_triggering_properties.contains(property)
    }

    /// Optimize element for animations
    pub fn optimize_element(
        &mut self,
        element: &Element,
        properties: &[String],
    ) -> Result<(), JsValue> {
        let style = element.unchecked_ref::<web_sys::HtmlElement>().style();

        // Check if we need GPU acceleration
        let needs_gpu = properties.iter().any(|prop| self.is_gpu_accelerated(prop));

        if needs_gpu {
            // Set will-change for GPU acceleration
            let will_change = self.calculate_will_change(properties);
            if !will_change.is_empty() {
                style.set_property("will-change", &will_change)?;
            }

            // Ensure transform3d for hardware acceleration
            if properties.iter().any(|prop| prop == "transform") {
                let current_transform = style
                    .get_property_value("transform")
                    .unwrap_or_else(|_| "".to_string());

                if !current_transform.contains("translate3d")
                    && !current_transform.contains("translateZ")
                {
                    style.set_property("transform", "translate3d(0, 0, 0)")?;
                }
            }
        }

        Ok(())
    }

    /// Calculate optimal will-change value
    fn calculate_will_change(&mut self, properties: &[String]) -> String {
        let mut gpu_properties = Vec::new();

        for property in properties {
            if self.is_gpu_accelerated(property) {
                gpu_properties.push(property.clone());
            }
        }

        if gpu_properties.is_empty() {
            return String::new();
        }

        // Limit to most important properties
        let limited_properties: Vec<String> = gpu_properties
            .into_iter()
            .take(3) // Limit to 3 properties for performance
            .collect();

        limited_properties.join(", ")
    }

    /// Clean up optimizations
    pub fn cleanup(&mut self, element: &Element) -> Result<(), JsValue> {
        let style = element.unchecked_ref::<web_sys::HtmlElement>().style();

        // Remove will-change after animation
        style.remove_property("will-change")?;

        // Reset transform if it was set for hardware acceleration
        let transform = style
            .get_property_value("transform")
            .unwrap_or_else(|_| "".to_string());

        if transform == "translate3d(0, 0, 0)" {
            style.remove_property("transform")?;
        }

        Ok(())
    }
}

/// DOM batching manager for efficient updates
pub struct DOMBatcher {
    pending_updates: Vec<PendingUpdate>,
    batch_size: usize,
    max_batch_time: f64, // milliseconds
}

/// Pending DOM update
#[derive(Debug)]
struct PendingUpdate {
    element: Element,
    property: String,
    value: String,
    priority: UpdatePriority,
    timestamp: f64,
}

/// Update priority levels
/// Priority levels for DOM updates to optimize performance
///
/// Higher priority updates are processed first, allowing critical
/// animations to take precedence over less important ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum UpdatePriority {
    /// Low priority updates (e.g., background animations)
    Low = 0,
    /// Normal priority updates (e.g., standard animations)
    Normal = 1,
    /// High priority updates (e.g., user interaction animations)
    High = 2,
    /// Critical priority updates (e.g., gesture feedback)
    Critical = 3,
}

impl DOMBatcher {
    /// Create a new DOM batcher
    pub fn new(batch_size: usize, max_batch_time: f64) -> Self {
        Self {
            pending_updates: Vec::new(),
            batch_size,
            max_batch_time,
        }
    }

    /// Queue a DOM update
    pub fn queue_update(
        &mut self,
        element: Element,
        property: String,
        value: String,
        priority: UpdatePriority,
    ) {
        let timestamp = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        self.pending_updates.push(PendingUpdate {
            element,
            property,
            value,
            priority,
            timestamp,
        });
    }

    /// Process all pending updates
    pub fn flush(&mut self) -> Result<usize, JsValue> {
        if self.pending_updates.is_empty() {
            return Ok(0);
        }

        // Sort by priority (highest first)
        self.pending_updates
            .sort_by(|a, b| b.priority.cmp(&a.priority));

        let current_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let mut processed = 0;
        let mut to_remove = Vec::new();

        for (index, update) in self.pending_updates.iter().enumerate() {
            // Check batch size limit
            if processed >= self.batch_size {
                break;
            }

            // Check time limit
            if current_time - update.timestamp > self.max_batch_time {
                to_remove.push(index);
                continue;
            }

            // Apply the update
            let style = update
                .element
                .unchecked_ref::<web_sys::HtmlElement>()
                .style();
            style.set_property(&update.property, &update.value)?;

            to_remove.push(index);
            processed += 1;
        }

        // Remove processed updates (in reverse order to maintain indices)
        for &index in to_remove.iter().rev() {
            if index < self.pending_updates.len() {
                self.pending_updates.remove(index);
            }
        }

        Ok(processed)
    }

    /// Get number of pending updates
    pub fn pending_count(&self) -> usize {
        self.pending_updates.len()
    }

    /// Clear all pending updates
    pub fn clear(&mut self) {
        self.pending_updates.clear();
    }
}

/// Element cache for avoiding repeated DOM queries
pub struct ElementCache {
    cache: HashMap<String, Element>,
    max_size: usize,
}

impl ElementCache {
    /// Create a new element cache
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_size,
        }
    }

    /// Get element by selector
    pub fn get(&mut self, selector: &str) -> Option<Element> {
        if let Some(element) = self.cache.get(selector) {
            return Some(element.clone());
        }

        // Query DOM
        if let Some(element) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.query_selector(selector).ok())
            .flatten()
        {
            // Add to cache if not full
            if self.cache.len() < self.max_size {
                self.cache.insert(selector.to_string(), element.clone());
            }

            Some(element)
        } else {
            None
        }
    }

    /// Invalidate cache entry
    pub fn invalidate(&mut self, selector: &str) {
        self.cache.remove(selector);
    }

    /// Clear entire cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}

/// Performance monitoring for DOM operations
pub struct DOMPerformanceMonitor {
    operation_times: Vec<f64>,
    max_samples: usize,
    total_operations: u64,
}

impl DOMPerformanceMonitor {
    /// Create a new DOM performance monitor
    pub fn new(max_samples: usize) -> Self {
        Self {
            operation_times: Vec::with_capacity(max_samples),
            max_samples,
            total_operations: 0,
        }
    }

    /// Start timing a DOM operation
    pub fn start_operation(&self) -> f64 {
        web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0)
    }

    /// End timing a DOM operation
    pub fn end_operation(&mut self, start_time: f64) -> f64 {
        let end_time = web_sys::window()
            .and_then(|w| w.performance())
            .map(|p| p.now())
            .unwrap_or(0.0);

        let duration = end_time - start_time;

        self.operation_times.push(duration);
        if self.operation_times.len() > self.max_samples {
            self.operation_times.remove(0);
        }

        self.total_operations += 1;
        duration
    }

    /// Get average operation time
    pub fn get_avg_operation_time(&self) -> f64 {
        if self.operation_times.is_empty() {
            return 0.0;
        }

        let total: f64 = self.operation_times.iter().sum();
        total / self.operation_times.len() as f64
    }

    /// Get total operations count
    pub fn get_total_operations(&self) -> u64 {
        self.total_operations
    }

    /// Check if performance is acceptable
    pub fn is_performance_acceptable(&self) -> bool {
        let avg_time = self.get_avg_operation_time();
        avg_time < 1.0 // Less than 1ms average
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_css_optimizer() {
        let optimizer = CSSOptimizer::new();

        assert!(optimizer.is_gpu_accelerated("transform"));
        assert!(optimizer.is_gpu_accelerated("opacity"));
        assert!(!optimizer.is_gpu_accelerated("width"));

        assert!(optimizer.triggers_layout("width"));
        assert!(optimizer.triggers_layout("height"));
        assert!(!optimizer.triggers_layout("transform"));
    }

    #[wasm_bindgen_test]
    fn test_dom_batcher() {
        let mut batcher = DOMBatcher::new(10, 16.0);

        // Create a test element
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();

        // Queue some updates
        batcher.queue_update(
            element.clone(),
            "transform".to_string(),
            "translateX(100px)".to_string(),
            UpdatePriority::High,
        );

        assert_eq!(batcher.pending_count(), 1);

        // Flush updates
        let processed = batcher.flush().unwrap();
        assert_eq!(processed, 1);
        assert_eq!(batcher.pending_count(), 0);
    }

    #[wasm_bindgen_test]
    fn test_element_cache() {
        let mut cache = ElementCache::new(10);

        // Create a test element
        let document = web_sys::window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        element.set_id("test-element");
        document.body().unwrap().append_child(&element).unwrap();

        // Cache should find the element
        let cached_element = cache.get("#test-element");
        assert!(cached_element.is_some());

        // Second lookup should use cache
        let cached_element2 = cache.get("#test-element");
        assert!(cached_element2.is_some());

        // Clean up
        document.body().unwrap().remove_child(&element).unwrap();
    }

    #[wasm_bindgen_test]
    fn test_dom_performance_monitor() {
        let mut monitor = DOMPerformanceMonitor::new(10);

        let start_time = monitor.start_operation();

        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(1));

        let duration = monitor.end_operation(start_time);
        assert!(duration > 0.0);
        assert_eq!(monitor.get_total_operations(), 1);
    }
}
