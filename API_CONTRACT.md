# 🎯 Leptos Motion - API Contract

**Version**: 1.0.0  
**Status**: ✅ **STABLE** (Phase 1 Complete)  
**Last Updated**: December 2024

## 📋 **Contract Overview**

This document defines the stable API contract for the `leptos-motion` library.
This contract ensures backward compatibility and provides clear expectations for
users and contributors.

**⚠️ Breaking Changes**: Any changes to this contract require a major version
bump and migration guide.

---

## 🧩 **Core Components**

### `MotionDiv` Component

The primary animation component for div elements.

#### **Component Signature**

```rust
#[component]
pub fn MotionDiv(
    // Basic Props
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,

    // Animation Props
    #[prop(optional)] initial: Option<AnimationTarget>,
    #[prop(optional)] animate: Option<AnimationTarget>,
    #[prop(optional)] transition: Option<Transition>,

    // Interactive Props
    #[prop(optional)] while_hover: Option<AnimationTarget>,
    #[prop(optional)] while_tap: Option<AnimationTarget>,
    #[prop(optional)] layout: Option<bool>,

    // Drag Props
    #[prop(optional)] drag: Option<DragConfig>,
    #[prop(optional)] drag_constraints: Option<DragConstraints>,

    // Children
    children: Children,
) -> impl IntoView
```

#### **Prop Types**

##### `AnimationTarget`

```rust
pub type AnimationTarget = HashMap<String, AnimationValue>;
```

- **Purpose**: Defines animation properties and their target values
- **Usage**: Used for `initial`, `animate`, `while_hover`, `while_tap` props
- **Stability**: ✅ **STABLE** - Type alias, not a constructor

##### `AnimationValue`

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationValue {
    Number(f64),
    String(String),
    Transform(Transform),
}
```

- **Purpose**: Represents different types of animatable values
- **Stability**: ✅ **STABLE** - Core enum, variants may be added but not
  removed

##### `Transition`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Transition {
    pub duration: Option<f64>,
    pub delay: Option<f64>,
    pub ease: Easing,
    pub repeat: RepeatConfig,
    pub stagger: Option<StaggerConfig>,
}
```

- **Purpose**: Defines animation timing and behavior
- **Stability**: ✅ **STABLE** - Core struct, fields may be added but not
  removed

##### `DragConfig`

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct DragConfig {
    pub axis: Option<DragAxis>,
    pub momentum: Option<bool>,
    pub elastic: Option<f64>,
    pub constraints: Option<DragConstraints>,
}
```

- **Purpose**: Configures drag behavior
- **Stability**: ✅ **STABLE** - Core struct, fields may be added but not
  removed

---

## 🔄 **Reactivity Contract**

### **Animation Reactivity**

- ✅ `animate` prop changes trigger reactive style updates
- ✅ `while_hover` activates on mouse enter/leave
- ✅ `while_tap` activates on click with 150ms duration
- ✅ All animation props use `Effect::new()` for proper reactivity

### **Event Handling**

- ✅ `on:mouseenter` → sets hover state
- ✅ `on:mouseleave` → clears hover state
- ✅ `on:click` → sets tap state (150ms auto-clear)
- ✅ `on:mousedown` → initiates drag (if drag enabled)
- ✅ `on:mousemove` → updates drag position (if dragging)
- ✅ `on:mouseup` → ends drag and starts momentum (if enabled)

---

## 🎨 **Animation Behavior Contract**

### **Style Application**

- ✅ Initial styles applied on component mount
- ✅ Animate styles applied reactively when prop changes
- ✅ Hover styles applied when `_is_hovered` is true
- ✅ Tap styles applied when `_is_tapped` is true
- ✅ Drag position applied as transform when dragging

### **Style Merging**

- ✅ Animation styles merge with existing styles
- ✅ Drag transform overrides other transforms
- ✅ Style prop merges with generated styles

---

## 🚫 **Breaking Change Policy**

### **What CAN Change (Minor Versions)**

- ✅ Adding new enum variants to `AnimationValue`
- ✅ Adding new fields to structs (with defaults)
- ✅ Adding new component props (optional)
- ✅ Performance improvements
- ✅ Bug fixes that don't change behavior

### **What CANNOT Change (Requires Major Version)**

- ❌ Removing or renaming existing props
- ❌ Changing prop types (e.g., `Option<T>` → `T`)
- ❌ Removing enum variants
- ❌ Changing default values
- ❌ Breaking reactivity behavior
- ❌ Changing event handler behavior

---

## 🧪 **Testing Contract**

### **Required Test Coverage**

- ✅ All prop combinations compile
- ✅ Type system consistency
- ✅ Reactive behavior verification
- ✅ Event handler functionality
- ✅ Style application correctness

### **Test Categories**

1. **API Tests**: Verify all prop types work correctly
2. **Reactivity Tests**: Verify reactive behavior
3. **Integration Tests**: Verify component works in real scenarios
4. **Performance Tests**: Verify acceptable performance

---

## 📚 **Usage Examples**

### **Basic Animation**

```rust
view! { cx,
    <MotionDiv
        initial=initial_opacity
        animate=target_opacity
    >
        "Animated Content"
    </MotionDiv>
}
```

### **Interactive Animation**

```rust
view! { cx,
    <MotionDiv
        while_hover=hover_scale
        while_tap=tap_scale
    >
        "Interactive Content"
    </MotionDiv>
}
```

### **Drag Animation**

```rust
view! { cx,
    <MotionDiv
        drag=drag_config
        drag_constraints=constraints
    >
        "Draggable Content"
    </MotionDiv>
}
```

---

## 🔮 **Future Compatibility**

### **Planned Additions (Non-Breaking)**

- 🚧 Animation engine integration (Phase 2)
- 🚧 Spring physics (Phase 2)
- 🚧 Layout animations (Phase 3)
- 🚧 Scroll animations (Phase 3)
- 🚧 Timeline sequences (Phase 4)

### **Migration Strategy**

- All new features will be additive
- Existing APIs will remain unchanged
- New props will be optional with sensible defaults
- Deprecation warnings will be provided before removal

---

## ✅ **Contract Validation**

This contract is validated by:

- ✅ 241 passing tests
- ✅ Type system consistency
- ✅ Reactive behavior verification
- ✅ Event handler functionality
- ✅ Style application correctness

**Contract Status**: ✅ **VALIDATED AND STABLE**
