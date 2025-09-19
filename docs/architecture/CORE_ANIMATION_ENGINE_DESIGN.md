# Core Animation Engine Design

## Overview

This document outlines the design for the unified core animation engine that will replace the current multiple competing implementations. The design prioritizes performance, maintainability, and WASM compatibility.

## Design Principles

### 1. Single Source of Truth
- One primary animation engine with fallback mechanisms
- Consistent API across all animation types
- Unified error handling and state management

### 2. WASM-First Architecture
- Thread-safe design patterns
- Minimal memory allocations
- Efficient serialization/deserialization

### 3. Performance Optimization
- 60fps target performance
- Hardware acceleration where possible
- Minimal CPU overhead

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Animation Engine Core                    │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   WAAPI     │  │     RAF     │  │     CSS     │        │
│  │   Engine    │  │   Engine    │  │   Engine    │        │
│  │ (Primary)   │  │ (Fallback)  │  │ (Fallback)  │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   State     │  │   Memory    │  │  Performance│        │
│  │  Manager    │  │  Manager    │  │   Monitor   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐        │
│  │   Easing    │  │   Spring    │  │   Timeline  │        │
│  │  Functions  │  │   Physics   │  │   Manager   │        │
│  └─────────────┘  └─────────────┘  └─────────────┘        │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Animation Engine Interface

```rust
pub trait AnimationEngine {
    /// Check if engine is available in current environment
    fn is_available(&self) -> bool;
    
    /// Start a new animation
    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle>;
    
    /// Stop an animation
    fn stop(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Pause an animation
    fn pause(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Resume an animation
    fn resume(&mut self, handle: AnimationHandle) -> Result<()>;
    
    /// Update engine state (for RAF-based engines)
    fn tick(&mut self, timestamp: f64) -> Result<()>;
    
    /// Get animation state
    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState>;
    
    /// Check if animation is running
    fn is_running(&self, handle: AnimationHandle) -> bool;
}
```

### 2. Animation Configuration

```rust
pub struct AnimationConfig {
    /// Target element
    pub element: Element,
    
    /// Animation properties
    pub properties: HashMap<String, PropertyAnimation>,
    
    /// Duration in milliseconds
    pub duration: f64,
    
    /// Delay before start
    pub delay: f64,
    
    /// Easing function
    pub easing: EasingFunction,
    
    /// Repeat configuration
    pub repeat: RepeatConfig,
    
    /// Direction (normal, reverse, alternate)
    pub direction: AnimationDirection,
    
    /// Fill mode (none, forwards, backwards, both)
    pub fill_mode: FillMode,
}
```

### 3. Property Animation

```rust
pub struct PropertyAnimation {
    /// Property name (e.g., "opacity", "transform")
    pub name: String,
    
    /// From value
    pub from: AnimationValue,
    
    /// To value
    pub to: AnimationValue,
    
    /// Current value (updated during animation)
    pub current: AnimationValue,
    
    /// Property unit (px, %, deg, etc.)
    pub unit: String,
    
    /// Whether to use hardware acceleration
    pub hardware_accelerated: bool,
}
```

## Engine Implementations

### 1. WAAPI Engine (Primary)

**Advantages:**
- Native browser optimization
- Hardware acceleration
- Minimal CPU overhead
- Built-in timing control

**Implementation Strategy:**
```rust
pub struct WaapiEngine {
    animations: HashMap<AnimationHandle, WaapiAnimation>,
    next_handle: AnimationHandle,
}

impl AnimationEngine for WaapiEngine {
    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        let keyframes = self.build_keyframes(config)?;
        let animation = self.element.animate(&keyframes, &options)?;
        
        let handle = self.next_handle;
        self.animations.insert(handle, WaapiAnimation { animation });
        self.next_handle = handle.next();
        
        Ok(handle)
    }
}
```

### 2. RAF Engine (Fallback)

**Use Cases:**
- Older browsers without WAAPI
- Custom timing requirements
- Complex easing functions

**Implementation Strategy:**
```rust
pub struct RafEngine {
    animations: HashMap<AnimationHandle, RafAnimation>,
    raf_id: Option<i32>,
    last_timestamp: f64,
}

impl AnimationEngine for RafEngine {
    fn tick(&mut self, timestamp: f64) -> Result<()> {
        let delta = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;
        
        for animation in self.animations.values_mut() {
            animation.update(delta)?;
        }
        
        Ok(())
    }
}
```

### 3. CSS Engine (Fallback)

**Use Cases:**
- Maximum compatibility
- Simple animations
- CSS-only implementations

**Implementation Strategy:**
```rust
pub struct CssEngine {
    animations: HashMap<AnimationHandle, CssAnimation>,
}

impl AnimationEngine for CssEngine {
    fn animate(&mut self, config: &AnimationConfig) -> Result<AnimationHandle> {
        let css_transition = self.build_css_transition(config)?;
        self.element.style().set_property("transition", &css_transition)?;
        
        // Apply final values
        for (name, value) in &config.properties {
            self.element.style().set_property(name, &value.to_css())?;
        }
        
        Ok(handle)
    }
}
```

## State Management

### Animation State

```rust
pub enum PlaybackState {
    Idle,
    Running,
    Paused,
    Finished,
    Cancelled,
    Error(String),
}

pub struct AnimationState {
    pub handle: AnimationHandle,
    pub state: PlaybackState,
    pub progress: f64, // 0.0 to 1.0
    pub current_time: f64,
    pub duration: f64,
    pub start_time: f64,
    pub end_time: f64,
}
```

### State Manager

```rust
pub struct AnimationStateManager {
    states: HashMap<AnimationHandle, AnimationState>,
    event_listeners: Vec<Box<dyn Fn(AnimationEvent)>>,
}

impl AnimationStateManager {
    pub fn update_state(&mut self, handle: AnimationHandle, state: PlaybackState) {
        if let Some(anim_state) = self.states.get_mut(&handle) {
            anim_state.state = state;
            self.notify_listeners(AnimationEvent::StateChanged(handle, state));
        }
    }
    
    pub fn update_progress(&mut self, handle: AnimationHandle, progress: f64) {
        if let Some(anim_state) = self.states.get_mut(&handle) {
            anim_state.progress = progress;
            self.notify_listeners(AnimationEvent::ProgressChanged(handle, progress));
        }
    }
}
```

## Memory Management

### WASM-Compatible Patterns

```rust
pub struct AnimationManager {
    engine: Box<dyn AnimationEngine>,
    state_manager: AnimationStateManager,
    memory_pool: MemoryPool,
}

impl AnimationManager {
    pub fn new() -> Self {
        let engine = Self::select_best_engine();
        Self {
            engine,
            state_manager: AnimationStateManager::new(),
            memory_pool: MemoryPool::new(1024 * 1024), // 1MB pool
        }
    }
    
    fn select_best_engine() -> Box<dyn AnimationEngine> {
        if WaapiEngine::is_available() {
            Box::new(WaapiEngine::new())
        } else if RafEngine::is_available() {
            Box::new(RafEngine::new())
        } else {
            Box::new(CssEngine::new())
        }
    }
}
```

## Performance Considerations

### 1. Hardware Acceleration
- Use `transform` and `opacity` for hardware acceleration
- Avoid animating layout properties
- Batch DOM updates

### 2. Memory Optimization
- Object pooling for animation objects
- Reuse of animation handles
- Minimal allocations in hot paths

### 3. Timing Optimization
- Use `requestAnimationFrame` for smooth timing
- Batch multiple animations
- Skip frames when necessary

## Error Handling

### Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum AnimationError {
    #[error("Engine not available: {0}")]
    EngineUnavailable(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    
    #[error("Animation not found: {0}")]
    AnimationNotFound(AnimationHandle),
    
    #[error("Property error: {0}")]
    PropertyError(String),
    
    #[error("Timing error: {0}")]
    TimingError(String),
}
```

### Error Recovery

```rust
impl AnimationManager {
    fn handle_engine_error(&mut self, error: AnimationError) -> Result<()> {
        match error {
            AnimationError::EngineUnavailable(_) => {
                // Fallback to next available engine
                self.engine = Self::select_fallback_engine();
                Ok(())
            }
            _ => Err(error),
        }
    }
}
```

## Testing Strategy

### 1. Unit Tests
- Engine availability detection
- Configuration validation
- State management
- Error handling

### 2. Integration Tests
- Cross-engine compatibility
- Performance benchmarks
- Memory usage validation
- WASM compatibility

### 3. Browser Tests
- Cross-browser compatibility
- Hardware acceleration validation
- Performance in real scenarios
- User interaction testing

## Migration Strategy

### Phase 1: Core Implementation
1. Implement WAAPI engine
2. Add RAF fallback
3. Implement state management
4. Add basic testing

### Phase 2: Integration
1. Integrate with existing components
2. Add CSS fallback
3. Implement error handling
4. Add performance monitoring

### Phase 3: Optimization
1. Memory optimization
2. Performance tuning
3. Advanced features
4. Comprehensive testing

## Conclusion

This design provides a solid foundation for a unified, performant animation engine that addresses the current architectural issues while maintaining compatibility and performance requirements.
