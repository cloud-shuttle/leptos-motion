# Quick Fix Guide - Immediate Action Required

## 🚨 EMERGENCY FIXES (Do These First)

### 1. Stop Browser Crashes (5 minutes)

**File**: `crates/leptos-motion-dom/src/animation_engine.rs`

Replace lines 194-197:
```rust
// REPLACE THIS:
let handle = web_sys::window()
    .unwrap()
    .request_animation_frame(closure.as_ref().unchecked_ref())
    .unwrap();

// WITH THIS:
let window = web_sys::window().ok_or("Window not available")?;
let handle = window
    .request_animation_frame(closure.as_ref().unchecked_ref())
    .map_err(|_| "Failed to request animation frame")?;
```

### 2. Fix Memory Leak (2 minutes)

**File**: `crates/leptos-motion-dom/src/animation_engine.rs`

Replace line 199:
```rust
// REPLACE THIS:
closure.forget();

// WITH THIS:
self.animation_closure = Some(closure);
```

Add to AnimationEngine struct:
```rust
pub struct AnimationEngine {
    // ... existing fields ...
    animation_closure: Option<Closure<dyn FnMut()>>,
}
```

### 3. Stop Infinite Recursion (3 minutes)

**File**: `crates/leptos-motion-dom/src/animation_engine.rs`

Replace lines 283-286:
```rust
// REPLACE THIS:
} else if self.is_running {
    self.start_animation_loop();
}

// WITH THIS:
} else if self.is_running && !self.recursion_guard {
    self.recursion_guard = true;
    self.start_animation_loop();
    self.recursion_guard = false;
}
```

Add to AnimationEngine struct:
```rust
pub struct AnimationEngine {
    // ... existing fields ...
    recursion_guard: bool,
}
```

### 4. Use Safe Component (1 minute)

**File**: Your application code

Replace any usage of the broken MotionDiv with:
```rust
use leptos_motion_dom::ReactiveMotionDiv; // Use the fixed version

// This component is already fixed and won't crash
<ReactiveMotionDiv
    initial=initial_values
    animate=animate_values
    transition=transition_config
>
    <div>"Your content"</div>
</ReactiveMotionDiv>
```

---

## ✅ VERIFICATION (2 minutes)

After making these changes:

1. **Build the project**:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

2. **Test in browser**:
   - Load your application
   - Navigate to animation pages
   - Verify no browser crashes

3. **Check console**:
   - No panic messages
   - No memory leak warnings
   - Animations working (even if basic)

---

## 🎯 EXPECTED RESULTS

After these fixes:
- ✅ **No browser crashes**
- ✅ **No memory leaks**
- ✅ **Basic animations working**
- ✅ **Page remains responsive**

---

## 🚨 IF STILL CRASHING

If you still get crashes after these fixes:

1. **Use the safe fallback**:
   ```rust
   use leptos_motion_dom::SafeMotionDiv; // Use this instead
   ```

2. **Disable animations temporarily**:
   ```rust
   // Comment out all MotionDiv usage
   // <MotionDiv>...</MotionDiv>
   <div>Static content</div>
   ```

3. **Check for other issues**:
   - Look for other `unwrap()` calls in your code
   - Check for infinite loops in your components
   - Verify WASM build is successful

---

## 📞 NEXT STEPS

Once these emergency fixes are working:

1. **Read the full remediation plan**: `LEPTOS_MOTION_REMEDIATION_PLAN.md`
2. **Implement comprehensive fixes**: `CRITICAL_FIXES_IMPLEMENTATION.md`
3. **Add proper testing**: Follow the testing strategy
4. **Monitor for issues**: Set up error tracking

---

**Status**: 🚨 **EMERGENCY - IMPLEMENT NOW**  
**Time Required**: 10 minutes  
**Risk**: LOW (these are safe fixes)  
**Impact**: HIGH (stops crashes immediately)
