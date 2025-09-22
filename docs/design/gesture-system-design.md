# Gesture System Component Design

## Overview
Memory-safe gesture detection system with proper event listener lifecycle management and multi-touch support.

## Current Issues
- **CRITICAL**: Memory leaks - event listeners never removed
- `remove_*` methods are no-ops
- `Closure::forget()` creates permanent memory leaks
- No cleanup on component unmount

## Design Goals
- Zero memory leaks with proper listener cleanup
- Multi-touch gesture recognition
- Performant event handling
- Proper Rust/WASM memory management
- Extensible gesture recognition system

## API Design

### Core Types
```rust
pub struct GestureDetector {
    element: HtmlElement,
    listeners: HashMap<GestureType, EventListener>,
    gesture_recognizers: Vec<Box<dyn GestureRecognizer>>,
    active_touches: HashMap<i32, TouchInfo>,
    state: GestureState,
}

pub struct EventListener {
    event_type: String,
    closure: Closure<dyn FnMut(web_sys::Event)>,
    options: AddEventListenerOptions,
}

pub trait GestureRecognizer {
    fn recognize(&mut self, events: &[GestureEvent]) -> Option<Gesture>;
    fn reset(&mut self);
}

pub enum Gesture {
    Tap { point: Point2D, timestamp: f64 },
    Pan { delta: Vector2D, velocity: Vector2D },
    Pinch { scale: f64, center: Point2D },
    Rotate { angle: f64, center: Point2D },
}
```

### Public Interface
```rust
impl GestureDetector {
    pub fn new(element: HtmlElement) -> Self
    pub fn add_recognizer(&mut self, recognizer: Box<dyn GestureRecognizer>)
    pub fn start_listening(&mut self) -> Result<()>
    pub fn stop_listening(&mut self) -> Result<()>
    pub fn on_gesture<F>(&mut self, callback: F) where F: Fn(Gesture) + 'static
}

impl Drop for GestureDetector {
    fn drop(&mut self) {
        let _ = self.stop_listening(); // Ensure cleanup on drop
    }
}
```

## Implementation Plan

### Phase 1: Event Listener Management (Week 2, Day 1-2)
**File**: `crates/leptos-motion-gestures/src/event_listener_manager.rs`
**Target Lines**: <150

```rust
pub struct EventListenerManager {
    element: HtmlElement,
    active_listeners: HashMap<String, StoredListener>,
}

struct StoredListener {
    closure: Rc<RefCell<Closure<dyn FnMut(web_sys::Event)>>>,
    options: AddEventListenerOptions,
}

impl EventListenerManager {
    pub fn add_listener<F>(&mut self, event_type: &str, callback: F) -> Result<()>
    where F: FnMut(web_sys::Event) + 'static
    
    pub fn remove_listener(&mut self, event_type: &str) -> Result<()> {
        if let Some(listener) = self.active_listeners.remove(event_type) {
            let closure_ref = listener.closure.borrow();
            self.element.remove_event_listener_with_callback_and_event_listener_options(
                event_type,
                closure_ref.as_ref().unchecked_ref(),
                &listener.options,
            )?;
        }
        Ok(())
    }
    
    pub fn remove_all_listeners(&mut self) -> Result<()> {
        let event_types: Vec<_> = self.active_listeners.keys().cloned().collect();
        for event_type in event_types {
            self.remove_listener(&event_type)?;
        }
        Ok(())
    }
}

impl Drop for EventListenerManager {
    fn drop(&mut self) {
        let _ = self.remove_all_listeners();
    }
}
```

### Phase 2: Touch Tracking (Week 2, Day 3)
**File**: `crates/leptos-motion-gestures/src/touch_tracker.rs`
**Target Lines**: <120

```rust
pub struct TouchTracker {
    active_touches: HashMap<i32, TouchInfo>,
    touch_history: VecDeque<TouchSnapshot>,
}

#[derive(Clone)]
pub struct TouchInfo {
    id: i32,
    start_position: Point2D,
    current_position: Point2D,
    last_position: Point2D,
    start_time: f64,
    last_update: f64,
}

impl TouchTracker {
    pub fn handle_touch_start(&mut self, touch_event: &web_sys::TouchEvent)
    pub fn handle_touch_move(&mut self, touch_event: &web_sys::TouchEvent) 
    pub fn handle_touch_end(&mut self, touch_event: &web_sys::TouchEvent)
    pub fn get_active_touches(&self) -> &HashMap<i32, TouchInfo>
    pub fn calculate_velocity(&self, touch_id: i32) -> Option<Vector2D>
}
```

### Phase 3: Gesture Recognizers (Week 2, Day 4-5)
**File**: `crates/leptos-motion-gestures/src/recognizers/mod.rs`
**Target Lines**: <200 (split into multiple files)

```rust
// Pan gesture recognizer
pub struct PanRecognizer {
    threshold: f64,
    active_touch: Option<i32>,
}

impl GestureRecognizer for PanRecognizer {
    fn recognize(&mut self, events: &[GestureEvent]) -> Option<Gesture> {
        // Recognize pan gestures from touch/mouse movement
    }
}

// Pinch gesture recognizer  
pub struct PinchRecognizer {
    min_distance: f64,
    initial_distance: Option<f64>,
}

// Tap gesture recognizer
pub struct TapRecognizer {
    max_duration: f64,
    max_distance: f64,
}
```

## File Structure
```
crates/leptos-motion-gestures/src/
├── lib.rs                        # Main gesture detector (<150 lines)
├── event_listener_manager.rs     # Event lifecycle (<150 lines)
├── touch_tracker.rs              # Touch state tracking (<120 lines)
├── recognizers/
│   ├── mod.rs                    # Recognizer trait (<50 lines)
│   ├── pan.rs                    # Pan gesture (<100 lines)
│   ├── pinch.rs                  # Pinch gesture (<100 lines)
│   ├── tap.rs                    # Tap gesture (<80 lines)
│   └── rotate.rs                 # Rotation gesture (<100 lines)
└── gesture_event.rs              # Event types (<80 lines)
```

## Memory Management Strategy

### Proper Closure Storage
```rust
use std::rc::Rc;
use std::cell::RefCell;

// Store closures properly instead of using forget()
pub struct GestureDetector {
    // Store closures to prevent them from being dropped
    _touch_start_closure: Option<Rc<RefCell<Closure<dyn FnMut(web_sys::Event)>>>>,
    _touch_move_closure: Option<Rc<RefCell<Closure<dyn FnMut(web_sys::Event)>>>>,
    _touch_end_closure: Option<Rc<RefCell<Closure<dyn FnMut(web_sys::Event)>>>>,
}
```

### Cleanup Implementation
```rust
impl GestureDetector {
    pub fn cleanup(&mut self) -> Result<(), JsValue> {
        // Remove all event listeners
        self.remove_touch_events()?;
        self.remove_mouse_events()?;
        self.remove_pointer_events()?;
        
        // Clear stored closures
        self._touch_start_closure = None;
        self._touch_move_closure = None;
        self._touch_end_closure = None;
        
        // Clear gesture state
        self.active_touches.clear();
        self.gesture_recognizers.clear();
        
        Ok(())
    }
}
```

## Testing Strategy
- Memory leak detection tests
- Multi-touch gesture recognition
- Event listener cleanup verification
- Performance tests with many simultaneous gestures
- Cross-platform compatibility (desktop/mobile)

## Performance Requirements
- Event processing: <1ms per event
- Gesture recognition: <2ms per frame
- Memory cleanup: Complete within 100ms
- No memory growth over time

## Browser Compatibility
- Touch Events: All modern mobile browsers
- Pointer Events: Modern desktop browsers
- Mouse Events: Universal fallback

## Dependencies
```rust
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use web_sys::{
    HtmlElement, TouchEvent, MouseEvent, PointerEvent,
    Touch, TouchList, AddEventListenerOptions
};
use wasm_bindgen::{prelude::*, JsCast};
```

## Success Criteria
- [ ] Zero memory leaks in long-running tests
- [ ] All event listeners properly removed
- [ ] Multi-touch gestures work correctly
- [ ] Pan, pinch, tap, rotate all functional
- [ ] Drop trait cleanup works
- [ ] All files under 200 lines
- [ ] Performance requirements met
