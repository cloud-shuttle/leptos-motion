# 🚨 Leptos Motion Fix Roadmap

## **Current State: BROKEN** ❌
- Custom Axum server trying to serve WASM (wrong approach)
- Mixed server/WASM dependencies causing build conflicts
- Complex build process that doesn't follow Leptos conventions
- Examples that don't actually work

## **Target State: WORKING** ✅
- Use Trunk (standard Leptos build tool)
- Clean separation of concerns
- Simple, reliable build process
- Examples that actually demonstrate leptos-motion

---

## **Phase 1: Fix the Build System** 🔧

### 1.1 Remove Custom Server
- [ ] Delete `examples/comprehensive-showcase/src/main.rs` (Axum server)
- [ ] Remove server dependencies from `Cargo.toml`
- [ ] Use Trunk for building and serving (like working examples)

### 1.2 Fix Dependencies
- [ ] Clean up `Cargo.toml` to match working examples
- [ ] Remove server-only dependencies
- [ ] Use proper leptos-motion features: `["csr", "minimal"]`

### 1.3 Fix Build Configuration
- [ ] Use `Trunk.toml` like working examples
- [ ] Proper `index.html` with Trunk directives
- [ ] Remove custom WASM serving logic

---

## **Phase 2: Fix the Examples** 🎨

### 2.1 Use Working Template
- [ ] Copy structure from `examples/motion-showcase-working/`
- [ ] Adapt the comprehensive showcase to use Trunk
- [ ] Test that basic rendering works

### 2.2 Fix Component Issues
- [ ] Fix MotionDiv usage (use proper props)
- [ ] Fix AnimationTarget construction
- [ ] Fix component mounting

### 2.3 Add Proper Styling
- [ ] Use tailwind-rs-core properly
- [ ] Remove CDN dependencies
- [ ] Ensure styles are generated correctly

---

## **Phase 3: Add Testing** 🧪

### 3.1 Playwright Tests
- [ ] Test against Trunk dev server (not custom server)
- [ ] Test component rendering
- [ ] Test interactions

### 3.2 Integration Tests
- [ ] Test WASM loading
- [ ] Test leptos-motion components
- [ ] Test performance

---

## **Phase 4: Documentation** 📚

### 4.1 Update Examples
- [ ] Document the working approach
- [ ] Remove broken examples
- [ ] Create clear getting started guide

### 4.2 Best Practices
- [ ] Document proper Trunk usage
- [ ] Document leptos-motion integration
- [ ] Document common pitfalls

---

## **Immediate Actions** ⚡

1. **STOP** using custom Axum server
2. **START** using Trunk like working examples
3. **COPY** the working example structure
4. **TEST** that basic rendering works
5. **ITERATE** on components once build works

---

## **Success Criteria** ✅

- [ ] `trunk serve` works without errors
- [ ] WASM loads and renders components
- [ ] leptos-motion components work
- [ ] No custom server needed
- [ ] Examples are actually functional
- [ ] Playwright tests pass

---

## **Why This Approach Works** 💡

1. **Trunk is the standard** - All Leptos examples use it
2. **Separation of concerns** - Build tool handles WASM, not custom server
3. **Proven approach** - Working examples already exist
4. **Simpler maintenance** - Less custom code to maintain
5. **Better DX** - Standard Leptos development workflow

---

**The fundamental issue: We're reinventing the wheel instead of using the standard Leptos toolchain.**
