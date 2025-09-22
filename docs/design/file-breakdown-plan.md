# File Breakdown Plan

## Overview
Strategy for breaking down 19 oversized files (>300 lines) into maintainable components under 300 lines each.

## Current Oversized Files

### Critical Priority (>1000 lines)
1. **export.rs** (1,492 lines) - Studio export functionality
2. **types_tests.rs** (779 lines) - Core type tests
3. **shared_elements/mod.rs** (774 lines) - Layout shared elements
4. **keyframe_animation_tests.rs** (726 lines) - Keyframe animation tests
5. **preview.rs** (723 lines) - Studio preview
6. **post_processing.rs** (722 lines) - WebGL post-processing

## Breakdown Strategy by File

### 1. export.rs (1,492 → 4 files <400 lines each)
**Target Directory**: `crates/leptos-motion-studio/src/export/`

```
export/
├── mod.rs                    # Main export coordinator (<200 lines)
├── formats/
│   ├── json_exporter.rs     # JSON export format (<250 lines)  
│   ├── css_exporter.rs      # CSS export format (<250 lines)
│   ├── lottie_exporter.rs   # Lottie export format (<300 lines)
│   └── wasm_exporter.rs     # WASM export format (<200 lines)
├── validation/
│   ├── schema_validator.rs  # Export validation (<150 lines)
│   └── format_checker.rs    # Format compatibility (<100 lines)
└── utils/
    ├── file_writer.rs       # File I/O utilities (<100 lines)
    └── compression.rs       # Data compression (<150 lines)
```

**Breakdown Logic**:
- Separate by export format (JSON, CSS, Lottie, WASM)
- Extract validation logic into separate module
- Isolate file I/O and compression utilities

### 2. types_tests.rs (779 → 3 files <300 lines each)
**Target Directory**: `crates/leptos-motion-core/src/tests/types/`

```
tests/types/
├── mod.rs                   # Test coordination (<50 lines)
├── animation_value_tests.rs # AnimationValue tests (<250 lines)
├── transition_tests.rs      # Transition tests (<200 lines)
├── easing_tests.rs         # Easing function tests (<150 lines)
└── property_tests.rs       # Property-specific tests (<200 lines)
```

### 3. shared_elements/mod.rs (774 → 3 files <300 lines each)
**Target Directory**: `crates/leptos-motion-layout/src/shared_elements/`

```
shared_elements/
├── mod.rs                   # Public API & coordination (<100 lines)
├── element_tracker.rs       # Element lifecycle tracking (<250 lines)
├── shared_state.rs         # Shared animation state (<200 lines)
├── transition_manager.rs   # Cross-element transitions (<250 lines)
└── persistence.rs          # State persistence (<150 lines)
```

### 4. keyframe_animation_tests.rs (726 → 3 files <300 lines each)
**Target Directory**: `crates/leptos-motion-dom/src/tests/keyframe/`

```
tests/keyframe/
├── mod.rs                   # Test utilities (<50 lines)
├── creation_tests.rs        # Animation creation tests (<250 lines)
├── interpolation_tests.rs   # Keyframe interpolation tests (<200 lines)
├── lifecycle_tests.rs       # Start/stop/pause tests (<150 lines)
└── performance_tests.rs     # Performance benchmarks (<200 lines)
```

### 5. preview.rs (723 → 3 files <300 lines each)
**Target Directory**: `crates/leptos-motion-studio/src/preview/`

```
preview/
├── mod.rs                   # Preview coordinator (<100 lines)
├── renderer.rs              # Animation rendering (<250 lines)
├── controls.rs              # Playback controls (<200 lines)
├── timeline.rs              # Timeline visualization (<200 lines)
└── viewport.rs              # Preview viewport management (<150 lines)
```

### 6. post_processing.rs (722 → 3 files <300 lines each)
**Target Directory**: `crates/leptos-motion-webgl/src/post_processing/`

```
post_processing/
├── mod.rs                   # Pipeline coordinator (<100 lines)
├── effects/
│   ├── blur.rs             # Blur effects (<200 lines)
│   ├── bloom.rs            # Bloom effects (<150 lines)
│   ├── color_correction.rs # Color grading (<200 lines)
│   └── distortion.rs       # Distortion effects (<150 lines)
├── pipeline.rs             # Rendering pipeline (<250 lines)
└── shaders.rs              # Shader management (<200 lines)
```

## Implementation Plan

### Week 1: Critical Files (>1000 lines)
- **Day 1-2**: Break down `export.rs` 
- **Day 3**: Break down `types_tests.rs`
- **Day 4**: Break down `shared_elements/mod.rs`
- **Day 5**: Break down `keyframe_animation_tests.rs`

### Week 2: Large Files (700-999 lines)
- **Day 1**: Break down `preview.rs`
- **Day 2**: Break down `post_processing.rs`
- **Day 3-5**: Handle remaining 13 files (600-699 lines)

## Breakdown Guidelines

### Code Organization Principles
1. **Single Responsibility**: Each file has one clear purpose
2. **Logical Grouping**: Related functionality stays together
3. **Clear Dependencies**: Minimize cross-file dependencies
4. **Test Co-location**: Tests near the code they test

### File Size Targets
- **Module files (`mod.rs`)**: <100 lines (just public API)
- **Implementation files**: <250 lines
- **Test files**: <200 lines  
- **Utility files**: <150 lines

### Migration Process
```rust
// 1. Create new module structure
// 2. Move code to appropriate files
// 3. Update imports and exports
// 4. Run tests to verify no breakage
// 5. Update documentation
```

## Validation Strategy

### Post-Breakdown Verification
```bash
# Verify no file exceeds 300 lines
find crates -name "*.rs" -exec wc -l {} + | awk '$1 > 300 {print $2 " has " $1 " lines"}'

# Ensure all tests still pass
cargo test --all

# Check compilation
cargo build --all-features
```

### Testing Protocol
1. **Before breakdown**: Run full test suite
2. **During breakdown**: Run affected tests after each file split
3. **After breakdown**: Full integration test + performance benchmarks

## Dependencies to Update

### Import Statements
```rust
// Before: use crate::export::*;
// After: use crate::export::{json_exporter, css_exporter};
```

### Module Declarations
```rust
// In mod.rs files
pub mod json_exporter;
pub mod css_exporter;
pub mod validation;

// Re-exports for backward compatibility
pub use json_exporter::JsonExporter;
pub use css_exporter::CssExporter;
```

## Success Criteria
- [ ] All files under 300 lines
- [ ] No functionality broken
- [ ] All tests passing
- [ ] Build times improved
- [ ] Code is more maintainable
- [ ] Better test isolation
- [ ] Clearer module boundaries
- [ ] Documentation updated

## Tools for Automation

### File Analysis Script
```bash
#!/bin/bash
# analyze_files.sh
find crates -name "*.rs" -exec wc -l {} + | \
  sort -nr | \
  head -20 | \
  awk '{printf "%-50s %4d lines\n", $2, $1}'
```

### Breakdown Helper
```rust
// Rust script to analyze function boundaries and suggest splits
// Could be implemented as a cargo extension
```

This systematic approach ensures all oversized files are broken down into maintainable components while preserving functionality and improving code organization.
