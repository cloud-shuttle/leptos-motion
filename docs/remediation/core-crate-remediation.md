# Core Crate Remediation Plan

## Overview
**File**: `crates/leptos-motion-core/`  
**Status**: Mostly functional, over-tested  
**Lines of Code**: 40+ test files, multiple >600 lines  
**Priority**: P1 (after build fixes)

## Current Issues

### Code Size Violations
- `types.rs`: 698 lines (needs splitting)
- `types_tests.rs`: 779 lines (consolidate tests)
- `dependency_optimization_phase4_tests.rs`: 703 lines
- `feature_flags_optimization_tests.rs`: 693 lines

### Test Over-engineering
- 40+ test files in single crate
- Redundant test functionality
- Test files contributing to code bloat

## Remediation Strategy

### Phase 1: Code Size Reduction (Week 1)

#### Split `types.rs` (698 lines)
**Target Structure**:
```
src/types/
├── lib.rs (re-exports)
├── animation.rs (<200 lines)
├── easing.rs (<200 lines)
├── spring.rs (<200 lines)
├── transition.rs (<200 lines)
└── values.rs (<200 lines)
```

#### Implementation:
```rust
// types/lib.rs
pub mod animation;
pub mod easing;
pub mod spring;
pub mod transition;
pub mod values;

// Re-exports for backward compatibility
pub use animation::*;
pub use easing::*;
pub use spring::*;
pub use transition::*;
pub use values::*;
```

### Phase 2: Test Consolidation (Week 2)

#### Current Test Files → Target Structure
**Before**: 40+ scattered test files
**After**: 5 focused test modules

```
src/
├── unit_tests/
│   ├── animation_tests.rs (<300 lines)
│   ├── easing_tests.rs (<300 lines)
│   ├── spring_tests.rs (<300 lines)
│   └── types_tests.rs (<300 lines)
├── integration_tests/
│   └── core_integration_tests.rs (<300 lines)
└── performance_tests/
    └── benchmarks.rs (<300 lines)
```

#### Consolidation Rules:
1. **Unit Tests**: Group by functionality, not file structure
2. **Integration Tests**: End-to-end workflows
3. **Performance Tests**: Benchmark-focused
4. **Remove Redundancy**: Delete duplicate test logic

### Phase 3: API Contract Validation (Week 3)

#### Verify Contract Compliance
- [ ] AnimationConfig API stability
- [ ] Easing function contracts
- [ ] Spring physics accuracy
- [ ] Type conversion safety

#### Update Contract Tests
```rust
#[cfg(test)]
mod contract_tests {
    use super::*;
    use crate::contracts::*;

    #[test]
    fn animation_config_contract() {
        let config = AnimationConfig::default();
        assert!(validate_animation_contract(&config).is_ok());
    }
}
```

## Success Criteria

### Code Quality
- [ ] All files <300 lines
- [ ] <8 total test files
- [ ] Clean module structure
- [ ] 90%+ test coverage maintained

### Functionality
- [ ] All existing APIs preserved
- [ ] No breaking changes
- [ ] Performance characteristics maintained
- [ ] Memory safety verified

### Maintainability
- [ ] Clear module boundaries
- [ ] Reduced cognitive load
- [ ] Easier debugging
- [ ] LLM-friendly structure

## Risk Mitigation

### Testing Risks
- **Risk**: Reduced coverage during consolidation
- **Mitigation**: Run full test suite after each consolidation step
- **Validation**: Maintain 90%+ coverage minimum

### API Stability Risks
- **Risk**: Unintended breaking changes
- **Mitigation**: Comprehensive contract testing
- **Validation**: All examples still compile

### Performance Risks
- **Risk**: Module splitting overhead
- **Mitigation**: Benchmark before/after
- **Validation**: No performance regression >5%

## Implementation Timeline

| Week | Task | Deliverables |
|------|------|-------------|
| 1 | Split types.rs | 5 modules <200 lines each |
| 2 | Consolidate tests | 5 test files, coverage maintained |
| 3 | Contract validation | All contracts passing |
| 4 | Integration testing | Full system validation |

## Dependencies
- **Blocks**: None (can run parallel to DOM fixes)
- **Blocked by**: None
- **Enables**: DOM crate refactoring, WebGL improvements

## Resources Required
- **Engineers**: 1 senior Rust developer
- **Time**: 4 weeks
- **Tools**: cargo-expand, cargo-llvm-cov, benchmarks

## Validation Commands
```bash
# Test coverage
cargo llvm-cov --workspace --lcov --output-path coverage.lcov

# Performance benchmarks
cargo bench --package leptos-motion-core

# Contract tests
cargo test --package leptos-motion-contracts
```
