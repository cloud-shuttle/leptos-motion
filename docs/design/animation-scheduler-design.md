# Animation Scheduler Component Design

## Overview
Central animation scheduler that coordinates all animation engines and provides unified update loop management.

## Current Issues
- **CRITICAL**: No centralized scheduler exists
- OptimizedAnimationManager has no update driver
- EventDrivenMotionDiv registers animations but nothing ticks them
- SimpleAnimationEngine schedules empty callbacks

## Design Goals
- Single source of truth for animation scheduling
- Coordinate RAF, WAAPI, and CSS transition engines
- Optimal performance with batched updates
- Automatic cleanup of finished animations
- Priority-based animation queuing

## API Design

### Core Types
```rust
pub struct AnimationScheduler {
    engines: HashMap<EngineType, Box<dyn AnimationEngine>>,
    active_animations: HashMap<AnimationHandle, ScheduledAnimation>,
    update_queue: VecDeque<SchedulerCommand>,
    raf_handle: Option<i32>,
    performance: web_sys::Performance,
}

pub struct ScheduledAnimation {
    handle: AnimationHandle,
    engine_type: EngineType,
    priority: AnimationPriority,
    start_time: f64,
    last_update: f64,
}

pub enum SchedulerCommand {
    StartAnimation(AnimationConfig),
    StopAnimation(AnimationHandle),
    PauseAnimation(AnimationHandle),
    ResumeAnimation(AnimationHandle),
    UpdateEngine(EngineType),
}
```

### Public Interface
```rust
impl AnimationScheduler {
    pub fn new() -> Self
    pub fn register_engine(&mut self, engine_type: EngineType, engine: Box<dyn AnimationEngine>)
    pub fn schedule_animation(&mut self, config: AnimationConfig) -> Result<AnimationHandle>
    pub fn start(&mut self) -> Result<()>
    pub fn stop(&mut self)
    pub fn get_active_count(&self) -> usize
    pub fn get_performance_stats(&self) -> PerformanceStats
}
```

## Implementation Plan

### Phase 1: Core Scheduler (Week 2, Day 1-2)
**File**: `crates/leptos-motion-core/src/scheduler/mod.rs`
**Target Lines**: <250

```rust
impl AnimationScheduler {
    fn start_update_loop(&mut self) {
        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            self.tick(timestamp);
            if self.has_active_animations() {
                self.request_next_frame();
            }
        }) as Box<dyn FnMut(f64)>);
        
        self.raf_handle = Some(request_animation_frame(&closure));
        self.store_closure(closure); // Proper memory management
    }
    
    fn tick(&mut self, timestamp: f64) {
        self.process_command_queue();
        self.update_all_engines(timestamp);
        self.cleanup_finished_animations();
    }
}
```

### Phase 2: Engine Coordination (Week 2, Day 3)
**File**: `crates/leptos-motion-core/src/scheduler/engine_coordinator.rs`
**Target Lines**: <150

```rust
pub struct EngineCoordinator {
    engines: HashMap<EngineType, Box<dyn AnimationEngine>>,
    engine_stats: HashMap<EngineType, EngineStats>,
}

impl EngineCoordinator {
    pub fn route_animation(&mut self, config: &AnimationConfig) -> EngineType
    pub fn update_engine(&mut self, engine_type: EngineType, delta: f64)
    pub fn get_optimal_engine(&self, properties: &[String]) -> EngineType
}
```

### Phase 3: Priority Queue (Week 2, Day 4)
**File**: `crates/leptos-motion-core/src/scheduler/priority_queue.rs`
**Target Lines**: <100

```rust
pub struct AnimationPriorityQueue {
    high_priority: VecDeque<ScheduledAnimation>,
    normal_priority: VecDeque<ScheduledAnimation>,
    low_priority: VecDeque<ScheduledAnimation>,
}

impl AnimationPriorityQueue {
    pub fn push(&mut self, animation: ScheduledAnimation)
    pub fn pop_highest_priority(&mut self) -> Option<ScheduledAnimation>
    pub fn remove(&mut self, handle: AnimationHandle) -> Option<ScheduledAnimation>
}
```

## File Structure
```
crates/leptos-motion-core/src/scheduler/
├── mod.rs                    # Main scheduler (<250 lines)
├── engine_coordinator.rs     # Engine routing (<150 lines)
├── priority_queue.rs         # Animation prioritization (<100 lines)
├── performance_monitor.rs    # Performance tracking (<120 lines)
└── memory_manager.rs         # Cleanup and GC (<80 lines)
```

## Engine Selection Logic

### Automatic Engine Routing
```rust
fn select_optimal_engine(config: &AnimationConfig) -> EngineType {
    match config.properties.as_slice() {
        // WAAPI optimal for complex keyframe animations
        props if props.len() > 3 && has_keyframes(config) => EngineType::Waapi,
        
        // RAF optimal for transforms and frequent updates
        props if contains_transforms(props) => EngineType::Raf,
        
        // CSS optimal for simple transitions
        props if is_simple_transition(props, config) => EngineType::CssTransition,
        
        // Default to RAF
        _ => EngineType::Raf,
    }
}
```

### Performance Monitoring
```rust
pub struct PerformanceStats {
    pub fps: f64,
    pub frame_time_ms: f64,
    pub active_animations: usize,
    pub dropped_frames: u32,
    pub engine_utilization: HashMap<EngineType, f64>,
}
```

## Integration Points

### With OptimizedAnimationManager
```rust
impl AnimationScheduler {
    fn integrate_with_manager(&mut self, manager: &mut OptimizedAnimationManager) {
        // Replace manager's missing update loop
        self.register_update_callback(Box::new(move |delta| {
            manager.update_optimized(delta);
        }));
    }
}
```

### With EventDrivenMotionDiv
```rust
impl EventDrivenMotionDiv {
    pub fn register_with_scheduler(&self, scheduler: &mut AnimationScheduler) {
        // Route all animations through scheduler instead of direct engine calls
        for animation in &self.registered_animations {
            scheduler.schedule_animation(animation.config.clone())?;
        }
    }
}
```

## Testing Strategy
- Unit tests for scheduler logic
- Integration tests with all engine types
- Performance benchmarks (target: 60fps)
- Memory leak detection
- Stress tests with 100+ simultaneous animations

## Performance Requirements
- Frame processing: <16.67ms (60fps)
- Animation routing: <0.1ms
- Command queue processing: <1ms
- Memory cleanup: <5ms per cleanup cycle

## Memory Management
- Automatic cleanup of finished animations
- Weak references to DOM elements
- Closure lifecycle management
- Periodic garbage collection of unused resources

## Dependencies
```rust
use std::collections::{HashMap, VecDeque};
use web_sys::{window, Performance};
use wasm_bindgen::{prelude::*, JsCast};
```

## Success Criteria
- [ ] Single RAF loop driving all animations
- [ ] OptimizedAnimationManager receiving updates
- [ ] EventDrivenMotionDiv animations scheduled
- [ ] All engines coordinated through scheduler
- [ ] 60fps performance maintained
- [ ] No memory leaks
- [ ] All files under 250 lines
