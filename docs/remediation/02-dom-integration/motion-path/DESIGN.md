# MotionPath Component Design Document

## Overview

MotionPath is a specialized Leptos Motion component for creating SVG path drawing animations. It automatically calculates SVG path lengths and animates the `stroke-dashoffset` property to create smooth "drawing" effects, similar to Framer Motion's path drawing capabilities but with full WASM performance and automatic path length calculation.

## Component Architecture

### Core Structure

```rust
#[component]
pub fn MotionPath(
    // Path Definition
    d: String,                              // SVG path data

    // Animation Properties
    animate: Option<AnimateProp>,
    initial: Option<HashMap<String, AnimationValue>>,
    transition: Option<Transition>,

    // SVG Styling Properties
    stroke: Option<String>,                 // Stroke color
    stroke_width: Option<String>,           // Stroke width
    stroke_linecap: Option<String>,         // Line cap style
    stroke_linejoin: Option<String>,        // Line join style
    stroke_dasharray: Option<String>,       // Dash pattern
    fill: Option<String>,                   // Fill color

    // DOM Properties
    class: Option<String>,
    style: Option<String>,

    // Event Handlers
    on_animation_start: Option<Box<dyn Fn()>>,
    on_animation_complete: Option<Box<dyn Fn()>>,
) -> impl IntoView
```

### Internal Architecture

```
MotionPath
├── PathParser            # Parse and validate SVG path data
├── LengthCalculator      # Calculate total path length via web_sys
├── DashArrayManager      # Manage stroke-dasharray for drawing effect
├── AnimationController   # Handle path drawing animations
├── SVGRenderer          # Render optimized SVG elements
└── MemoryManager        # Manage path data and calculations
```

## Path Drawing Mechanism

### Automatic Path Length Calculation

```rust
struct LengthCalculator {
    element_ref: NodeRef,
    path_length: RwSignal<Option<f64>>,
}

impl LengthCalculator {
    // Calculate path length using browser's SVG API
    fn calculate_length(&self) -> Result<f64, PathError> {
        let element = self.element_ref.get()
            .ok_or(PathError::ElementNotMounted)?;

        // Cast to SVGPathElement
        let path_element: web_sys::SvgPathElement = element
            .dyn_into()
            .map_err(|_| PathError::NotPathElement)?;

        // Get total length using browser's calculation
        let length = path_element.get_total_length();

        Ok(length as f64)
    }

    // Reactive length calculation on mount
    fn setup_reactive_calculation(&self) {
        create_effect(move |_| {
            if let Ok(length) = self.calculate_length() {
                self.path_length.set(Some(length));
            }
        });
    }
}
```

### Stroke Dash Array Management

```rust
struct DashArrayManager {
    path_length: ReadSignal<Option<f64>>,
    dash_array: RwSignal<String>,
}

impl DashArrayManager {
    // Set dash array to full path length for drawing effect
    fn setup_dash_array(&self) {
        create_effect(move |_| {
            if let Some(length) = self.path_length.get() {
                let dash_array = format!("{}", length);
                self.dash_array.set(dash_array);
            }
        });
    }

    // Update SVG style with calculated dash array
    fn apply_to_element(&self, element: &web_sys::Element) {
        let dash_array = self.dash_array.get();
        element.style()
            .set_property("stroke-dasharray", &dash_array)
            .unwrap();
    }
}
```

## Animation System

### Path Drawing Animation

```rust
// Animate from hidden (full offset) to visible (zero offset)
let animation_target = HashMap::from([
    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
]);

let initial_state = HashMap::from([
    ("stroke-dashoffset".to_string(), AnimationValue::Pixels(path_length))
]);
```

### Animation Configuration

```rust
let transition = Transition {
    duration: Some(2.0),                    // 2 second drawing animation
    easing: Some(Easing::EaseInOut),        // Smooth easing
    delay: Some(0.5),                       // Optional start delay
};
```

### Reactive Animation Control

```rust
#[component]
fn AnimatedPath(
    is_drawing: ReadSignal<bool>
) -> impl IntoView {
    let animate = Memo::new(move |_| {
        if is_drawing.get() {
            // Draw the path
            HashMap::from([
                ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
            ])
        } else {
            // Hide the path
            HashMap::from([
                ("stroke-dashoffset".to_string(), AnimationValue::Pixels(1000.0))
            ])
        }
    });

    view! {
        <MotionPath
            d="M 20 100 A 80 80 0 1 1 180 100 A 80 80 0 1 1 20 100"
            animate=AnimateProp::Derived(animate)
            initial=HashMap::from([
                ("stroke-dashoffset".to_string(), AnimationValue::Pixels(1000.0))
            ])
            transition=Transition {
                duration: Some(2.0),
                easing: Some(Easing::EaseInOut),
            }
            stroke="currentColor"
            stroke_width="3"
            fill="transparent"
        />
    }
}
```

## SVG Optimization

### Path Data Validation

```rust
struct PathParser {
    path_data: String,
}

impl PathParser {
    fn validate_path(&self) -> Result<(), PathError> {
        // Check for valid SVG path syntax
        if self.path_data.trim().is_empty() {
            return Err(PathError::EmptyPath);
        }

        // Basic syntax validation
        if !self.path_data.chars().any(|c| c.is_alphabetic()) {
            return Err(PathError::NoCommands);
        }

        Ok(())
    }

    fn optimize_path(&self) -> String {
        // Remove unnecessary whitespace
        // Convert absolute to relative commands where beneficial
        // Optimize decimal precision
        // TODO: Implement path optimization
        self.path_data.clone()
    }
}
```

### Memory Management

```rust
struct PathMemoryManager {
    cached_lengths: HashMap<String, f64>,
    element_refs: HashMap<String, NodeRef>,
}

impl PathMemoryManager {
    // Cache path lengths to avoid recalculation
    fn get_cached_length(&self, path_data: &str) -> Option<f64> {
        self.cached_lengths.get(path_data).copied()
    }

    // Store calculated length
    fn cache_length(&mut self, path_data: String, length: f64) {
        self.cached_lengths.insert(path_data, length);
    }

    // Clean up unused cached data
    fn cleanup_cache(&mut self) {
        // Remove entries for unmounted components
        // TODO: Implement cleanup logic
    }
}
```

## Performance Optimizations

### 1. Length Calculation Caching
- Cache path lengths to avoid repeated browser API calls
- Share calculations across identical paths

### 2. Lazy Calculation
- Calculate path length only when needed
- Defer calculation until element is mounted

### 3. Memory Efficiency
- Reuse path data structures
- Automatic cleanup of cached data
- Weak references to prevent memory leaks

### 4. Animation Batching
- Batch path drawing animations
- Use hardware acceleration for smooth drawing

## Browser Compatibility

### SVG Support Requirements
- **SVG 1.1 Support**: Basic path drawing
- **getTotalLength() API**: Path length calculation
- **stroke-dasharray**: Dash pattern support
- **stroke-dashoffset**: Offset animation support

### Fallback Strategy
```rust
// Detect SVG capabilities
fn check_svg_support() -> SvgCapabilities {
    // Check for SVG element creation
    // Check for getTotalLength method
    // Check for stroke-dash properties
}

// Graceful degradation
if !svg_supported {
    // Render static path without animation
    view! {
        <path d=path_data stroke=stroke fill=fill />
    }
}
```

## Error Handling

### Path Validation Errors

```rust
#[derive(Debug, Clone)]
pub enum PathError {
    EmptyPath,
    InvalidSyntax(String),
    NoCommands,
    ElementNotMounted,
    NotPathElement,
    CalculationFailed(String),
}
```

### Runtime Error Recovery

```rust
// Handle path length calculation failures
match length_calculator.calculate_length() {
    Ok(length) => {
        // Use calculated length
        setup_dash_array(length);
    }
    Err(error) => {
        log::warn!("Path length calculation failed: {:?}", error);
        // Fallback to estimated length or static rendering
        setup_fallback_rendering();
    }
}
```

## Testing Strategy

### Path Validation Tests
- Valid SVG path syntax
- Invalid path data handling
- Empty path edge cases

### Length Calculation Tests
- Accurate length calculation
- Caching behavior
- Error recovery

### Animation Tests
- Path drawing animations
- Stroke offset behavior
- Timing and easing

### Browser Compatibility Tests
- SVG API availability
- Fallback behavior
- Cross-browser rendering

## Usage Examples

### Basic Path Drawing

```rust
view! {
    <MotionPath
        d="M 50 50 L 150 50 L 150 150 L 50 150 Z"
        animate=AnimateProp::Static(HashMap::from([
            ("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))
        ]))
        initial=HashMap::from([
            ("stroke-dashoffset".to_string(), AnimationValue::Pixels(400.0))
        ])
        transition=Transition {
            duration: Some(3.0),
            easing: Some(Easing::EaseInOut),
        }
        stroke="#ff6b6b"
        stroke_width="4"
        fill="transparent"
    />
}
```

### Reactive Path Drawing

```rust
let (is_visible, set_is_visible) = signal(false);

let animate = Memo::new(move |_| {
    if is_visible.get() {
        HashMap::from([("stroke-dashoffset".to_string(), AnimationValue::Pixels(0.0))])
    } else {
        HashMap::from([("stroke-dashoffset".to_string(), AnimationValue::Pixels(400.0))])
    }
});

view! {
    <MotionPath
        d="M 100 100 Q 200 50 300 100 T 500 100"
        animate=AnimateProp::Derived(animate)
        stroke="#4ecdc4"
        stroke_width="3"
    />
}
```

### Complex SVG with Multiple Paths

```rust
view! {
    <svg viewBox="0 0 400 400">
        <MotionPath d="M 50 200 Q 200 100 350 200" /* circle */ />
        <MotionPath d="M 100 250 L 300 250 L 300 350 L 100 350 Z" /* rectangle */ />
        <MotionPath d="M 200 50 L 250 150 L 150 150 Z" /* triangle */ />
    </svg>
}
```

## Future Enhancements

### Advanced Path Features
- **Path Morphing**: Smooth transitions between different path shapes
- **Progressive Drawing**: Draw path in segments with different timings
- **Path Following**: Animate elements along path trajectories

### Performance Features
- **WebGL Path Rendering**: GPU-accelerated path drawing
- **Path Precomputation**: Pre-calculate complex path data
- **Lazy Path Loading**: Load path data on demand

### Animation Features
- **Custom Drawing Patterns**: Dashed lines, dotted patterns
- **Reverse Drawing**: Draw paths in reverse direction
- **Multi-stage Drawing**: Complex multi-phase drawing sequences

---

*MotionPath provides automatic SVG path drawing animations with WASM performance, making complex path animations as simple as declaring the target path and animation properties.*
