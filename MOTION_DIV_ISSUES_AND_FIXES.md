# MotionDiv Issues and Fixes Documentation

## 🎯 **Overview**
This document identifies and documents the real issues with the MotionDiv component in leptos-motion, and provides working solutions.

## ❌ **Critical Issues with MotionDiv**

### **1. Missing Required Props**
**Issue**: `node_ref` is required but not documented
```rust
// ❌ This fails with "Missing required field node_ref"
<MotionDiv
    class="motion-box"
    initial=HashMap::new()
>
    "Content"
</MotionDiv>

// ✅ This works
<MotionDiv
    node_ref=NodeRef::new()  // ← Required but not documented!
    class="motion-box"
    initial=HashMap::new()
>
    "Content"
</MotionDiv>
```

### **2. WASM Time System Panic**
**Issue**: `std::time::SystemTime::now()` panics in WASM
```
panicked at library/std/src/sys/pal/wasm/../unsupported/time.rs:31:9:
time not implemented on this platform
```
**Root Cause**: MotionDiv tries to use `SystemTime::now()` for animations, but WASM doesn't support it.

### **3. RefCell Borrowing Panic**
**Issue**: `RefCell already borrowed` panic in animation system
```
panicked at core::cell::panic_already_borrowed::h0e775208d711c2bb
```
**Root Cause**: Animation system tries to borrow RefCell that's already borrowed.

### **4. Reactive Signal Context Issues**
**Issue**: Accessing signals outside reactive context
```
you access a reactive_graph::signal::read::ReadSignal<bool> outside a reactive tracking context
```
**Root Cause**: MotionDiv accesses signals in wrong context.

### **2. Inconsistent Prop Naming**
**Issue**: Some props use underscore prefix, others don't
```rust
// ❌ Confusing API
_transition=Transition { ... }  // ← Why underscore?
_layout=true                    // ← Why underscore?
class="motion-box"              // ← No underscore
style="color: red"              // ← No underscore
```

### **3. Poor Type Safety**
**Issue**: Props are inconsistently typed
```rust
// ❌ Inconsistent patterns
class: String,                   // ← Not Option<String>
style: String,                   // ← Not Option<String>
initial: Option<HashMap<...>>,  // ← Option<T>
animate: Option<HashMap<...>>,  // ← Option<T>
```

### **4. Missing Click Handler Support**
**Issue**: No `on_click` prop despite internal click handling
```rust
// ❌ This doesn't work
<MotionDiv
    on_click=move |_| { ... }  // ← Not supported as prop
>
    "Content"
</MotionDiv>

// ✅ Workaround: Use wrapper div
<div on:click=move |_| { ... }>
    <MotionDiv>
        "Content"
    </MotionDiv>
</div>
```

### **5. Deprecated API Warnings**
**Issue**: Using deprecated methods without clear migration path
```
warning: use of deprecated method `EventDrivenMotionDivPropsBuilder::build`: 
Missing required field node_ref
```

## ✅ **Working MotionDiv Pattern**

### **Complete Working Example**
```rust
use leptos::*;
use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;

#[component]
pub fn WorkingMotionDiv() -> impl IntoView {
    let (is_animated, set_is_animated) = signal(false);

    view! {
        <div>
            <button on:click=move |_| set_is_animated.set(!is_animated.get())>
                "Toggle Animation"
            </button>
            
            <MotionDiv
                node_ref=NodeRef::new()  // ← REQUIRED
                class="motion-box".to_string()
                style="width: 100px; height: 100px; background: #ff6b6b; border-radius: 8px; display: flex; align-items: center; justify-content: center; color: white;".to_string()
                initial=HashMap::from([
                    ("x".to_string(), AnimationValue::Pixels(0.0)),
                    ("opacity".to_string(), AnimationValue::Number(1.0)),
                ])
                animate=if is_animated.get() { 
                    HashMap::from([
                        ("x".to_string(), AnimationValue::Pixels(100.0)),
                        ("opacity".to_string(), AnimationValue::Number(0.8)),
                    ])
                } else { HashMap::new() }
                while_hover=HashMap::from([
                    ("scale".to_string(), AnimationValue::Number(1.1)),
                ])
                while_tap=HashMap::from([
                    ("scale".to_string(), AnimationValue::Number(0.95)),
                ])
                _transition=Transition {
                    duration: Some(0.6),
                    ease: Easing::EaseInOut,
                    ..Default::default()
                }
            >
                "Animated Content"
            </MotionDiv>
        </div>
    }
}
```

## 🔧 **Required Fixes for MotionDiv**

### **1. Documentation Fixes**
- [ ] Document that `node_ref` is required
- [ ] Explain why some props have underscore prefix
- [ ] Provide complete working examples
- [ ] Document click handler workaround

### **2. API Design Fixes**
- [ ] Make `node_ref` optional with default
- [ ] Standardize prop naming (remove underscores)
- [ ] Add `on_click` prop support
- [ ] Improve type consistency

### **3. Code Quality Fixes**
- [ ] Remove deprecated method warnings
- [ ] Add better error messages
- [ ] Improve component documentation

## 📋 **Current Status**

### **✅ What Works**
- Basic animations with `initial` and `animate`
- Hover animations with `while_hover`
- Tap animations with `while_tap`
- Transition configuration with `_transition`
- CSS classes and styles

### **❌ What's Broken**
- Missing `node_ref` requirement
- No `on_click` prop support
- Inconsistent prop naming
- Poor error messages
- Missing documentation

### **🔄 Workarounds**
- Always include `node_ref=NodeRef::new()`
- Use wrapper divs for click handlers
- Use `_transition` instead of `transition`
- Use `_layout` instead of `layout`

## 🎯 **Recommendations**

1. **Immediate**: Fix the `node_ref` requirement issue
2. **Short-term**: Add proper documentation
3. **Long-term**: Redesign API for consistency
4. **Testing**: Add comprehensive examples

## 📚 **References**

- Source: `crates/leptos-motion-dom/src/event_driven_motion_div.rs`
- Tests: `crates/leptos-motion-dom/src/api_contract_tests.rs`
- Examples: `examples/simple-animation-demo/src/simple_demo.rs`
