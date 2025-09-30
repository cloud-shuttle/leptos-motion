# MotionDiv Component Design
## Core Animation Component

**File**: `crates/leptos-motion-dom/src/motion_div.rs`  
**Lines**: Target <300 (currently ~400)  
**Status**: BROKEN - Multiple compilation errors  

---

## 🎯 **Component Overview**

MotionDiv is the primary animation component providing Framer Motion-inspired API for declarative animations in Leptos.

### **Core Responsibilities**
1. **Animation Orchestration**: Coordinate multiple animation properties
2. **State Management**: Handle animation lifecycle and transitions
3. **DOM Integration**: Apply animations to HTML elements
4. **Event Handling**: Process user interactions (hover, tap, drag)

### **Key Features**
- Declarative animation props (`animate`, `initial`, `exit`)
- Gesture support (`while_hover`, `while_tap`, `drag`)
- Layout animations with FLIP technique
- Performance optimizations for 60fps

---

## 🏗️ **Architecture**

### **Component Structure**
```rust
#[component]
pub fn MotionDiv(
    // Animation props
    #[prop(into)] animate: AnimateProp,
    #[prop(into)] initial: Option<AnimateProp>,
    #[prop(into)] exit: Option<AnimateProp>,

    // Transition config
    #[prop(into)] transition: Option<Transition>,

    // Gesture props
    #[prop(into)] while_hover: Option<AnimateProp>,
    #[prop(into)] while_tap: Option<AnimateProp>,
    #[prop(into)] drag: Option<DragConfig>,

    // Layout props
    #[prop(into)] layout: Option<bool>,

    // DOM props
    #[prop(into)] class: Option<String>,
    #[prop(into)] style: Option<String>,
    node_ref: NodeRef<html::Div>,

    children: Children,
) -> impl IntoView
```

### **Internal State Management**
```rust
#[derive(Clone)]
pub struct MotionDivState {
    // Animation state
    current_values: HashMap<String, AnimationValue>,
    target_values: HashMap<String, AnimationValue>,

    // Lifecycle state
    is_animating: bool,
    is_hovered: bool,
    is_tapped: bool,
    is_dragging: bool,

    // Performance tracking
    animation_start_time: Option<f64>,
    frame_count: usize,
}
```

---

## 🔄 **Animation Lifecycle**

### **Phase 1: Initialization**
```rust
fn create() -> MotionDivState {
    // 1. Parse initial prop into current_values
    // 2. Set up DOM element references
    // 3. Initialize animation engine
    // 4. Register event listeners
}
```

### **Phase 2: Animation Trigger**
```rust
fn trigger_animation(target: AnimateProp) {
    // 1. Parse target values
    // 2. Calculate transition parameters
    // 3. Start animation engine
    // 4. Update DOM element styles
}
```

### **Phase 3: Animation Loop**
```rust
fn animation_frame(progress: f64) {
    // 1. Interpolate between current and target values
    // 2. Apply easing functions
    // 3. Update DOM element styles
    // 4. Handle completion callbacks
}
```

### **Phase 4: Cleanup**
```rust
fn cleanup() {
    // 1. Cancel pending animations
    // 2. Reset DOM element styles
    // 3. Clean up event listeners
    // 4. Free animation resources
}
```

---

## 🎨 **Animation Value System**

### **Supported Value Types**
```rust
pub enum AnimationValue {
    Number(f64),           // opacity: 0.5
    Pixels(f64),           // x: 100px
    Percentage(f64),       // width: 50%
    Degrees(f64),          // rotate: 45deg
    Radians(f64),          // rotate: 0.785rad
    Color(String),         // background: "#ff0000"
    String(String),        // transform: "scale(1.1)"
}
```

### **Value Interpolation**
```rust
impl AnimationValue {
    fn interpolate(&self, target: &Self, progress: f64) -> Self {
        match (self, target) {
            (Number(a), Number(b)) => Number(a + (b - a) * progress),
            (Pixels(a), Pixels(b)) => Pixels(a + (b - a) * progress),
            // ... other interpolations
        }
    }
}
```

---

## 🎭 **Transition System**

### **Transition Configuration**
```rust
#[derive(Clone, Debug)]
pub struct Transition {
    pub duration: Option<f64>,        // seconds
    pub delay: Option<f64>,           // seconds
    pub ease: Easing,
    pub repeat: RepeatConfig,
    pub stagger: Option<StaggerConfig>,
}
```

### **Easing Functions**
```rust
#[derive(Clone, Debug)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f64, f64, f64, f64),
    Spring(SpringConfig),
}
```

---

## 👆 **Gesture Integration**

### **Gesture Types**
```rust
#[derive(Clone, Debug)]
pub enum GestureType {
    Hover { target: AnimateProp },
    Tap { target: AnimateProp },
    Drag { config: DragConfig },
    Pan { config: PanConfig },
}
```

### **Event Handling**
```rust
fn handle_gesture_event(event: GestureEvent) {
    match event {
        GestureEvent::HoverStart => {
            // Apply while_hover animation
        }
        GestureEvent::TapStart => {
            // Apply while_tap animation
        }
        GestureEvent::DragStart => {
            // Initialize drag state
        }
    }
}
```

---

## 🚀 **Performance Optimizations**

### **Animation Batching**
```rust
struct AnimationBatch {
    elements: Vec<Element>,
    properties: HashMap<String, Vec<AnimationValue>>,
    start_time: f64,
}

impl AnimationBatch {
    fn add_animation(&mut self, element: Element, property: String, value: AnimationValue) {
        // Batch related animations together
    }

    fn commit(&self) {
        // Apply all batched animations in single frame
    }
}
```

### **Memory Management**
```rust
struct AnimationPool {
    active_animations: Vec<Animation>,
    recycled_animations: Vec<Animation>,
}

impl AnimationPool {
    fn acquire(&mut self) -> Animation {
        self.recycled_animations.pop()
            .unwrap_or_else(|| Animation::new())
    }

    fn release(&mut self, animation: Animation) {
        // Reset and recycle animation object
        animation.reset();
        self.recycled_animations.push(animation);
    }
}
```

---

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[test]
fn test_animation_value_interpolation() {
    let start = AnimationValue::Number(0.0);
    let end = AnimationValue::Number(1.0);
    let result = start.interpolate(&end, 0.5);
    assert_eq!(result, AnimationValue::Number(0.5));
}

#[test]
fn test_motion_div_creation() {
    let motion_div = MotionDiv::new()
        .animate(AnimateProp::static_props([("opacity", 1.0)]))
        .initial(AnimateProp::static_props([("opacity", 0.0)]));
    assert!(motion_div.is_valid());
}
```

### **Integration Tests**
```rust
#[test]
fn test_hover_animation() {
    // 1. Create MotionDiv with while_hover
    // 2. Simulate hover event
    // 3. Verify animation triggers
    // 4. Check final values applied
}
```

---

## 📊 **Performance Targets**

- **60fps**: Maintain 60fps for <100 concurrent animations
- **Memory**: <10MB memory usage for typical applications
- **Bundle Size**: <100KB gzipped for core functionality
- **Startup**: <50ms initialization time

---

## 🔄 **Migration Path**

### **Breaking Changes from Current API**
1. **AnimateProp enum** instead of HashMap
2. **Required children** parameter
3. **NodeRef mandatory** for DOM manipulation
4. **Transition structure** changes

### **Migration Helper**
```rust
// Before (broken)
<MotionDiv animate={hashmap} />

// After (working)
<MotionDiv
    animate=AnimateProp::Static(hashmap)
    node_ref=node_ref
>
    {children}
</MotionDiv>
```

---

## 🎯 **Implementation Plan**

### **Phase 1: Core Structure (Week 1-2)**
- [ ] Define AnimateProp enum correctly
- [ ] Implement basic component structure
- [ ] Fix compilation errors
- [ ] Add required children/node_ref props

### **Phase 2: Animation Logic (Week 3-4)**
- [ ] Implement animation lifecycle
- [ ] Add value interpolation
- [ ] Integrate with animation engine
- [ ] Handle transition configurations

### **Phase 3: Gesture Support (Week 5-6)**
- [ ] Add hover/tap gesture handling
- [ ] Implement drag functionality
- [ ] Add gesture state management

### **Phase 4: Optimization (Week 7-8)**
- [ ] Add animation batching
- [ ] Implement memory pooling
- [ ] Performance monitoring
- [ ] Bundle size optimization

**Target Completion**: 8 weeks for fully functional MotionDiv component.
