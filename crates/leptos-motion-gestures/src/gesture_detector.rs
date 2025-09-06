//! Main gesture detector that coordinates all gesture types

use crate::drag::DragGesture;
use crate::hover::HoverGesture;
use crate::tap::TapGesture;
use crate::{
    GestureConfig, GestureEvent, GestureHandler, GestureResult, MultiTouchGestureDetector,
    MultiTouchGestureType, MultiTouchState, TouchPoint,
};
use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use web_sys::{Element, MouseEvent, PointerEvent, TouchEvent};

/// Main gesture detector that coordinates all gesture types
pub struct GestureDetector {
    /// Configuration for all gestures
    config: GestureConfig,
    /// Multi-touch gesture detector
    multi_touch: MultiTouchGestureDetector,
    /// Drag gesture detector
    drag: DragGesture,
    /// Hover gesture detector
    hover: HoverGesture,
    /// Tap gesture detector
    tap: TapGesture,
    /// Element this detector is attached to
    element: Option<Element>,
    /// Active gesture callbacks
    callbacks: HashMap<String, Box<dyn Fn(GestureResult) + Send + Sync>>,
    /// Current gesture state
    current_gesture: Option<MultiTouchState>,
    /// Gesture event history
    event_history: Vec<GestureEvent>,
}

impl GestureDetector {
    /// Create a new gesture detector
    pub fn new(config: GestureConfig) -> Self {
        Self {
            config: config.clone(),
            multi_touch: MultiTouchGestureDetector::new(config),
            drag: DragGesture::new(),
            hover: HoverGesture::new(),
            tap: TapGesture::new(),
            element: None,
            callbacks: HashMap::new(),
            current_gesture: None,
            event_history: Vec::new(),
        }
    }

    /// Create with default configuration
    pub fn default() -> Self {
        Self::new(GestureConfig::default())
    }

    /// Attach the detector to a DOM element
    pub fn attach(&mut self, element: Element) -> Result<(), JsValue> {
        self.element = Some(element.clone());

        // Set up event listeners
        self.setup_touch_events(&element)?;
        self.setup_mouse_events(&element)?;
        self.setup_pointer_events(&element)?;

        Ok(())
    }

    /// Detach the detector from the current element
    pub fn detach(&mut self) -> Result<(), JsValue> {
        if let Some(element) = &self.element {
            // Remove event listeners
            self.remove_touch_events(element)?;
            self.remove_mouse_events(element)?;
            self.remove_pointer_events(element)?;
        }

        self.element = None;
        Ok(())
    }

    /// Register a callback for a specific gesture type
    pub fn on_gesture<F>(&mut self, gesture_type: &str, callback: F)
    where
        F: Fn(GestureResult) + Send + Sync + 'static,
    {
        self.callbacks
            .insert(gesture_type.to_string(), Box::new(callback));
    }

    /// Register a callback for multi-touch gestures
    pub fn on_multi_touch<F>(&mut self, callback: F)
    where
        F: Fn(MultiTouchState) + Send + Sync + 'static,
    {
        let wrapped_callback = move |result: GestureResult| {
            if let Some(data) = result.data {
                if let Ok(multi_touch_state) =
                    serde_json::from_value::<MultiTouchState>(serde_json::to_value(data).unwrap())
                {
                    callback(multi_touch_state);
                }
            }
        };

        self.callbacks
            .insert("multi_touch".to_string(), Box::new(wrapped_callback));
    }

    /// Register a callback for pinch gestures
    pub fn on_pinch<F>(&mut self, callback: F)
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        let wrapped_callback = move |result: GestureResult| {
            if let Some(data) = result.data {
                if let Ok(multi_touch_state) =
                    serde_json::from_value::<MultiTouchState>(serde_json::to_value(data).unwrap())
                {
                    if multi_touch_state.gesture_type == MultiTouchGestureType::Pinch
                        || multi_touch_state.gesture_type == MultiTouchGestureType::PinchAndRotate
                    {
                        callback(multi_touch_state.scale);
                    }
                }
            }
        };

        self.callbacks
            .insert("pinch".to_string(), Box::new(wrapped_callback));
    }

    /// Register a callback for rotation gestures
    pub fn on_rotation<F>(&mut self, callback: F)
    where
        F: Fn(f64) + Send + Sync + 'static,
    {
        let wrapped_callback = move |result: GestureResult| {
            if let Some(data) = result.data {
                if let Ok(multi_touch_state) =
                    serde_json::from_value::<MultiTouchState>(serde_json::to_value(data).unwrap())
                {
                    if multi_touch_state.gesture_type == MultiTouchGestureType::Rotation
                        || multi_touch_state.gesture_type == MultiTouchGestureType::PinchAndRotate
                    {
                        callback(multi_touch_state.rotation);
                    }
                }
            }
        };

        self.callbacks
            .insert("rotation".to_string(), Box::new(wrapped_callback));
    }

    /// Update the gesture configuration
    pub fn update_config(&mut self, config: GestureConfig) {
        self.config = config.clone();
        self.multi_touch.update_config(config);
    }

    /// Get the current multi-touch state
    pub fn get_multi_touch_state(&self) -> Option<&MultiTouchState> {
        self.current_gesture.as_ref()
    }

    /// Check if a specific gesture type is currently active
    pub fn is_gesture_active(&self, gesture_type: MultiTouchGestureType) -> bool {
        if let Some(state) = &self.current_gesture {
            state.active && state.gesture_type == gesture_type
        } else {
            false
        }
    }

    /// Set up touch event listeners
    fn setup_touch_events(&self, element: &Element) -> Result<(), JsValue> {
        let touchstart_callback = Closure::wrap(Box::new(move |event: TouchEvent| {
            let touches = Self::extract_touch_points(&event);
            let _gesture_event = GestureEvent::TouchStart { touches };
            // Process the gesture event (would need reference to self)
            log::info!("Touch start: {} touches", event.touches().length());
        }) as Box<dyn FnMut(TouchEvent)>);

        let touchmove_callback = Closure::wrap(Box::new(move |event: TouchEvent| {
            let touches = Self::extract_touch_points(&event);
            let _gesture_event = GestureEvent::TouchMove { touches };
            // Process the gesture event (would need reference to self)
            log::info!("Touch move: {} touches", event.touches().length());
        }) as Box<dyn FnMut(TouchEvent)>);

        let touchend_callback = Closure::wrap(Box::new(move |event: TouchEvent| {
            let touches = Self::extract_touch_points(&event);
            let _gesture_event = GestureEvent::TouchEnd { touches };
            // Process the gesture event (would need reference to self)
            log::info!("Touch end: {} touches", event.touches().length());
        }) as Box<dyn FnMut(TouchEvent)>);

        element.add_event_listener_with_callback(
            "touchstart",
            touchstart_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "touchmove",
            touchmove_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "touchend",
            touchend_callback.as_ref().unchecked_ref(),
        )?;

        // Store callbacks to prevent them from being dropped
        touchstart_callback.forget();
        touchmove_callback.forget();
        touchend_callback.forget();

        Ok(())
    }

    /// Set up mouse event listeners
    fn setup_mouse_events(&self, element: &Element) -> Result<(), JsValue> {
        let mousedown_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            // Handle mouse down
        }) as Box<dyn FnMut(MouseEvent)>);

        let mousemove_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            // Handle mouse move
        }) as Box<dyn FnMut(MouseEvent)>);

        let mouseup_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            // Handle mouse up
        }) as Box<dyn FnMut(MouseEvent)>);

        let mouseenter_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            // Handle mouse enter
        }) as Box<dyn FnMut(MouseEvent)>);

        let mouseleave_callback = Closure::wrap(Box::new(move |_event: MouseEvent| {
            // Handle mouse leave
        }) as Box<dyn FnMut(MouseEvent)>);

        element.add_event_listener_with_callback(
            "mousedown",
            mousedown_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "mousemove",
            mousemove_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "mouseup",
            mouseup_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "mouseenter",
            mouseenter_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "mouseleave",
            mouseleave_callback.as_ref().unchecked_ref(),
        )?;

        // Store callbacks
        mousedown_callback.forget();
        mousemove_callback.forget();
        mouseup_callback.forget();
        mouseenter_callback.forget();
        mouseleave_callback.forget();

        Ok(())
    }

    /// Set up pointer event listeners
    fn setup_pointer_events(&self, element: &Element) -> Result<(), JsValue> {
        let pointerdown_callback = Closure::wrap(Box::new(move |_event: PointerEvent| {
            // Handle pointer down
        }) as Box<dyn FnMut(PointerEvent)>);

        let pointermove_callback = Closure::wrap(Box::new(move |_event: PointerEvent| {
            // Handle pointer move
        }) as Box<dyn FnMut(PointerEvent)>);

        let pointerup_callback = Closure::wrap(Box::new(move |_event: PointerEvent| {
            // Handle pointer up
        }) as Box<dyn FnMut(PointerEvent)>);

        element.add_event_listener_with_callback(
            "pointerdown",
            pointerdown_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "pointermove",
            pointermove_callback.as_ref().unchecked_ref(),
        )?;
        element.add_event_listener_with_callback(
            "pointerup",
            pointerup_callback.as_ref().unchecked_ref(),
        )?;

        // Store callbacks
        pointerdown_callback.forget();
        pointermove_callback.forget();
        pointerup_callback.forget();

        Ok(())
    }

    /// Remove touch event listeners
    fn remove_touch_events(&self, _element: &Element) -> Result<(), JsValue> {
        // Implementation would remove event listeners
        Ok(())
    }

    /// Remove mouse event listeners
    fn remove_mouse_events(&self, _element: &Element) -> Result<(), JsValue> {
        // Implementation would remove event listeners
        Ok(())
    }

    /// Remove pointer event listeners
    fn remove_pointer_events(&self, _element: &Element) -> Result<(), JsValue> {
        // Implementation would remove event listeners
        Ok(())
    }

    /// Process a gesture event and trigger appropriate callbacks
    fn process_gesture_event(&mut self, event: GestureEvent) {
        // Add to history
        self.event_history.push(event.clone());

        // Keep only last 100 events
        if self.event_history.len() > 100 {
            self.event_history.remove(0);
        }

        // Process with multi-touch detector
        let result = self.multi_touch.handle_gesture(event.clone());

        // Update current gesture state
        if let Some(data) = &result.data {
            if let Ok(multi_touch_state) =
                serde_json::from_value::<MultiTouchState>(serde_json::to_value(data).unwrap())
            {
                self.current_gesture = Some(multi_touch_state);
            }
        }

        // Trigger callbacks based on gesture type
        match result.gesture_type {
            MultiTouchGestureType::Pinch => {
                if let Some(callback) = self.callbacks.get("pinch") {
                    callback(result.clone());
                }
            }
            MultiTouchGestureType::Rotation => {
                if let Some(callback) = self.callbacks.get("rotation") {
                    callback(result.clone());
                }
            }
            MultiTouchGestureType::PinchAndRotate => {
                if let Some(callback) = self.callbacks.get("pinch") {
                    callback(result.clone());
                }
                if let Some(callback) = self.callbacks.get("rotation") {
                    callback(result.clone());
                }
            }
            _ => {}
        }

        // Always trigger multi-touch callback if registered
        if let Some(callback) = self.callbacks.get("multi_touch") {
            callback(result.clone());
        }

        // Trigger general gesture callback
        if let Some(callback) = self.callbacks.get("gesture") {
            callback(result);
        }
    }

    /// Get gesture event history
    pub fn get_event_history(&self) -> &[GestureEvent] {
        &self.event_history
    }

    /// Clear event history
    pub fn clear_history(&mut self) {
        self.event_history.clear();
    }

    /// Extract touch points from TouchEvent
    fn extract_touch_points(event: &TouchEvent) -> Vec<TouchPoint> {
        let mut touches = Vec::new();
        let touch_list = event.touches();

        for i in 0..touch_list.length() {
            if let Some(touch) = touch_list.get(i) {
                touches.push(TouchPoint {
                    id: touch.identifier() as u64,
                    x: touch.client_x() as f64,
                    y: touch.client_y() as f64,
                    pressure: touch.force() as f64,
                    timestamp: js_sys::Date::now() as u64,
                });
            }
        }

        touches
    }
}

impl GestureHandler for GestureDetector {
    fn handle_gesture(&mut self, event: GestureEvent) -> GestureResult {
        self.process_gesture_event(event.clone());

        // Delegate to multi-touch detector for the actual result
        self.multi_touch.handle_gesture(event)
    }

    fn is_active(&self) -> bool {
        self.multi_touch.is_active()
    }

    fn reset(&mut self) {
        self.multi_touch.reset();
        self.current_gesture = None;
        self.event_history.clear();
    }
}

impl Default for GestureDetector {
    fn default() -> Self {
        Self::default()
    }
}

/// Builder pattern for creating gesture detectors
pub struct GestureDetectorBuilder {
    config: GestureConfig,
}

impl GestureDetectorBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            config: GestureConfig::default(),
        }
    }

    /// Enable basic gestures only
    pub fn basic_only(mut self) -> Self {
        self.config = self.config.basic_only();
        self
    }

    /// Enable multi-touch gestures
    pub fn enable_multi_touch(mut self) -> Self {
        self.config = self.config.enable_multi_touch();
        self
    }

    /// Enable pinch-to-zoom
    pub fn enable_pinch_to_zoom(mut self) -> Self {
        self.config = self.config.enable_pinch_to_zoom();
        self
    }

    /// Enable rotation gestures
    pub fn enable_rotation(mut self) -> Self {
        self.config = self.config.enable_rotation();
        self
    }

    /// Set gesture sensitivity
    pub fn sensitivity(mut self, sensitivity: f64) -> Self {
        self.config = self.config.sensitivity(sensitivity);
        self
    }

    /// Set minimum distance threshold
    pub fn min_distance(mut self, distance: f64) -> Self {
        self.config = self.config.min_distance(distance);
        self
    }

    /// Set maximum number of touches
    pub fn max_touches(mut self, max: usize) -> Self {
        self.config = self.config.max_touches(max);
        self
    }

    /// Set gesture timeout
    pub fn timeout(mut self, timeout_ms: u64) -> Self {
        self.config = self.config.timeout(timeout_ms);
        self
    }

    /// Build the gesture detector
    pub fn build(self) -> GestureDetector {
        GestureDetector::new(self.config)
    }
}

impl Default for GestureDetectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TouchPoint;

    fn create_touch_point(id: u64, x: f64, y: f64) -> TouchPoint {
        TouchPoint {
            id,
            x,
            y,
            pressure: 1.0,
            timestamp: 0,
        }
    }

    #[test]
    fn test_gesture_detector_creation() {
        let detector = GestureDetector::default();
        assert!(!detector.is_active());
        assert!(detector.get_multi_touch_state().is_none());
    }

    #[test]
    fn test_gesture_detector_builder() {
        let detector = GestureDetectorBuilder::new()
            .enable_pinch_to_zoom()
            .enable_rotation()
            .sensitivity(0.8)
            .min_distance(20.0)
            .max_touches(3)
            .timeout(500)
            .build();

        assert!(!detector.is_active());
        assert!(detector.get_multi_touch_state().is_none());
    }

    #[test]
    fn test_gesture_event_processing() {
        let mut detector = GestureDetector::default();

        // Test with a simple touch event
        let touches = vec![
            create_touch_point(1, 100.0, 100.0),
            create_touch_point(2, 200.0, 200.0),
        ];

        let event = GestureEvent::TouchStart { touches };
        let result = detector.handle_gesture(event);

        // Should recognize the gesture
        assert!(result.recognized);
    }
}
