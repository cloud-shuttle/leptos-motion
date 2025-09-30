# Animation Engine Core Design
## High-Performance Animation System

**File**: `crates/leptos-motion-core/src/engine/` (multiple files)  
**Lines**: Target <300 per file (currently 400-700 lines)  
**Status**: PARTIAL - Missing method implementations  

---

## 🎯 **Engine Overview**

The Animation Engine is the core performance-critical component responsible for orchestrating all animations with 60fps target and efficient memory usage.

### **Core Responsibilities**
1. **Animation Scheduling**: Coordinate animation timing and sequencing
2. **Value Interpolation**: Calculate intermediate animation values
3. **Performance Monitoring**: Track fps, memory usage, dropped frames
4. **Resource Management**: Pool and recycle animation objects
5. **Cross-Platform Support**: Work in both DOM and WebGL contexts

### **Performance Targets**
- **60fps** minimum frame rate
- **<16ms** per frame budget
- **<10MB** memory usage for typical applications
- **<100** concurrent animations without performance degradation

---

## 🏗️ **Architecture**

### **Engine Types**
```rust
pub enum AnimationEngineType {
    /// High-performance engine for complex animations
    Advanced(AdvancedAnimationEngine),

    /// Simplified engine for basic animations
    Simplified(SimplifiedAnimationEngine),

    /// Test-driven development engine
    Tdd(TddAnimationEngine),
}

pub struct AnimationEngine {
    engine_type: AnimationEngineType,
    config: EngineConfig,
    stats: EngineStats,
}
```

### **Core Components**
```rust
pub struct AnimationEngine {
    // Active animations
    active_animations: HashMap<AnimationId, Box<dyn Animation>>,

    // Resource pools
    animation_pool: AnimationPool,
    value_pool: ValuePool,

    // Performance monitoring
    performance_monitor: PerformanceMonitor,

    // Timing
    raf_callback: Option<RafCallback>,
    last_frame_time: f64,
}
```

---

## 🔄 **Animation Lifecycle**

### **Phase 1: Animation Creation**
```rust
fn create_animation(&mut self, config: AnimationConfig) -> AnimationHandle {
    // 1. Acquire animation from pool
    let mut animation = self.animation_pool.acquire();

    // 2. Configure animation parameters
    animation.configure(config);

    // 3. Register with active animations
    let id = self.generate_id();
    self.active_animations.insert(id, animation);

    // 4. Return handle for control
    AnimationHandle::new(id)
}
```

### **Phase 2: Animation Execution**
```rust
fn start_animation(&mut self, handle: AnimationHandle) {
    if let Some(animation) = self.active_animations.get_mut(&handle.id) {
        // 1. Set start time
        animation.start_time = self.current_time();

        // 2. Initialize start values
        animation.capture_start_values();

        // 3. Schedule first frame
        self.schedule_frame();
    }
}
```

### **Phase 3: Frame Processing**
```rust
fn process_frame(&mut self, current_time: f64) {
    let delta_time = current_time - self.last_frame_time;
    self.last_frame_time = current_time;

    // 1. Update all active animations
    let mut completed = Vec::new();
    for (id, animation) in &mut self.active_animations {
        if animation.update(current_time) {
            completed.push(*id);
        }
    }

    // 2. Remove completed animations
    for id in completed {
        if let Some(animation) = self.active_animations.remove(&id) {
            self.animation_pool.release(animation);
        }
    }

    // 3. Schedule next frame if animations remain
    if !self.active_animations.is_empty() {
        self.schedule_frame();
    }
}
```

---

## 🎨 **Value Interpolation System**

### **Interpolation Types**
```rust
pub enum InterpolationType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    CubicBezier(f64, f64, f64, f64),
    Spring(SpringConfig),
    Custom(Box<dyn Fn(f64) -> f64>),
}
```

### **Value Interpolation**
```rust
pub trait Interpolatable {
    fn interpolate(&self, target: &Self, progress: f64) -> Self;
}

impl Interpolatable for f64 {
    fn interpolate(&self, target: &Self, progress: f64) -> Self {
        self + (target - self) * progress
    }
}

impl Interpolatable for AnimationValue {
    fn interpolate(&self, target: &Self, progress: f64) -> Self {
        match (self, target) {
            (AnimationValue::Number(a), AnimationValue::Number(b)) => {
                AnimationValue::Number(a.interpolate(b, progress))
            }
            // ... other value types
        }
    }
}
```

---

## ⚡ **Performance Optimizations**

### **Animation Pooling**
```rust
pub struct AnimationPool {
    available: Vec<Box<dyn Animation>>,
    max_size: usize,
}

impl AnimationPool {
    pub fn acquire(&mut self) -> Box<dyn Animation> {
        self.available.pop()
            .unwrap_or_else(|| Box::new(BasicAnimation::new()))
    }

    pub fn release(&mut self, animation: Box<dyn Animation>) {
        if self.available.len() < self.max_size {
            // Reset animation state
            animation.reset();
            self.available.push(animation);
        }
    }
}
```

### **Batched Updates**
```rust
pub struct AnimationBatch {
    updates: Vec<PropertyUpdate>,
    start_time: f64,
}

pub struct PropertyUpdate {
    element_id: String,
    property: String,
    value: AnimationValue,
}

impl AnimationBatch {
    pub fn commit(&self) {
        // Apply all updates in single DOM operation
        for update in &self.updates {
            self.apply_property_update(update);
        }
    }
}
```

---

## 📊 **Performance Monitoring**

### **Frame Rate Tracking**
```rust
pub struct FrameStats {
    frame_count: u64,
    dropped_frames: u64,
    average_frame_time: f64,
    fps_history: Vec<f64>,
}

impl FrameStats {
    pub fn record_frame(&mut self, frame_time: f64) {
        self.frame_count += 1;
        self.average_frame_time = (self.average_frame_time + frame_time) / 2.0;

        let fps = 1000.0 / frame_time;
        self.fps_history.push(fps);

        // Keep last 60 frames for rolling average
        if self.fps_history.len() > 60 {
            self.fps_history.remove(0);
        }
    }

    pub fn current_fps(&self) -> f64 {
        if self.fps_history.is_empty() {
            0.0
        } else {
            self.fps_history.iter().sum::<f64>() / self.fps_history.len() as f64
        }
    }
}
```

### **Memory Tracking**
```rust
pub struct MemoryStats {
    animation_count: usize,
    pooled_animations: usize,
    estimated_memory_usage: usize,
    peak_memory_usage: usize,
}

impl MemoryStats {
    pub fn update(&mut self, engine: &AnimationEngine) {
        self.animation_count = engine.active_animations.len();
        self.pooled_animations = engine.animation_pool.available.len();

        // Estimate memory usage
        let animation_memory = self.animation_count * std::mem::size_of::<BasicAnimation>();
        let pool_memory = self.pooled_animations * std::mem::size_of::<BasicAnimation>();

        self.estimated_memory_usage = animation_memory + pool_memory;
        self.peak_memory_usage = self.peak_memory_usage.max(self.estimated_memory_usage);
    }
}
```

---

## 🔧 **Engine Configuration**

### **Performance Settings**
```rust
#[derive(Clone, Debug)]
pub struct EngineConfig {
    /// Target frame rate (default: 60)
    pub target_fps: u32,

    /// Maximum concurrent animations (default: 100)
    pub max_concurrent_animations: usize,

    /// Pool size for animation objects (default: 50)
    pub animation_pool_size: usize,

    /// Enable performance monitoring (default: true)
    pub enable_monitoring: bool,

    /// Memory limit in MB (default: 50)
    pub memory_limit_mb: usize,
}
```

### **Quality Settings**
```rust
#[derive(Clone, Debug)]
pub enum QualityPreset {
    /// Maximum performance, minimal features
    Performance,

    /// Balanced performance and quality
    Balanced,

    /// Maximum quality, reduced performance
    Quality,
}
```

---

## 🧪 **Testing Strategy**

### **Performance Tests**
```rust
#[test]
fn test_animation_performance() {
    let mut engine = AnimationEngine::new();

    // Create 100 concurrent animations
    for i in 0..100 {
        let config = AnimationConfig {
            property: format!("test-prop-{}", i),
            from: AnimationValue::Number(0.0),
            to: AnimationValue::Number(1.0),
            duration: 1.0,
            easing: Easing::Linear,
        };

        engine.create_animation(config);
    }

    // Measure frame time
    let start_time = instant::now();
    engine.process_frame(start_time + 16.0); // Simulate 60fps
    let frame_time = instant::now() - start_time;

    // Assert performance target
    assert!(frame_time < 16.0, "Frame time {}ms exceeds 16ms budget", frame_time);
}
```

### **Memory Tests**
```rust
#[test]
fn test_memory_pooling() {
    let mut pool = AnimationPool::new(10);

    // Acquire and release animations
    let mut animations = Vec::new();
    for _ in 0..10 {
        animations.push(pool.acquire());
    }

    // Release all
    for animation in animations {
        pool.release(animation);
    }

    // Pool should contain released animations
    assert_eq!(pool.available.len(), 10);
}
```

---

## 📈 **Engine Metrics**

### **Success Metrics**
- ✅ **60fps** sustained with 100+ concurrent animations
- ✅ **<16ms** average frame time
- ✅ **<10MB** memory usage
- ✅ **Zero memory leaks** in long-running applications
- ✅ **<1ms** animation creation time

### **Monitoring Dashboard**
```rust
pub struct EngineMetrics {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub active_animations: usize,
    pub memory_usage_mb: f64,
    pub pool_hit_rate: f64,
    pub dropped_frames: u64,
}

impl EngineMetrics {
    pub fn report(&self) {
        log::info!("Animation Engine Metrics:");
        log::info!("  FPS: {:.1}", self.fps);
        log::info!("  Frame Time: {:.2}ms", self.frame_time_ms);
        log::info!("  Active Animations: {}", self.active_animations);
        log::info!("  Memory Usage: {:.2}MB", self.memory_usage_mb);
        log::info!("  Pool Hit Rate: {:.1}%", self.pool_hit_rate * 100.0);
    }
}
```

---

## 🎯 **Implementation Plan**

### **Phase 1: Core Engine (Week 1-2)**
- [ ] Implement AnimationEngine struct
- [ ] Add animation lifecycle management
- [ ] Basic value interpolation
- [ ] Frame scheduling with RAF

### **Phase 2: Performance (Week 3-4)**
- [ ] Animation pooling system
- [ ] Batched property updates
- [ ] Memory usage tracking
- [ ] Frame rate monitoring

### **Phase 3: Advanced Features (Week 5-6)**
- [ ] Spring physics animations
- [ ] Custom easing functions
- [ ] Animation sequencing
- [ ] Performance presets

### **Phase 4: Optimization (Week 7-8)**
- [ ] SIMD acceleration for value interpolation
- [ ] Web Workers for heavy computations
- [ ] Bundle size optimization
- [ ] Cross-platform compatibility

**Target Completion**: 8 weeks for production-ready animation engine.
