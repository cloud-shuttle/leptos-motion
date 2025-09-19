# Animation Engine Design Document

## Problem Analysis

### Current Issues
1. **Borrowing Conflicts**: RAF callbacks try to move `Rc<RefCell<T>>` values into closures, causing `FnOnce` vs `FnMut` conflicts
2. **Memory Leaks**: Closures that forget themselves with no cleanup mechanism
3. **Performance Issues**: Continuous RAF loops even when no animations are running
4. **Complex State Management**: Multiple animation engines with overlapping responsibilities
5. **API Conflicts**: Leptos `ElementExt` vs `web_sys` style APIs

### Root Cause
The fundamental issue is trying to use RAF (requestAnimationFrame) with Rust's ownership system. RAF callbacks need to be `FnMut` but we're trying to move owned values into them.

## Solution: Event-Driven Callback Architecture

### Core Principles
1. **Event-Driven**: Animations trigger on property changes, not continuous polling
2. **Callback-Based**: Store callbacks instead of moving state into closures
3. **Weak References**: Use `Weak<RefCell<T>>` to avoid circular references
4. **Proper Cleanup**: Automatic cleanup when components drop
5. **Single Responsibility**: One engine per animation type

## Architecture Design

### 1. Animation Manager (Singleton)
```rust
pub struct AnimationManager {
    // Global registry of active animations
    animations: Rc<RefCell<HashMap<String, Box<dyn Animation>>>>
}

impl AnimationManager {
    // Register animation with unique ID
    pub fn register(&self, id: String, animation: Box<dyn Animation>) -> AnimationHandle
    
    // Unregister animation
    pub fn unregister(&self, handle: AnimationHandle)
    
    // Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<Weak<RefCell<Box<dyn Animation>>>>
}
```

### 2. Animation Trait
```rust
pub trait Animation: Send + Sync {
    // Start the animation
    fn start(&mut self) -> Result<()>;
    
    // Stop the animation
    fn stop(&mut self) -> Result<()>;
    
    // Check if animation is complete
    fn is_complete(&self) -> bool;
    
    // Get animation progress (0.0 to 1.0)
    fn progress(&self) -> f64;
    
    // Update animation state
    fn update(&mut self, delta_time: f64) -> Result<()>;
    
    // Get animation ID
    fn id(&self) -> &str;
}
```

### 3. CSS Transition Animation
```rust
pub struct CssTransitionAnimation {
    id: String,
    element: Element,
    properties: HashMap<String, AnimationValue>,
    transition: Transition,
    start_time: Option<f64>,
    duration: f64,
    is_complete: bool,
}

impl Animation for CssTransitionAnimation {
    fn start(&mut self) -> Result<()> {
        // Apply CSS transition and target values
        // No RAF loop needed - CSS handles the animation
    }
    
    fn update(&mut self, _delta_time: f64) -> Result<()> {
        // CSS transitions don't need updates
        Ok(())
    }
}
```

### 4. JavaScript Animation (for complex cases)
```rust
pub struct JavaScriptAnimation {
    id: String,
    element: Element,
    properties: HashMap<String, AnimationValue>,
    transition: Transition,
    start_time: Option<f64>,
    duration: f64,
    is_complete: bool,
    // Callback for completion
    on_complete: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl Animation for JavaScriptAnimation {
    fn start(&mut self) -> Result<()> {
        // Use CSS transitions for simple properties
        // Use JavaScript for complex interpolations
    }
    
    fn update(&mut self, delta_time: f64) -> Result<()> {
        // Update complex animations that CSS can't handle
    }
}
```

### 5. Animation Handle
```rust
pub struct AnimationHandle {
    id: String,
    manager: Weak<RefCell<AnimationManager>>,
}

impl AnimationHandle {
    pub fn stop(self) -> Result<()> {
        if let Some(manager) = self.manager.upgrade() {
            manager.borrow_mut().unregister(self.id.clone());
        }
        Ok(())
    }
    
    pub fn is_running(&self) -> bool {
        if let Some(manager) = self.manager.upgrade() {
            if let Some(animation) = manager.borrow().get_animation(&self.id) {
                if let Some(animation) = animation.upgrade() {
                    return !animation.borrow().is_complete();
                }
            }
        }
        false
    }
}
```

## Implementation Plan

### Phase 1: Core Infrastructure (Week 1)
1. **AnimationManager**: Global singleton for managing animations
2. **Animation Trait**: Base trait for all animation types
3. **AnimationHandle**: Handle for controlling animations
4. **Basic CSS Animation**: Simple CSS transition-based animations

### Phase 2: Advanced Animations (Week 2)
1. **JavaScript Animation**: For complex interpolations
2. **Keyframe Animation**: Multi-step animations
3. **Stagger Animation**: Delayed animations for multiple elements
4. **Spring Animation**: Physics-based animations

### Phase 3: Integration (Week 3)
1. **MotionDiv Integration**: Connect to Leptos components
2. **Event Handling**: Hover, tap, drag events
3. **Performance Optimization**: Lazy loading, cleanup
4. **Testing**: Unit tests and integration tests

### Phase 4: Advanced Features (Week 4)
1. **Layout Animations**: Animate layout changes
2. **Gesture Recognition**: Drag, pinch, rotate
3. **Animation Sequences**: Chain multiple animations
4. **Performance Monitoring**: FPS tracking, memory usage

## Benefits of This Approach

### 1. No Borrowing Issues
- **Problem**: RAF callbacks moving `Rc<RefCell<T>>` values
- **Solution**: Event-driven updates, no continuous loops
- **Result**: Clean compilation, no ownership conflicts

### 2. Better Performance
- **Problem**: Continuous RAF loops even when idle
- **Solution**: CSS transitions + event-driven updates
- **Result**: Lower CPU usage, better battery life

### 3. Proper Memory Management
- **Problem**: Memory leaks from forgotten closures
- **Solution**: Weak references + automatic cleanup
- **Result**: No memory leaks, proper resource management

### 4. Easier Testing
- **Problem**: Complex state management, hard to test
- **Solution**: Clear interfaces, dependency injection
- **Result**: Unit testable, mockable components

### 5. Better Architecture
- **Problem**: Multiple overlapping animation engines
- **Solution**: Single responsibility, clear separation
- **Result**: Maintainable, extensible codebase

## Migration Strategy

### Step 1: Create New Architecture
- Implement `AnimationManager` and `Animation` trait
- Create `CssTransitionAnimation` for simple cases
- Add `AnimationHandle` for control

### Step 2: Update MotionDiv
- Replace complex animation engines with `AnimationManager`
- Use CSS transitions for simple properties
- Use JavaScript animations for complex cases

### Step 3: Remove Old Code
- Delete broken animation engines
- Remove RAF-based implementations
- Clean up unused imports and dependencies

### Step 4: Add Advanced Features
- Implement keyframe animations
- Add stagger animations
- Create layout animations

## Success Metrics

### Technical Metrics
- ✅ Zero compilation errors
- ✅ Zero memory leaks
- ✅ < 5ms animation setup time
- ✅ 60fps smooth animations
- ✅ < 1MB memory usage

### Code Quality Metrics
- ✅ 100% test coverage for core components
- ✅ Zero clippy warnings
- ✅ Clear documentation
- ✅ Consistent API design

### User Experience Metrics
- ✅ Smooth animations on all devices
- ✅ Responsive to user interactions
- ✅ Consistent behavior across browsers
- ✅ Easy to use API

## Conclusion

This event-driven, callback-based architecture solves all the current issues:

1. **Eliminates borrowing conflicts** by avoiding RAF loops
2. **Improves performance** by using CSS transitions
3. **Prevents memory leaks** with proper cleanup
4. **Simplifies testing** with clear interfaces
5. **Follows Rust best practices** with proper ownership

The key insight is to work with Rust's ownership system, not against it. By using events and callbacks instead of continuous polling, we get better performance and cleaner code.
