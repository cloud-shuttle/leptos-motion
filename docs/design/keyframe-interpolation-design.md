# Keyframe Interpolation Component Design

## Overview
Proper keyframe interpolation system for smooth animations between multiple keyframe points during runtime.

## Current Issues
- **CRITICAL**: `interpolate_keyframes` function exists but is never called in `update()`
- KeyframeAnimation only applies boundary keyframes, not in-between values
- No smooth interpolation between keyframe segments
- Missing easing function application between keyframes

## Design Goals
- Smooth interpolation between any number of keyframes
- Per-segment easing function support
- Efficient real-time calculation during animation updates
- Support for all CSS property types (colors, transforms, numbers)
- Bezier curve and custom easing support

## API Design

### Core Types
```rust
pub struct KeyframeInterpolator {
    keyframes: Vec<Keyframe>,
    property_interpolators: HashMap<String, Box<dyn PropertyInterpolator>>,
}

pub struct Keyframe {
    offset: f64, // 0.0 to 1.0
    properties: HashMap<String, AnimationValue>,
    easing: Option<EasingFunction>,
    composite: CompositeOperation,
}

pub trait PropertyInterpolator {
    fn interpolate(&self, from: &AnimationValue, to: &AnimationValue, progress: f64) -> AnimationValue;
    fn property_type(&self) -> PropertyType;
}

pub struct InterpolationResult {
    pub properties: HashMap<String, AnimationValue>,
    pub progress: f64,
    pub current_segment: usize,
}
```

### Public Interface
```rust
impl KeyframeInterpolator {
    pub fn new(keyframes: Vec<Keyframe>) -> Self
    pub fn interpolate_at(&self, global_progress: f64) -> InterpolationResult
    pub fn add_keyframe(&mut self, keyframe: Keyframe)
    pub fn remove_keyframe(&mut self, offset: f64) -> Option<Keyframe>
    pub fn get_segment_at(&self, progress: f64) -> Option<(usize, &Keyframe, &Keyframe)>
}
```

## Implementation Plan

### Phase 1: Core Interpolation Engine (Week 1, Day 1-2)
**File**: `crates/leptos-motion-dom/src/interpolation/keyframe_interpolator.rs`
**Target Lines**: <200

```rust
impl KeyframeInterpolator {
    pub fn interpolate_at(&self, global_progress: f64) -> InterpolationResult {
        let global_progress = global_progress.clamp(0.0, 1.0);
        
        // Find current keyframe segment
        let (segment_index, from_keyframe, to_keyframe) = 
            self.get_segment_at(global_progress)?;
        
        // Calculate local progress within segment
        let segment_progress = self.calculate_segment_progress(
            global_progress, 
            from_keyframe.offset, 
            to_keyframe.offset
        );
        
        // Apply easing to segment progress
        let eased_progress = self.apply_easing(
            segment_progress, 
            &from_keyframe.easing
        );
        
        // Interpolate all properties in this segment
        let mut interpolated_properties = HashMap::new();
        for (property, from_value) in &from_keyframe.properties {
            if let Some(to_value) = to_keyframe.properties.get(property) {
                let interpolated = self.interpolate_property(
                    property, 
                    from_value, 
                    to_value, 
                    eased_progress
                );
                interpolated_properties.insert(property.clone(), interpolated);
            }
        }
        
        InterpolationResult {
            properties: interpolated_properties,
            progress: eased_progress,
            current_segment: segment_index,
        }
    }
}
```

### Phase 2: Property-Specific Interpolators (Week 1, Day 3)
**File**: `crates/leptos-motion-dom/src/interpolation/property_interpolators.rs`
**Target Lines**: <250

```rust
// Number interpolation
pub struct NumberInterpolator;
impl PropertyInterpolator for NumberInterpolator {
    fn interpolate(&self, from: &AnimationValue, to: &AnimationValue, progress: f64) -> AnimationValue {
        match (from, to) {
            (AnimationValue::Number(f), AnimationValue::Number(t)) => {
                AnimationValue::Number(f + (t - f) * progress)
            }
            _ => from.clone(),
        }
    }
}

// Color interpolation (RGB, HSL, etc.)
pub struct ColorInterpolator;
impl PropertyInterpolator for ColorInterpolator {
    fn interpolate(&self, from: &AnimationValue, to: &AnimationValue, progress: f64) -> AnimationValue {
        match (from, to) {
            (AnimationValue::Color(from_color), AnimationValue::Color(to_color)) => {
                AnimationValue::Color(self.interpolate_rgb(from_color, to_color, progress))
            }
            _ => from.clone(),
        }
    }
}

// Transform interpolation
pub struct TransformInterpolator;
impl PropertyInterpolator for TransformInterpolator {
    fn interpolate(&self, from: &AnimationValue, to: &AnimationValue, progress: f64) -> AnimationValue {
        // Interpolate transform matrices or individual transform functions
        match (from, to) {
            (AnimationValue::Transform(from_transform), AnimationValue::Transform(to_transform)) => {
                AnimationValue::Transform(self.interpolate_transforms(from_transform, to_transform, progress))
            }
            _ => from.clone(),
        }
    }
}
```

### Phase 3: Easing Functions (Week 1, Day 4)
**File**: `crates/leptos-motion-dom/src/interpolation/easing.rs`
**Target Lines**: <150

```rust
pub enum EasingFunction {
    Linear,
    Ease,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f64, f64, f64, f64),
    Steps(i32, StepPosition),
    Custom(Box<dyn Fn(f64) -> f64>),
}

impl EasingFunction {
    pub fn apply(&self, progress: f64) -> f64 {
        match self {
            EasingFunction::Linear => progress,
            EasingFunction::Ease => self.cubic_bezier(0.25, 0.1, 0.25, 1.0, progress),
            EasingFunction::EaseIn => self.cubic_bezier(0.42, 0.0, 1.0, 1.0, progress),
            EasingFunction::EaseOut => self.cubic_bezier(0.0, 0.0, 0.58, 1.0, progress),
            EasingFunction::EaseInOut => self.cubic_bezier(0.42, 0.0, 0.58, 1.0, progress),
            EasingFunction::CubicBezier(x1, y1, x2, y2) => self.cubic_bezier(*x1, *y1, *x2, *y2, progress),
            EasingFunction::Steps(steps, position) => self.step_function(*steps, position, progress),
            EasingFunction::Custom(func) => func(progress),
        }
    }
    
    fn cubic_bezier(&self, x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
        // Cubic bezier implementation for smooth easing
        // Using Newton-Raphson method for precise calculation
    }
}
```

## File Structure
```
crates/leptos-motion-dom/src/interpolation/
├── mod.rs                      # Public API (<50 lines)
├── keyframe_interpolator.rs    # Main interpolation engine (<200 lines)
├── property_interpolators.rs   # Property-specific interpolation (<250 lines)
├── easing.rs                   # Easing functions (<150 lines)
└── segment_calculator.rs       # Keyframe segment math (<100 lines)
```

## Integration with KeyframeAnimation

### Updated KeyframeAnimation::update()
```rust
impl KeyframeAnimation {
    pub fn update(&mut self, delta_time: f64) -> Result<(), AnimationError> {
        self.current_time += delta_time;
        let progress = (self.current_time / self.duration).clamp(0.0, 1.0);
        
        // Use interpolator instead of boundary keyframes only
        let interpolation_result = self.interpolator.interpolate_at(progress);
        
        // Apply interpolated properties to DOM
        for (property, value) in interpolation_result.properties {
            self.apply_property_to_dom(&property, &value)?;
        }
        
        // Update animation state
        if progress >= 1.0 {
            self.state = AnimationState::Finished;
            self.call_completion_callback();
        }
        
        Ok(())
    }
}
```

## Advanced Features

### Multi-Segment Keyframes
```rust
// Support for complex keyframe sequences
let keyframes = vec![
    Keyframe { offset: 0.0, properties: initial_props, easing: Some(EasingFunction::EaseOut) },
    Keyframe { offset: 0.3, properties: mid_props1, easing: Some(EasingFunction::Linear) },
    Keyframe { offset: 0.7, properties: mid_props2, easing: Some(EasingFunction::EaseIn) },
    Keyframe { offset: 1.0, properties: final_props, easing: None },
];
```

### Color Space Interpolation
```rust
pub enum ColorSpace {
    RGB,
    HSL,
    LAB,
    LCH,
}

impl ColorInterpolator {
    pub fn with_color_space(color_space: ColorSpace) -> Self {
        Self { color_space }
    }
    
    fn interpolate_in_space(&self, from: &Color, to: &Color, progress: f64) -> Color {
        match self.color_space {
            ColorSpace::RGB => self.interpolate_rgb(from, to, progress),
            ColorSpace::HSL => self.interpolate_hsl(from, to, progress),
            ColorSpace::LAB => self.interpolate_lab(from, to, progress),
            ColorSpace::LCH => self.interpolate_lch(from, to, progress),
        }
    }
}
```

## Testing Strategy
- Unit tests for each interpolator type
- Keyframe boundary condition tests
- Easing function accuracy tests
- Performance benchmarks (target: <1ms per interpolation)
- Visual regression tests for smooth animations

## Performance Requirements
- Keyframe interpolation: <1ms per frame
- Property interpolation: <0.1ms per property
- Easing calculation: <0.05ms per function
- Memory usage: <10KB per keyframe set

## Dependencies
```rust
use std::collections::HashMap;
use crate::animation::{AnimationValue, AnimationError};
use crate::easing::EasingFunction;
```

## Success Criteria
- [ ] KeyframeAnimation::update() calls interpolation
- [ ] Smooth interpolation between all keyframes
- [ ] Per-segment easing working correctly
- [ ] All CSS property types supported
- [ ] Performance requirements met
- [ ] All files under 250 lines
- [ ] Visual smoothness verified in tests
