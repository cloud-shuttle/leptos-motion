# MotionDiv API Design

## Goal
Single `MotionDiv` component combining all variants.

## Basic Usage
```rust
use leptos_motion_dom::MotionDiv;

view! {
    <MotionDiv
        initial=HashMap::from([("opacity".to_string(), AnimationValue::Number(0.0))])
        animate=HashMap::from([("opacity".to_string(), AnimationValue::Number(1.0))])
    >
        "Hello World"
    </MotionDiv>
}
```

## Advanced Usage
```rust
view! {
    <MotionDiv
        initial=initial_values
        animate=animate_values
        while_hover=hover_values
        while_tap=tap_values
        transition=transition_config
        animation_type=AnimationType::Spring
        drag=true
    >
        "Advanced Animation"
    </MotionDiv>
}
```

## Key Props
- `initial` - Starting animation values
- `animate` - Target animation values  
- `while_hover` - Hover state values
- `while_tap` - Tap state values
- `transition` - Animation configuration
- `animation_type` - CSS, Keyframe, Stagger, or Spring
- `drag` - Enable drag functionality

## Status
⏳ **PENDING** - Need to implement unified component
