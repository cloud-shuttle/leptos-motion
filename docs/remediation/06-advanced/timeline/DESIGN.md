# Timeline System Design Document

## Overview

The Timeline System provides orchestration capabilities for complex animations involving multiple elements with precise timing control. Unlike individual component animations, timelines allow coordinating animations across the entire application with fine-grained control over when each animation starts, how long it runs, and how elements relate to each other.

## Core Concepts

### Timeline Definition
A timeline orchestrates multiple animations with precise timing:

```rust
let timeline = Timeline::new()
    .add(AnimationTrack {
        target: "#hero-title".to_string(),
        animation: hashmap! {
            "opacity" => AnimationValue::Number(1.0),
            "y" => AnimationValue::Pixels(0.0),
        },
        start_time: 0.0,
        duration: 0.8,
        easing: Some(EasingFunction::EaseOut),
    })
    .add(AnimationTrack {
        target: "#hero-subtitle".to_string(),
        animation: hashmap! {
            "opacity" => AnimationValue::Number(1.0),
            "x" => AnimationValue::Pixels(0.0),
        },
        start_time: 0.3,  // Starts 300ms after timeline begins
        duration: 0.6,
    })
    .add(AnimationTrack {
        target: "#cta-button".to_string(),
        animation: hashmap! {
            "scale" => AnimationValue::Number(1.0),
            "boxShadow" => AnimationValue::String("0 4px 12px rgba(0,0,0,0.15)".to_string()),
        },
        start_time: 0.8,
        duration: 0.4,
    });
```

### Timeline Control
Control timeline playback with fine-grained control:

```rust
// Start timeline
timeline.play();

// Pause at specific time
timeline.pause();
timeline.seek(1.5);  // Jump to 1.5 seconds

// Control playback rate
timeline.playback_rate(0.5);  // Half speed
timeline.playback_rate(2.0);  // Double speed

// Event callbacks
timeline.on_complete(|| {
    console::log("Timeline animation completed!");
});
```

## API Design

### AnimationTrack Structure
```rust
#[derive(Clone)]
pub struct AnimationTrack {
    pub target: String,                    // CSS selector or element ID
    pub animation: HashMap<String, AnimationValue>,
    pub start_time: f64,                   // When to start (seconds)
    pub duration: f64,                     // How long to run (seconds)
    pub easing: Option<EasingFunction>,    // Optional easing override
    pub delay: Option<f64>,                // Additional delay
    pub repeat: Option<RepeatConfig>,      // Repeat configuration
}

impl AnimationTrack {
    pub fn new(target: String, animation: HashMap<String, AnimationValue>) -> Self;
    pub fn start_time(mut self, time: f64) -> Self;
    pub fn duration(mut self, duration: f64) -> Self;
    pub fn easing(mut self, easing: EasingFunction) -> Self;
}
```

### Timeline Structure
```rust
#[derive(Clone)]
pub struct Timeline {
    tracks: Vec<AnimationTrack>,
    current_time: f64,
    playback_rate: f64,
    is_playing: bool,
    is_paused: bool,
    loop_count: Option<u32>,
    on_complete: Option<Callback<()>>,
    on_update: Option<Callback<f64>>,  // Called with current time
}

impl Timeline {
    pub fn new() -> Self;
    pub fn add(mut self, track: AnimationTrack) -> Self;
    pub fn play(&mut self) -> Result<(), TimelineError>;
    pub fn pause(&mut self);
    pub fn stop(&mut self);
    pub fn seek(&mut self, time: f64);
    pub fn playback_rate(&mut self, rate: f64);
    pub fn loop_count(mut self, count: u32) -> Self;
    pub fn on_complete<F>(mut self, callback: F) -> Self
        where F: Fn() + 'static;
    pub fn on_update<F>(mut self, callback: F) -> Self
        where F: Fn(f64) + 'static;
}
```

### Timeline Component
```rust
#[component]
pub fn TimelineController(
    timeline: Timeline,
    auto_play: bool,
    children: Children,
) -> impl IntoView
```

## Implementation Strategy

### 1. Timeline Engine
- Central timeline controller managing multiple animation tracks
- Precise timing coordination using `requestAnimationFrame`
- Efficient track scheduling and execution

### 2. Track Management
- Register tracks with their target elements
- Calculate absolute timing for each track
- Handle overlapping animations efficiently

### 3. State Management
- Track current playback position
- Handle play/pause/seek operations
- Maintain timeline state across component updates

## Advanced Features

### Timeline Composition
Combine multiple timelines for complex sequences:

```rust
let intro_timeline = create_intro_timeline();
let main_timeline = create_main_timeline();

let combined_timeline = TimelineSequence::new()
    .add_timeline(intro_timeline)
    .add_timeline(main_timeline);
```

### Dynamic Timelines
Modify timelines at runtime:

```rust
// Add new tracks dynamically
timeline.add_track(new_track);

// Modify existing tracks
timeline.update_track("hero-title", |track| {
    track.duration = 1.0;
});

// Remove tracks
timeline.remove_track("old-element");
```

### Timeline Labels
Add named labels for easier timeline navigation:

```rust
let timeline = Timeline::new()
    .label("start", 0.0)
    .label("hero_complete", 1.2)
    .label("content_ready", 2.5);

// Seek to labeled position
timeline.seek_to_label("content_ready");
```

## Integration Points

### With Variants System
Timelines can trigger variant changes:

```rust
timeline.add(AnimationTrack {
    target: "#card".to_string(),
    animation: VariantTrigger::new("expanded"),
    start_time: 1.0,
    duration: 0.5,
});
```

### With Scroll Animations
Timeline progress tied to scroll position:

```rust
let scroll_timeline = ScrollTimeline::new()
    .start_scroll(0.0)    // Start when scroll = 0
    .end_scroll(1000.0)   // End when scroll = 1000px
    .timeline(my_timeline);
```

### With Gesture System
Timeline control via user interactions:

```rust
// Play timeline on click
on_click(move |_| timeline.play());

// Scrub timeline with drag
on_drag(move |progress| timeline.seek(progress * timeline.duration()));
```

## Performance Considerations

- Efficient track scheduling using priority queues
- Minimal DOM queries through element caching
- Batched animation updates
- Memory-efficient timeline state storage

## Playback Control

### Basic Controls
```rust
timeline.play();      // Start/resume playback
timeline.pause();     // Pause at current position
timeline.stop();      // Stop and reset to beginning
timeline.seek(time);  // Jump to specific time
```

### Advanced Controls
```rust
timeline.reverse();   // Play backwards
timeline.looping(true); // Enable looping
timeline.pingpong();  // Alternate forward/backward
```

## Event System

### Timeline Events
```rust
timeline
    .on_start(|| console::log("Timeline started"))
    .on_pause(|| console::log("Timeline paused"))
    .on_complete(|| console::log("Timeline finished"))
    .on_update(|time| update_ui(time));
```

### Track Events
Individual tracks can have their own callbacks:

```rust
AnimationTrack {
    target: "#element".to_string(),
    animation: properties,
    on_start: Some(|| highlight_element()),
    on_complete: Some(|| show_next_content()),
    // ... other properties
}
```

## Testing Strategy

### Unit Tests
- Timeline creation and track management
- Timing calculations and scheduling
- Playback control (play, pause, seek)
- Event callback execution

### Integration Tests
- TimelineController component
- Multiple track coordination
- Performance with many tracks

### E2E Tests
- Complex timeline animations
- User interaction integration
- Browser compatibility

### Performance Tests
- Timeline with 100+ tracks
- Memory usage during long timelines
- Frame rate maintenance under load
