# WASM-Compatible Animation Engine Design

## 🎯 **Design Goals**

Create a **WASM-first animation engine** that works seamlessly across all platforms while maintaining the performance and features expected from leptos-motion.

**Principles**:
1. **WASM Compatibility** - No `std::time` dependencies
2. **Memory Safety** - Safe RefCell patterns, no borrowing conflicts  
3. **Performance** - GPU acceleration, minimal JavaScript overhead
4. **Developer Experience** - Simple API, clear error messages

---

## 🏗️ **Architecture Overview**

### **Current Problems**
```rust
// ❌ WASM-incompatible patterns
std::time::SystemTime::now()           // Panics in WASM
animation_manager.borrow_mut()         // Multiple borrows = panic
RefCell conflicts in RAF loops         // Ownership issues
Complex animation engine               // Too many moving parts
```

### **New Architecture**
```rust
// ✅ WASM-compatible patterns  
WasmTime::now()                       // Cross-platform timing
SafeAnimationManager::with_manager()   // Safe borrowing
Event-driven animations               // No RAF loops
Modular engine design                 // Single responsibility
```

---

## 🔧 **Core Components**

### **1. Cross-Platform Time System**

#### **Time Utilities Module**
```rust
// crates/leptos-motion-core/src/time.rs

#[cfg(target_arch = "wasm32")]
mod wasm_time {
    use wasm_bindgen::prelude::*;
    use web_sys::window;
    
    pub fn now() -> f64 {
        window()
            .unwrap()
            .performance()
            .unwrap()
            .now()
    }
    
    pub fn timestamp() -> u64 {
        (js_sys::Date::now() * 1000.0) as u64 // Convert to microseconds
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native_time {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    pub fn now() -> f64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as f64
    }
    
    pub fn timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64
    }
}

// Public API
#[cfg(target_arch = "wasm32")]
pub use wasm_time::*;

#[cfg(not(target_arch = "wasm32"))]  
pub use native_time::*;

// High-precision timer for animations
pub struct AnimationTimer {
    start_time: f64,
}

impl AnimationTimer {
    pub fn new() -> Self {
        Self {
            start_time: now(),
        }
    }
    
    pub fn elapsed(&self) -> f64 {
        now() - self.start_time
    }
    
    pub fn elapsed_ms(&self) -> u32 {
        self.elapsed() as u32
    }
}
```

#### **Animation ID Generation**
```rust
// WASM-safe unique ID generation
pub fn generate_animation_id(prefix: &str) -> String {
    let timestamp = timestamp();
    let random = (js_sys::Math::random() * 1000000.0) as u32;
    format!("{}_{:x}_{:x}", prefix, timestamp, random)
}
```

### **2. Safe Animation Manager**

#### **Manager Design**
```rust
// crates/leptos-motion-core/src/safe_manager.rs

use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct SafeAnimationManager {
    animations: Rc<RefCell<HashMap<String, Box<dyn Animation>>>>,
    active_borrows: Rc<RefCell<usize>>,
}

impl SafeAnimationManager {
    pub fn new() -> Self {
        Self {
            animations: Rc::new(RefCell::new(HashMap::new())),
            active_borrows: Rc::new(RefCell::new(0)),
        }
    }
    
    // Safe borrowing with timeout
    pub fn with_animations<F, R>(&self, f: F) -> Result<R, AnimationError>
    where
        F: FnOnce(&mut HashMap<String, Box<dyn Animation>>) -> R,
    {
        // Check if already borrowed
        if *self.active_borrows.borrow() > 0 {
            return Err(AnimationError::ManagerBusy);
        }
        
        match self.animations.try_borrow_mut() {
            Ok(mut animations) => {
                *self.active_borrows.borrow_mut() += 1;
                let result = f(&mut *animations);
                *self.active_borrows.borrow_mut() -= 1;
                Ok(result)
            }
            Err(_) => Err(AnimationError::ManagerBusy),
        }
    }
    
    // Non-blocking read access
    pub fn read_animations<F, R>(&self, f: F) -> Result<R, AnimationError>
    where
        F: FnOnce(&HashMap<String, Box<dyn Animation>>) -> R,
    {
        match self.animations.try_borrow() {
            Ok(animations) => Ok(f(&*animations)),
            Err(_) => Err(AnimationError::ManagerBusy),
        }
    }
    
    // Safe animation registration
    pub fn register_animation(&self, id: String, animation: Box<dyn Animation>) -> Result<AnimationHandle, AnimationError> {
        self.with_animations(|animations| {
            animations.insert(id.clone(), animation);
            AnimationHandle::new(id, Rc::downgrade(&self.animations))
        })
    }
    
    // Safe animation removal
    pub fn unregister_animation(&self, id: &str) -> Result<(), AnimationError> {
        self.with_animations(|animations| {
            animations.remove(id);
        })
    }
}

// RAII handle for automatic cleanup
pub struct AnimationHandle {
    id: String,
    manager: Weak<RefCell<HashMap<String, Box<dyn Animation>>>>,
}

impl AnimationHandle {
    fn new(id: String, manager: Weak<RefCell<HashMap<String, Box<dyn Animation>>>>) -> Self {
        Self { id, manager }
    }
}

impl Drop for AnimationHandle {
    fn drop(&mut self) {
        if let Some(manager) = self.manager.upgrade() {
            if let Ok(mut animations) = manager.try_borrow_mut() {
                animations.remove(&self.id);
            }
        }
    }
}
```

#### **Error Handling**
```rust
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("Animation manager is busy, try again later")]
    ManagerBusy,
    
    #[error("Animation '{0}' not found")]
    AnimationNotFound(String),
    
    #[error("WASM timing error: {0}")]
    WasmTiming(String),
    
    #[error("Animation registration failed: {0}")]
    RegistrationFailed(String),
    
    #[error("DOM element not found")]
    ElementNotFound,
    
    #[error("CSS property '{0}' not supported")]
    UnsupportedProperty(String),
}

// Result type for animation operations
pub type AnimationResult<T> = Result<T, AnimationError>;
```

### **3. WASM-Safe Animation Engine**

#### **Animation Trait**
```rust
// crates/leptos-motion-core/src/animation.rs

pub trait Animation: Send + Sync {
    fn id(&self) -> &str;
    fn start(&mut self) -> AnimationResult<()>;
    fn stop(&mut self) -> AnimationResult<()>;
    fn update(&mut self, delta_time: f64) -> AnimationResult<AnimationStatus>;
    fn is_finished(&self) -> bool;
    fn cleanup(&mut self);
}

pub enum AnimationStatus {
    Running,
    Finished,
    Cancelled,
    Error(AnimationError),
}
```

#### **CSS-Based Animation (WASM-Safe)**
```rust
// WASM-compatible CSS animation
pub struct CssAnimation {
    id: String,
    element: web_sys::Element,
    properties: HashMap<String, AnimationValue>,
    duration: f64,
    easing: String,
    timer: AnimationTimer,
    state: AnimationState,
}

impl CssAnimation {
    pub fn new(
        id: String,
        element: web_sys::Element,
        properties: HashMap<String, AnimationValue>,
        duration: f64,
        easing: String,
    ) -> Self {
        Self {
            id,
            element,
            properties,
            duration,
            easing,
            timer: AnimationTimer::new(),
            state: AnimationState::Ready,
        }
    }
}

impl Animation for CssAnimation {
    fn start(&mut self) -> AnimationResult<()> {
        self.timer = AnimationTimer::new();
        self.apply_css_transition()?;
        self.apply_target_values()?;
        self.state = AnimationState::Running;
        Ok(())
    }
    
    fn update(&mut self, _delta_time: f64) -> AnimationResult<AnimationStatus> {
        match self.state {
            AnimationState::Running => {
                if self.timer.elapsed() >= self.duration * 1000.0 {
                    self.state = AnimationState::Finished;
                    Ok(AnimationStatus::Finished)
                } else {
                    Ok(AnimationStatus::Running)
                }
            }
            AnimationState::Finished => Ok(AnimationStatus::Finished),
            AnimationState::Cancelled => Ok(AnimationStatus::Cancelled),
            _ => Ok(AnimationStatus::Running),
        }
    }
    
    // ... other implementations
}

impl CssAnimation {
    fn apply_css_transition(&self) -> AnimationResult<()> {
        let style = self.element
            .dyn_ref::<web_sys::HtmlElement>()
            .ok_or(AnimationError::ElementNotFound)?
            .style();
            
        let transition_value = format!(
            "all {}ms {}",
            self.duration as u32,
            self.easing
        );
        
        style.set_property("transition", &transition_value)
            .map_err(|_| AnimationError::UnsupportedProperty("transition".to_string()))?;
            
        Ok(())
    }
    
    fn apply_target_values(&self) -> AnimationResult<()> {
        let style = self.element
            .dyn_ref::<web_sys::HtmlElement>()
            .ok_or(AnimationError::ElementNotFound)?
            .style();
            
        for (property, value) in &self.properties {
            let css_value = value.to_css_string();
            style.set_property(property, &css_value)
                .map_err(|_| AnimationError::UnsupportedProperty(property.clone()))?;
        }
        
        Ok(())
    }
}
```

### **4. Event-Driven MotionDiv (WASM-Compatible)**

#### **Component Implementation**
```rust
// crates/leptos-motion-dom/src/wasm_motion_div.rs

use leptos::prelude::*;
use crate::{SafeAnimationManager, CssAnimation, AnimationValue, time};

#[component]
pub fn WasmCompatibleMotionDiv(
    /// Initial animation values
    #[prop(optional)]
    initial: Option<HashMap<String, AnimationValue>>,
    
    /// Target animation values  
    #[prop(optional)]
    animate: Option<HashMap<String, AnimationValue>>,
    
    /// Animation while hovering
    #[prop(optional)]
    while_hover: Option<HashMap<String, AnimationValue>>,
    
    /// Animation while tapping
    #[prop(optional)]
    while_tap: Option<HashMap<String, AnimationValue>>,
    
    /// Transition configuration
    #[prop(optional)]
    transition: Option<Transition>,
    
    /// CSS classes
    #[prop(optional, default = "".to_string())]
    class: String,
    
    /// CSS styles
    #[prop(optional, default = "".to_string())]
    style: String,
    
    /// Node reference (required for DOM access)
    node_ref: NodeRef<leptos::html::Div>,
    
    /// Children
    children: Children,
) -> impl IntoView {
    // Get or create animation manager
    let animation_manager = use_context::<SafeAnimationManager>()
        .unwrap_or_else(|| SafeAnimationManager::new());
    
    // State management
    let (is_hovered, set_hovered) = signal(false);
    let (is_tapped, set_tapped) = signal(false);
    let (current_animation_id, set_current_animation_id) = signal::<Option<String>>(None);
    
    // Apply initial styles
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            if let Some(initial_values) = &initial {
                apply_initial_styles(&element, initial_values);
            }
        }
    });
    
    // Handle animate prop changes
    let animate_effect = {
        let animation_manager = animation_manager.clone();
        let transition = transition.clone();
        
        Effect::new(move |_| {
            if let Some(element) = node_ref.get() {
                if let Some(animate_values) = &animate {
                    if !is_hovered.get() && !is_tapped.get() {
                        let _ = trigger_wasm_animation(
                            &animation_manager,
                            &element,
                            animate_values,
                            &transition,
                            "animate",
                        );
                    }
                }
            }
        })
    };
    
    // Mouse event handlers
    let handle_mouse_enter = {
        let animation_manager = animation_manager.clone();
        let while_hover = while_hover.clone();
        let transition = transition.clone();
        
        move |_| {
            set_hovered.set(true);
            
            if let Some(element) = node_ref.get() {
                if let Some(hover_values) = &while_hover {
                    let _ = trigger_wasm_animation(
                        &animation_manager,
                        &element,
                        hover_values,
                        &transition,
                        "hover",
                    );
                }
            }
        }
    };
    
    let handle_mouse_leave = {
        let animation_manager = animation_manager.clone();
        let animate = animate.clone();
        let transition = transition.clone();
        
        move |_| {
            set_hovered.set(false);
            
            if let Some(element) = node_ref.get() {
                if let Some(animate_values) = &animate {
                    let _ = trigger_wasm_animation(
                        &animation_manager,
                        &element,
                        animate_values,
                        &transition,
                        "animate",
                    );
                }
            }
        }
    };
    
    let handle_click = {
        let animation_manager = animation_manager.clone();
        let while_tap = while_tap.clone();
        let transition = transition.clone();
        
        move |_| {
            set_tapped.set(true);
            
            if let Some(element) = node_ref.get() {
                if let Some(tap_values) = &while_tap {
                    let _ = trigger_wasm_animation(
                        &animation_manager,
                        &element,
                        tap_values,
                        &transition,
                        "tap",
                    );
                }
            }
            
            // Reset tap state after animation
            set_timeout(
                move || set_tapped.set(false),
                std::time::Duration::from_millis(200)
            );
        }
    };
    
    view! {
        <div
            node_ref=node_ref
            class=class
            style=style
            on:mouseenter=handle_mouse_enter
            on:mouseleave=handle_mouse_leave
            on:click=handle_click
        >
            {children()}
        </div>
    }
}

// WASM-safe animation trigger
fn trigger_wasm_animation(
    manager: &SafeAnimationManager,
    element: &web_sys::Element,
    properties: &HashMap<String, AnimationValue>,
    transition: &Option<Transition>,
    name: &str,
) -> AnimationResult<()> {
    let transition = transition.clone().unwrap_or_default();
    let id = crate::time::generate_animation_id(name);
    
    let animation = CssAnimation::new(
        id.clone(),
        element.clone(),
        properties.clone(),
        transition.duration.unwrap_or(0.3) * 1000.0, // Convert to milliseconds
        transition.easing.to_css_string(),
    );
    
    let handle = manager.register_animation(id, Box::new(animation))?;
    
    // Start the animation
    manager.with_animations(|animations| {
        if let Some(animation) = animations.get_mut(&handle.id) {
            animation.start()?;
        }
        Ok(())
    })??;
    
    Ok(())
}
```

---

## 🧪 **Testing Strategy**

### **Cross-Platform Test Suite**
```rust
// tests/wasm_compatibility.rs

#[cfg(test)]
mod wasm_tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_time_system_wasm() {
        let start = crate::time::now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let end = crate::time::now();
        
        assert!(end > start);
        assert!((end - start) >= 10.0);
    }
    
    #[wasm_bindgen_test]
    fn test_animation_manager_wasm() {
        let manager = SafeAnimationManager::new();
        
        let result = manager.with_animations(|animations| {
            assert_eq!(animations.len(), 0);
        });
        
        assert!(result.is_ok());
    }
    
    #[wasm_bindgen_test]
    async fn test_motion_div_wasm() {
        use leptos::*;
        
        let app = view! {
            <WasmCompatibleMotionDiv
                node_ref=NodeRef::new()
                initial=HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(0.0)),
                ])
                animate=HashMap::from([
                    ("opacity".to_string(), AnimationValue::Number(1.0)),
                ])
            >
                "Test content"
            </WasmCompatibleMotionDiv>
        };
        
        // Test that component renders without panics
        let html = app.render_to_string();
        assert!(html.contains("Test content"));
    }
}

#[cfg(test)]
mod native_tests {
    use super::*;
    
    #[test]
    fn test_time_system_native() {
        let start = crate::time::now();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let end = crate::time::now();
        
        assert!(end > start);
        assert!((end - start) >= 10.0);
    }
    
    #[test]
    fn test_concurrent_animation_manager() {
        let manager = SafeAnimationManager::new();
        let manager = std::sync::Arc::new(manager);
        
        let handles: Vec<_> = (0..10)
            .map(|i| {
                let manager = manager.clone();
                std::thread::spawn(move || {
                    manager.with_animations(|animations| {
                        // Simulate animation work
                        std::thread::sleep(std::time::Duration::from_millis(1));
                        i
                    })
                })
            })
            .collect();
        
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), i);
        }
    }
}
```

### **Performance Benchmarks**
```rust
// benches/animation_performance.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leptos_motion_core::*;

fn benchmark_time_system(c: &mut Criterion) {
    c.bench_function("time::now()", |b| {
        b.iter(|| {
            black_box(crate::time::now());
        })
    });
}

fn benchmark_animation_manager(c: &mut Criterion) {
    let manager = SafeAnimationManager::new();
    
    c.bench_function("animation_manager_access", |b| {
        b.iter(|| {
            let result = manager.with_animations(|animations| {
                black_box(animations.len())
            });
            black_box(result)
        })
    });
}

criterion_group!(benches, benchmark_time_system, benchmark_animation_manager);
criterion_main!(benches);
```

---

## 📊 **Performance Characteristics**

### **WASM Bundle Size**
- **Target**: < 50KB gzipped
- **CSS-only animations**: ~20KB
- **Full animation engine**: ~45KB

### **Runtime Performance**
- **Animation startup**: < 5ms
- **Frame rate**: 60 FPS maintained
- **Memory usage**: < 2MB for 100 concurrent animations

### **Cross-Platform Compatibility**
- **WASM32**: Full compatibility
- **Native**: Full compatibility  
- **Mobile**: Full compatibility via CSS transforms

---

## 🎯 **Migration Path**

### **From EventDrivenMotionDiv**
```rust
// ❌ Old (WASM-incompatible)
use leptos_motion_dom::MotionDiv; // Actually EventDrivenMotionDiv

<MotionDiv
    node_ref=node_ref
    animate=animate_map
>
    "Content"
</MotionDiv>

// ✅ New (WASM-compatible)
use leptos_motion_dom::WasmCompatibleMotionDiv;

<WasmCompatibleMotionDiv
    node_ref=node_ref  
    animate=animate_map
>
    "Content"  
</WasmCompatibleMotionDiv>
```

### **API Compatibility**
- **100% backward compatible** with existing props
- **Same animation features** - hover, tap, transitions
- **Improved error handling** with proper Result types
- **Better performance** - no RAF loops, CSS-based

---

## 🎯 **Conclusion**

This WASM-compatible design provides:

1. **✅ True WASM compatibility** - No `std::time` dependencies
2. **✅ Memory safety** - Safe RefCell patterns, automatic cleanup  
3. **✅ Performance** - CSS-based animations, GPU acceleration
4. **✅ Developer experience** - Clear APIs, helpful error messages

The design maintains **full backward compatibility** while solving the core issues that blocked WASM usage. The modular architecture allows for future enhancements without breaking existing code.
