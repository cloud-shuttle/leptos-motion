# 📋 API Specification

**Purpose**: Define the complete API surface for leptos-motion  
**Audience**: Library users and implementers  
**Status**: Design Phase  

---

## 🎯 **Core API Overview**

### **Single MotionDiv Component**
```rust
#[component]
pub fn MotionDiv(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<AnimationTarget>,
    
    /// Target animation values
    #[prop(optional)]
    animate: Option<AnimationTarget>,
    
    /// Animation transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    
    /// Animation while hovering
    #[prop(optional)]
    while_hover: Option<AnimationTarget>,
    
    /// Animation while tapping
    #[prop(optional)]
    while_tap: Option<AnimationTarget>,
    
    /// Animation while dragging
    #[prop(optional)]
    while_drag: Option<AnimationTarget>,
    
    /// Drag configuration
    #[prop(optional)]
    drag: Option<DragConfig>,
    
    /// Drag constraints
    #[prop(optional)]
    drag_constraints: Option<DragConstraints>,
    
    /// Drag axis
    #[prop(optional)]
    drag_axis: Option<DragAxis>,
    
    /// Animation callbacks
    #[prop(optional)]
    on_animation_start: Option<Box<dyn Fn() + Send + Sync>>,
    
    #[prop(optional)]
    on_animation_complete: Option<Box<dyn Fn() + Send + Sync>>,
    
    /// CSS class
    #[prop(optional)]
    class: Option<String>,
    
    /// CSS style
    #[prop(optional)]
    style: Option<String>,
    
    /// Node reference
    node_ref: NodeRef<leptos::html::Div>,
    
    /// Children
    children: Children,
) -> impl IntoView
```

---

## 🎨 **Animation Types**

### **AnimationTarget**
```rust
/// Target for animation properties
pub type AnimationTarget = HashMap<String, AnimationValue>;

/// Individual animation values
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationValue {
    /// Numeric value (unitless)
    Number(f64),
    /// String value (CSS values)
    String(String),
    /// Pixel value
    Pixels(f64),
    /// Percentage value
    Percentage(f64),
    /// Degree value
    Degrees(f64),
    /// Radian value
    Radians(f64),
    /// Transform value
    Transform(String),
    /// Complex value (multiple properties)
    Complex(HashMap<String, AnimationValue>),
}
```

### **Transition Configuration**
```rust
#[derive(Debug, Clone, Default)]
pub struct Transition {
    /// Animation duration in seconds
    pub duration: Option<f64>,
    /// Animation delay in seconds
    pub delay: Option<f64>,
    /// Easing function
    pub ease: Easing,
    /// Repeat configuration
    pub repeat: RepeatConfig,
    /// Stagger configuration
    pub stagger: Option<StaggerConfig>,
    /// Spring configuration
    pub spring: Option<SpringConfig>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Easing {
    /// Linear easing
    Linear,
    /// Ease in
    EaseIn,
    /// Ease out
    EaseOut,
    /// Ease in-out
    EaseInOut,
    /// Custom cubic bezier
    CubicBezier(f64, f64, f64, f64),
    /// Custom easing function
    Custom(Box<dyn Fn(f64) -> f64 + Send + Sync>),
}

#[derive(Debug, Clone, Default)]
pub struct RepeatConfig {
    /// Number of repeats (None = infinite)
    pub count: Option<u32>,
    /// Repeat type
    pub repeat_type: RepeatType,
    /// Reverse on repeat
    pub reverse: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RepeatType {
    /// Loop from start
    Loop,
    /// Reverse direction
    Reverse,
    /// Alternate direction
    Alternate,
}
```

---

## 🎭 **Animation Variants**

### **Keyframe Animation**
```rust
/// Keyframe animation configuration
#[derive(Debug, Clone)]
pub struct KeyframeConfig {
    /// Keyframes with offsets (0.0 to 1.0)
    pub keyframes: Vec<Keyframe>,
    /// Transition between keyframes
    pub transition: Option<Transition>,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    /// Offset in animation (0.0 to 1.0)
    pub offset: f64,
    /// Properties at this keyframe
    pub properties: AnimationTarget,
    /// Easing for this keyframe
    pub easing: Option<Easing>,
}
```

### **Spring Animation**
```rust
/// Spring physics configuration
#[derive(Debug, Clone, Default)]
pub struct SpringConfig {
    /// Spring stiffness
    pub stiffness: f64,
    /// Spring damping
    pub damping: f64,
    /// Spring mass
    pub mass: f64,
    /// Rest displacement threshold
    pub rest_displacement_threshold: f64,
    /// Rest velocity threshold
    pub rest_velocity_threshold: f64,
}
```

### **Stagger Animation**
```rust
/// Stagger animation configuration
#[derive(Debug, Clone, Default)]
pub struct StaggerConfig {
    /// Delay between animations
    pub delay: f64,
    /// Start from first element
    pub from_first: bool,
    /// Maximum delay
    pub max_delay: Option<f64>,
}
```

---

## 🖱️ **Gesture Configuration**

### **Drag Configuration**
```rust
/// Drag configuration
#[derive(Debug, Clone, Default)]
pub struct DragConfig {
    /// Enable dragging
    pub enabled: bool,
    /// Drag axis
    pub axis: DragAxis,
    /// Drag constraints
    pub constraints: Option<DragConstraints>,
    /// Drag callbacks
    pub on_drag_start: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_drag_move: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
    pub on_drag_end: Option<Box<dyn Fn(f64, f64) + Send + Sync>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DragAxis {
    /// Horizontal only
    X,
    /// Vertical only
    Y,
    /// Both axes
    Both,
}

#[derive(Debug, Clone)]
pub struct DragConstraints {
    /// Minimum X position
    pub min_x: Option<f64>,
    /// Maximum X position
    pub max_x: Option<f64>,
    /// Minimum Y position
    pub min_y: Option<f64>,
    /// Maximum Y position
    pub max_y: Option<f64>,
}
```

---

## 🎯 **Animation Properties**

### **Supported Properties**
```rust
/// Supported animation properties
pub const SUPPORTED_PROPERTIES: &[&str] = &[
    // Transform properties
    "x", "y", "z",
    "scale", "scaleX", "scaleY", "scaleZ",
    "rotate", "rotateX", "rotateY", "rotateZ",
    "skewX", "skewY",
    
    // Layout properties
    "width", "height",
    "top", "right", "bottom", "left",
    "margin", "padding",
    
    // Visual properties
    "opacity",
    "backgroundColor", "color",
    "borderRadius", "borderWidth",
    "boxShadow", "textShadow",
    
    // Filter properties
    "blur", "brightness", "contrast",
    "grayscale", "hueRotate", "invert",
    "saturate", "sepia",
];
```

### **Property Value Types**
```rust
/// Property value type mapping
pub fn get_property_value_type(property: &str) -> PropertyValueType {
    match property {
        // Transform properties
        "x" | "y" | "z" => PropertyValueType::Pixels,
        "scale" | "scaleX" | "scaleY" | "scaleZ" => PropertyValueType::Number,
        "rotate" | "rotateX" | "rotateY" | "rotateZ" => PropertyValueType::Degrees,
        "skewX" | "skewY" => PropertyValueType::Degrees,
        
        // Layout properties
        "width" | "height" => PropertyValueType::Pixels,
        "top" | "right" | "bottom" | "left" => PropertyValueType::Pixels,
        "margin" | "padding" => PropertyValueType::Pixels,
        
        // Visual properties
        "opacity" => PropertyValueType::Number,
        "backgroundColor" | "color" => PropertyValueType::String,
        "borderRadius" | "borderWidth" => PropertyValueType::Pixels,
        "boxShadow" | "textShadow" => PropertyValueType::String,
        
        // Filter properties
        "blur" => PropertyValueType::Pixels,
        "brightness" | "contrast" | "saturate" => PropertyValueType::Percentage,
        "grayscale" | "invert" | "sepia" => PropertyValueType::Percentage,
        "hueRotate" => PropertyValueType::Degrees,
        
        _ => PropertyValueType::String,
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyValueType {
    Number,
    String,
    Pixels,
    Percentage,
    Degrees,
    Radians,
}
```

---

## 🎮 **Usage Examples**

### **Basic Animation**
```rust
#[component]
fn BasicAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            initial=create_animation_target("opacity", 0.0)
            animate=create_animation_target("opacity", 1.0)
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "Hello, World!"
        </MotionDiv>
    }
}
```

### **Transform Animation**
```rust
#[component]
fn TransformAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            initial=create_animation_target("x", -100.0)
            animate=create_animation_target("x", 100.0)
            transition=Transition {
                duration: Some(1.0),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Sliding Element"
        </MotionDiv>
    }
}
```

### **Hover Animation**
```rust
#[component]
fn HoverAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            animate=create_animation_target("scale", 1.0)
            while_hover=create_animation_target("scale", 1.1)
            transition=Transition {
                duration: Some(0.2),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        >
            "Hover me!"
        </MotionDiv>
    }
}
```

### **Drag Animation**
```rust
#[component]
fn DragAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            drag=DragConfig {
                enabled: true,
                axis: DragAxis::Both,
                constraints: Some(DragConstraints {
                    min_x: Some(-100.0),
                    max_x: Some(100.0),
                    min_y: Some(-100.0),
                    max_y: Some(100.0),
                }),
                ..Default::default()
            }
            while_drag=create_animation_target("scale", 1.1)
        >
            "Drag me!"
        </MotionDiv>
    }
}
```

### **Keyframe Animation**
```rust
#[component]
fn KeyframeAnimation() -> impl IntoView {
    let keyframes = vec![
        Keyframe {
            offset: 0.0,
            properties: create_animation_target("x", 0.0),
            easing: Some(Easing::EaseIn),
        },
        Keyframe {
            offset: 0.5,
            properties: create_animation_target("x", 100.0),
            easing: Some(Easing::EaseOut),
        },
        Keyframe {
            offset: 1.0,
            properties: create_animation_target("x", 0.0),
            easing: Some(Easing::EaseInOut),
        },
    ];
    
    view! {
        <MotionDiv
            animate=AnimationTarget::Keyframes(KeyframeConfig {
                keyframes,
                transition: Some(Transition {
                    duration: Some(2.0),
                    ease: Easing::Linear,
                    ..Default::default()
                }),
            })
        >
            "Keyframe Animation"
        </MotionDiv>
    }
}
```

### **Spring Animation**
```rust
#[component]
fn SpringAnimation() -> impl IntoView {
    view! {
        <MotionDiv
            animate=create_animation_target("x", 100.0)
            transition=Transition {
                spring: Some(SpringConfig {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                    ..Default::default()
                }),
                ..Default::default()
            }
        >
            "Spring Animation"
        </MotionDiv>
    }
}
```

---

## 🛠️ **Helper Functions**

### **Animation Target Creation**
```rust
/// Create animation target from property and value
pub fn create_animation_target(property: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), AnimationValue::Number(value));
    target
}

/// Create animation target from multiple properties
pub fn create_animation_targets(properties: &[(&str, f64)]) -> AnimationTarget {
    let mut target = HashMap::new();
    for (property, value) in properties {
        target.insert(property.to_string(), AnimationValue::Number(*value));
    }
    target
}

/// Create drag constraints
pub fn create_drag_constraints(
    min_x: Option<f64>,
    max_x: Option<f64>,
    min_y: Option<f64>,
    max_y: Option<f64>,
) -> DragConstraints {
    DragConstraints {
        min_x,
        max_x,
        min_y,
        max_y,
    }
}
```

---

## 📋 **API Stability**

### **Version 1.0 API**
- ✅ **Stable**: Core MotionDiv component
- ✅ **Stable**: Animation types (AnimationValue, Transition, etc.)
- ✅ **Stable**: Gesture configuration (DragConfig, etc.)
- ✅ **Stable**: Helper functions

### **Future Extensions**
- 🔄 **Planned**: Layout animations
- 🔄 **Planned**: Scroll-triggered animations
- 🔄 **Planned**: 3D animations
- 🔄 **Planned**: Advanced easing functions

### **Breaking Changes**
- ❌ **None planned** for v1.0
- 🔄 **Future**: May add new properties or configuration options

---

## 🧪 **Testing Requirements**

### **API Tests**
```rust
#[test]
fn test_animation_target_creation() {
    let target = create_animation_target("opacity", 1.0);
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));
}

#[test]
fn test_transition_configuration() {
    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        ..Default::default()
    };
    
    assert_eq!(transition.duration, Some(0.5));
    assert_eq!(transition.ease, Easing::EaseInOut);
}
```

### **Integration Tests**
```rust
#[wasm_bindgen_test]
async fn test_motion_div_animation() {
    let element = create_test_element();
    let component = view! {
        <MotionDiv
            animate=create_animation_target("opacity", 1.0)
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        >
            "Test"
        </MotionDiv>
    };
    
    mount_to_body(component);
    wait_for_animation_completion().await;
    
    let final_opacity = element.style().get_property_value("opacity").unwrap();
    assert_eq!(final_opacity, "1");
}
```

---

## 🎯 **Success Criteria**

### **API Completeness**
- [ ] All core animation properties supported
- [ ] All animation types implemented
- [ ] All gesture types supported
- [ ] All configuration options available

### **API Usability**
- [ ] Intuitive component interface
- [ ] Clear property naming
- [ ] Consistent value types
- [ ] Helpful error messages

### **API Performance**
- [ ] Fast component creation
- [ ] Efficient animation updates
- [ ] Minimal memory overhead
- [ ] Smooth 60fps animations

**This API specification provides a complete, stable interface for leptos-motion.**
