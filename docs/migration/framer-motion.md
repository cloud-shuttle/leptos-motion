# Migration Guide: Framer Motion to Leptos Motion

This guide helps you migrate from Framer Motion (React) to Leptos Motion (Rust/Leptos).

## Key Differences

### 1. Language and Framework

- **Framer Motion**: JavaScript/TypeScript with React
- **Leptos Motion**: Rust with Leptos

### 2. Type Safety

- **Framer Motion**: Runtime type checking
- **Leptos Motion**: Compile-time type safety

### 3. Component Names

- **Framer Motion**: `motion.div`, `motion.span`
- **Leptos Motion**: `MotionDiv`, `MotionSpan`

### 4. Animation Values

- **Framer Motion**: Object literals with mixed types
- **Leptos Motion**: Strongly typed `AnimationValue` enum

## Basic Migration

### Simple Animation

**Framer Motion (JavaScript)**

```javascript
import { motion } from 'framer-motion';

function App() {
  return (
    <motion.div
      initial={{ opacity: 0, scale: 0.5 }}
      animate={{ opacity: 1, scale: 1 }}
      transition={{ duration: 0.5 }}
    >
      Hello Framer Motion!
    </motion.div>
  );
}
```

**Leptos Motion (Rust)**

```rust
use leptos::*;
use leptos_motion::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <MotionDiv
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.5)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "scale" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseOut,
                ..Default::default()
            }
            class="my-element".to_string()
        >
            "Hello Leptos Motion!"
        </MotionDiv>
    }
}
```

## Animation Values Migration

### Framer Motion Values

```javascript
// Framer Motion - mixed types in object
{
  opacity: 0.5,           // number
  scale: 1.2,             // number
  x: 100,                 // number (pixels)
  y: 50,                  // number (pixels)
  rotate: 45,             // number (degrees)
  backgroundColor: "#ff0000" // string
}
```

### Leptos Motion Values

```rust
// Leptos Motion - strongly typed
motion_target!(
    "opacity" => AnimationValue::Number(0.5),           // unitless
    "scale" => AnimationValue::Number(1.2),             // unitless
    "x" => AnimationValue::Pixels(100.0),               // pixels
    "y" => AnimationValue::Pixels(50.0),                // pixels
    "rotate" => AnimationValue::Degrees(45.0),          // degrees
    "backgroundColor" => AnimationValue::Color("#ff0000".to_string()) // color
)
```

## Component Migration

### Basic Motion Components

**Framer Motion**

```javascript
<motion.div>Content</motion.div>
<motion.span>Text</motion.span>
<motion.button>Button</motion.button>
```

**Leptos Motion**

```rust
<MotionDiv class="my-class".to_string()>Content</MotionDiv>
<MotionSpan class="my-class".to_string()>Text</MotionSpan>
// Note: No MotionButton - use MotionDiv with button styling
<MotionDiv class="button-styles".to_string()>Button</MotionDiv>
```

### Props Migration

**Framer Motion**

```javascript
<motion.div
  className='my-class'
  style={{ color: 'red' }}
  initial={{ opacity: 0 }}
  animate={{ opacity: 1 }}
  exit={{ opacity: 0 }}
  transition={{ duration: 0.5 }}
  whileHover={{ scale: 1.1 }}
  whileTap={{ scale: 0.9 }}
  drag
  dragConstraints={{ left: -100, right: 100 }}
>
  Content
</motion.div>
```

**Leptos Motion**

```rust
<MotionDiv
    class="my-class".to_string()
    style="color: red;".to_string()
    initial=motion_target!("opacity" => AnimationValue::Number(0.0))
    animate=motion_target!("opacity" => AnimationValue::Number(1.0))
    exit=motion_target!("opacity" => AnimationValue::Number(0.0))
    transition=Transition {
        duration: Some(0.5),
        ease: Easing::EaseOut,
        ..Default::default()
    }
    while_hover=motion_target!("scale" => AnimationValue::Number(1.1))
    while_tap=motion_target!("scale" => AnimationValue::Number(0.9))
    drag=DragConfig::default()
    drag_constraints=DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: None,
        bottom: None,
    }
>
    "Content"
</MotionDiv>
```

## Gesture Migration

### Hover Gestures

**Framer Motion**

```javascript
<motion.div whileHover={{ scale: 1.1, backgroundColor: '#ff0000' }} transition={{ duration: 0.2 }}>
  Hover me!
</motion.div>
```

**Leptos Motion**

```rust
<MotionDiv
    while_hover=motion_target!(
        "scale" => AnimationValue::Number(1.1),
        "backgroundColor" => AnimationValue::Color("#ff0000".to_string())
    )
    transition=Transition {
        duration: Some(0.2),
        ease: Easing::EaseOut,
        ..Default::default()
    }
    class="hover-element".to_string()
>
    "Hover me!"
</MotionDiv>
```

### Drag Gestures

**Framer Motion**

```javascript
<motion.div
  drag
  dragConstraints={{ left: -100, right: 100, top: -50, bottom: 50 }}
  dragElastic={0.7}
  whileDrag={{ scale: 1.05 }}
>
  Drag me!
</motion.div>
```

**Leptos Motion**

```rust
<MotionDiv
    drag=DragConfig::default()
    drag_constraints=DragConstraints {
        left: Some(-100.0),
        right: Some(100.0),
        top: Some(-50.0),
        bottom: Some(50.0),
    }
    while_drag=motion_target!("scale" => AnimationValue::Number(1.05))
    class="drag-element".to_string()
>
    "Drag me!"
</MotionDiv>
```

## Layout Animations

**Framer Motion**

```javascript
<motion.div layout>
  <motion.div layout>Item 1</motion.div>
  <motion.div layout>Item 2</motion.div>
</motion.div>
```

**Leptos Motion**

```rust
<MotionDiv layout=true class="container".to_string()>
    <MotionDiv layout=true class="item".to_string()>"Item 1"</MotionDiv>
    <MotionDiv layout=true class="item".to_string()>"Item 2"</MotionDiv>
</MotionDiv>
```

## Presence Animations

**Framer Motion**

```javascript
import { AnimatePresence } from 'framer-motion';

<AnimatePresence mode='wait'>
  {show && (
    <motion.div
      initial={{ opacity: 0, scale: 0.8 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.8 }}
    >
      Content
    </motion.div>
  )}
</AnimatePresence>;
```

**Leptos Motion**

```rust
<AnimatePresence mode=PresenceMode::Wait>
    {move || if show() {
        Some(view! {
            <MotionDiv
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "scale" => AnimationValue::Number(0.8)
                )
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "scale" => AnimationValue::Number(1.0)
                )
                exit=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "scale" => AnimationValue::Number(0.8)
                )
                class="presence-element".to_string()
            >
                "Content"
            </MotionDiv>
        })
    } else {
        None
    }}
</AnimatePresence>
```

## Variants Migration

**Framer Motion**

```javascript
const variants = {
  hidden: { opacity: 0, x: -100 },
  visible: { opacity: 1, x: 0 },
  exit: { opacity: 0, x: 100 },
};

<motion.div variants={variants} initial='hidden' animate='visible' exit='exit'>
  Content
</motion.div>;
```

**Leptos Motion**

```rust
let variants = Variants::new()
    .variant("hidden", motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "x" => AnimationValue::Pixels(-100.0)
    ))
    .variant("visible", motion_target!(
        "opacity" => AnimationValue::Number(1.0),
        "x" => AnimationValue::Pixels(0.0)
    ))
    .variant("exit", motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "x" => AnimationValue::Pixels(100.0)
    ));

<MotionDiv
    variants=variants
    initial="hidden"
    animate="visible"
    exit="exit"
    class="variants-demo".to_string()
>
    "Content"
</MotionDiv>
```

## Transition Migration

**Framer Motion**

```javascript
// Simple transition
transition={{ duration: 0.5 }}

// Spring transition
transition={{
  type: "spring",
  stiffness: 100,
  damping: 10,
  mass: 1
}}

// Custom easing
transition={{
  duration: 0.8,
  ease: [0.25, 0.1, 0.25, 1.0]
}}
```

**Leptos Motion**

```rust
// Simple transition
transition=Transition {
    duration: Some(0.5),
    ease: Easing::EaseOut,
    ..Default::default()
}

// Spring transition
transition=Transition {
    duration: Some(0.6),
    ease: Easing::Spring(SpringConfig::default()
        .stiffness(100.0)
        .damping(10.0)
        .mass(1.0)
    ),
    ..Default::default()
}

// Custom easing
transition=Transition {
    duration: Some(0.8),
    ease: Easing::CubicBezier(0.25, 0.1, 0.25, 1.0),
    ..Default::default()
}
```

## Keyframes Migration

**Framer Motion**

```javascript
<motion.div
  animate={{
    opacity: [0, 0.5, 1],
    scale: [0.5, 1.1, 1],
  }}
  transition={{ duration: 2 }}
>
  Keyframe Animation
</motion.div>
```

**Leptos Motion**

```rust
use leptos_motion_core::animation::Keyframes;

let keyframes = Keyframes::new()
    .add_keyframe(0.0, motion_target!(
        "opacity" => AnimationValue::Number(0.0),
        "scale" => AnimationValue::Number(0.5)
    ))
    .add_keyframe(0.5, motion_target!(
        "opacity" => AnimationValue::Number(0.5),
        "scale" => AnimationValue::Number(1.1)
    ))
    .add_keyframe(1.0, motion_target!(
        "opacity" => AnimationValue::Number(1.0),
        "scale" => AnimationValue::Number(1.0)
    ));

<MotionDiv
    animate=keyframes.to_animation_target()
    transition=Transition {
        duration: Some(2.0),
        ease: Easing::EaseInOut,
        ..Default::default()
    }
    class="keyframe-demo".to_string()
>
    "Keyframe Animation"
</MotionDiv>
```

## Event Handlers Migration

**Framer Motion**

```javascript
<motion.div
  onAnimationStart={() => console.log('Animation started')}
  onAnimationComplete={() => console.log('Animation completed')}
  onHoverStart={() => console.log('Hover started')}
  onHoverEnd={() => console.log('Hover ended')}
>
  Content
</motion.div>
```

**Leptos Motion**

```rust
let event_handlers = EventHandlers::new()
    .on_animation_start(|_| {
        console_log!("Animation started");
    })
    .on_animation_end(|_| {
        console_log!("Animation completed");
    });

<MotionDiv
    event_handlers=Some(event_handlers)
    class="event-demo".to_string()
>
    "Content"
</MotionDiv>
```

## Common Migration Patterns

### 1. State Management

**Framer Motion**

```javascript
const [isVisible, setIsVisible] = useState(false);

<motion.div animate={isVisible ? { opacity: 1 } : { opacity: 0 }}>Content</motion.div>;
```

**Leptos Motion**

```rust
let (is_visible, set_is_visible) = signal(false);

<MotionDiv
    animate=motion_target!(
        "opacity" => AnimationValue::Number(if is_visible.get() { 1.0 } else { 0.0 })
    )
    class="state-demo".to_string()
>
    "Content"
</MotionDiv>
```

### 2. Conditional Rendering

**Framer Motion**

```javascript
{
  showModal && (
    <motion.div initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
      Modal Content
    </motion.div>
  );
}
```

**Leptos Motion**

```rust
{move || if show_modal() {
    Some(view! {
        <MotionDiv
            initial=motion_target!("opacity" => AnimationValue::Number(0.0))
            animate=motion_target!("opacity" => AnimationValue::Number(1.0))
            exit=motion_target!("opacity" => AnimationValue::Number(0.0))
            class="modal".to_string()
        >
            "Modal Content"
        </MotionDiv>
    })
} else {
    None
}}
```

### 3. List Animations

**Framer Motion**

```javascript
<motion.div>
  {items.map((item, index) => (
    <motion.div
      key={item.id}
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ delay: index * 0.1 }}
    >
      {item.content}
    </motion.div>
  ))}
</motion.div>
```

**Leptos Motion**

```rust
<MotionDiv class="list-container".to_string()>
    <For
        each=items
        key=|item| item.id
        children=move |item| {
            let index = items.get().iter().position(|i| i.id == item.id).unwrap_or(0);
            view! {
                <MotionDiv
                    key=item.id.to_string()
                    initial=motion_target!(
                        "opacity" => AnimationValue::Number(0.0),
                        "y" => AnimationValue::Pixels(20.0)
                    )
                    animate=motion_target!(
                        "opacity" => AnimationValue::Number(1.0),
                        "y" => AnimationValue::Pixels(0.0)
                    )
                    transition=Transition {
                        duration: Some(0.3),
                        delay: Some(index as f64 * 0.1),
                        ease: Easing::EaseOut,
                        ..Default::default()
                    }
                    class="list-item".to_string()
                >
                    {item.content}
                </MotionDiv>
            }
        }
    />
</MotionDiv>
```

## Migration Checklist

### ✅ Pre-Migration

- [ ] Understand Rust and Leptos basics
- [ ] Set up Leptos development environment
- [ ] Review Leptos Motion documentation

### ✅ Basic Migration

- [ ] Convert `motion.div` to `MotionDiv`
- [ ] Convert `motion.span` to `MotionSpan`
- [ ] Update animation values to use `AnimationValue` enum
- [ ] Convert object literals to `motion_target!` macro
- [ ] Update string props to use `.to_string()`

### ✅ Advanced Migration

- [ ] Convert variants to `Variants` type
- [ ] Update transitions to `Transition` type
- [ ] Convert keyframes to `Keyframes` type
- [ ] Update event handlers to `EventHandlers` type
- [ ] Convert presence animations to `AnimatePresence`

### ✅ Testing and Optimization

- [ ] Test all animations work correctly
- [ ] Optimize performance for Rust/WebAssembly
- [ ] Update CSS classes and styling
- [ ] Test on different browsers and devices

## Common Issues and Solutions

### Issue: Type Mismatches

**Problem**: `expected String, found &str`
**Solution**: Use `.to_string()` for string literals

```rust
// ❌ Wrong
class="my-class"

// ✅ Correct
class="my-class".to_string()
```

### Issue: Missing Required Props

**Problem**: Missing required props for components
**Solution**: Provide all required props

```rust
// ❌ Wrong
<MotionDiv>Content</MotionDiv>

// ✅ Correct
<MotionDiv class="my-class".to_string()>Content</MotionDiv>
```

### Issue: Wrong Animation Value Types

**Problem**: Using wrong `AnimationValue` variant
**Solution**: Use correct variant for property type

```rust
// ❌ Wrong
"opacity" => AnimationValue::Pixels(1.0)

// ✅ Correct
"opacity" => AnimationValue::Number(1.0)
```

### Issue: Option Type Handling

**Problem**: Not handling `Option` types properly
**Solution**: Use `unwrap_or()` or pattern matching

```rust
// ❌ Wrong
let duration = transition.duration;

// ✅ Correct
let duration = transition.duration.unwrap_or(0.3);
```

## Performance Considerations

### 1. Use Transform Properties

Prefer `x`, `y`, `scale`, `rotate` over layout properties for better performance.

### 2. Limit Concurrent Animations

Too many simultaneous animations can impact performance.

### 3. Use Hardware Acceleration

Prefer `EaseOut` and transform properties for hardware acceleration.

### 4. Optimize for WebAssembly

Consider the overhead of Rust/WebAssembly compilation.

## Getting Help

- **Documentation**: Check `docs/` directory for comprehensive guides
- **Examples**: Look at working examples in `examples/` directory
- **Community**: Join Leptos community for help and support
- **Issues**: Report bugs or ask questions on GitHub

Happy migrating! 🚀✨
