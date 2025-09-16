# Immediate Action Plan - Leptos Motion Critical Fixes

## 🚨 CRITICAL: Stop All WASM Demo Deployments

**Action Required**: Immediately stop serving any WASM demos until memory safety issues are resolved.

**Current Status**: 
- ❌ WASM demos are crashing with memory safety violations
- ❌ Animation engine has critical bugs
- ❌ Reference counting issues causing crashes
- ✅ CSS demos work perfectly (safe fallback)

## Phase 1: Emergency Stabilization (Days 1-3)

### Day 1: Immediate Assessment

#### 1.1 Create Minimal Reproduction Case
```bash
# Create a minimal test case that reproduces the crashes
cd crates/leptos-motion-dom
cargo test --target wasm32-unknown-unknown -- --nocapture
```

#### 1.2 Set Up Miri Testing
```bash
# Install Miri for memory safety testing
rustup component add miri
cargo miri test
```

#### 1.3 Document Current State
- [ ] Document all crash scenarios
- [ ] Create error reproduction steps
- [ ] Identify exact code locations causing issues

### Day 2: Critical Memory Safety Fixes

#### 2.1 Fix Hash Map Memory Issues
**File**: `crates/leptos-motion-dom/src/animation_engine.rs`
**Issue**: String pointer validation in hash operations

```rust
// Add before hash operations:
fn validate_string_ptr(s: &str) -> bool {
    !s.as_ptr().is_null() && s.as_ptr() as usize % std::mem::align_of::<u8>() == 0
}
```

#### 2.2 Fix Reference Counting Issues
**File**: `crates/leptos-motion-dom/src/animation_engine.rs`
**Issue**: Rc reference counting in animation callbacks

```rust
// Replace unsafe Rc operations:
let rc = animation_state.try_clone().ok_or("Animation state dropped")?;
```

#### 2.3 Fix Borrowing Conflicts
**File**: `crates/leptos-motion-dom/src/animation_engine.rs`
**Issue**: RefCell borrowing in async contexts

```rust
// Use try_borrow_mut instead of borrow_mut:
let mut borrow = cell.try_borrow_mut().map_err(|_| "Already borrowed")?;
```

### Day 3: Basic Error Handling

#### 3.1 Add Graceful Error Handling
```rust
// Replace panics with Result types:
fn animation_frame_callback() -> Result<(), AnimationError> {
    // Implementation with proper error handling
}
```

#### 3.2 Add Memory Bounds Checking
```rust
// Add bounds checking for all memory operations:
fn safe_slice_from_ptr(ptr: *const u8, len: usize) -> Result<&[u8], MemoryError> {
    if ptr.is_null() || len > MAX_SLICE_LEN {
        return Err(MemoryError::InvalidBounds);
    }
    Ok(unsafe { std::slice::from_raw_parts(ptr, len) })
}
```

## Phase 2: Build System Fixes (Days 4-7)

### Day 4: Fix Trunk Builds

#### 4.1 Update Trunk Configuration
**File**: `examples/*/Trunk.toml`
```toml
[build]
target = "wasm32-unknown-unknown"
release = true

[serve]
port = 8080
```

#### 4.2 Ensure WASM File Generation
```bash
# Test build process:
cd examples/ultra-minimal
trunk build --release
ls -la dist/*.wasm  # Should show WASM files
```

### Day 5: Fix Demo HTML Files

#### 5.1 Remove Integrity Attributes
**Files**: All `dist/index.html` files
```html
<!-- Remove integrity attributes that cause loading failures -->
<link rel="modulepreload" href="/app.js" crossorigin="anonymous">
```

#### 5.2 Fix WASM File References
```html
<!-- Ensure correct WASM file references -->
<script type="module">
import init from './app.js';
await init();
</script>
```

### Day 6: Add Demo Validation

#### 6.1 Create Validation Script
**File**: `scripts/validate-demos.sh`
```bash
#!/bin/bash
# Validate all demos have required files
for demo in examples/*/dist; do
    if [ ! -f "$demo/index.html" ]; then
        echo "❌ Missing index.html in $demo"
    fi
    if [ ! -f "$demo/*.wasm" ]; then
        echo "❌ Missing WASM file in $demo"
    fi
done
```

#### 6.2 Add Build Validation
```bash
# Add to CI/CD pipeline:
cargo check --workspace
cargo test --workspace
./scripts/validate-demos.sh
```

### Day 7: Create Working Demo

#### 7.1 Build Minimal Working Demo
```bash
cd examples/ultra-minimal
trunk build --release
# Test that demo loads without crashes
```

#### 7.2 Validate Demo Functionality
- [ ] Demo loads without errors
- [ ] Basic animations work
- [ ] No memory safety violations
- [ ] No crashes in browser console

## Phase 3: Testing & Validation (Days 8-14)

### Week 2: Comprehensive Testing

#### Day 8: Memory Safety Testing
```bash
# Run Miri tests on all crates:
cargo miri test --workspace
```

#### Day 9: WASM-Specific Testing
```bash
# Test in actual WASM environment:
wasm-pack test --node
```

#### Day 10: Stress Testing
```rust
#[test]
fn test_animation_engine_stress() {
    // Create 1000 animations simultaneously
    // Verify no memory leaks or crashes
}
```

#### Day 11: Browser Testing
- [ ] Test in Chrome
- [ ] Test in Firefox
- [ ] Test in Safari
- [ ] Test on mobile devices

#### Day 12: Performance Testing
```rust
#[bench]
fn bench_animation_engine(b: &mut Bencher) {
    // Benchmark animation performance
}
```

#### Day 13: Integration Testing
- [ ] Test with different Leptos versions
- [ ] Test with different browser versions
- [ ] Test with different WASM targets

#### Day 14: Documentation
- [ ] Update README with current status
- [ ] Document known issues and workarounds
- [ ] Create troubleshooting guide

## Success Criteria

### Phase 1 Success (Day 3)
- [ ] No more panics in WASM environment
- [ ] Basic animations work without crashes
- [ ] Memory usage stays within bounds
- [ ] Error handling is graceful

### Phase 2 Success (Day 7)
- [ ] All demos build successfully
- [ ] WASM files are generated correctly
- [ ] HTML files reference correct assets
- [ ] At least one demo works end-to-end

### Phase 3 Success (Day 14)
- [ ] All tests pass
- [ ] No memory safety violations
- [ ] Performance meets targets
- [ ] Documentation is complete

## Risk Mitigation

### High Risk: Memory Safety Issues
**Mitigation**: 
- Use Miri for continuous memory safety testing
- Add comprehensive bounds checking
- Implement proper error handling

### Medium Risk: Build System Issues
**Mitigation**:
- Automate build validation
- Add CI/CD checks
- Create reproducible build environment

### Low Risk: Demo Issues
**Mitigation**:
- Keep CSS demos as fallback
- Create simple working examples
- Document current limitations

## Communication Plan

### Day 1: Internal Communication
- [ ] Notify team of critical issues
- [ ] Document current state
- [ ] Create issue tracking

### Day 7: Status Update
- [ ] Provide progress report
- [ ] Update documentation
- [ ] Share working demo

### Day 14: Final Report
- [ ] Complete analysis
- [ ] Provide recommendations
- [ ] Plan next steps

## Resources Required

### Development Time
- **Phase 1**: 3 days (critical fixes)
- **Phase 2**: 4 days (build system)
- **Phase 3**: 7 days (testing & validation)
- **Total**: 14 days

### Tools Needed
- Miri for memory safety testing
- WASM testing tools
- Browser testing environment
- CI/CD pipeline updates

### Expertise Required
- Rust memory safety
- WASM development
- Browser animation APIs
- Leptos framework knowledge

---

**Next Action**: Begin Phase 1 immediately - create minimal reproduction case and set up Miri testing environment.

*This plan provides a structured approach to resolving the critical issues in leptos-motion while maintaining development velocity and ensuring quality.*
