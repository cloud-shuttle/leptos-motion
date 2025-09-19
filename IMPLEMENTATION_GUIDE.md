# 🛠️ Leptos Motion Implementation Guide

**Companion to**: [MOTION_DEV_PARITY_REMEDIATION_PLAN.md](./MOTION_DEV_PARITY_REMEDIATION_PLAN.md)  
**Purpose**: Technical implementation details and code examples  
**Audience**: Developers implementing the remediation plan  

---

## 🚨 **Phase 1: Emergency Stabilization - Technical Details**

### **Week 1: Fix Animation Engine**

#### **Current Broken Code**
```rust
// In animation_engine.rs - REMOVE THIS
let closure = Closure::wrap(Box::new(move || {
    // For now, just log that the animation frame was called
    web_sys::console::log_1(&"Animation frame called".into());
}) as Box<dyn FnMut()>);
```

#### **Target Implementation**
```rust
// In animation_engine.rs - IMPLEMENT THIS
pub struct AnimationEngine {
    animations: HashMap<AnimationHandle, ActiveAnimation>,
    raf_handle: Option<i32>,
    last_frame_time: f64,
}

impl AnimationEngine {
    pub fn start_animation_loop(&mut self) -> Result<()> {
        let engine = Rc::new(RefCell::new(self));
        let engine_clone = engine.clone();
        
        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            if let Ok(mut engine) = engine_clone.try_borrow_mut() {
                engine.update_animations(timestamp);
                engine.schedule_next_frame();
            }
        }) as Box<dyn FnMut(f64)>);
        
        let handle = window()
            .ok_or_else(|| AnimationError::EngineUnavailable("Window not available".to_string()))?
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .map_err(|_| AnimationError::EngineUnavailable("RAF not available".to_string()))?;
            
        closure.forget();
        self.raf_handle = Some(handle);
        Ok(())
    }
    
    fn update_animations(&mut self, timestamp: f64) {
        let delta_time = timestamp - self.last_frame_time;
        self.last_frame_time = timestamp;
        
        let mut to_remove = Vec::new();
        
        for (handle, animation) in &mut self.animations {
            if animation.update(delta_time) {
                // Animation completed
                to_remove.push(*handle);
            }
        }
        
        // Remove completed animations
        for handle in to_remove {
            self.animations.remove(&handle);
        }
    }
}
```

#### **Animation Update Logic**
```rust
impl ActiveAnimation {
    fn update(&mut self, delta_time: f64) -> bool {
        self.elapsed_time += delta_time;
        let progress = (self.elapsed_time / self.duration).min(1.0);
        
        // Apply easing
        let eased_progress = self.easing.apply(progress);
        
        // Interpolate values
        for (property, value) in &mut self.current_values {
            if let Some(target) = self.target_values.get(property) {
                *value = self.interpolate(*value, *target, eased_progress);
            }
        }
        
        // Update DOM
        self.update_dom();
        
        // Return true if animation is complete
        progress >= 1.0
    }
    
    fn interpolate(&self, from: f64, to: f64, progress: f64) -> f64 {
        from + (to - from) * progress
    }
    
    fn update_dom(&self) {
        if let Some(element) = &self.element {
            for (property, value) in &self.current_values {
                self.set_css_property(element, property, value);
            }
        }
    }
}
```

### **Week 2: Consolidate Components**

#### **Single MotionDiv Implementation**
```rust
// In components.rs - REPLACE ALL VARIANTS WITH THIS
#[component]
pub fn MotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    /// Target animation values
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    /// Hover animation values
    #[prop(optional)]
    while_hover: Option<HashMap<String, AnimationValue>>,
    /// Tap animation values
    #[prop(optional)]
    while_tap: Option<HashMap<String, AnimationValue>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let (is_hovered, set_hovered) = create_signal(false);
    let (is_tapped, set_tapped) = create_signal(false);
    
    // Create animation engine
    let animation_engine = use_context::<Rc<RefCell<AnimationEngine>>>()
        .unwrap_or_else(|| Rc::new(RefCell::new(AnimationEngine::new())));
    
    // Determine current animation target
    let current_animate = move || {
        if is_tapped.get() {
            while_tap.clone().unwrap_or_default()
        } else if is_hovered.get() {
            while_hover.clone().unwrap_or_default()
        } else {
            animate.clone().unwrap_or_default()
        }
    };
    
    // Set up animation effect
    Effect::new(move |_| {
        let target = current_animate();
        if !target.is_empty() {
            let mut engine = animation_engine.borrow_mut();
            if let Ok(handle) = engine.animate(
                node_ref.get().unwrap(),
                target,
                transition.clone().unwrap_or_default()
            ) {
                // Store handle for cleanup
                // Implementation depends on your cleanup strategy
            }
        }
    });
    
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            on:mouseenter=move |_| set_hovered.set(true)
            on:mouseleave=move |_| set_hovered.set(false)
            on:click=move |_| {
                set_tapped.set(true);
                // Reset tap after animation
                set_timeout(move || set_tapped.set(false), 200);
            }
        >
            {children()}
        </div>
    }
}
```

### **Week 3: Remove Placeholder Code**

#### **Files to Delete**
```bash
# Remove all disabled test files
find . -name "*.disabled" -delete
find . -name "*.backup" -delete

# Remove placeholder implementations
rm crates/leptos-motion-dom/src/reactive_motion_div_fixed.rs
rm crates/leptos-motion-dom/src/signal_based_controller.rs
rm crates/leptos-motion-dom/src/signal_based_motion_div.rs

# Remove theoretical benchmark files
rm LIVE_PERFORMANCE_RESULTS.md
rm WASM_OPTIMIZATION_DEMO.md
```

#### **Clean Up Cargo.toml**
```toml
# Remove unused dependencies
[dependencies]
# Keep only essential dependencies
leptos = { version = "0.8.6", features = ["csr"] }
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = ["Window", "Document", "Element"] }
js-sys = "0.3"
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"

# Remove all the unused dependencies
# Remove: tailwind-rs-core, futures, tokio, etc.
```

---

## 🏗️ **Phase 2: Core Feature Implementation - Technical Details**

### **Weeks 5-6: Core Components**

#### **Component Factory Pattern**
```rust
// In components.rs
macro_rules! create_motion_component {
    ($element:ident, $tag:literal) => {
        paste::paste! {
            #[component]
            pub fn [<Motion $element>](
                #[prop(optional)] class: Option<String>,
                #[prop(optional)] style: Option<String>,
                #[prop(optional)] node_ref: Option<NodeRef<leptos::html::$element>>,
                #[prop(optional)] initial: Option<HashMap<String, AnimationValue>>,
                #[prop(optional)] animate: Option<HashMap<String, AnimationValue>>,
                #[prop(optional)] transition: Option<Transition>,
                #[prop(optional)] while_hover: Option<HashMap<String, AnimationValue>>,
                #[prop(optional)] while_tap: Option<HashMap<String, AnimationValue>>,
                children: Children,
            ) -> impl IntoView {
                let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
                let (is_hovered, set_hovered) = create_signal(false);
                let (is_tapped, set_tapped) = create_signal(false);
                
                let animation_engine = use_context::<Rc<RefCell<AnimationEngine>>>()
                    .unwrap_or_else(|| Rc::new(RefCell::new(AnimationEngine::new())));
                
                let current_animate = move || {
                    if is_tapped.get() {
                        while_tap.clone().unwrap_or_default()
                    } else if is_hovered.get() {
                        while_hover.clone().unwrap_or_default()
                    } else {
                        animate.clone().unwrap_or_default()
                    }
                };
                
                Effect::new(move |_| {
                    let target = current_animate();
                    if !target.is_empty() {
                        let mut engine = animation_engine.borrow_mut();
                        if let Ok(handle) = engine.animate(
                            node_ref.get().unwrap(),
                            target,
                            transition.clone().unwrap_or_default()
                        ) {
                            // Handle animation
                        }
                    }
                });
                
                view! {
                    <leptos::html::$element
                        node_ref=node_ref
                        class=class
                        style=style
                        on:mouseenter=move |_| set_hovered.set(true)
                        on:mouseleave=move |_| set_hovered.set(false)
                        on:click=move |_| {
                            set_tapped.set(true);
                            set_timeout(move || set_tapped.set(false), 200);
                        }
                    >
                        {children()}
                    </leptos::html::$element>
                }
            }
        }
    };
}

// Generate all motion components
create_motion_component!(Div, "div");
create_motion_component!(Span, "span");
create_motion_component!(Button, "button");
create_motion_component!(Img, "img");
create_motion_component!(Input, "input");
create_motion_component!(Textarea, "textarea");
create_motion_component!(Select, "select");
```

### **Weeks 7-8: Animation System**

#### **Property Animation System**
```rust
// In animation_properties.rs
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationProperty {
    // Transform properties
    TranslateX(f64),
    TranslateY(f64),
    TranslateZ(f64),
    Scale(f64),
    ScaleX(f64),
    ScaleY(f64),
    ScaleZ(f64),
    Rotate(f64),
    RotateX(f64),
    RotateY(f64),
    RotateZ(f64),
    SkewX(f64),
    SkewY(f64),
    
    // Layout properties
    Width(f64),
    Height(f64),
    Top(f64),
    Left(f64),
    Right(f64),
    Bottom(f64),
    
    // Visual properties
    Opacity(f64),
    BackgroundColor(String),
    Color(String),
    BorderColor(String),
    
    // Filter properties
    Blur(f64),
    Brightness(f64),
    Contrast(f64),
    Saturate(f64),
    HueRotate(f64),
}

impl AnimationProperty {
    pub fn to_css_property(&self) -> (String, String) {
        match self {
            AnimationProperty::TranslateX(value) => ("transform".to_string(), format!("translateX({}px)", value)),
            AnimationProperty::TranslateY(value) => ("transform".to_string(), format!("translateY({}px)", value)),
            AnimationProperty::Scale(value) => ("transform".to_string(), format!("scale({})", value)),
            AnimationProperty::Rotate(value) => ("transform".to_string(), format!("rotate({}deg)", value)),
            AnimationProperty::Opacity(value) => ("opacity".to_string(), value.to_string()),
            AnimationProperty::BackgroundColor(value) => ("background-color".to_string(), value.clone()),
            // ... implement all properties
        }
    }
}
```

#### **Easing Functions**
```rust
// In easing.rs
#[derive(Debug, Clone)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInQuint,
    EaseOutQuint,
    EaseInOutQuint,
    EaseInExpo,
    EaseOutExpo,
    EaseInOutExpo,
    EaseInCirc,
    EaseOutCirc,
    EaseInOutCirc,
    EaseInBack,
    EaseOutBack,
    EaseInOutBack,
    EaseInElastic,
    EaseOutElastic,
    EaseInOutElastic,
    EaseInBounce,
    EaseOutBounce,
    EaseInOutBounce,
    CubicBezier(f64, f64, f64, f64),
    Spring { stiffness: f64, damping: f64, mass: f64 },
}

impl Easing {
    pub fn apply(&self, t: f64) -> f64 {
        match self {
            Easing::Linear => t,
            Easing::EaseIn => t * t,
            Easing::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Easing::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            },
            Easing::EaseInSine => 1.0 - ((t * std::f64::consts::PI) / 2.0).cos(),
            Easing::EaseOutSine => ((t * std::f64::consts::PI) / 2.0).sin(),
            Easing::EaseInOutSine => -(((std::f64::consts::PI * t).cos()) - 1.0) / 2.0,
            // ... implement all easing functions
            Easing::CubicBezier(x1, y1, x2, y2) => self.cubic_bezier(t, *x1, *y1, *x2, *y2),
            Easing::Spring { stiffness, damping, mass } => self.spring(t, *stiffness, *damping, *mass),
        }
    }
    
    fn cubic_bezier(&self, t: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
        // Implement cubic bezier calculation
        // This is a simplified version - full implementation would be more complex
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1.0 - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        
        3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
    }
    
    fn spring(&self, t: f64, stiffness: f64, damping: f64, mass: f64) -> f64 {
        // Implement spring physics
        // This is a simplified version - full implementation would be more complex
        let omega = (stiffness / mass).sqrt();
        let zeta = damping / (2.0 * (stiffness * mass).sqrt());
        
        if zeta < 1.0 {
            // Underdamped
            let omega_d = omega * (1.0 - zeta * zeta).sqrt();
            (-zeta * omega * t).exp() * (omega_d * t).cos()
        } else {
            // Overdamped or critically damped
            (-zeta * omega * t).exp()
        }
    }
}
```

### **Weeks 9-10: Gesture System**

#### **Drag Gesture Implementation**
```rust
// In gestures.rs
pub struct DragGesture {
    element: Element,
    is_dragging: bool,
    start_position: (f64, f64),
    current_position: (f64, f64),
    constraints: Option<DragConstraints>,
    on_drag_start: Option<Box<dyn Fn((f64, f64))>>,
    on_drag: Option<Box<dyn Fn((f64, f64))>>,
    on_drag_end: Option<Box<dyn Fn((f64, f64))>>,
}

impl DragGesture {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            is_dragging: false,
            start_position: (0.0, 0.0),
            current_position: (0.0, 0.0),
            constraints: None,
            on_drag_start: None,
            on_drag: None,
            on_drag_end: None,
        }
    }
    
    pub fn with_constraints(mut self, constraints: DragConstraints) -> Self {
        self.constraints = Some(constraints);
        self
    }
    
    pub fn on_drag_start<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + 'static,
    {
        self.on_drag_start = Some(Box::new(callback));
        self
    }
    
    pub fn on_drag<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + 'static,
    {
        self.on_drag = Some(Box::new(callback));
        self
    }
    
    pub fn on_drag_end<F>(mut self, callback: F) -> Self
    where
        F: Fn((f64, f64)) + 'static,
    {
        self.on_drag_end = Some(Box::new(callback));
        self
    }
    
    pub fn enable(&self) -> Result<()> {
        let gesture = Rc::new(RefCell::new(self));
        let gesture_clone = gesture.clone();
        
        // Mouse events
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Ok(mut g) = gesture_clone.try_borrow_mut() {
                g.handle_drag_start(event.client_x() as f64, event.client_y() as f64);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        
        let gesture_clone = gesture.clone();
        let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Ok(mut g) = gesture_clone.try_borrow_mut() {
                g.handle_drag(event.client_x() as f64, event.client_y() as f64);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        
        let gesture_clone = gesture.clone();
        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            if let Ok(mut g) = gesture_clone.try_borrow_mut() {
                g.handle_drag_end(event.client_x() as f64, event.client_y() as f64);
            }
        }) as Box<dyn FnMut(MouseEvent)>);
        
        // Add event listeners
        self.element
            .add_event_listener_with_callback("mousedown", mouse_down.as_ref().unchecked_ref())
            .map_err(|_| AnimationError::DomError("Failed to add mousedown listener".to_string()))?;
            
        self.element
            .add_event_listener_with_callback("mousemove", mouse_move.as_ref().unchecked_ref())
            .map_err(|_| AnimationError::DomError("Failed to add mousemove listener".to_string()))?;
            
        self.element
            .add_event_listener_with_callback("mouseup", mouse_up.as_ref().unchecked_ref())
            .map_err(|_| AnimationError::DomError("Failed to add mouseup listener".to_string()))?;
        
        mouse_down.forget();
        mouse_move.forget();
        mouse_up.forget();
        
        Ok(())
    }
    
    fn handle_drag_start(&mut self, x: f64, y: f64) {
        self.is_dragging = true;
        self.start_position = (x, y);
        self.current_position = (x, y);
        
        if let Some(callback) = &self.on_drag_start {
            callback((x, y));
        }
    }
    
    fn handle_drag(&mut self, x: f64, y: f64) {
        if !self.is_dragging {
            return;
        }
        
        let (dx, dy) = (x - self.start_position.0, y - self.start_position.1);
        
        // Apply constraints
        let (constrained_x, constrained_y) = if let Some(constraints) = &self.constraints {
            let x = constraints.constrain_x(dx);
            let y = constraints.constrain_y(dy);
            (x, y)
        } else {
            (dx, dy)
        };
        
        self.current_position = (constrained_x, constrained_y);
        
        if let Some(callback) = &self.on_drag {
            callback((constrained_x, constrained_y));
        }
    }
    
    fn handle_drag_end(&mut self, x: f64, y: f64) {
        if !self.is_dragging {
            return;
        }
        
        self.is_dragging = false;
        
        if let Some(callback) = &self.on_drag_end {
            callback((x, y));
        }
    }
}
```

---

## 🎨 **Phase 3: Advanced Features - Technical Details**

### **Weeks 13-14: Layout Animations (FLIP)**

#### **FLIP Implementation**
```rust
// In layout_animations.rs
pub struct FLIPAnimation {
    element: Element,
    initial_rect: DomRect,
    final_rect: DomRect,
    animation_handle: Option<AnimationHandle>,
}

impl FLIPAnimation {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            initial_rect: DomRect::new(),
            final_rect: DomRect::new(),
            animation_handle: None,
        }
    }
    
    pub fn record_initial_position(&mut self) -> Result<()> {
        self.initial_rect = self.element.get_bounding_client_rect();
        Ok(())
    }
    
    pub fn record_final_position(&mut self) -> Result<()> {
        self.final_rect = self.element.get_bounding_client_rect();
        Ok(())
    }
    
    pub fn play(&mut self, engine: &mut AnimationEngine) -> Result<()> {
        // Calculate the difference between initial and final positions
        let dx = self.initial_rect.left() - self.final_rect.left();
        let dy = self.initial_rect.top() - self.final_rect.top();
        let scale_x = self.initial_rect.width() / self.final_rect.width();
        let scale_y = self.initial_rect.height() / self.final_rect.height();
        
        // Set initial transform to "invert" the element to its initial position
        self.element.style().set_property("transform", &format!(
            "translate({}px, {}px) scale({}, {})",
            dx, dy, scale_x, scale_y
        )).map_err(|_| AnimationError::DomError("Failed to set initial transform".to_string()))?;
        
        // Animate to final position (no transform)
        let target = HashMap::from([
            ("transform".to_string(), AnimationValue::String("translate(0px, 0px) scale(1, 1)".to_string())),
        ]);
        
        let transition = Transition {
            duration: Some(0.3),
            ease: Easing::EaseOut,
            ..Default::default()
        };
        
        let handle = engine.animate(&self.element, target, transition)?;
        self.animation_handle = Some(handle);
        
        Ok(())
    }
}
```

### **Weeks 15-16: Scroll Animations**

#### **Scroll Animation System**
```rust
// In scroll_animations.rs
pub struct ScrollAnimation {
    element: Element,
    trigger_point: f64, // 0.0 to 1.0 (0 = top of viewport, 1 = bottom)
    animation_range: f64, // How much scroll distance to animate over
    on_progress: Option<Box<dyn Fn(f64)>>, // Progress from 0.0 to 1.0
}

impl ScrollAnimation {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            trigger_point: 0.5,
            animation_range: 100.0,
            on_progress: None,
        }
    }
    
    pub fn with_trigger_point(mut self, point: f64) -> Self {
        self.trigger_point = point.clamp(0.0, 1.0);
        self
    }
    
    pub fn with_animation_range(mut self, range: f64) -> Self {
        self.animation_range = range;
        self
    }
    
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(f64) + 'static,
    {
        self.on_progress = Some(Box::new(callback));
        self
    }
    
    pub fn enable(&self) -> Result<()> {
        let animation = Rc::new(RefCell::new(self));
        let animation_clone = animation.clone();
        
        let scroll_handler = Closure::wrap(Box::new(move |_event: Event| {
            if let Ok(mut anim) = animation_clone.try_borrow_mut() {
                anim.handle_scroll();
            }
        }) as Box<dyn FnMut(Event)>);
        
        window()
            .ok_or_else(|| AnimationError::EngineUnavailable("Window not available".to_string()))?
            .add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
            .map_err(|_| AnimationError::DomError("Failed to add scroll listener".to_string()))?;
        
        scroll_handler.forget();
        Ok(())
    }
    
    fn handle_scroll(&mut self) {
        let window = window().unwrap();
        let scroll_y = window.scroll_y().unwrap() as f64;
        let window_height = window.inner_height().unwrap().as_f64().unwrap();
        
        let element_rect = self.element.get_bounding_client_rect();
        let element_top = element_rect.top();
        let element_height = element_rect.height();
        
        // Calculate when animation should start
        let trigger_y = scroll_y + (window_height * self.trigger_point);
        let animation_start = trigger_y - element_height;
        let animation_end = animation_start + self.animation_range;
        
        // Calculate progress
        let progress = if scroll_y < animation_start {
            0.0
        } else if scroll_y > animation_end {
            1.0
        } else {
            (scroll_y - animation_start) / self.animation_range
        };
        
        if let Some(callback) = &self.on_progress {
            callback(progress);
        }
    }
}
```

---

## 🧪 **Phase 4: Testing & Quality Assurance - Technical Details**

### **Comprehensive Test Suite**

#### **Unit Tests**
```rust
// In tests/animation_engine_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_animation_engine_creation() {
        let engine = AnimationEngine::new();
        assert_eq!(engine.active_animations_count(), 0);
    }
    
    #[wasm_bindgen_test]
    fn test_animation_engine_animate() {
        let mut engine = AnimationEngine::new();
        let element = create_test_element();
        
        let target = HashMap::from([
            ("opacity".to_string(), AnimationValue::Number(0.5)),
            ("scale".to_string(), AnimationValue::Number(1.5)),
        ]);
        
        let transition = Transition {
            duration: Some(0.3),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        let handle = engine.animate(&element, target, transition).unwrap();
        assert!(engine.is_running(handle));
    }
    
    #[wasm_bindgen_test]
    fn test_animation_engine_cleanup() {
        let mut engine = AnimationEngine::new();
        let element = create_test_element();
        
        let target = HashMap::from([
            ("opacity".to_string(), AnimationValue::Number(0.0)),
        ]);
        
        let transition = Transition {
            duration: Some(0.1), // Very short duration
            ease: Easing::Linear,
            ..Default::default()
        };
        
        let handle = engine.animate(&element, target, transition).unwrap();
        
        // Wait for animation to complete
        std::thread::sleep(std::time::Duration::from_millis(150));
        
        // Animation should be cleaned up
        assert!(!engine.is_running(handle));
        assert_eq!(engine.active_animations_count(), 0);
    }
    
    fn create_test_element() -> Element {
        let document = window().unwrap().document().unwrap();
        let element = document.create_element("div").unwrap();
        element.set_attribute("id", "test-element").unwrap();
        document.body().unwrap().append_child(&element).unwrap();
        element
    }
}
```

#### **Integration Tests**
```rust
// In tests/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_motion_div_integration() {
        let document = window().unwrap().document().unwrap();
        let container = document.create_element("div").unwrap();
        document.body().unwrap().append_child(&container).unwrap();
        
        // Test that MotionDiv renders and animates
        let initial = HashMap::from([
            ("opacity".to_string(), AnimationValue::Number(0.0)),
        ]);
        
        let animate = HashMap::from([
            ("opacity".to_string(), AnimationValue::Number(1.0)),
        ]);
        
        let transition = Transition {
            duration: Some(0.3),
            ease: Easing::EaseInOut,
            ..Default::default()
        };
        
        // This would test the actual Leptos component
        // Implementation depends on your testing setup
    }
}
```

#### **Performance Tests**
```rust
// In tests/performance_tests.rs
#[cfg(test)]
mod performance_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_animation_performance() {
        let mut engine = AnimationEngine::new();
        let elements = create_multiple_test_elements(100);
        
        let start_time = window().unwrap().performance().unwrap().now();
        
        // Start 100 animations simultaneously
        for element in elements {
            let target = HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(0.5)),
                ("scale".to_string(), AnimationValue::Number(1.2)),
            ]);
            
            let transition = Transition {
                duration: Some(0.3),
                ease: Easing::EaseInOut,
                ..Default::default()
            };
            
            engine.animate(&element, target, transition).unwrap();
        }
        
        let end_time = window().unwrap().performance().unwrap().now();
        let duration = end_time - start_time;
        
        // Should be able to start 100 animations in less than 100ms
        assert!(duration < 100.0);
    }
    
    #[wasm_bindgen_test]
    fn test_memory_usage() {
        let mut engine = AnimationEngine::new();
        let element = create_test_element();
        
        // Start and complete many animations
        for _ in 0..1000 {
            let target = HashMap::from([
                ("opacity".to_string(), AnimationValue::Number(0.5)),
            ]);
            
            let transition = Transition {
                duration: Some(0.01), // Very short
                ease: Easing::Linear,
                ..Default::default()
            };
            
            let handle = engine.animate(&element, target, transition).unwrap();
            
            // Wait for completion
            std::thread::sleep(std::time::Duration::from_millis(20));
            
            // Animation should be cleaned up
            assert!(!engine.is_running(handle));
        }
        
        // Should have no active animations
        assert_eq!(engine.active_animations_count(), 0);
    }
    
    fn create_multiple_test_elements(count: usize) -> Vec<Element> {
        let document = window().unwrap().document().unwrap();
        let mut elements = Vec::new();
        
        for i in 0..count {
            let element = document.create_element("div").unwrap();
            element.set_attribute("id", &format!("test-element-{}", i)).unwrap();
            document.body().unwrap().append_child(&element).unwrap();
            elements.push(element);
        }
        
        elements
    }
}
```

---

## 📚 **Phase 5: Documentation & Community - Technical Details**

### **API Documentation Generation**

#### **Cargo.toml Configuration**
```toml
[package]
name = "leptos-motion"
version = "1.0.0"
authors = ["Your Name <your.email@example.com>"]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/your-org/leptos-motion"
homepage = "https://leptos-motion.dev"
documentation = "https://docs.rs/leptos-motion"
description = "High-performance animation library for Leptos"
keywords = ["animation", "leptos", "wasm", "web"]
categories = ["gui", "wasm", "web-programming"]

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
# ... your dependencies

[dev-dependencies]
# Documentation generation
mdbook = "0.4"
mdbook-mermaid = "0.12"
mdbook-admonish = "1.13"
```

#### **Documentation Structure**
```
docs/
├── book.toml
├── src/
│   ├── SUMMARY.md
│   ├── getting-started.md
│   ├── api-reference.md
│   ├── examples.md
│   ├── advanced.md
│   └── contributing.md
├── examples/
│   ├── basic-animations.md
│   ├── gesture-animations.md
│   ├── layout-animations.md
│   └── scroll-animations.md
└── assets/
    ├── images/
    └── videos/
```

#### **book.toml Configuration**
```toml
[book]
title = "Leptos Motion"
description = "High-performance animation library for Leptos"
authors = ["Leptos Motion Team"]
language = "en"
multilingual = false
src = "src"
build-dir = "book"

[build]
build-dir = "book"
create-missing = true

[output.html]
default-theme = "light"
preferred-dark-theme = "navy"
git-repository-url = "https://github.com/your-org/leptos-motion"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/your-org/leptos-motion/edit/main/docs/src/{path}"
additional-css = ["assets/css/custom.css"]
additional-js = ["assets/js/custom.js"]

[output.html.search]
enable = true
limit-results = 30
teaser-word-count = 30
use-boolean-and = true
boost-title = 2
boost-hierarchy = 1
boost-paragraph = 1
expand = true
heading-split-level = 3
copy-js = true
```

### **Community Building**

#### **GitHub Templates**
```markdown
<!-- .github/ISSUE_TEMPLATE/bug_report.md -->
---
name: Bug report
about: Create a report to help us improve
title: ''
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Go to '...'
2. Click on '....'
3. Scroll down to '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots**
If applicable, add screenshots to help explain your problem.

**Environment (please complete the following information):**
 - OS: [e.g. macOS, Windows, Linux]
 - Browser: [e.g. Chrome, Firefox, Safari]
 - Version: [e.g. 1.0.0]

**Additional context**
Add any other context about the problem here.
```

```markdown
<!-- .github/ISSUE_TEMPLATE/feature_request.md -->
---
name: Feature request
about: Suggest an idea for this project
title: ''
labels: enhancement
assignees: ''
---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

#### **Contributing Guidelines**
```markdown
<!-- CONTRIBUTING.md -->
# Contributing to Leptos Motion

Thank you for your interest in contributing to Leptos Motion! This document provides guidelines and information for contributors.

## Getting Started

1. Fork the repository
2. Clone your fork: `git clone https://github.com/your-username/leptos-motion.git`
3. Create a feature branch: `git checkout -b feature/amazing-feature`
4. Make your changes
5. Run tests: `cargo test`
6. Commit your changes: `git commit -m 'Add amazing feature'`
7. Push to your fork: `git push origin feature/amazing-feature`
8. Open a Pull Request

## Development Setup

### Prerequisites
- Rust 1.70+
- Node.js 18+
- pnpm 8+

### Setup
```bash
# Install dependencies
pnpm install

# Run tests
cargo test
pnpm test:e2e

# Build examples
cargo build --examples
```

## Code Style

- Follow Rust formatting: `cargo fmt`
- Run clippy: `cargo clippy`
- Ensure all tests pass: `cargo test`

## Testing

- Write tests for new features
- Ensure existing tests still pass
- Add integration tests for complex features
- Update documentation for API changes

## Pull Request Process

1. Ensure your code follows the style guidelines
2. Add tests for new functionality
3. Update documentation as needed
4. Ensure all tests pass
5. Request review from maintainers

## Reporting Issues

- Use the issue templates
- Provide clear reproduction steps
- Include environment information
- Add screenshots if applicable

## License

By contributing, you agree that your contributions will be licensed under the MIT OR Apache-2.0 license.
```

---

## 🎯 **Conclusion**

This implementation guide provides the technical details needed to execute the remediation plan. Each phase includes:

1. **Specific code examples** for implementation
2. **Testing strategies** for validation
3. **Documentation approaches** for community building
4. **Quality assurance** measures

The guide is designed to be practical and actionable, with real code that can be implemented immediately. Each section builds upon the previous one, ensuring a solid foundation for the next phase.

**Key Implementation Principles**:
1. **Test-Driven Development**: Write tests first, then implementation
2. **Incremental Delivery**: Working software at end of each week
3. **Quality Focus**: No shortcuts on testing or documentation
4. **Community First**: Open development and clear communication

**Success Metrics**:
- Working animations in Phase 1
- Feature parity in Phase 2
- Advanced features in Phase 3
- Production readiness in Phase 4
- Community adoption in Phase 5

This guide, combined with the remediation plan, provides a complete roadmap for transforming leptos-motion into a world-class animation library.

---

*Implementation Guide Version: 1.0*  
*Last Updated: January 2025*  
*Status: Ready for Implementation*
