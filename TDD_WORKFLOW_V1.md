# TDD Workflow & Standards for Leptos Motion v1.0

## 🎯 **Daily TDD Workflow**

### **Morning Routine: Red Phase** (30-45 minutes)
```bash
# 1. Review yesterday's progress
git log --oneline -5

# 2. Start TDD watch mode
cargo watch -x "nextest run --fail-fast" -c

# 3. Write failing test for today's feature
vim tests/phase1_production_hardening.rs

# 4. Verify test fails for right reason
cargo nextest run --fail-fast
```

### **Implementation Phase: Green Phase** (2-3 hours)
```bash
# 1. Write minimal code to make test pass
# Focus: Make it work, don't make it perfect yet

# 2. Run tests frequently
cargo nextest run --fail-fast

# 3. Commit when test passes
git add . && git commit -m "feat: make test_feature_name pass"
```

### **Afternoon Routine: Refactor Phase** (1-2 hours)
```bash
# 1. Improve code quality while keeping tests green
# 2. Add edge case tests
# 3. Optimize performance
# 4. Update documentation

# Final verification
cargo test --all-features --workspace
cargo tarpaulin --out Html --output-dir target/tarpaulin
```

## 🧪 **TDD Standards & Conventions**

### **Test File Organization**
```
tests/
├── phase1_production_hardening/
│   ├── core_engine_refinement.rs
│   ├── bundle_optimization.rs
│   └── cross_browser_compatibility.rs
├── phase2_advanced_features/
│   ├── timeline_animations.rs
│   ├── gesture_recognition.rs
│   └── performance_monitoring.rs
├── phase3_developer_experience/
│   └── enhanced_dev_tools.rs
└── phase4_production_launch/
    └── ecosystem_integration.rs
```

### **Test Naming Convention**
```rust
// Pattern: test_{what}_{when}_{expected}
#[test]
fn test_animation_engine_when_concurrent_animations_then_handles_gracefully() {
    // Test implementation
}

#[test]
fn test_bundle_size_when_minimal_features_then_under_50kb() {
    // Test implementation
}

#[test] 
fn test_memory_cleanup_when_animation_completes_then_no_leaks() {
    // Test implementation
}
```

### **Test Structure Template**
```rust
#[rstest]
#[case::basic_scenario(input1, expected1)]
#[case::edge_case(input2, expected2)]
#[case::error_condition(input3, expected3)]
fn test_feature_behavior(
    #[case] input: InputType,
    #[case] expected: OutputType,
) {
    // Arrange
    let context = setup_test_context();
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected);
    
    // Cleanup (if needed)
    cleanup_test_context(context);
}
```

### **Property-Based Testing Template**
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_interpolation_properties(
        start in any::<f64>(),
        end in any::<f64>(),
        progress in 0.0..=1.0f64
    ) {
        let result = interpolate(start, end, progress);
        
        // Property 1: Result is between start and end
        prop_assert!(result >= start.min(end));
        prop_assert!(result <= start.max(end));
        
        // Property 2: Progress 0 gives start
        prop_assert!((interpolate(start, end, 0.0) - start).abs() < f64::EPSILON);
        
        // Property 3: Progress 1 gives end
        prop_assert!((interpolate(start, end, 1.0) - end).abs() < f64::EPSILON);
    }
}
```

## 📊 **Quality Gates & Metrics**

### **Pre-Commit Checks**
```bash
#!/bin/bash
# .git/hooks/pre-commit

echo "Running TDD quality gates..."

# 1. All tests must pass
cargo nextest run --workspace || exit 1

# 2. Coverage must be above threshold
coverage=$(cargo tarpaulin --output-format Json | jq '.files | to_entries | map(.value.coverage) | add / length')
if (( $(echo "$coverage < 85" | bc -l) )); then
    echo "Coverage $coverage% below 85% threshold"
    exit 1
fi

# 3. No clippy warnings
cargo clippy --workspace --all-targets -- -D warnings || exit 1

# 4. Code is formatted
cargo fmt --check || exit 1

echo "All quality gates passed! ✅"
```

### **Performance Benchmarks**
```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_animation_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("animation_engine");
    
    group.bench_function("single_animation", |b| {
        let engine = AnimationEngine::new();
        let config = create_test_animation_config();
        
        b.iter(|| engine.animate(config.clone()))
    });
    
    group.bench_function("concurrent_animations", |b| {
        let engine = AnimationEngine::new();
        let configs: Vec<_> = (0..10).map(|_| create_test_animation_config()).collect();
        
        b.iter(|| {
            for config in &configs {
                engine.animate(config.clone());
            }
        })
    });
    
    group.finish();
}

criterion_group!(benches, benchmark_animation_engine);
criterion_main!(benches);
```

## 🚀 **Phase 1 Implementation Plan**

### **Week 1: Core Engine TDD Implementation**

#### **Day 1: Animation Engine Concurrent Handling**
```rust
// File: tests/phase1_production_hardening/core_engine_refinement.rs

use rstest::*;
use leptos_motion_core::*;

#[rstest]
#[case::two_animations(2)]
#[case::ten_animations(10)]
#[case::hundred_animations(100)]
fn test_animation_engine_handles_concurrent_animations(#[case] animation_count: usize) {
    // Red Phase: This test will fail initially
    let engine = AnimationEngine::new();
    let animations: Vec<_> = (0..animation_count)
        .map(|i| create_test_animation(i))
        .collect();
    
    // Act: Start all animations
    let handles: Vec<_> = animations.iter()
        .map(|anim| engine.start_animation(anim.clone()))
        .collect();
    
    // Assert: All animations should run without conflicts
    for handle in handles {
        assert!(handle.is_active());
    }
    
    // Assert: Engine should maintain stable state
    assert_eq!(engine.active_animations_count(), animation_count);
}

#[test]
fn test_memory_cleanup_after_animation_completion() {
    // Red Phase: Test memory management
    let engine = AnimationEngine::new();
    let initial_memory = get_memory_usage();
    
    // Start and complete 100 animations
    for i in 0..100 {
        let animation = create_test_animation(i);
        let handle = engine.start_animation(animation);
        
        // Wait for completion
        wait_for_animation_completion(&handle);
    }
    
    // Force garbage collection
    force_gc();
    
    let final_memory = get_memory_usage();
    let memory_growth = final_memory - initial_memory;
    
    // Assert: Memory growth should be minimal (<1MB)
    assert!(memory_growth < 1024 * 1024, "Memory leak detected: {}bytes", memory_growth);
}

#[test]
fn test_error_recovery_from_invalid_animation_values() {
    let engine = AnimationEngine::new();
    
    // Test invalid animation values
    let invalid_animations = vec![
        create_animation_with_nan_values(),
        create_animation_with_infinite_values(),
        create_animation_with_negative_duration(),
    ];
    
    for invalid_anim in invalid_animations {
        // Should not panic, should return error
        match engine.start_animation(invalid_anim) {
            Ok(_) => panic!("Should have returned error for invalid animation"),
            Err(e) => assert!(matches!(e, AnimationError::InvalidValue(_))),
        }
    }
    
    // Engine should still be functional
    let valid_animation = create_test_animation(1);
    assert!(engine.start_animation(valid_animation).is_ok());
}

// Helper functions for tests
fn create_test_animation(id: usize) -> AnimationConfig {
    AnimationConfig {
        id: format!("test_anim_{}", id),
        target: motion_target!(
            "opacity" => AnimationValue::Number(1.0),
            "scale" => AnimationValue::Number(1.1)
        ),
        duration: Some(0.1), // Short for fast tests
        ease: Easing::Linear,
        ..Default::default()
    }
}

fn get_memory_usage() -> usize {
    // Platform-specific memory measurement
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::window()
            .and_then(|w| w.performance())
            .and_then(|p| p.memory())
            .map(|m| m.used_js_heap_size() as usize)
            .unwrap_or(0)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        0 // Simplified for non-WASM
    }
}

fn force_gc() {
    #[cfg(target_arch = "wasm32")]
    {
        // Force GC if available
        js_sys::global()
            .dyn_into::<web_sys::Window>()
            .ok()
            .and_then(|w| w.get("gc"))
            .and_then(|gc| gc.dyn_into::<js_sys::Function>().ok())
            .map(|gc| gc.call0(&js_sys::global()));
    }
}

fn wait_for_animation_completion(handle: &AnimationHandle) {
    // Wait for animation to complete
    let start = instant::Instant::now();
    while handle.is_active() && start.elapsed().as_millis() < 1000 {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[cfg(test)]
mod helper_functions {
    use super::*;

    pub fn create_animation_with_nan_values() -> AnimationConfig {
        AnimationConfig {
            target: motion_target!("opacity" => AnimationValue::Number(f64::NAN)),
            ..Default::default()
        }
    }

    pub fn create_animation_with_infinite_values() -> AnimationConfig {
        AnimationConfig {
            target: motion_target!("scale" => AnimationValue::Number(f64::INFINITY)),
            ..Default::default()
        }
    }

    pub fn create_animation_with_negative_duration() -> AnimationConfig {
        AnimationConfig {
            duration: Some(-1.0),
            ..Default::default()
        }
    }
}
```

#### **Day 2-3: Implement Green Phase**
```rust
// File: crates/leptos-motion-core/src/engine.rs

impl AnimationEngine {
    pub fn new() -> Self {
        Self {
            animations: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicUsize::new(1)),
            active_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn start_animation(&self, config: AnimationConfig) -> Result<AnimationHandle, AnimationError> {
        // Validate animation config
        self.validate_animation_config(&config)?;
        
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let handle = AnimationHandle::new(id);
        
        let animation = Animation::from_config(config);
        
        {
            let mut animations = self.animations.lock().unwrap();
            animations.insert(id, animation);
        }
        
        self.active_count.fetch_add(1, Ordering::SeqCst);
        
        Ok(handle)
    }
    
    pub fn active_animations_count(&self) -> usize {
        self.active_count.load(Ordering::SeqCst)
    }
    
    fn validate_animation_config(&self, config: &AnimationConfig) -> Result<(), AnimationError> {
        // Validate duration
        if let Some(duration) = config.duration {
            if duration < 0.0 || !duration.is_finite() {
                return Err(AnimationError::InvalidValue("Invalid duration".to_string()));
            }
        }
        
        // Validate animation values
        for (_, value) in &config.target.values {
            if let AnimationValue::Number(n) = value {
                if !n.is_finite() {
                    return Err(AnimationError::InvalidValue("Invalid animation value".to_string()));
                }
            }
        }
        
        Ok(())
    }
}
```

### **Command Sequences for Implementation**

#### **Set up TDD environment:**
```bash
# Install required tools
cargo install cargo-nextest cargo-tarpaulin cargo-watch

# Set up git hooks
cp .git/hooks/pre-commit.sample .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit
```

#### **Daily TDD workflow:**
```bash
# Start development session
cargo watch -x "nextest run --fail-fast phase1" -c

# Write failing test
vim tests/phase1_production_hardening/core_engine_refinement.rs

# Implement minimal solution
vim crates/leptos-motion-core/src/engine.rs

# Refactor and optimize
cargo test --workspace
cargo tarpaulin --out Html

# Commit when done
git add . && git commit -m "feat(tdd): implement concurrent animation handling"
```

---

**This TDD workflow provides a systematic approach to implementing v1.0 features with high quality and comprehensive test coverage. Ready to begin Phase 1 implementation!** 🚀