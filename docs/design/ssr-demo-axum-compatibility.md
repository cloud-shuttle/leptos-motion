# SSR Demo Axum Compatibility Design

## Problem Statement

The SSR demo fails to compile due to axum API version conflicts and incorrect usage of the leptos_axum integration. Multiple axum versions in the dependency tree cause API incompatibilities that prevent the server from building.

## Root Cause Analysis

### Dependency Version Conflicts

**Issue**: Multiple axum versions causing API incompatibilities.

**Dependency Tree Analysis**:
```
leptos-axum v0.8.0
├── axum v0.7.x  (via leptos-axum)

ssr-demo
├── axum v0.8.x  (direct dependency)
├── leptos-axum v0.8.0 (via Cargo.toml)
```

**Result**: Two different axum versions causing API mismatches.

### API Usage Errors

**Current Code Issues**:
```rust
// WRONG: leptos_axum doesn't have generate_route_list_with_ssg
use leptos_axum::generate_route_list_with_ssg;

// WRONG: .into_router() doesn't exist on the return type
let app = leptos_axum::generate_route_list_with_ssg(App)
    .into_router()  // This method doesn't exist
    .with_state(leptos_options);

// WRONG: .leptos_routes_with_handler doesn't exist
.route("/api/*fn_name", get(leptos_axum::handle_server_fns))
.leptos_routes_with_handler(routes, get(leptos_axum::render_app_to_stream))
```

## Solution Design

### Dependency Resolution Strategy

**Option 1: Align Versions (Recommended)**
```toml
# demos/ssr-demo/Cargo.toml
[dependencies]
axum = "0.7"  # Match leptos-axum version
leptos-axum = "0.8"
```

**Option 2: Use Workspace Versions**
```toml
# Use workspace dependency resolution
axum = { workspace = true }
leptos-axum = { workspace = true }
```

**Design Decision**: Option 1 - Explicit version alignment for predictability.

### Correct Axum Server Setup

**Proper Implementation**:
```rust
use leptos::prelude::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    console_error_panic_hook::set_once();

    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Correct axum router setup
    let app = Router::new()
        .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
        .leptos_routes(routes, get(leptos_axum::render_app_to_stream))
        .fallback(leptos_axum::file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("🚀 SSR Demo server running on http://{}", addr);
    axum::serve(listener, app).await.unwrap();
}
```

### API Mapping Reference

| Current (Broken) | Correct Implementation |
|------------------|------------------------|
| `generate_route_list_with_ssg` | `generate_route_list` |
| `.into_router()` | Remove (not needed) |
| `.leptos_routes_with_handler()` | `.leptos_routes()` |
| `handle_server_fns` | `leptos_axum::handle_server_fns` |
| `render_app_to_stream` | `leptos_axum::render_app_to_stream` |

## Implementation Strategy

### Phase 1: Dependency Resolution

1. **Audit Current Dependencies**
   ```bash
   cd demos/ssr-demo
   cargo tree | grep axum
   ```

2. **Update Cargo.toml**
   ```toml
   [dependencies]
   axum = "0.7.5"  # Exact version match
   leptos-axum = "0.8.0"
   leptos = { version = "0.8.0", features = ["ssr"] }
   ```

3. **Verify Dependency Resolution**
   ```bash
   cargo check
   cargo tree | grep axum  # Should show single version
   ```

### Phase 2: Code Corrections

1. **Fix Imports**
   ```rust
   // Remove incorrect imports
   // use leptos_axum::generate_route_list_with_ssg;

   // Add correct imports
   use leptos_axum::{generate_route_list, LeptosRoutes};
   use axum::{routing::get, Router};
   ```

2. **Fix Server Setup**
   ```rust
   // Replace incorrect server setup with correct implementation
   let app = Router::new()
       .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
       .leptos_routes(routes, get(leptos_axum::render_app_to_stream))
       .fallback(leptos_axum::file_and_error_handler)
       .with_state(leptos_options);
   ```

3. **Add Error Handling**
   ```rust
   let listener = tokio::net::TcpListener::bind(&addr).await
       .expect("Failed to bind to address");
   ```

### Phase 3: Testing & Validation

1. **Compilation Test**
   ```bash
   cd demos/ssr-demo
   cargo check
   cargo build
   ```

2. **Server Startup Test**
   ```bash
   cd demos/ssr-demo
   cargo run &
   sleep 5
   curl http://localhost:3000
   pkill -f "cargo run"
   ```

3. **SSR Functionality Test**
   ```bash
   # Test that SSR renders correctly
   # Test that hydration works
   # Test that animations function in SSR mode
   ```

## Testing Strategy

### Unit Tests for Server Setup
```rust
#[cfg(test)]
mod server_tests {
    use super::*;

    #[test]
    fn test_axum_router_creation() {
        // Test that router can be created without panicking
        // This validates dependency compatibility
    }

    #[test]
    fn test_leptos_routes_integration() {
        // Test leptos_axum integration works
    }
}
```

### Integration Tests
```rust
#[tokio::test]
async fn test_ssr_server_startup() {
    // Test that server can start and respond
    // Test SSR rendering
    // Test API endpoints
}
```

### E2E Tests (Future)
```typescript
// playwright.config.ts
test.describe('SSR Demo', () => {
  test('server-side rendering works', async ({ page }) => {
    await page.goto('http://localhost:3000');

    // Check that content is rendered server-side
    const content = await page.innerHTML('html');
    expect(content).toContain('SSR rendered content');

    // Check that hydration completes
    await page.waitForFunction(() => {
      return window.__LEPTOS_HYDRATED__ === true;
    });
  });

  test('animations work in SSR mode', async ({ page }) => {
    // Test MotionDiv animations after hydration
  });
});
```

## Error Prevention

### Dependency Management
1. **Version Pinning**: Pin exact versions in Cargo.toml
2. **Workspace Dependencies**: Use workspace for shared dependencies
3. **Lock File Management**: Commit Cargo.lock for reproducible builds

### Code Quality
1. **API Documentation**: Document correct usage patterns
2. **Type Safety**: Use strong typing to prevent API misuse
3. **Error Handling**: Comprehensive error handling for server setup

## Documentation Updates

### Developer Guide
```markdown
# SSR Demo Setup

## Dependencies

Ensure compatible axum versions:

```toml
[dependencies]
axum = "0.7.5"  # Must match leptos-axum
leptos-axum = "0.8.0"
```

## Server Setup

Use the correct axum integration:

```rust
let app = Router::new()
    .route("/api/*fn_name", get(leptos_axum::handle_server_fns))
    .leptos_routes(routes, get(leptos_axum::render_app_to_stream))
    .fallback(leptos_axum::file_and_error_handler)
    .with_state(leptos_options);
```
```

### Troubleshooting
```markdown
# Common SSR Issues

## "Method not found" errors
**Cause**: Version mismatch between axum and leptos-axum
**Solution**: Align versions in Cargo.toml

## "Handler trait not implemented"
**Cause**: Incorrect function signature for axum handlers
**Solution**: Use leptos_axum handler functions directly

## Server won't start
**Cause**: Port binding issues or missing dependencies
**Solution**: Check port availability and dependency versions
```

## Risk Assessment

### High Risk
- **Dependency Conflicts**: Could affect other parts of the application
- **Breaking Changes**: Axum API changes could require future updates

### Medium Risk
- **SSR Complexity**: More complex than CSR, harder to debug
- **Performance Impact**: SSR has different performance characteristics

### Low Risk
- **Code Changes**: Well-understood axum patterns
- **Testing**: Existing test patterns can be adapted

## Success Criteria

1. **Compilation**: SSR demo compiles without errors
2. **Server Startup**: Server starts and responds to requests
3. **SSR Rendering**: Pages render correctly on server
4. **Hydration**: Client-side hydration works properly
5. **Animations**: MotionDiv animations work in SSR mode
6. **API Compatibility**: All axum APIs used correctly
7. **Documentation**: Setup and troubleshooting guides complete

## Implementation Timeline

- **Day 1**: Dependency analysis and version alignment
- **Day 2**: Code corrections and server setup fixes
- **Day 3**: Compilation and basic server testing
- **Day 4**: SSR functionality validation
- **Day 5**: Documentation and final testing

## Future Considerations

### Version Management
- **Automated Updates**: CI checks for dependency updates
- **Compatibility Matrix**: Documented version compatibility
- **Migration Guides**: For future axum version updates

### Performance Optimization
- **Streaming**: Optimize SSR streaming performance
- **Caching**: Implement SSR result caching
- **Concurrent Rendering**: Parallel SSR processing

### Advanced Features
- **Dynamic Imports**: Code splitting for SSR
- **Edge Rendering**: CDN-based SSR
- **Hybrid Rendering**: CSR + SSR optimization

This design provides a comprehensive solution to the SSR demo axum compatibility issues, ensuring reliable SSR functionality with proper server-side rendering and client-side hydration. 🚀
