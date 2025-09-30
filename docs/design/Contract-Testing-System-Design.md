# Contract Testing System Design
## API Stability and Quality Assurance

**File**: `tests/contracts/` (multiple files)  
**Lines**: Target <300 per file (currently 200-400 lines)  
**Status**: BROKEN - Compilation errors  

---

## 🎯 **Contract Testing Overview**

The Contract Testing System ensures API stability, performance guarantees, and behavioral consistency across the Leptos Motion ecosystem.

### **Core Responsibilities**
1. **API Contract Verification**: Ensure public APIs remain stable
2. **Performance Contracts**: Enforce performance guarantees
3. **Error Contract Validation**: Verify error handling consistency
4. **Memory Contract Testing**: Prevent memory leaks and usage violations
5. **Cross-Crate Integration**: Test interactions between crates

### **Contract Types**
- **API Contracts**: Method signatures, return types, trait implementations
- **Performance Contracts**: Execution time limits, throughput requirements
- **Error Contracts**: Error types, recovery mechanisms, error messages
- **Memory Contracts**: Memory usage limits, leak detection, cleanup verification

---

## 🏗️ **Architecture**

### **Core Components**
```rust
pub struct ContractTestRunner {
    api_contracts: Vec<ApiContract>,
    performance_contracts: Vec<PerformanceContract>,
    error_contracts: Vec<ErrorContract>,
    memory_contracts: Vec<MemoryContract>,
    results: Vec<ContractTestResult>,
}

#[derive(Debug, Clone)]
pub struct ContractTestResult {
    pub test_name: String,
    pub contract_type: ContractType,
    pub passed: bool,
    pub duration: Duration,
    pub metrics: HashMap<String, f64>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ContractType {
    Api,
    Performance,
    Error,
    Memory,
}
```

### **Test Execution Flow**
```rust
impl ContractTestRunner {
    pub fn run_all_contracts(&mut self) -> Vec<ContractTestResult> {
        let mut results = Vec::new();

        // Run API contract tests
        results.extend(self.run_api_contracts());

        // Run performance contract tests
        results.extend(self.run_performance_contracts());

        // Run error contract tests
        results.extend(self.run_error_contracts());

        // Run memory contract tests
        results.extend(self.run_memory_contracts());

        self.results = results.clone();
        results
    }

    fn run_api_contracts(&self) -> Vec<ContractTestResult> {
        vec![
            self.test_animation_value_api(),
            self.test_transition_api(),
            self.test_animation_engine_api(),
            self.test_motion_div_api(),
        ]
    }
}
```

---

## 📋 **API Contract Testing**

### **AnimationValue Contract**
```rust
fn test_animation_value_api(&self) -> ContractTestResult {
    let start_time = Instant::now();

    // Test all AnimationValue variants exist and constructible
    let test_cases = vec![
        ("Number", || AnimationValue::Number(1.0)),
        ("String", || AnimationValue::String("test".to_string())),
        ("Pixels", || AnimationValue::Pixels(100.0)),
        ("Percentage", || AnimationValue::Percentage(50.0)),
        ("Degrees", || AnimationValue::Degrees(90.0)),
        ("Radians", || AnimationValue::Radians(1.57)),
        ("Color", || AnimationValue::Color("#ff0000".to_string())),
    ];

    let mut passed = true;
    let mut metrics = HashMap::new();

    for (variant_name, constructor) in test_cases {
        let variant_start = Instant::now();
        let result = panic::catch_unwind(constructor);
        let variant_duration = variant_start.elapsed();

        metrics.insert(
            format!("{}_construction_time_ms", variant_name),
            variant_duration.as_secs_f64() * 1000.0
        );

        if result.is_err() {
            passed = false;
            break;
        }
    }

    let total_duration = start_time.elapsed();

    ContractTestResult {
        test_name: "AnimationValue_API_Contract".to_string(),
        contract_type: ContractType::Api,
        passed,
        duration: total_duration,
        metrics,
        error_message: if passed { None } else { Some("Failed to construct AnimationValue variants".to_string()) },
    }
}
```

### **Method Signature Verification**
```rust
fn test_method_signatures(&self) -> ContractTestResult {
    let start_time = Instant::now();

    // Test that required methods exist with correct signatures
    let required_methods = vec![
        ("AnimationEngine::animate_property", test_animate_property_signature),
        ("AnimationEngine::start_animation", test_start_animation_signature),
        ("MotionDiv::new", test_motion_div_new_signature),
    ];

    let mut passed = true;
    let mut failed_methods = Vec::new();

    for (method_name, test_fn) in required_methods {
        if !test_fn() {
            passed = false;
            failed_methods.push(method_name.to_string());
        }
    }

    let duration = start_time.elapsed();

    ContractTestResult {
        test_name: "Method_Signature_Contract".to_string(),
        contract_type: ContractType::Api,
        passed,
        duration,
        metrics: HashMap::new(),
        error_message: if passed {
            None
        } else {
            Some(format!("Missing or incorrect method signatures: {}", failed_methods.join(", ")))
        },
    }
}
```

---

## ⚡ **Performance Contract Testing**

### **Animation Performance Contract**
```rust
#[derive(Debug, Clone)]
pub struct PerformanceContract {
    pub operation_name: String,
    pub max_duration_ms: f64,
    pub min_iterations: usize,
    pub warmup_iterations: usize,
}

fn test_animation_performance(&self) -> ContractTestResult {
    let contract = PerformanceContract {
        operation_name: "single_animation_frame".to_string(),
        max_duration_ms: 16.0, // 60fps budget
        min_iterations: 100,
        warmup_iterations: 10,
    };

    let start_time = Instant::now();
    let mut durations = Vec::new();

    // Create test animation
    let mut engine = SimplifiedAnimationEngine::new();
    let animation = create_test_animation();

    // Warmup
    for _ in 0..contract.warmup_iterations {
        let _ = engine.animate_property(
            "test".to_string(),
            0.0,
            1.0,
            Transition::default()
        );
    }

    // Measure performance
    for _ in 0..contract.min_iterations {
        let iteration_start = Instant::now();

        let _ = engine.animate_property(
            "test".to_string(),
            0.0,
            1.0,
            Transition::default()
        );

        let iteration_duration = iteration_start.elapsed();
        durations.push(iteration_duration.as_secs_f64() * 1000.0);
    }

    // Calculate statistics
    let avg_duration = durations.iter().sum::<f64>() / durations.len() as f64;
    let max_duration = durations.iter().fold(0.0, |a, &b| a.max(b));
    let min_duration = durations.iter().fold(f64::INFINITY, |a, &b| a.min(b));

    let passed = max_duration <= contract.max_duration_ms;
    let total_duration = start_time.elapsed();

    let mut metrics = HashMap::new();
    metrics.insert("average_duration_ms".to_string(), avg_duration);
    metrics.insert("max_duration_ms".to_string(), max_duration);
    metrics.insert("min_duration_ms".to_string(), min_duration);
    metrics.insert("iterations".to_string(), contract.min_iterations as f64);

    ContractTestResult {
        test_name: format!("{}_Performance_Contract", contract.operation_name),
        contract_type: ContractType::Performance,
        passed,
        duration: total_duration,
        metrics,
        error_message: if passed {
            None
        } else {
            Some(format!(
                "Performance violation: max duration {:.2}ms exceeds limit {:.2}ms",
                max_duration, contract.max_duration_ms
            ))
        },
    }
}
```

---

## 🚨 **Error Contract Testing**

### **Error Handling Verification**
```rust
#[derive(Debug, Clone)]
pub struct ErrorContract {
    pub operation_name: String,
    pub expected_error_type: String,
    pub should_panic: bool,
    pub recovery_required: bool,
}

fn test_error_contracts(&self) -> Vec<ContractTestResult> {
    vec![
        self.test_invalid_animation_value_error(),
        self.test_missing_property_error(),
        self.test_memory_limit_error(),
    ]
}

fn test_invalid_animation_value_error(&self) -> ContractTestResult {
    let start_time = Instant::now();

    // Test that invalid animation values produce expected errors
    let result = panic::catch_unwind(|| {
        let engine = SimplifiedAnimationEngine::new();
        // This should fail with InvalidAnimationValue error
        engine.animate_property(
            "invalid_prop".to_string(),
            f64::NAN, // Invalid value
            1.0,
            Transition::default()
        )
    });

    let passed = result.is_err(); // Should panic with error
    let duration = start_time.elapsed();

    ContractTestResult {
        test_name: "Invalid_Animation_Value_Error_Contract".to_string(),
        contract_type: ContractType::Error,
        passed,
        duration,
        metrics: HashMap::new(),
        error_message: if passed {
            None
        } else {
            Some("Expected error for invalid animation value, but none occurred".to_string())
        },
    }
}
```

---

## 🧠 **Memory Contract Testing**

### **Memory Usage Monitoring**
```rust
#[derive(Debug, Clone)]
pub struct MemoryContract {
    pub operation_name: String,
    pub max_memory_mb: f64,
    pub max_leak_mb: f64,
    pub test_duration_secs: u64,
}

fn test_memory_contract(&self, contract: &MemoryContract) -> ContractTestResult {
    let start_time = Instant::now();

    // Get initial memory usage
    let initial_memory = get_current_memory_usage();

    // Run operation for specified duration
    let operation_start = Instant::now();
    while operation_start.elapsed().as_secs() < contract.test_duration_secs {
        // Perform memory-intensive operation
        let _animations = create_memory_test_animations(100);

        // Small delay to prevent tight loop
        std::thread::sleep(Duration::from_millis(10));
    }

    // Check final memory usage
    let final_memory = get_current_memory_usage();
    let memory_delta = final_memory - initial_memory;

    let passed = memory_delta <= contract.max_memory_mb;
    let leak_detected = memory_delta > contract.max_leak_mb;

    let total_duration = start_time.elapsed();

    let mut metrics = HashMap::new();
    metrics.insert("initial_memory_mb".to_string(), initial_memory);
    metrics.insert("final_memory_mb".to_string(), final_memory);
    metrics.insert("memory_delta_mb".to_string(), memory_delta);
    metrics.insert("leak_detected".to_string(), if leak_detected { 1.0 } else { 0.0 });

    ContractTestResult {
        test_name: format!("{}_Memory_Contract", contract.operation_name),
        contract_type: ContractType::Memory,
        passed: passed && !leak_detected,
        duration: total_duration,
        metrics,
        error_message: if passed && !leak_detected {
            None
        } else {
            Some(format!(
                "Memory contract violation: {}MB used (limit: {}MB), leak: {}MB (limit: {}MB)",
                memory_delta, contract.max_memory_mb,
                if leak_detected { memory_delta } else { 0.0 }, contract.max_leak_mb
            ))
        },
    }
}

#[cfg(target_arch = "wasm32")]
fn get_current_memory_usage() -> f64 {
    // Use performance.memory if available
    web_sys::window()
        .and_then(|w| w.performance())
        .and_then(|p| p.memory())
        .map(|m| m.used_js_heap_size() as f64 / 1024.0 / 1024.0)
        .unwrap_or(0.0)
}

#[cfg(not(target_arch = "wasm32"))]
fn get_current_memory_usage() -> f64 {
    // Placeholder for native platforms
    0.0
}
```

---

## 📊 **Contract Test Reporting**

### **Comprehensive Report Generation**
```rust
pub struct ContractTestReport {
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub total_duration: Duration,
    pub avg_duration_ms: f64,
    pub test_categories: HashMap<String, (usize, usize)>, // (passed, failed)
    pub results: Vec<ContractTestResult>,
    pub performance_baseline: Option<PerformanceBaseline>,
}

impl ContractTestReport {
    pub fn generate(&self) -> String {
        format!(
            r#"# Leptos Motion Contract Test Report

## Summary
- **Total Tests**: {}
- **Passed**: {} ({:.1}%)
- **Failed**: {} ({:.1}%)
- **Total Duration**: {:.2}s
- **Average Duration**: {:.2}ms

## Test Categories
{}"#,
            self.total_tests,
            self.passed_tests,
            (self.passed_tests as f64 / self.total_tests as f64) * 100.0,
            self.failed_tests,
            (self.failed_tests as f64 / self.total_tests as f64) * 100.0,
            self.total_duration.as_secs_f64(),
            self.avg_duration_ms,
            self.format_categories()
        )
    }

    fn format_categories(&self) -> String {
        self.test_categories
            .iter()
            .map(|(category, (passed, failed))| {
                let total = passed + failed;
                let success_rate = if total > 0 {
                    (*passed as f64 / total as f64) * 100.0
                } else {
                    0.0
                };
                format!("- **{}**: {}/{} passed ({:.1}%)", category, passed, total, success_rate)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
```

---

## 🔧 **Contract Test Configuration**

### **Baseline Management**
```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PerformanceBaseline {
    pub test_name: String,
    pub expected_duration_ms: f64,
    pub tolerance_percent: f64,
    pub last_updated: DateTime<Utc>,
}

impl PerformanceBaseline {
    pub fn is_within_baseline(&self, actual_duration_ms: f64) -> bool {
        let tolerance = self.expected_duration_ms * (self.tolerance_percent / 100.0);
        let min_duration = self.expected_duration_ms - tolerance;
        let max_duration = self.expected_duration_ms + tolerance;

        actual_duration_ms >= min_duration && actual_duration_ms <= max_duration
    }

    pub fn update_baseline(&mut self, new_duration_ms: f64) {
        self.expected_duration_ms = new_duration_ms;
        self.last_updated = Utc::now();
    }
}
```

### **Test Configuration**
```rust
#[derive(Clone, Debug)]
pub struct ContractTestConfig {
    pub enable_performance_tests: bool,
    pub enable_memory_tests: bool,
    pub performance_tolerance_percent: f64,
    pub memory_tolerance_mb: f64,
    pub test_timeout_secs: u64,
    pub baseline_file_path: Option<String>,
}

impl Default for ContractTestConfig {
    fn default() -> Self {
        Self {
            enable_performance_tests: true,
            enable_memory_tests: true,
            performance_tolerance_percent: 10.0,
            memory_tolerance_mb: 5.0,
            test_timeout_secs: 300, // 5 minutes
            baseline_file_path: Some("contract-baselines.json".to_string()),
        }
    }
}
```

---

## 🧪 **Testing Strategy**

### **Unit Tests for Contract System**
```rust
#[test]
fn test_api_contract_validation() {
    let runner = ContractTestRunner::new();

    // Test API contract validation
    let result = runner.test_animation_value_api();
    assert!(result.passed, "API contract failed: {:?}", result.error_message);
}

#[test]
fn test_performance_contract_enforcement() {
    let runner = ContractTestRunner::new();

    // Test performance contract
    let result = runner.test_animation_performance();
    assert!(result.passed, "Performance contract failed: {:?}", result.error_message);

    // Verify metrics are recorded
    assert!(result.metrics.contains_key("average_duration_ms"));
    assert!(result.metrics.contains_key("max_duration_ms"));
}

#[test]
fn test_memory_contract_detection() {
    let runner = ContractTestRunner::new();

    // Test memory contract
    let contract = MemoryContract {
        operation_name: "memory_test".to_string(),
        max_memory_mb: 10.0,
        max_leak_mb: 1.0,
        test_duration_secs: 1,
    };

    let result = runner.test_memory_contract(&contract);
    assert!(result.passed, "Memory contract failed: {:?}", result.error_message);
}
```

### **Integration Tests**
```rust
#[test]
fn test_cross_crate_contracts() {
    // Test that leptos-motion-core and leptos-motion-dom
    // maintain compatible contracts
    let core_result = test_leptos_motion_core_contracts();
    let dom_result = test_leptos_motion_dom_contracts();

    assert!(core_result.passed, "Core crate contract failed");
    assert!(dom_result.passed, "DOM crate contract failed");

    // Test cross-crate integration
    let integration_result = test_core_dom_integration();
    assert!(integration_result.passed, "Cross-crate integration failed");
}
```

---

## 🎯 **Implementation Plan**

### **Phase 1: Core Infrastructure (Week 1-2)**
- [ ] Implement ContractTestRunner struct
- [ ] Add basic API contract testing
- [ ] Create test result reporting
- [ ] Set up test configuration system

### **Phase 2: Performance Contracts (Week 3-4)**
- [ ] Add performance measurement utilities
- [ ] Implement performance contract validation
- [ ] Create performance baselines
- [ ] Add performance regression detection

### **Phase 3: Error & Memory Contracts (Week 5-6)**
- [ ] Implement error contract testing
- [ ] Add memory usage monitoring
- [ ] Create memory leak detection
- [ ] Add cross-platform memory measurement

### **Phase 4: Advanced Features (Week 7-8)**
- [ ] Add baseline management system
- [ ] Implement contract test CI/CD integration
- [ ] Create detailed reporting and visualization
- [ ] Add historical performance tracking

**Target Completion**: 8 weeks for comprehensive contract testing system with all contract types implemented and validated.
