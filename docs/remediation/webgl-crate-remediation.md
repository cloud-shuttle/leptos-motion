# WebGL Crate Remediation Plan

## Overview
**File**: `crates/leptos-motion-webgl/`  
**Status**: Broken - 14 compilation errors  
**Lines of Code**: Multiple files >600 lines  
**Priority**: P1 (after DOM fixes)

## Current Issues

### Compilation Errors
- Import resolution failures (6 locations)
- Type mismatches with web-sys (5 locations)
- Missing trait implementations (3 locations)
- Borrow checker violations (2 locations)

### Code Size Violations
- `post_processing.rs`: 722 lines
- `shadow_mapping.rs`: 667 lines
- `post_processing_tests.rs`: 667 lines
- `texture.rs`: 657 lines (multiple structs/methods)

### Incomplete Implementations
- Stub methods in renderer
- Missing WebGL buffer management
- Incomplete shader system
- Placeholder physics engine

## Remediation Strategy

### Phase 1: Critical Build Fixes (Week 1)

#### Fix Import Resolution
**Problem**: Missing web-sys feature flags
```rust
// Current (broken)
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

// Fixed - add to Cargo.toml
[features]
default = ["webgl2"]
webgl2 = ["web-sys/webgl2"]
```

#### Fix Type Mismatches
**Problem**: Float32Array vs &[f32] conversions
```rust
// Current (broken)
context.uniform3fv_with_f32_array(location, &array);

// Fixed
let slice: &[f32] = &array.to_vec();
context.uniform3fv_with_f32_array(location, slice);
```

#### Fix Borrow Checker Issues
**Problem**: Improper lifetime management
```rust
// Current (broken)
let buffer = self.create_buffer(&data);

// Fixed
let buffer = self.create_buffer(data.as_slice());
```

### Phase 2: Code Size Reduction (Week 2-3)

#### Split `texture.rs` (657 lines)
**Target Structure**:
```
src/texture/
├── lib.rs (main Texture struct)
├── manager.rs (TextureManager)
├── loader.rs (loading logic)
├── formats.rs (format handling)
└── cache.rs (caching logic)
```

#### Split `post_processing.rs` (722 lines)
**Target Structure**:
```
src/post_processing/
├── lib.rs (PostProcessor)
├── effects/ (subfolder)
│   ├── bloom.rs
│   ├── blur.rs
│   ├── color_correction.rs
│   └── tonemapping.rs
├── pipelines.rs (render pipelines)
└── shaders.rs (shader management)
```

### Phase 3: Stub Implementation (Week 4-5)

#### Complete Renderer Implementation
```rust
impl WebGLRenderer {
    pub fn bind_geometry(&self, geometry: &Geometry) -> Result<()> {
        // TODO: Implement geometry binding
        Ok(())
    }

    pub fn bind_material(&self, material: &Material) -> Result<()> {
        // TODO: Implement material binding
        Ok(())
    }
}
```

#### Implement Shader System
```rust
pub struct ShaderProgram {
    program: WebGlProgram,
    uniforms: HashMap<String, WebGlUniformLocation>,
    attributes: HashMap<String, u32>,
}

impl ShaderProgram {
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Self> {
        // Implementation
    }
}
```

#### Complete Buffer Management
```rust
pub struct BufferManager {
    vertex_buffers: HashMap<String, WebGlBuffer>,
    index_buffers: HashMap<String, WebGlBuffer>,
    uniform_buffers: HashMap<String, WebGlBuffer>,
}

impl BufferManager {
    pub fn create_vertex_buffer(&mut self, data: &[f32]) -> Result<String> {
        // Implementation
    }
}
```

### Phase 4: Physics Integration (Week 6)

#### Implement Collision Detection
```rust
pub trait CollisionAlgorithm {
    fn detect(&self, a: &RigidBody, b: &RigidBody) -> Option<Collision>;
    fn resolve(&self, collision: &Collision) -> Vec<ContactPoint>;
}
```

#### Complete Physics World
```rust
pub struct PhysicsWorld {
    bodies: Vec<RigidBody>,
    constraints: Vec<Box<dyn Constraint>>,
    broad_phase: Box<dyn BroadPhase>,
    narrow_phase: NarrowPhaseDetector,
    solver: ConstraintSolver,
}
```

## Success Criteria

### Build Health
- [ ] 0 compilation errors
- [ ] All web-sys bindings correct
- [ ] Lifetime management proper
- [ ] Feature flags working

### Code Quality
- [ ] All files <300 lines
- [ ] Clear module separation
- [ ] Proper error handling
- [ ] WebGL best practices

### Functionality
- [ ] Basic rendering works
- [ ] Texture loading functional
- [ ] Shader compilation succeeds
- [ ] Post-processing applies

### Performance
- [ ] 60fps WebGL rendering
- [ ] Minimal draw calls
- [ ] Efficient buffer usage
- [ ] Memory leaks prevented

## Risk Mitigation

### WebGL Complexity
- **Risk**: Complex WebGL API misuse
- **Mitigation**: Comprehensive testing with different browsers
- **Validation**: Works in Chrome, Firefox, Safari

### Memory Management
- **Risk**: WebGL resource leaks
- **Mitigation**: RAII pattern implementation
- **Validation**: Memory profiling tools

### Browser Compatibility
- **Risk**: WebGL2 support inconsistent
- **Mitigation**: Fallback to WebGL1 where needed
- **Validation**: Test across target browsers

## Implementation Timeline

| Week | Task | Deliverables |
|------|------|-------------|
| 1 | Fix compilation errors | Clean WebGL build |
| 2-3 | Split large files | Modules <300 lines |
| 4-5 | Complete renderer | Basic WebGL rendering |
| 6 | Physics integration | Collision detection |

## Dependencies
- **Blocks**: WebGL examples, advanced demos
- **Blocked by**: DOM crate fixes
- **Enables**: 3D animation features

## Resources Required
- **Engineers**: 2 senior WebGL developers
- **Time**: 6 weeks
- **Tools**: WebGL inspector, browser dev tools

## Validation Commands
```bash
# WebGL build
cargo check --package leptos-motion-webgl --features webgl2

# Shader compilation tests
cargo test --package leptos-motion-webgl -- test_shaders

# Rendering tests
wasm-pack test --chrome --headless

# Performance benchmarks
cargo bench --package leptos-motion-webgl
```
