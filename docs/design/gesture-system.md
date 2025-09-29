# Gesture System Design

## Overview
**Purpose**: Touch and mouse gesture recognition for interactive animations  
**Status**: Separate crate, needs integration  
**Complexity**: Medium (event handling, state machines)  
**Lines**: Target <300 lines per module

## Architecture

### Gesture Types
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum GestureType {
    Tap,
    DoubleTap,
    LongPress,
    Drag,
    Pinch,
    Rotate,
    Swipe(SwipeDirection),
    Pan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

pub trait GestureRecognizer {
    fn recognize(&self, event: &GestureEvent) -> Option<GestureType>;
    fn get_config(&self) -> &GestureConfig;
}
```

### Gesture Event Structure
```rust
#[derive(Debug, Clone)]
pub struct GestureEvent {
    /// Event type
    pub event_type: GestureEventType,

    /// Pointer position
    pub position: (f64, f64),

    /// Movement delta
    pub delta: (f64, f64),

    /// Touch points (for multi-touch)
    pub touches: Vec<TouchPoint>,

    /// Timestamp
    pub timestamp: f64,

    /// Target element
    pub target: web_sys::Element,
}

#[derive(Debug, Clone)]
pub struct TouchPoint {
    pub id: i32,
    pub position: (f64, f64),
    pub force: Option<f64>,
}
```

## Module Structure
```
gestures/
├── lib.rs              (<100 lines) - Main gesture API
├── recognizers/        (<250 lines) - Individual recognizers
│   ├── tap.rs          (<100 lines) - Tap gesture
│   ├── drag.rs         (<150 lines) - Drag gesture
│   ├── pinch.rs        (<100 lines) - Pinch gesture
│   └── swipe.rs        (<100 lines) - Swipe gesture
├── state_machine.rs    (<200 lines) - Gesture state management
├── event_handler.rs    (<150 lines) - DOM event processing
├── config.rs           (<100 lines) - Gesture configuration
└── utils.rs            (<100 lines) - Helper functions
```

## Gesture Recognizers

### Tap Recognizer
```rust
pub struct TapRecognizer {
    config: TapConfig,
    touch_start_time: Option<f64>,
    touch_start_pos: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct TapConfig {
    /// Maximum time for tap (ms)
    pub max_duration: f64,

    /// Maximum movement tolerance (px)
    pub max_distance: f64,

    /// Minimum time between taps for double tap (ms)
    pub double_tap_interval: f64,
}

impl GestureRecognizer for TapRecognizer {
    fn recognize(&self, event: &GestureEvent) -> Option<GestureType> {
        match event.event_type {
            GestureEventType::TouchStart => {
                self.touch_start_time = Some(event.timestamp);
                self.touch_start_pos = Some(event.position);
                None
            }

            GestureEventType::TouchEnd => {
                if let (Some(start_time), Some(start_pos)) = (self.touch_start_time, self.touch_start_pos) {
                    let duration = event.timestamp - start_time;
                    let distance = self.calculate_distance(start_pos, event.position);

                    if duration <= self.config.max_duration && distance <= self.config.max_distance {
                        // Check for double tap
                        if self.is_double_tap(event.timestamp) {
                            return Some(GestureType::DoubleTap);
                        } else {
                            return Some(GestureType::Tap);
                        }
                    }
                }
                None
            }

            _ => None,
        }
    }
}
```

### Drag Recognizer
```rust
pub struct DragRecognizer {
    config: DragConfig,
    is_dragging: bool,
    start_pos: Option<(f64, f64)>,
    last_pos: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct DragConfig {
    /// Minimum distance to start drag (px)
    pub min_distance: f64,

    /// Velocity threshold for fling (px/ms)
    pub velocity_threshold: f64,

    /// Drag axes
    pub axis: DragAxis,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DragAxis {
    X,
    Y,
    Both,
}

impl GestureRecognizer for DragRecognizer {
    fn recognize(&self, event: &GestureEvent) -> Option<GestureType> {
        match event.event_type {
            GestureEventType::TouchStart => {
                self.start_pos = Some(event.position);
                self.last_pos = Some(event.position);
                self.is_dragging = false;
                None
            }

            GestureEventType::TouchMove => {
                if let Some(start_pos) = self.start_pos {
                    let distance = self.calculate_distance(start_pos, event.position);

                    if distance >= self.config.min_distance {
                        self.is_dragging = true;
                        self.last_pos = Some(event.position);
                        return Some(GestureType::Drag);
                    }
                }
                None
            }

            GestureEventType::TouchEnd => {
                if self.is_dragging {
                    // Calculate velocity for potential fling
                    let velocity = self.calculate_velocity(event);
                    if velocity > self.config.velocity_threshold {
                        // Handle fling
                    }
                    self.is_dragging = false;
                }
                None
            }

            _ => None,
        }
    }
}
```

### Pinch Recognizer
```rust
pub struct PinchRecognizer {
    config: PinchConfig,
    initial_distance: Option<f64>,
    current_distance: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct PinchConfig {
    /// Minimum scale change to recognize pinch
    pub min_scale_change: f64,

    /// Pointer distance threshold (px)
    pub pointer_distance_threshold: f64,
}

impl GestureRecognizer for PinchRecognizer {
    fn recognize(&self, event: &GestureEvent) -> Option<GestureType> {
        if event.touches.len() >= 2 {
            let distance = self.calculate_touch_distance(&event.touches);

            match event.event_type {
                GestureEventType::TouchStart => {
                    self.initial_distance = Some(distance);
                    None
                }

                GestureEventType::TouchMove => {
                    if let Some(initial) = self.initial_distance {
                        let scale = distance / initial;
                        if (scale - 1.0).abs() >= self.config.min_scale_change {
                            return Some(GestureType::Pinch);
                        }
                    }
                    None
                }

                GestureEventType::TouchEnd => {
                    self.initial_distance = None;
                    None
                }

                _ => None,
            }
        } else {
            None
        }
    }
}
```

## State Machine

### Gesture State Management
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum GestureState {
    Idle,
    Recognizing,
    Recognized(GestureType),
    Failed,
}

pub struct GestureStateMachine {
    recognizers: Vec<Box<dyn GestureRecognizer>>,
    current_state: GestureState,
    active_gesture: Option<GestureType>,
}

impl GestureStateMachine {
    pub fn new() -> Self {
        Self {
            recognizers: vec![
                Box::new(TapRecognizer::new()),
                Box::new(DragRecognizer::new()),
                Box::new(PinchRecognizer::new()),
                Box::new(SwipeRecognizer::new()),
            ],
            current_state: GestureState::Idle,
            active_gesture: None,
        }
    }

    pub fn process_event(&mut self, event: GestureEvent) -> Vec<GestureResult> {
        let mut results = Vec::new();

        for recognizer in &mut self.recognizers {
            if let Some(gesture_type) = recognizer.recognize(&event) {
                match self.current_state {
                    GestureState::Idle => {
                        self.current_state = GestureState::Recognized(gesture_type.clone());
                        self.active_gesture = Some(gesture_type.clone());
                        results.push(GestureResult::Started(gesture_type));
                    }

                    GestureState::Recognized(ref current_gesture) => {
                        if *current_gesture == gesture_type {
                            results.push(GestureResult::Updated(gesture_type));
                        }
                    }

                    _ => {}
                }
            }
        }

        results
    }
}
```

## Event Processing

### DOM Event Handler
```rust
pub struct GestureEventHandler {
    element: web_sys::Element,
    state_machine: GestureStateMachine,
    callbacks: HashMap<GestureType, Vec<Box<dyn Fn(GestureEvent)>>>,
}

impl GestureEventHandler {
    pub fn setup_event_listeners(&mut self) {
        // Touch events
        self.add_touch_listener("touchstart", |event| {
            self.handle_touch_start(event);
        });

        self.add_touch_listener("touchmove", |event| {
            self.handle_touch_move(event);
        });

        self.add_touch_listener("touchend", |event| {
            self.handle_touch_end(event);
        });

        // Mouse events (for desktop)
        self.add_mouse_listener("mousedown", |event| {
            self.handle_mouse_down(event);
        });

        self.add_mouse_listener("mousemove", |event| {
            self.handle_mouse_move(event);
        });

        self.add_mouse_listener("mouseup", |event| {
            self.handle_mouse_up(event);
        });
    }

    fn handle_touch_start(&mut self, event: web_sys::TouchEvent) {
        let gesture_event = self.create_gesture_event(&event, GestureEventType::TouchStart);
        let results = self.state_machine.process_event(gesture_event);

        for result in results {
            self.trigger_callbacks(result);
        }
    }
}
```

## Configuration

### Gesture Configuration
```rust
#[derive(Debug, Clone)]
pub struct GestureConfig {
    /// Enable multi-touch gestures
    pub enable_multi_touch: bool,

    /// Maximum number of concurrent gestures
    pub max_concurrent_gestures: usize,

    /// Gesture recognition timeout (ms)
    pub recognition_timeout: f64,

    /// Require gesture to be claimed
    pub require_claim: bool,
}

impl Default for GestureConfig {
    fn default() -> Self {
        Self {
            enable_multi_touch: true,
            max_concurrent_gestures: 1,
            recognition_timeout: 500.0,
            require_claim: false,
        }
    }
}
```

## Integration with Animation

### Gesture-Driven Animation
```rust
pub struct GestureAnimator {
    gesture_handler: GestureEventHandler,
    animation_engine: Box<dyn AnimationEngine>,
}

impl GestureAnimator {
    pub fn new(element: web_sys::Element) -> Self {
        let mut gesture_handler = GestureEventHandler::new(element.clone());
        let animation_engine = Box::new(HybridAnimationEngine::new());

        // Setup gesture callbacks
        gesture_handler.on_gesture(GestureType::Drag, |event| {
            self.handle_drag_gesture(event);
        });

        gesture_handler.on_gesture(GestureType::Pinch, |event| {
            self.handle_pinch_gesture(event);
        });

        Self {
            gesture_handler,
            animation_engine,
        }
    }

    fn handle_drag_gesture(&mut self, event: GestureEvent) {
        let transform = format!("translate({}px, {}px)", event.delta.0, event.delta.1);

        self.animation_engine.animate_property(
            &self.gesture_handler.element,
            "transform",
            AnimationValue::String(transform),
            Transition::default(),
        );
    }

    fn handle_pinch_gesture(&mut self, event: GestureEvent) {
        if event.touches.len() >= 2 {
            let scale = self.calculate_pinch_scale(&event.touches);
            let transform = format!("scale({})", scale);

            self.animation_engine.animate_property(
                &self.gesture_handler.element,
                "transform",
                AnimationValue::String(transform),
                Transition::default(),
            );
        }
    }
}
```

## Testing Strategy

### Gesture Simulation
```rust
#[cfg(test)]
pub struct GestureSimulator {
    element: web_sys::Element,
    event_handler: GestureEventHandler,
}

#[cfg(test)]
impl GestureSimulator {
    pub fn simulate_tap(&mut self, x: f64, y: f64) {
        // Simulate touchstart
        let start_event = self.create_touch_event("touchstart", x, y);
        self.event_handler.handle_touch_start(start_event);

        // Simulate touchend
        let end_event = self.create_touch_event("touchend", x, y);
        self.event_handler.handle_touch_end(end_event);
    }

    pub fn simulate_drag(&mut self, start: (f64, f64), end: (f64, f64)) {
        // Simulate drag sequence
        let start_event = self.create_touch_event("touchstart", start.0, start.1);
        self.event_handler.handle_touch_start(start_event);

        // Simulate move events
        let steps = 10;
        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let x = start.0 + (end.0 - start.0) * t;
            let y = start.1 + (end.1 - start.1) * t;

            let move_event = self.create_touch_event("touchmove", x, y);
            self.event_handler.handle_touch_move(move_event);
        }

        let end_event = self.create_touch_event("touchend", end.0, end.1);
        self.event_handler.handle_touch_end(end_event);
    }
}
```

This design provides a comprehensive gesture recognition system that integrates smoothly with the animation engine while maintaining clean module boundaries and staying under 300 lines per module.
