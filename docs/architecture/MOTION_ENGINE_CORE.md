# 🎯 Motion Engine Core Architecture

**Purpose**: Define the core animation engine architecture  
**Audience**: Core developers implementing the animation system  
**Status**: Design Phase  

---

## 🏗️ **Core Architecture Overview**

### **Single Animation Engine Design**
```rust
// ONE animation engine to rule them all
pub struct MotionEngine {
    /// Central animation manager
    manager: AnimationManager,
    /// Performance monitoring
    performance: PerformanceMonitor,
    /// Gesture handling
    gestures: GestureHandler,
    /// Memory management
    memory: MemoryManager,
}

impl MotionEngine {
    /// Create new motion engine
    pub fn new() -> Self { /* ... */ }
    
    /// Start animation
    pub fn animate(&self, target: AnimationTarget) -> AnimationHandle { /* ... */ }
    
    /// Stop animation
    pub fn stop(&self, handle: AnimationHandle) { /* ... */ }
    
    /// Update all animations (called by RAF)
    pub fn update(&self, delta_time: f64) { /* ... */ }
}
```

### **Animation Manager**
```rust
pub struct AnimationManager {
    /// Active animations
    animations: HashMap<String, Box<dyn Animation>>,
    /// Next animation ID
    next_id: u64,
    /// RAF handle
    raf_handle: Option<i32>,
    /// Last frame time
    last_time: Option<f64>,
}

impl AnimationManager {
    /// Register new animation
    pub fn register(&self, animation: Box<dyn Animation>) -> AnimationHandle { /* ... */ }
    
    /// Unregister animation
    pub fn unregister(&self, handle: AnimationHandle) { /* ... */ }
    
    /// Update all animations
    pub fn update_all(&self, delta_time: f64) { /* ... */ }
    
    /// Start animation loop
    pub fn start_loop(&self) { /* ... */ }
    
    /// Stop animation loop
    pub fn stop_loop(&self) { /* ... */ }
}
```

---

## 🎨 **Animation Types**

### **1. CSS Transition Animation**
```rust
pub struct CssTransitionAnimation {
    id: String,
    element: Element,
    target_styles: HashMap<String, AnimationValue>,
    transition: Transition,
    is_running: bool,
    start_time: Option<f64>,
    duration: f64,
    initial_styles: HashMap<String, String>,
}

impl Animation for CssTransitionAnimation {
    fn start(&mut self) -> Result<(), AnimationError> {
        // Apply CSS transition properties
        // Set target styles
        // Start transition
    }
    
    fn update(&mut self, delta_time: f64) -> AnimationResult<AnimationState> {
        // Check if animation is complete
        // Return current state
    }
    
    fn stop(&mut self) -> Result<(), AnimationError> {
        // Remove CSS transition
        // Reset to initial state
    }
}
```

### **2. Keyframe Animation**
```rust
pub struct KeyframeAnimation {
    id: String,
    element: Element,
    keyframes: Vec<Keyframe>,
    transition: Transition,
    is_running: bool,
    start_time: Option<f64>,
    duration: f64,
    current_keyframe_index: usize,
    initial_styles: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Keyframe {
    pub offset: f64, // 0.0 to 1.0
    pub properties: HashMap<String, AnimationValue>,
    pub easing: Option<Easing>,
}
```

### **3. Spring Animation**
```rust
pub struct SpringAnimation {
    id: String,
    element: Element,
    spring_states: HashMap<String, SpringState>,
    spring_config: SpringConfig,
    is_running: bool,
    last_time: Option<f64>,
    initial_styles: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct SpringState {
    pub initial: f64,
    pub target: f64,
    pub position: f64,
    pub velocity: f64,
}

#[derive(Debug, Clone, Default)]
pub struct SpringConfig {
    pub stiffness: f64,
    pub damping: f64,
    pub mass: f64,
    pub rest_displacement_threshold: f64,
    pub rest_velocity_threshold: f64,
}
```

### **4. Stagger Animation**
```rust
pub struct StaggerAnimation {
    id: String,
    child_animations: Vec<Box<dyn Animation>>,
    stagger_config: StaggerConfig,
    animation_manager: Rc<RefCell<AnimationManager>>,
    is_running: bool,
}

#[derive(Debug, Clone, Default)]
pub struct StaggerConfig {
    pub delay: f64,
    pub from_first: bool,
    pub max_delay: Option<f64>,
}
```

---

## 🔄 **Animation Lifecycle**

### **1. Creation**
```rust
// Animation is created with target and transition
let animation = CssTransitionAnimation::new(
    element,
    target_styles,
    transition
);
```

### **2. Registration**
```rust
// Animation is registered with manager
let handle = manager.register(Box::new(animation));
```

### **3. Execution**
```rust
// Animation runs in RAF loop
pub fn animation_frame(timestamp: f64) {
    let delta_time = timestamp - last_time;
    manager.update_all(delta_time);
    last_time = timestamp;
    
    if manager.has_active_animations() {
        request_animation_frame(animation_frame);
    }
}
```

### **4. Completion**
```rust
// Animation completes and is cleaned up
impl Animation for CssTransitionAnimation {
    fn update(&mut self, delta_time: f64) -> AnimationResult<AnimationState> {
        if self.is_complete() {
            Ok(AnimationState::Completed)
        } else {
            Ok(AnimationState::Running)
        }
    }
}
```

---

## 🎯 **Animation Interface**

### **Core Animation Trait**
```rust
pub trait Animation {
    /// Start the animation
    fn start(&mut self) -> Result<(), AnimationError>;
    
    /// Stop the animation
    fn stop(&mut self) -> Result<(), AnimationError>;
    
    /// Update the animation state
    fn update(&mut self, delta_time: f64) -> AnimationResult<AnimationState>;
    
    /// Get the animation ID
    fn id(&self) -> &str;
    
    /// Check if the animation is running
    fn is_running(&self) -> bool;
}
```

### **Animation States**
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    /// Animation is waiting to start
    Waiting,
    /// Animation is currently running
    Running,
    /// Animation has completed
    Completed,
    /// Animation was stopped
    Stopped,
    /// Animation encountered an error
    Error(AnimationError),
}
```

### **Animation Results**
```rust
pub type AnimationResult<T> = Result<T, AnimationError>;

#[derive(Debug, Clone)]
pub enum AnimationError {
    /// Animation not found
    NotFound(String),
    /// Animation already running
    AlreadyRunning(String),
    /// Animation failed to start
    StartFailed(String),
    /// Animation failed to stop
    StopFailed(String),
    /// DOM manipulation failed
    DomError(String),
    /// Invalid animation configuration
    InvalidConfig(String),
}
```

---

## 🚀 **Performance Considerations**

### **1. Memory Management**
```rust
pub struct MemoryManager {
    /// Animation pool for reuse
    animation_pool: Vec<Box<dyn Animation>>,
    /// Element cache
    element_cache: HashMap<String, Element>,
    /// Style cache
    style_cache: HashMap<String, String>,
}

impl MemoryManager {
    /// Get animation from pool
    pub fn get_animation(&mut self) -> Option<Box<dyn Animation>> { /* ... */ }
    
    /// Return animation to pool
    pub fn return_animation(&mut self, animation: Box<dyn Animation>) { /* ... */ }
    
    /// Clear caches
    pub fn clear_caches(&mut self) { /* ... */ }
}
```

### **2. RAF Optimization**
```rust
impl AnimationManager {
    /// Only run RAF when animations are active
    pub fn start_loop(&self) {
        if !self.has_active_animations() {
            return;
        }
        
        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            self.update_all(timestamp);
            
            if self.has_active_animations() {
                request_animation_frame(closure);
            }
        }) as Box<dyn FnMut(f64)>);
        
        request_animation_frame(closure);
    }
}
```

### **3. Batch DOM Updates**
```rust
impl AnimationManager {
    /// Batch DOM updates for performance
    pub fn update_all(&self, delta_time: f64) {
        let mut updates = Vec::new();
        
        for animation in &self.animations {
            if let Some(update) = animation.update(delta_time) {
                updates.push(update);
            }
        }
        
        // Apply all updates in batch
        self.apply_batch_updates(updates);
    }
}
```

---

## 🧪 **Testing Strategy**

### **1. Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_animation_creation() {
        let animation = CssTransitionAnimation::new(
            element,
            target_styles,
            transition
        );
        
        assert_eq!(animation.id(), "test_animation");
        assert!(!animation.is_running());
    }
    
    #[test]
    fn test_animation_lifecycle() {
        let mut animation = create_test_animation();
        
        // Start animation
        assert!(animation.start().is_ok());
        assert!(animation.is_running());
        
        // Update animation
        let result = animation.update(16.0);
        assert!(result.is_ok());
        
        // Stop animation
        assert!(animation.stop().is_ok());
        assert!(!animation.is_running());
    }
}
```

### **2. Integration Tests**
```rust
#[wasm_bindgen_test]
async fn test_animation_actually_animates() {
    let element = create_test_element();
    let animation = create_test_animation();
    
    // Start animation
    animation.start().unwrap();
    
    // Wait for animation to complete
    wait_for_animation_completion().await;
    
    // Verify final state
    let final_style = element.style().get_property_value("opacity").unwrap();
    assert_eq!(final_style, "1");
}
```

### **3. Performance Tests**
```rust
#[test]
fn test_animation_performance() {
    let start_time = std::time::Instant::now();
    
    // Create 100 animations
    for _ in 0..100 {
        let animation = create_test_animation();
        animation.start().unwrap();
    }
    
    let duration = start_time.elapsed();
    assert!(duration.as_millis() < 100); // Should complete in <100ms
}
```

---

## 📋 **Implementation Checklist**

### **Core Engine**
- [ ] Implement `MotionEngine` struct
- [ ] Implement `AnimationManager` struct
- [ ] Implement `Animation` trait
- [ ] Implement animation lifecycle
- [ ] Add memory management
- [ ] Add performance monitoring

### **Animation Types**
- [ ] Implement `CssTransitionAnimation`
- [ ] Implement `KeyframeAnimation`
- [ ] Implement `SpringAnimation`
- [ ] Implement `StaggerAnimation`
- [ ] Add animation state management
- [ ] Add error handling

### **Performance**
- [ ] Implement RAF optimization
- [ ] Add batch DOM updates
- [ ] Add memory pooling
- [ ] Add performance benchmarks
- [ ] Add memory leak detection

### **Testing**
- [ ] Add unit tests for all components
- [ ] Add integration tests
- [ ] Add performance tests
- [ ] Add memory safety tests
- [ ] Add cross-browser tests

---

## 🎯 **Success Criteria**

### **Functional Requirements**
- [ ] All animation types work correctly
- [ ] Animations complete successfully
- [ ] No memory leaks
- [ ] Performance meets targets (60fps)

### **Non-Functional Requirements**
- [ ] Code compiles without errors
- [ ] Tests pass with 90%+ coverage
- [ ] Documentation is complete
- [ ] API is stable and consistent

### **Performance Targets**
- [ ] Animation creation: <1ms
- [ ] Animation update: <0.1ms per animation
- [ ] Memory usage: <1MB for 100 animations
- [ ] Frame rate: 60fps sustained

**This architecture provides a solid foundation for a production-ready animation engine.**
