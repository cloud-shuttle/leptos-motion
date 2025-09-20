# Type System Unification Design Document

**Date**: 2024-12-19  
**Status**: DESIGN - Implementation Ready  
**Priority**: P0 - Production Blocking  

## 🎯 **Objective**

Design a unified type system that eliminates conflicts between leptos-motion crates and ensures consistent API contracts.

## 🔍 **Problem Statement**

### **Current Type System Issues**
1. **Multiple AnimationConfig definitions** causing trait implementation failures
2. **AnimationHandle scope issues** preventing method access
3. **PlaybackState import problems** blocking enum variant access
4. **Type system inconsistencies** between core and DOM crates

### **Root Causes**
- **Duplicate type definitions** across crates
- **Import path conflicts** between modules
- **Feature flag dependencies** not properly managed
- **API contract mismatches** between trait and implementation

## 🏗️ **Design Solution**

### **Type System Architecture**

#### **Single Source of Truth Principle**
```
leptos-motion-core (Source of Truth)
├── types.rs              # Core types
├── engine/traits.rs      # Engine types
└── lib.rs               # Re-exports

leptos-motion-dom (Consumer)
├── Uses core types directly
├── No duplicate definitions
└── Consistent API contracts
```

### **Type Hierarchy Design**

#### **Core Types (Always Available)**
```rust
// types.rs - Core animation types
pub struct AnimationHandle(pub u64);
pub enum AnimationValue { ... }
pub struct AnimationTarget { ... }
pub struct Transition { ... }
pub enum Easing { ... }
```

#### **Engine Types (Feature Gated)**
```rust
// engine/traits.rs - Engine-specific types
#[cfg(feature = "basic-animation")]
pub enum PlaybackState { ... }

#[cfg(feature = "basic-animation")]
pub struct AnimationConfig { ... }

#[cfg(feature = "basic-animation")]
pub trait AnimationEngine { ... }
```

#### **Integration Types (Crate Specific)**
```rust
// DOM crate - Integration types only
pub struct DomAnimationHandle { ... }  // Wrapper around core handle
pub struct DomAnimationConfig { ... }  // DOM-specific configuration
```

## 🔧 **Implementation Strategy**

### **Phase 1: Core Type Unification**

#### **1.1 AnimationHandle Unification**
```rust
// Core crate - types.rs (Source of Truth)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationHandle(pub u64);

impl AnimationHandle {
    pub fn new(id: u64) -> Self {
        Self(id)
    }
    
    pub fn id(self) -> u64 {
        self.0
    }
    
    pub fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

// Core crate - lib.rs (Re-export)
pub use types::AnimationHandle;

// DOM crate - Use core definition
pub use leptos_motion_core::AnimationHandle;
```

#### **1.2 PlaybackState Unification**
```rust
// Core crate - engine/traits.rs (Source of Truth)
#[cfg(feature = "basic-animation")]
#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Idle,
    Pending,
    Running,
    Paused,
    Completed,
    Finished,
    Cancelled,
    Error(String),
}

// Core crate - lib.rs (Re-export)
#[cfg(feature = "basic-animation")]
pub use engine::PlaybackState;

// DOM crate - Use core definition
#[cfg(feature = "basic-animation")]
pub use leptos_motion_core::engine::PlaybackState;
```

#### **1.3 AnimationConfig Unification**
```rust
// Core crate - engine/traits.rs (Source of Truth)
#[cfg(feature = "basic-animation")]
pub struct AnimationConfig {
    pub element: Option<web_sys::Element>,
    pub values: HashMap<String, AnimationValue>,
    pub transition: Transition,
    pub hardware_accelerated: bool,
    pub priority: AnimationPriority,
}

// Core crate - lib.rs (Re-export)
#[cfg(feature = "basic-animation")]
pub use engine::AnimationConfig;

// DOM crate - Use core definition
#[cfg(feature = "basic-animation")]
pub use leptos_motion_core::engine::AnimationConfig;
```

### **Phase 2: DOM Crate Integration**

#### **2.1 Remove Duplicate Definitions**
```rust
// DOM crate - Remove local definitions
// ❌ Remove: pub struct AnimationHandle { ... }
// ❌ Remove: pub enum PlaybackState { ... }
// ❌ Remove: pub struct AnimationConfig { ... }

// ✅ Use core definitions
pub use leptos_motion_core::AnimationHandle;
pub use leptos_motion_core::engine::PlaybackState;
pub use leptos_motion_core::engine::AnimationConfig;
```

#### **2.2 Update AnimationEngine Implementation**
```rust
// DOM crate - dom_animation_engine.rs
use leptos_motion_core::engine::{AnimationEngine, PlaybackState, AnimationConfig};
use leptos_motion_core::{AnimationHandle, Result};

impl AnimationEngine for DomAnimationEngine {
    fn animate(&mut self, animation: &AnimationConfig) -> Result<AnimationHandle> {
        let handle = AnimationHandle::new(self.next_handle_id);
        self.next_handle_id += 1;
        self.handles.insert(handle, "dom_animation".to_string());
        Ok(handle)
    }

    fn get_state(&self, handle: AnimationHandle) -> Result<PlaybackState> {
        if self.handles.contains_key(&handle) {
            Ok(PlaybackState::Running)
        } else {
            Ok(PlaybackState::Idle)
        }
    }
}
```

### **Phase 3: Type System Validation**

#### **3.1 Compilation Tests**
```rust
#[cfg(test)]
mod type_system_tests {
    use super::*;

    #[test]
    fn test_animation_handle_creation() {
        let handle = AnimationHandle::new(1);
        assert_eq!(handle.id(), 1);
    }

    #[test]
    fn test_playback_state_variants() {
        let idle = PlaybackState::Idle;
        let running = PlaybackState::Running;
        assert_ne!(idle, running);
    }

    #[test]
    fn test_animation_config_creation() {
        let config = AnimationConfig {
            element: None,
            values: HashMap::new(),
            transition: Transition::default(),
            hardware_accelerated: false,
            priority: AnimationPriority::Normal,
        };
        assert!(config.values.is_empty());
    }
}
```

#### **3.2 Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_dom_animation_engine_creation() {
        let mut engine = DomAnimationEngine::new();
        assert!(engine.is_available());
    }

    #[test]
    fn test_animation_engine_trait_implementation() {
        let mut engine = DomAnimationEngine::new();
        let config = AnimationConfig::default();
        let handle = engine.animate(&config).unwrap();
        assert!(engine.is_running(handle));
    }
}
```

## 📊 **Type System Matrix**

| Type | Core Crate | DOM Crate | WebGL Crate | Studio Crate |
|------|------------|-----------|-------------|--------------|
| AnimationHandle | ✅ Source | ✅ Import | ✅ Import | ✅ Import |
| PlaybackState | ✅ Source | ✅ Import | ✅ Import | ✅ Import |
| AnimationConfig | ✅ Source | ✅ Import | ✅ Import | ✅ Import |
| AnimationValue | ✅ Source | ✅ Import | ✅ Import | ✅ Import |
| Transition | ✅ Source | ✅ Import | ✅ Import | ✅ Import |
| Easing | ✅ Source | ✅ Import | ✅ Import | ✅ Import |

## 🔄 **Migration Strategy**

### **Step 1: Core Crate Updates**
1. **Consolidate type definitions** in core crate
2. **Add feature flags** for engine types
3. **Update re-exports** in lib.rs
4. **Add comprehensive tests**

### **Step 2: DOM Crate Updates**
1. **Remove duplicate definitions**
2. **Update imports** to use core types
3. **Fix AnimationEngine implementation**
4. **Test compilation**

### **Step 3: Validation**
1. **Run all tests** across crates
2. **Validate examples** compilation
3. **Check API contracts**
4. **Performance testing**

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_animation_handle_methods() {
        let handle = AnimationHandle::new(42);
        assert_eq!(handle.id(), 42);
        assert_eq!(handle.next().id(), 43);
    }

    #[test]
    fn test_playback_state_equality() {
        assert_eq!(PlaybackState::Idle, PlaybackState::Idle);
        assert_ne!(PlaybackState::Idle, PlaybackState::Running);
    }
}
```

### **Integration Tests**
```rust
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_cross_crate_type_compatibility() {
        // Test that types work across crate boundaries
        let handle = AnimationHandle::new(1);
        let state = PlaybackState::Running;
        let config = AnimationConfig::default();
        
        // Verify types are compatible
        assert!(handle.id() > 0);
        assert_eq!(state, PlaybackState::Running);
        assert!(config.values.is_empty());
    }
}
```

### **Compilation Tests**
```rust
#[cfg(test)]
mod compilation_tests {
    use super::*;

    #[test]
    fn test_all_types_accessible() {
        // Test that all types can be imported and used
        let _handle = AnimationHandle::new(1);
        let _state = PlaybackState::Idle;
        let _config = AnimationConfig::default();
        let _value = AnimationValue::Number(1.0);
        let _transition = Transition::default();
        let _easing = Easing::Linear;
    }
}
```

## 📈 **Success Criteria**

### **Compilation Success**
- [ ] 0 compilation errors across all crates
- [ ] All type imports resolved correctly
- [ ] AnimationEngine trait implementation works

### **Functional Success**
- [ ] AnimationHandle::new accessible and working
- [ ] PlaybackState::Idle accessible and working
- [ ] AnimationConfig type alignment resolved

### **Integration Success**
- [ ] Cross-crate type compatibility
- [ ] API contracts satisfied
- [ ] No type system conflicts

## 🚀 **Implementation Plan**

### **Week 1: Core Crate Unification**
- [ ] Consolidate type definitions
- [ ] Add feature flags
- [ ] Update re-exports
- [ ] Add tests

### **Week 2: DOM Crate Integration**
- [ ] Remove duplicate definitions
- [ ] Update imports
- [ ] Fix trait implementation
- [ ] Test compilation

### **Week 3: Validation & Testing**
- [ ] Run comprehensive tests
- [ ] Validate examples
- [ ] Check API contracts
- [ ] Performance testing

### **Week 4: Documentation & Final Validation**
- [ ] Update documentation
- [ ] Final testing
- [ ] Release preparation

## 🔄 **Rollback Plan**

### **If Issues Arise**
1. **Revert type system changes**
2. **Restore original definitions**
3. **Investigate alternative approaches**
4. **Document lessons learned**

### **Alternative Approaches**
1. **Type aliases** for compatibility
2. **Conditional compilation** with cfg attributes
3. **Separate type modules** for different use cases

---

**Design completed**: 2024-12-19  
**Next action**: Implement type system unification
