# Animation Engine Design

## Overview
**Purpose**: High-performance animation system with WAAPI/RAF hybrid fallback  
**Status**: Core engine, mostly functional  
**Complexity**: High (performance-critical, multi-threaded)  
**Lines**: Target <300 lines per module

## Architecture

### Engine Types
```rust
#[derive(Debug, Clone)]
pub enum AnimationEngineType {
    /// Web Animations API (preferred)
    Waapi,
    /// RequestAnimationFrame fallback
    Raf,
    /// Hybrid system (automatic fallback)
    Hybrid,
}

pub trait AnimationEngine {
    fn animate_property(
        &mut self,
        element: &web_sys::Element,
        property: &str,
        value: AnimationValue,
        transition: Transition,
    ) -> Result<AnimationHandle, AnimationError>;

    fn cancel_animation(&mut self, handle: AnimationHandle) -> Result<(), AnimationError>;

    fn get_animation_state(&self, handle: AnimationHandle) -> Option<PlaybackState>;

    fn set_global_config(&mut self, config: EngineConfig);
}
```

### Hybrid Engine Implementation
```rust
pub struct HybridAnimationEngine {
    waapi_engine: WaapiEngine,
    raf_engine: RafEngine,
    config: EngineConfig,
    feature_detector: FeatureDetector,
}

impl HybridAnimationEngine {
    pub fn new() -> Self {
        Self {
            waapi_engine: WaapiEngine::new(),
            raf_engine: RafEngine::new(),
            config: EngineConfig::default(),
            feature_detector: FeatureDetector::new(),
        }
    }
}

impl AnimationEngine for HybridAnimationEngine {
    fn animate_property(
        &mut self,
        element: &web_sys::Element,
        property: &str,
        value: AnimationValue,
        transition: Transition,
    ) -> Result<AnimationHandle, AnimationError> {
        // Try WAAPI first
        if self.feature_detector.waapi_supported() {
            match self.waapi_engine.animate_property(element, property, value, transition) {
                Ok(handle) => return Ok(handle),
                Err(AnimationError::EngineUnavailable(_)) => {
                    // Fall back to RAF
                    return self.raf_engine.animate_property(element, property, value, transition);
                }
                Err(e) => return Err(e),
            }
        }

        // Use RAF as fallback
        self.raf_engine.animate_property(element, property, value, transition)
    }
}
```

## Module Structure
```
engine/
├── lib.rs              (<100 lines) - Main engine traits
├── hybrid.rs           (<200 lines) - Hybrid engine implementation
├── waapi.rs            (<250 lines) - Web Animations API engine
├── raf.rs              (<200 lines) - RAF fallback engine
├── feature_detector.rs (<150 lines) - Browser capability detection
├── state.rs            (<150 lines) - Animation state management
├── config.rs           (<100 lines) - Engine configuration
└── performance.rs      (<150 lines) - Performance monitoring
```

## WAAPI Engine

### Web Animations API Implementation
```rust
pub struct WaapiEngine {
    animations: HashMap<AnimationHandle, web_sys::Animation>,
    config: WaapiConfig,
}

impl WaapiEngine {
    pub fn animate_property(
        &mut self,
        element: &web_sys::Element,
        property: &str,
        value: AnimationValue,
        transition: Transition,
    ) -> Result<AnimationHandle, AnimationError> {
        let keyframes = self.create_keyframes(property, value)?;
        let timing = self.create_timing(transition)?;

        let animation = element.animate_with_keyframes_and_options(
            &keyframes,
            &timing
        ).map_err(|_| AnimationError::EngineUnavailable("WAAPI failed".to_string()))?;

        let handle = AnimationHandle::new();
        self.animations.insert(handle, animation);

        Ok(handle)
    }

    fn create_keyframes(&self, property: &str, value: AnimationValue) -> Result<js_sys::Object, AnimationError> {
        let keyframes = js_sys::Object::new();
        let start_value = self.get_current_value(element, property)?;
        let end_value = self.convert_animation_value(value)?;

        // Set keyframes
        js_sys::Reflect::set(&keyframes, &property.into(), &start_value)?;
        js_sys::Reflect::set(&keyframes, &property.into(), &end_value)?;

        Ok(keyframes)
    }

    fn create_timing(&self, transition: Transition) -> Result<web_sys::KeyframeAnimationOptions, AnimationError> {
        let timing = web_sys::KeyframeAnimationOptions::new();

        if let Some(duration) = transition.duration {
            timing.duration(duration * 1000.0); // Convert to milliseconds
        }

        match &transition.ease {
            Easing::EaseIn => timing.easing("ease-in"),
            Easing::EaseOut => timing.easing("ease-out"),
            Easing::EaseInOut => timing.easing("ease-in-out"),
            Easing::CubicBezier(x1, y1, x2, y2) => {
                timing.easing(&format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2));
            }
            _ => timing.easing("ease"),
        }

        if let Some(delay) = transition.delay {
            timing.delay(delay * 1000.0);
        }

        Ok(timing)
    }
}
```

## RAF Engine

### RequestAnimationFrame Implementation
```rust
pub struct RafEngine {
    animations: HashMap<AnimationHandle, RafAnimation>,
    start_time: f64,
}

struct RafAnimation {
    element: web_sys::Element,
    property: String,
    start_value: f64,
    end_value: f64,
    duration: f64,
    easing: EasingFunction,
    start_time: f64,
    callback_id: Option<i32>,
}

impl RafEngine {
    pub fn animate_property(
        &mut self,
        element: &web_sys::Element,
        property: &str,
        value: AnimationValue,
        transition: Transition,
    ) -> Result<AnimationHandle, AnimationError> {
        let handle = AnimationHandle::new();
        let start_value = self.get_current_value(element, property)?;
        let end_value = self.convert_to_number(value)?;
        let duration = transition.duration.unwrap_or(0.3);

        let animation = RafAnimation {
            element: element.clone(),
            property: property.to_string(),
            start_value,
            end_value,
            duration,
            easing: self.create_easing_function(&transition.ease),
            start_time: self.get_current_time(),
            callback_id: None,
        };

        self.start_animation(handle, animation);
        Ok(handle)
    }

    fn start_animation(&mut self, handle: AnimationHandle, mut animation: RafAnimation) {
        let handle_clone = handle;
        let engine_clone = self.clone();

        let callback = Closure::wrap(Box::new(move |timestamp: f64| {
            engine_clone.update_animation(handle_clone, timestamp);
        }) as Box<dyn FnMut(f64)>);

        let callback_id = web_sys::window()
            .unwrap()
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .unwrap();

        animation.callback_id = Some(callback_id);
        callback.forget(); // Leak to keep alive

        self.animations.insert(handle, animation);
    }

    fn update_animation(&mut self, handle: AnimationHandle, timestamp: f64) {
        if let Some(animation) = self.animations.get_mut(&handle) {
            let elapsed = timestamp - animation.start_time;
            let progress = (elapsed / (animation.duration * 1000.0)).min(1.0);

            let eased_progress = (animation.easing)(progress);
            let current_value = animation.start_value +
                (animation.end_value - animation.start_value) * eased_progress;

            self.apply_value(&animation.element, &animation.property, current_value);

            if progress < 1.0 {
                // Continue animation
                let handle_clone = handle;
                let engine_clone = self.clone();
                let callback = Closure::wrap(Box::new(move |ts: f64| {
                    engine_clone.update_animation(handle_clone, ts);
                }) as Box<dyn FnMut(f64)>);

                animation.callback_id = Some(web_sys::window()
                    .unwrap()
                    .request_animation_frame(callback.as_ref().unchecked_ref())
                    .unwrap());
                callback.forget();
            } else {
                // Animation complete
                self.animations.remove(&handle);
            }
        }
    }
}
```

## Feature Detection

### Browser Capability Detection
```rust
pub struct FeatureDetector {
    waapi_supported: bool,
    webgl_supported: bool,
    performance_api_supported: bool,
}

impl FeatureDetector {
    pub fn new() -> Self {
        Self {
            waapi_supported: Self::detect_waapi(),
            webgl_supported: Self::detect_webgl(),
            performance_api_supported: Self::detect_performance_api(),
        }
    }

    fn detect_waapi() -> bool {
        let element = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();

        element.animate_with_keyframes_and_options(
            &js_sys::Array::new(),
            &web_sys::KeyframeAnimationOptions::new(),
        ).is_ok()
    }

    fn detect_webgl() -> bool {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas.get_context("webgl2")
            .or_else(|_| canvas.get_context("webgl"));

        context.is_ok()
    }
}
```

## Performance Monitoring

### Animation Performance Tracking
```rust
pub struct PerformanceMonitor {
    frame_times: Vec<f64>,
    animation_count: usize,
    dropped_frames: usize,
}

impl PerformanceMonitor {
    pub fn record_frame(&mut self, timestamp: f64) {
        self.frame_times.push(timestamp);
        if self.frame_times.len() > 60 {
            self.frame_times.remove(0);
        }

        // Check for dropped frames (assuming 60fps target)
        let target_frame_time = 1000.0 / 60.0;
        if self.frame_times.len() >= 2 {
            let last_frame_time = self.frame_times[self.frame_times.len() - 1];
            let prev_frame_time = self.frame_times[self.frame_times.len() - 2];
            let frame_duration = last_frame_time - prev_frame_time;

            if frame_duration > target_frame_time * 1.5 {
                self.dropped_frames += 1;
            }
        }
    }

    pub fn get_fps(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }

        let total_time = self.frame_times.last().unwrap() - self.frame_times[0];
        let frame_count = self.frame_times.len() - 1;
        (frame_count as f64) / (total_time / 1000.0)
    }
}
```

## Error Handling

### Animation Error Types
```rust
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("Animation engine not available: {0}")]
    EngineUnavailable(String),

    #[error("Invalid animation property: {property}")]
    InvalidProperty { property: String },

    #[error("Animation already running: {handle:?}")]
    AlreadyRunning { handle: AnimationHandle },

    #[error("Animation not found: {handle:?}")]
    NotFound { handle: AnimationHandle },

    #[error("Invalid animation value: {0}")]
    InvalidValue(String),

    #[error("Performance budget exceeded: {0}")]
    PerformanceBudgetExceeded(String),
}
```

## Configuration

### Engine Configuration
```rust
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Maximum concurrent animations
    pub max_concurrent_animations: usize,

    /// Target frame rate
    pub target_fps: f64,

    /// Enable performance monitoring
    pub performance_monitoring: bool,

    /// Animation throttling threshold
    pub throttle_threshold: usize,

    /// Memory usage limit (MB)
    pub memory_limit_mb: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_animations: 100,
            target_fps: 60.0,
            performance_monitoring: true,
            throttle_threshold: 50,
            memory_limit_mb: 50,
        }
    }
}
```

This design provides a robust, performant animation engine that automatically falls back from WAAPI to RAF while maintaining clean module boundaries and staying under 300 lines per module.
