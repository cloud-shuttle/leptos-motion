# Contract Testing for Leptos Motion

This document describes the comprehensive contract testing system implemented for the leptos-motion ecosystem.

## 🎯 Overview

Contract testing ensures that the leptos-motion library maintains its promises and guarantees across all crates. It validates:

- **API Stability**: Public interfaces remain consistent and backward-compatible
- **Performance Guarantees**: Operations meet specified performance criteria
- **Error Handling**: Consistent error behavior across all crates
- **Memory Usage**: Memory consumption stays within defined limits
- **Cross-Crate Compatibility**: Data flows correctly between crates

## 📁 Structure

```
tests/contracts/
├── lib.rs                    # Main contract testing library
├── mod.rs                    # Contract testing framework
├── api_contracts.rs          # API contract tests
├── cross_crate_contracts.rs  # Cross-crate validation
├── performance_contracts.rs  # Performance guarantees
├── error_contracts.rs        # Error handling consistency
├── memory_contracts.rs       # Memory usage contracts
└── Cargo.toml               # Contract testing dependencies
```

## 🧪 Contract Types

### 1. API Contracts

**Purpose**: Ensure public APIs remain stable and backward-compatible.

**Tests**:
- All public types and functions exist
- Method signatures remain consistent
- Return types are compatible
- Error types are properly defined

**Example**:
```rust
#[test]
fn test_animation_value_contract() {
    // Test that all AnimationValue variants exist and are constructible
    let value = leptos_motion_core::AnimationValue::Number(1.0);
    // Contract: AnimationValue::Number should be constructible
}
```

### 2. Performance Contracts

**Purpose**: Guarantee specific performance characteristics.

**Contracts**:
- AnimationEngine creation: < 1ms
- Property animation setup: < 0.5ms
- Value retrieval: < 0.1ms
- Animation frame updates: < 16.67ms (60fps)
- Memory usage per animation: < 100KB

**Example**:
```rust
#[test]
fn test_animation_engine_creation_performance() {
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _engine = AnimationEngine::new();
    }
    
    let avg_duration = start.elapsed() / iterations;
    assert!(avg_duration.as_millis() <= 1); // Contract: < 1ms
}
```

### 3. Error Handling Contracts

**Purpose**: Ensure consistent error behavior across all crates.

**Contracts**:
- Invalid inputs are handled gracefully (no panics)
- Error messages follow consistent format
- Error recovery is possible where specified
- Cross-crate error propagation works correctly

**Example**:
```rust
#[test]
fn test_invalid_input_handling() {
    let mut engine = AnimationEngine::new();
    
    // Contract: Invalid inputs should not panic
    engine.animate_property("", f64::NAN, f64::INFINITY, Transition::default());
    // Should handle gracefully without panicking
}
```

### 4. Memory Contracts

**Purpose**: Control memory usage and prevent memory leaks.

**Contracts**:
- AnimationEngine creation: < 100KB
- AnimationValue storage: < 10KB per value
- Memory cleanup on object drop
- Linear memory growth patterns
- No memory leaks in repeated operations

**Example**:
```rust
#[test]
fn test_memory_cleanup_contract() {
    // Create many objects in a scope
    {
        let mut engines = Vec::new();
        for i in 0..1000 {
            let engine = AnimationEngine::new();
            engines.push(engine);
        }
        // Objects should be dropped here
    }
    
    // Contract: Memory should be cleaned up
    // In real implementation, measure memory before/after
}
```

### 5. Cross-Crate Contracts

**Purpose**: Ensure compatibility between different crates.

**Contracts**:
- Core types can be used in DOM layer
- DOM layer respects core performance contracts
- Error propagation works across crate boundaries
- Data flow maintains integrity
- Version compatibility is maintained

**Example**:
```rust
#[test]
fn test_core_types_in_dom() {
    // Contract: Core types should be usable in DOM layer
    let animation_value = leptos_motion_core::AnimationValue::Number(1.0);
    let transition = leptos_motion_core::Transition::default();
    
    let mut engine = leptos_motion_dom::AnimationEngine::new();
    engine.animate_property("scale", 1.0, 2.0, transition);
    // Should work without issues
}
```

## 🚀 Running Contract Tests

### Command Line

```bash
# Run all contract tests
cargo test --package leptos-motion-contracts

# Run specific contract types
cargo test --package leptos-motion-contracts --lib api_contracts
cargo test --package leptos-motion-contracts --lib performance_contracts
cargo test --package leptos-motion-contracts --lib memory_contracts
cargo test --package leptos-motion-contracts --lib error_contracts
cargo test --package leptos-motion-contracts --lib cross_crate_contracts
```

### Using Makefile

```bash
# Run all contract tests
make contract-tests

# Run specific contract types
make contract-api
make contract-perf
make contract-mem
make contract-error
make contract-cross

# Validate all contracts
make validate-contracts
```

### Programmatic Usage

```rust
use leptos_motion_contracts::*;

// Run all contract tests
let report = run_contract_tests();
report.print_report();

// Run tests for specific crate
let report = run_crate_contract_tests("leptos-motion-core");
report.print_report();
```

## 📊 Contract Test Reports

Contract tests generate detailed reports including:

- **Summary**: Total tests, passed/failed counts, success rate
- **Performance Metrics**: Duration, throughput, memory usage
- **Category Breakdown**: Results by test category
- **Failed Tests**: Detailed error messages for failed tests
- **Performance Data**: Specific metrics for each operation

### Report Format

```
=== Leptos Motion Contract Test Report ===
Total Tests: 45
Passed: 44
Failed: 1
Success Rate: 97.8%
Total Duration: 2.3s
Average Duration: 51.1ms

=== Test Categories ===
API: 15/15 passed (100.0%)
Performance: 12/12 passed (100.0%)
Memory: 8/8 passed (100.0%)
Error: 7/8 passed (87.5%)
Cross-crate: 2/2 passed (100.0%)

=== Failed Tests ===
❌ Error_handling_contract: Invalid input handling failed

=== Performance Metrics ===
AnimationEngine_creation_performance:
  avg_duration_ms: 0.8
  throughput_ops_per_sec: 1250.0
  iterations: 1000
```

## 🔧 CI/CD Integration

Contract tests are automatically run in CI/CD pipeline:

- **On every push/PR**: Full contract test suite
- **Daily scheduled runs**: Performance benchmarks
- **Matrix testing**: Multiple Rust versions and targets
- **Artifact collection**: Test reports and benchmarks
- **PR comments**: Automatic test result summaries

### GitHub Actions Workflow

```yaml
name: Contract Tests
on: [push, pull_request, schedule]

jobs:
  contract-tests:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust-version: [stable, beta, nightly]
        target: [x86_64-unknown-linux-gnu, wasm32-unknown-unknown]
    
    steps:
    - uses: actions/checkout@v4
    - name: Run contract tests
      run: cargo test --package leptos-motion-contracts
```

## 📈 Performance Benchmarks

Contract tests include performance benchmarks that run:

- **Daily**: Automated performance regression detection
- **On demand**: Manual performance validation
- **Per commit**: Performance impact assessment

### Benchmark Categories

1. **Creation Performance**: Object instantiation speed
2. **Operation Performance**: Method execution speed
3. **Memory Performance**: Memory allocation patterns
4. **Throughput Performance**: Operations per second
5. **Frame Rate Performance**: Animation frame timing

## 🛠️ Adding New Contracts

### 1. Define Contract Specification

```rust
// In the appropriate contract file
pub fn test_new_contract() -> Vec<ContractTestResult> {
    let mut results = Vec::new();
    
    // Test implementation
    let (_, duration) = utils::measure_execution_time(|| {
        // Contract validation logic
    });
    
    let result = ContractTestResult {
        test_name: "New_contract_test".to_string(),
        passed: true, // Based on validation
        duration,
        metrics: HashMap::new(),
        error_message: None,
    };
    results.push(result);
    
    results
}
```

### 2. Add to Test Runner

```rust
// In lib.rs
pub fn run_all_contract_tests() -> Vec<ContractTestResult> {
    let mut all_results = Vec::new();
    
    // Existing tests...
    all_results.extend(test_new_contract());
    
    all_results
}
```

### 3. Update CI/CD

Add the new contract test to the GitHub Actions workflow and Makefile.

## 🎯 Best Practices

### Contract Design

1. **Be Specific**: Define exact performance thresholds
2. **Be Realistic**: Set achievable but meaningful targets
3. **Be Comprehensive**: Cover all critical paths
4. **Be Maintainable**: Keep contracts simple and clear

### Test Implementation

1. **Use Realistic Data**: Test with production-like inputs
2. **Measure Accurately**: Use proper timing and memory measurement
3. **Handle Edge Cases**: Test boundary conditions
4. **Provide Clear Feedback**: Include detailed error messages

### Maintenance

1. **Regular Review**: Update contracts as requirements change
2. **Performance Monitoring**: Track performance trends over time
3. **Contract Evolution**: Version contracts for backward compatibility
4. **Documentation**: Keep contract documentation up to date

## 🔍 Troubleshooting

### Common Issues

1. **Performance Regression**: Check for recent changes affecting performance
2. **Memory Leaks**: Use memory profiling tools to identify leaks
3. **API Changes**: Ensure backward compatibility is maintained
4. **Cross-Crate Issues**: Verify data flow between crates

### Debugging

1. **Enable Verbose Output**: Use `--nocapture` flag for detailed output
2. **Check Metrics**: Review performance metrics for anomalies
3. **Isolate Tests**: Run individual contract tests to identify issues
4. **Profile Performance**: Use profiling tools for performance issues

## 📚 References

- [Contract Testing Best Practices](https://docs.pact.io/)
- [Rust Performance Testing](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [WASM Performance](https://rustwasm.github.io/docs/book/game-of-life/time-profiling.html)
- [Memory Management in Rust](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

---

This contract testing system ensures that leptos-motion maintains its quality, performance, and reliability guarantees across all versions and platforms.
