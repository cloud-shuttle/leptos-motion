# WAAPI Engine Component Design

## Overview
The WAAPI (Web Animations API) Engine leverages native `Element.animate()` for maximum performance and browser optimization.

## Current Issues
- **CRITICAL**: Creates dummy `Animation::new()` instead of `Element.animate()`
- Identical from/to keyframes (no actual interpolation)
- Hardcoded duration in state queries
- No feature detection for WAAPI availability

## Design Goals
- Native `Element.animate()` integration
- Proper keyframe generation with from/to states
- Feature detection and graceful degradation
- Compatibility with all modern browsers
- Performance superior to RAF for complex animations

## API Design

### Core Types
```rust
pub struct WaapiEngine {
    animations: HashMap<AnimationHandle, WaapiAnimation>,
    feature_detector: WaapiFeatureDetector,
}

pub struct WaapiAnimation {
    web_animation: web_sys::Animation,
    handle: AnimationHandle,
    element: HtmlElement,
    config: AnimationConfig,
    keyframes: Vec<Keyframe>,
}

pub struct Keyframe {
    offset: Option<f64>,
    properties: HashMap<String, String>,
    easing: Option<String>,
}
```

### Public Interface
```rust
impl WaapiEngine {
    pub fn new() -> Self
    pub fn is_supported() -> bool
    pub fn create_animation(&mut self, config: AnimationConfig) -> Result<AnimationHandle>
    pub fn play_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn pause_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn cancel_animation(&mut self, handle: AnimationHandle) -> Result<()>
    pub fn get_animation_state(&self, handle: AnimationHandle) -> Option<AnimationPlayState>
}
```

## Implementation Plan

### Phase 1: Feature Detection (Week 1, Day 1)
**File**: `crates/leptos-motion-core/src/engine/waapi_feature_detector.rs`
**Target Lines**: <80

```rust
pub struct WaapiFeatureDetector;

impl WaapiFeatureDetector {
    pub fn is_waapi_supported() -> bool {
        window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("div").ok())
            .map(|e| js_sys::Reflect::has(&e, &"animate".into()).unwrap_or(false))
            .unwrap_or(false)
    }
    
    pub fn supports_keyframe_formats() -> bool
    pub fn supports_composite_modes() -> bool
}
```

### Phase 2: Keyframe Builder (Week 1, Day 2)
**File**: `crates/leptos-motion-core/src/engine/waapi_keyframe_builder.rs`
**Target Lines**: <120

```rust
pub struct KeyframeBuilder {
    keyframes: Vec<Keyframe>,
}

impl KeyframeBuilder {
    pub fn new() -> Self
    pub fn add_from_state(&mut self, element: &HtmlElement, properties: &[String])
    pub fn add_to_state(&mut self, properties: &HashMap<String, AnimationValue>)
    pub fn with_easing(&mut self, easing: &EasingFunction) -> &mut Self
    pub fn build(&self) -> Vec<Keyframe>
}
```

### Phase 3: WAAPI Integration (Week 1, Day 3-4)
**File**: `crates/leptos-motion-core/src/engine/waapi.rs`
**Target Lines**: <200

```rust
impl WaapiEngine {
    fn create_web_animation(
        &self,
        element: &HtmlElement,
        keyframes: &[Keyframe],
        options: &AnimationOptions,
    ) -> Result<web_sys::Animation> {
        let keyframes_js = self.keyframes_to_js_array(keyframes)?;
        let options_js = self.options_to_js_object(options)?;
        
        element
            .animate(&keyframes_js, &options_js)
            .map_err(|e| AnimationError::WaapiError(format!("{:?}", e)))
    }
}
```

## File Structure
```
crates/leptos-motion-core/src/engine/
├── waapi.rs                      # Main WAAPI engine (<200 lines)
├── waapi_feature_detector.rs     # Feature detection (<80 lines)
├── waapi_keyframe_builder.rs     # Keyframe generation (<120 lines)
└── waapi_js_interop.rs          # JS conversion utilities (<100 lines)
```

## Keyframe Generation Strategy

### From State Detection
```rust
fn detect_from_state(element: &HtmlElement, properties: &[String]) -> HashMap<String, String> {
    let computed_style = window()
        .unwrap()
        .get_computed_style(element)
        .unwrap()
        .unwrap();
    
    properties.iter()
        .map(|prop| {
            let value = computed_style.get_property_value(prop).unwrap_or_default();
            (prop.clone(), value)
        })
        .collect()
}
```

### Keyframe Structure
```javascript
// Generated keyframes
[
  { // From state (offset: 0)
    transform: "translateX(0px) scale(1)",
    opacity: "1",
    offset: 0
  },
  { // To state (offset: 1)  
    transform: "translateX(100px) scale(1.5)",
    opacity: "0.5",
    offset: 1,
    easing: "ease-out"
  }
]
```

## Testing Strategy
- Feature detection tests across browsers
- Keyframe generation validation
- Animation lifecycle tests (play/pause/cancel)
- Performance benchmarks vs RAF
- Fallback behavior when WAAPI unavailable

## Performance Requirements
- Feature detection: <1ms
- Keyframe generation: <2ms
- Animation creation: <1ms
- State queries: <0.1ms

## Browser Support
- Chrome 36+
- Firefox 48+  
- Safari 13.1+
- Edge 79+

## Dependencies
```rust
use web_sys::{
    window, HtmlElement, Animation, AnimationPlayState,
    AnimationTimeline, KeyframeEffect
};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::JsValue;
```

## Success Criteria
- [ ] Real `Element.animate()` integration
- [ ] Proper from/to keyframe detection
- [ ] Feature detection working
- [ ] All animation controls functional
- [ ] Performance meets targets
- [ ] All files under 200 lines
- [ ] Graceful fallback to RAF when unavailable
