use crate::{AnimationValue, AnimationTarget};
use std::collections::HashMap;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

/// Timeline event types for callbacks and orchestration
#[derive(Clone, Debug)]
pub enum TimelineEventType {
    /// Animation starts
    Start,
    /// Animation completes
    Complete,
    /// Animation is updated (progress changed)
    Update,
    /// Animation is paused
    Pause,
    /// Animation is resumed
    Resume,
    /// Animation is stopped
    Stop,
    /// Custom user-defined event
    Custom(String),
}

/// Timeline event with timing and callback
#[derive(Clone)]
pub struct TimelineEvent {
    /// Time offset from timeline start (in seconds)
    pub time: f64,
    /// Event type
    pub event_type: TimelineEventType,
    /// Optional callback function
    pub callback: Option<Rc<dyn Fn()>>,
    /// Event ID for identification
    pub id: String,
}

impl TimelineEvent {
    /// Create a new timeline event
    pub fn new(time: f64, event_type: TimelineEventType, id: String) -> Self {
        Self {
            time,
            event_type,
            callback: None,
            id,
        }
    }

    /// Add a callback to the event
    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.callback = Some(Rc::new(callback));
        self
    }
}

/// Individual timeline track for a single animation target
#[derive(Clone)]
pub struct TimelineTrack {
    /// Target element or animation target
    pub target: AnimationTarget,
    /// Start time offset from timeline start (in seconds)
    pub start_time: f64,
    /// Duration of this track (in seconds)
    pub duration: f64,
    /// Animation properties for this track
    pub properties: HashMap<String, AnimationValue>,
    /// Track ID for identification
    pub id: String,
    /// Easing function name
    pub easing: String,
    /// Track priority (higher numbers = higher priority)
    pub priority: u32,
}

impl TimelineTrack {
    /// Create a new timeline track
    pub fn new(target: AnimationTarget, start_time: f64, duration: f64, id: String) -> Self {
        Self {
            target,
            start_time,
            duration,
            properties: HashMap::new(),
            id,
            easing: "ease-out".to_string(),
            priority: 0,
        }
    }

    /// Add an animation property to this track
    pub fn with_property(mut self, property: String, value: AnimationValue) -> Self {
        self.properties.insert(property, value);
        self
    }

    /// Set the easing function
    pub fn with_easing(mut self, easing: String) -> Self {
        self.easing = easing;
        self
    }

    /// Set the track priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }

    /// Check if this track is active at the given timeline time
    pub fn is_active_at(&self, timeline_time: f64) -> bool {
        timeline_time >= self.start_time && timeline_time <= self.start_time + self.duration
    }

    /// Get the progress of this track (0.0 to 1.0) at the given timeline time
    pub fn get_progress_at(&self, timeline_time: f64) -> f64 {
        if !self.is_active_at(timeline_time) {
            return 0.0;
        }

        let elapsed = timeline_time - self.start_time;
        (elapsed / self.duration).min(1.0)
    }
}

/// Timeline playback state
#[derive(Clone, Debug, PartialEq)]
pub enum TimelineState {
    /// Timeline is stopped (not playing)
    Stopped,
    /// Timeline is playing
    Playing,
    /// Timeline is paused
    Paused,
    /// Timeline has completed
    Completed,
}

/// Main timeline orchestration structure
#[derive(Clone)]
pub struct Timeline {
    /// Unique timeline ID
    pub id: String,
    /// Total duration of the timeline (in seconds)
    pub duration: f64,
    /// Current playback time (in seconds)
    pub current_time: f64,
    /// Playback state
    pub state: TimelineState,
    /// Animation tracks
    pub tracks: Vec<TimelineTrack>,
    /// Timeline events
    pub events: Vec<TimelineEvent>,
    /// Playback speed multiplier (1.0 = normal speed)
    pub speed: f64,
    /// Whether to loop the timeline
    pub loop_enabled: bool,
    /// Start time offset for the timeline
    pub start_delay: f64,
    /// Animation frame callback ID
    animation_frame_id: Option<i32>,
    /// Last frame timestamp
    last_frame_time: Option<f64>,
}

impl Timeline {
    /// Create a new timeline
    pub fn new(id: String, duration: f64) -> Self {
        Self {
            id,
            duration,
            current_time: 0.0,
            state: TimelineState::Stopped,
            tracks: Vec::new(),
            events: Vec::new(),
            speed: 1.0,
            loop_enabled: false,
            start_delay: 0.0,
            animation_frame_id: None,
            last_frame_time: None,
        }
    }

    /// Add a track to the timeline
    pub fn add_track(mut self, track: TimelineTrack) -> Self {
        self.tracks.push(track);
        self
    }

    /// Add an event to the timeline
    pub fn add_event(mut self, event: TimelineEvent) -> Self {
        self.events.push(event);
        self
    }

    /// Set playback speed
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.speed = speed;
        self
    }

    /// Enable/disable looping
    pub fn with_loop(mut self, enabled: bool) -> Self {
        self.loop_enabled = enabled;
        self
    }

    /// Set start delay
    pub fn with_start_delay(mut self, delay: f64) -> Self {
        self.start_delay = delay;
        self
    }

    /// Start playing the timeline
    pub fn play(&mut self) {
        if self.state == TimelineState::Playing {
            return;
        }

        self.state = TimelineState::Playing;
        self.last_frame_time = None;
        self.request_animation_frame();
    }

    /// Pause the timeline
    pub fn pause(&mut self) {
        self.state = TimelineState::Paused;
        self.cancel_animation_frame();
    }

    /// Stop the timeline and reset to beginning
    pub fn stop(&mut self) {
        self.state = TimelineState::Stopped;
        self.current_time = 0.0;
        self.cancel_animation_frame();
        self.last_frame_time = None;

        // Trigger stop events
        self.trigger_events_at_time(0.0);
    }

    /// Seek to a specific time
    pub fn seek(&mut self, time: f64) {
        let clamped_time = time.max(0.0).min(self.duration);
        self.current_time = clamped_time;

        // Trigger events at the seek time
        self.trigger_events_at_time(clamped_time);
    }

    /// Get current progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.duration <= 0.0 {
            return 1.0;
        }
        (self.current_time / self.duration).min(1.0)
    }

    /// Check if timeline is completed
    pub fn is_completed(&self) -> bool {
        self.current_time >= self.duration
    }

    /// Get all active tracks at current time
    pub fn get_active_tracks(&self) -> Vec<&TimelineTrack> {
        self.tracks.iter()
            .filter(|track| track.is_active_at(self.current_time))
            .collect()
    }

    /// Request next animation frame
    fn request_animation_frame(&mut self) {
        let timeline = Rc::new(self.clone());
        let timeline_clone = timeline.clone();

        let closure = Closure::wrap(Box::new(move |timestamp: f64| {
            let mut timeline = (*timeline_clone).clone();
            timeline.update_animation_frame(timestamp);
        }) as Box<dyn FnMut(f64)>);

        if let Some(window) = window() {
            if let Ok(id) = window.request_animation_frame(closure.as_ref().unchecked_ref()) {
                self.animation_frame_id = Some(id);
            }
        }

        closure.forget(); // Leak the closure to keep it alive
    }

    /// Cancel current animation frame
    fn cancel_animation_frame(&mut self) {
        if let Some(id) = self.animation_frame_id.take() {
            if let Some(window) = window() {
                window.cancel_animation_frame(id);
            }
        }
    }

    /// Update animation frame
    fn update_animation_frame(&mut self, timestamp: f64) {
        if self.state != TimelineState::Playing {
            return;
        }

        // Calculate delta time
        let delta_time = if let Some(last_time) = self.last_frame_time {
            (timestamp - last_time) / 1000.0 // Convert to seconds
        } else {
            0.0
        };

        self.last_frame_time = Some(timestamp);

        // Update current time
        self.current_time += delta_time * self.speed;

        // Handle timeline completion
        if self.current_time >= self.duration {
            if self.loop_enabled {
                self.current_time = self.current_time % self.duration;
                // Trigger loop events
                self.trigger_events_at_time(0.0);
            } else {
                self.current_time = self.duration;
                self.state = TimelineState::Completed;
                self.cancel_animation_frame();
                // Trigger completion events
                self.trigger_events_at_time(self.duration);
                return;
            }
        }

        // Trigger events at current time
        self.trigger_events_at_time(self.current_time);

        // Continue animation loop
        self.request_animation_frame();
    }

    /// Trigger events at a specific time
    fn trigger_events_at_time(&self, time: f64) {
        for event in &self.events {
            if (event.time - time).abs() < 0.016 { // ~1 frame tolerance
                if let Some(callback) = &event.callback {
                    callback();
                }
            }
        }
    }
}

/// Builder for creating timelines
pub struct TimelineBuilder {
    timeline: Timeline,
}

impl TimelineBuilder {
    /// Create a new timeline builder
    pub fn new(id: String, duration: f64) -> Self {
        Self {
            timeline: Timeline::new(id, duration),
        }
    }

    /// Add a track to the timeline
    pub fn add_track(mut self, track: TimelineTrack) -> Self {
        self.timeline = self.timeline.add_track(track);
        self
    }

    /// Add an event to the timeline
    pub fn add_event(mut self, event: TimelineEvent) -> Self {
        self.timeline = self.timeline.add_event(event);
        self
    }

    /// Set playback speed
    pub fn with_speed(mut self, speed: f64) -> Self {
        self.timeline.speed = speed;
        self
    }

    /// Enable/disable looping
    pub fn with_loop(mut self, enabled: bool) -> Self {
        self.timeline.loop_enabled = enabled;
        self
    }

    /// Set start delay
    pub fn with_start_delay(mut self, delay: f64) -> Self {
        self.timeline.start_delay = delay;
        self
    }

    /// Build the timeline
    pub fn build(self) -> Timeline {
        self.timeline
    }
}

/// Timeline manager for coordinating multiple timelines
pub struct TimelineManager {
    timelines: HashMap<String, Timeline>,
}

impl TimelineManager {
    /// Create a new timeline manager
    pub fn new() -> Self {
        Self {
            timelines: HashMap::new(),
        }
    }

    /// Add a timeline to the manager
    pub fn add_timeline(&mut self, timeline: Timeline) {
        self.timelines.insert(timeline.id.clone(), timeline);
    }

    /// Get a timeline by ID
    pub fn get_timeline(&self, id: &str) -> Option<&Timeline> {
        self.timelines.get(id)
    }

    /// Get a mutable timeline by ID
    pub fn get_timeline_mut(&mut self, id: &str) -> Option<&mut Timeline> {
        self.timelines.get_mut(id)
    }

    /// Remove a timeline
    pub fn remove_timeline(&mut self, id: &str) -> Option<Timeline> {
        self.timelines.remove(id)
    }

    /// Play a timeline by ID
    pub fn play_timeline(&mut self, id: &str) {
        if let Some(timeline) = self.timelines.get_mut(id) {
            timeline.play();
        }
    }

    /// Pause a timeline by ID
    pub fn pause_timeline(&mut self, id: &str) {
        if let Some(timeline) = self.timelines.get_mut(id) {
            timeline.pause();
        }
    }

    /// Stop a timeline by ID
    pub fn stop_timeline(&mut self, id: &str) {
        if let Some(timeline) = self.timelines.get_mut(id) {
            timeline.stop();
        }
    }

    /// Seek a timeline by ID
    pub fn seek_timeline(&mut self, id: &str, time: f64) {
        if let Some(timeline) = self.timelines.get_mut(id) {
            timeline.seek(time);
        }
    }

    /// Get all active timelines
    pub fn get_active_timelines(&self) -> Vec<&Timeline> {
        self.timelines.values()
            .filter(|timeline| timeline.state == TimelineState::Playing)
            .collect()
    }

    /// Update all timelines (call this in animation loop)
    pub fn update(&mut self, delta_time: f64) {
        for timeline in self.timelines.values_mut() {
            if timeline.state == TimelineState::Playing {
                timeline.current_time += delta_time * timeline.speed;

                if timeline.current_time >= timeline.duration {
                    if timeline.loop_enabled {
                        timeline.current_time = timeline.current_time % timeline.duration;
                    } else {
                        timeline.current_time = timeline.duration;
                        timeline.state = TimelineState::Completed;
                    }
                }

                timeline.trigger_events_at_time(timeline.current_time);
            }
        }
    }
}

// Timeline manager instance - create new instances as needed
pub fn create_timeline_manager() -> TimelineManager {
    TimelineManager::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline::new("test-timeline".to_string(), 5.0);
        assert_eq!(timeline.id, "test-timeline");
        assert_eq!(timeline.duration, 5.0);
        assert_eq!(timeline.current_time, 0.0);
        assert_eq!(timeline.state, TimelineState::Stopped);
    }

    #[test]
    fn test_timeline_track() {
        let target = AnimationTarget {
            property: "opacity".to_string(),
            from_value: 0.0,
            to_value: 1.0,
            current_value: 0.0,
            duration: 1.0,
            start_time: 0.0,
            easing: "linear".to_string(),
        };

        let track = TimelineTrack::new(target, 1.0, 2.0, "track1".to_string())
            .with_property("opacity".to_string(), AnimationValue::Number(1.0))
            .with_easing("ease-in".to_string())
            .with_priority(1);

        assert_eq!(track.start_time, 1.0);
        assert_eq!(track.duration, 2.0);
        assert_eq!(track.easing, "ease-in");
        assert_eq!(track.priority, 1);
        assert!(track.is_active_at(1.5));
        assert!(!track.is_active_at(4.0));
        assert_eq!(track.get_progress_at(1.5), 0.25);
    }

    #[test]
    fn test_timeline_event() {
        let event = TimelineEvent::new(2.0, TimelineEventType::Start, "event1".to_string());

        assert_eq!(event.time, 2.0);
        assert_eq!(event.id, "event1");
        assert!(matches!(event.event_type, TimelineEventType::Start));
    }

    #[test]
    fn test_timeline_progress() {
        let mut timeline = Timeline::new("test".to_string(), 10.0);
        timeline.current_time = 5.0;

        assert_eq!(timeline.progress(), 0.5);

        timeline.current_time = 15.0;
        assert_eq!(timeline.progress(), 1.0); // clamped
    }

    #[test]
    fn test_timeline_builder() {
        let timeline = TimelineBuilder::new("builder-test".to_string(), 8.0)
            .with_speed(2.0)
            .with_loop(true)
            .with_start_delay(1.0)
            .build();

        assert_eq!(timeline.speed, 2.0);
        assert!(timeline.loop_enabled);
        assert_eq!(timeline.start_delay, 1.0);
        assert_eq!(timeline.duration, 8.0);
    }
}
