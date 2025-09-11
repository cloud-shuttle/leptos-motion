//! Advanced timeline editor with keyframe-based animations

use crate::{Result, StudioError, transforms::*};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

/// Timeline keyframe containing animation data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineKeyframe {
    /// Unique keyframe ID
    pub id: Uuid,
    /// Time position in seconds
    pub time: f32,
    /// Property being animated
    pub property: AnimationProperty,
    /// Value at this keyframe
    pub value: AnimationValue,
    /// Easing function to next keyframe
    pub easing: EasingFunction,
    /// Handle type for bezier curves
    pub handle_type: HandleType,
    /// Bezier control points (for custom easing)
    pub control_points: Option<(Vec2, Vec2)>,
}

impl TimelineKeyframe {
    /// Create new keyframe
    pub fn new(time: f32, property: AnimationProperty, value: AnimationValue) -> Self {
        Self {
            id: Uuid::new_v4(),
            time,
            property,
            value,
            easing: EasingFunction::Linear,
            handle_type: HandleType::Auto,
            control_points: None,
        }
    }

    /// Get keyframe time
    pub fn time(&self) -> f32 {
        self.time
    }

    /// Set keyframe time
    pub fn set_time(&mut self, time: f32) {
        self.time = time.max(0.0);
    }

    /// Get keyframe value
    pub fn value(&self) -> &AnimationValue {
        &self.value
    }

    /// Set keyframe value
    pub fn set_value(&mut self, value: AnimationValue) {
        self.value = value;
    }

    /// Interpolate value to another keyframe
    pub fn interpolate_to(&self, other: &Self, t: f32) -> Result<AnimationValue> {
        if self.property != other.property {
            return Err(StudioError::TimelineError(
                "Cannot interpolate between different properties".to_string(),
            ));
        }

        let eased_t = self.easing.apply(t);
        self.value.lerp(&other.value, eased_t)
    }
}

/// Animation property types that can be keyframed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum AnimationProperty {
    /// Transform properties
    TranslateX,
    TranslateY,
    TranslateZ,
    RotateX,
    RotateY,
    RotateZ,
    ScaleX,
    ScaleY,
    ScaleZ,

    /// Visual properties
    Opacity,
    BackgroundColor,
    BorderColor,
    BorderRadius,

    /// Layout properties
    Width,
    Height,
    Top,
    Left,

    /// Custom properties
    Custom(String),
}

impl AnimationProperty {
    /// Get property display name
    pub fn display_name(&self) -> &str {
        match self {
            Self::TranslateX => "Translate X",
            Self::TranslateY => "Translate Y",
            Self::TranslateZ => "Translate Z",
            Self::RotateX => "Rotate X",
            Self::RotateY => "Rotate Y",
            Self::RotateZ => "Rotate Z",
            Self::ScaleX => "Scale X",
            Self::ScaleY => "Scale Y",
            Self::ScaleZ => "Scale Z",
            Self::Opacity => "Opacity",
            Self::BackgroundColor => "Background Color",
            Self::BorderColor => "Border Color",
            Self::BorderRadius => "Border Radius",
            Self::Width => "Width",
            Self::Height => "Height",
            Self::Top => "Top",
            Self::Left => "Left",
            Self::Custom(name) => name,
        }
    }

    /// Get default value for property
    pub fn default_value(&self) -> AnimationValue {
        match self {
            Self::TranslateX | Self::TranslateY | Self::TranslateZ => AnimationValue::Number(0.0),
            Self::RotateX | Self::RotateY | Self::RotateZ => AnimationValue::Number(0.0),
            Self::ScaleX | Self::ScaleY | Self::ScaleZ => AnimationValue::Number(1.0),
            Self::Opacity => AnimationValue::Number(1.0),
            Self::BackgroundColor | Self::BorderColor => {
                AnimationValue::Color([255, 255, 255, 255])
            }
            Self::BorderRadius | Self::Width | Self::Height | Self::Top | Self::Left => {
                AnimationValue::Number(0.0)
            }
            Self::Custom(_) => AnimationValue::Number(0.0),
        }
    }
}

/// Animation value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationValue {
    Number(f32),
    Color([u8; 4]),
    Vec2(Vec2),
    Vec3(glam::Vec3),
    Transform(Transform3D),
    String(String),
    Boolean(bool),
}

impl AnimationValue {
    /// Linear interpolation between values
    pub fn lerp(&self, other: &Self, t: f32) -> Result<Self> {
        let t = t.clamp(0.0, 1.0);

        match (self, other) {
            (Self::Number(a), Self::Number(b)) => Ok(Self::Number(a + (b - a) * t)),
            (Self::Color(a), Self::Color(b)) => {
                let r = (a[0] as f32 + (b[0] as f32 - a[0] as f32) * t) as u8;
                let g = (a[1] as f32 + (b[1] as f32 - a[1] as f32) * t) as u8;
                let b_val = (a[2] as f32 + (b[2] as f32 - a[2] as f32) * t) as u8;
                let a_val = (a[3] as f32 + (b[3] as f32 - a[3] as f32) * t) as u8;
                Ok(Self::Color([r, g, b_val, a_val]))
            }
            (Self::Vec2(a), Self::Vec2(b)) => Ok(Self::Vec2(Vec2::new(
                a.x + (b.x - a.x) * t,
                a.y + (b.y - a.y) * t,
            ))),
            (Self::Vec3(a), Self::Vec3(b)) => Ok(Self::Vec3(a.lerp(*b, t))),
            (Self::Transform(a), Self::Transform(b)) => Ok(Self::Transform(a.lerp(b, t))),
            (Self::Boolean(a), Self::Boolean(_)) => Ok(Self::Boolean(*a)),
            _ => Err(StudioError::TimelineError(
                "Cannot interpolate between incompatible value types".to_string(),
            )),
        }
    }

    /// Convert to CSS value string
    pub fn to_css(&self, property: &AnimationProperty) -> String {
        match (self, property) {
            (
                Self::Number(n),
                AnimationProperty::TranslateX
                | AnimationProperty::TranslateY
                | AnimationProperty::TranslateZ,
            ) => {
                format!("{}px", n)
            }
            (
                Self::Number(n),
                AnimationProperty::RotateX
                | AnimationProperty::RotateY
                | AnimationProperty::RotateZ,
            ) => {
                format!("{}rad", n)
            }
            (
                Self::Number(n),
                AnimationProperty::ScaleX | AnimationProperty::ScaleY | AnimationProperty::ScaleZ,
            ) => n.to_string(),
            (Self::Number(n), AnimationProperty::Opacity) => n.clamp(0.0, 1.0).to_string(),
            (Self::Color(rgba), _) => {
                format!(
                    "rgba({}, {}, {}, {})",
                    rgba[0],
                    rgba[1],
                    rgba[2],
                    rgba[3] as f32 / 255.0
                )
            }
            (Self::Number(n), _) => {
                format!("{}px", n)
            }
            (Self::String(s), _) => s.clone(),
            (Self::Boolean(b), _) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            _ => "0".to_string(),
        }
    }
}

/// Keyframe handle types for custom easing curves
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HandleType {
    Auto,
    Linear,
    Bezier,
    Step,
}

/// 2D vector for control points
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

/// Animation track containing keyframes for a single property
#[derive(Debug, Clone)]
pub struct AnimationTrack {
    /// Track ID
    pub id: Uuid,
    /// Property being animated
    pub property: AnimationProperty,
    /// Track name/label
    pub name: String,
    /// Keyframes sorted by time
    pub keyframes: BTreeMap<u32, TimelineKeyframe>, // Use u32 key for time ordering
    /// Track enabled/disabled
    pub enabled: bool,
    /// Track color for UI
    pub color: [u8; 3],
}

impl AnimationTrack {
    /// Create new animation track
    pub fn new(property: AnimationProperty, name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            property,
            name,
            keyframes: BTreeMap::new(),
            enabled: true,
            color: [100, 150, 255],
        }
    }

    /// Add keyframe to track
    pub fn add_keyframe(&mut self, keyframe: TimelineKeyframe) {
        let time_key = (keyframe.time * 1000.0) as u32; // Convert to milliseconds for ordering
        self.keyframes.insert(time_key, keyframe);
    }

    /// Remove keyframe from track
    pub fn remove_keyframe(&mut self, keyframe_id: Uuid) -> bool {
        self.keyframes.retain(|_, kf| kf.id != keyframe_id);
        true
    }

    /// Get keyframe at specific time
    pub fn get_keyframe_at(&self, time: f32) -> Option<&TimelineKeyframe> {
        let time_key = (time * 1000.0) as u32;
        self.keyframes.get(&time_key)
    }

    /// Get value at specific time (with interpolation)
    pub fn value_at(&self, time: f32) -> Result<AnimationValue> {
        if self.keyframes.is_empty() {
            return Ok(self.property.default_value());
        }

        let time_key = (time * 1000.0) as u32;

        // Find surrounding keyframes
        let mut before = None;
        let mut after = None;

        for (&key, keyframe) in &self.keyframes {
            if key <= time_key {
                before = Some(keyframe);
            }
            if key >= time_key && after.is_none() {
                after = Some(keyframe);
                break;
            }
        }

        match (before, after) {
            (Some(kf), None) => Ok(kf.value.clone()),
            (None, Some(kf)) => Ok(kf.value.clone()),
            (Some(before_kf), Some(after_kf)) => {
                if before_kf.id == after_kf.id {
                    Ok(before_kf.value.clone())
                } else {
                    let t = (time - before_kf.time) / (after_kf.time - before_kf.time);
                    before_kf.interpolate_to(after_kf, t).map(|v| v)
                }
            }
            (None, None) => Ok(self.property.default_value()),
        }
    }

    /// Get all keyframes as vector
    pub fn keyframes(&self) -> Vec<&TimelineKeyframe> {
        self.keyframes.values().collect()
    }

    /// Get keyframe count
    pub fn keyframe_count(&self) -> usize {
        self.keyframes.len()
    }
}

/// 3D Timeline for advanced keyframe animation
#[derive(Debug, Clone)]
pub struct Timeline3D {
    /// Timeline ID
    pub id: Uuid,
    /// Timeline name
    pub name: String,
    /// Duration in seconds
    pub duration: f32,
    /// Current playhead position
    pub current_time: f32,
    /// Animation tracks by property
    pub tracks: HashMap<AnimationProperty, AnimationTrack>,
    /// Timeline settings
    pub settings: TimelineSettings,
    /// Playback state
    pub is_playing: bool,
    /// Loop enabled
    pub loop_enabled: bool,
}

impl Timeline3D {
    /// Create new timeline
    pub fn new(name: String, duration: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            duration: duration.max(0.1),
            current_time: 0.0,
            tracks: HashMap::new(),
            settings: TimelineSettings::default(),
            is_playing: false,
            loop_enabled: false,
        }
    }

    /// Add animation track
    pub fn add_track(&mut self, property: AnimationProperty) -> &mut AnimationTrack {
        let track_name = property.display_name().to_string();
        let track = AnimationTrack::new(property.clone(), track_name);
        self.tracks.insert(property.clone(), track);
        self.tracks.get_mut(&property).unwrap()
    }

    /// Remove animation track
    pub fn remove_track(&mut self, property: &AnimationProperty) -> bool {
        self.tracks.remove(property).is_some()
    }

    /// Get animation track
    pub fn get_track(&self, property: &AnimationProperty) -> Option<&AnimationTrack> {
        self.tracks.get(property)
    }

    /// Get mutable animation track
    pub fn get_track_mut(&mut self, property: &AnimationProperty) -> Option<&mut AnimationTrack> {
        self.tracks.get_mut(property)
    }

    /// Add keyframe to timeline
    pub fn add_keyframe(
        &mut self,
        property: AnimationProperty,
        time: f32,
        value: AnimationValue,
    ) -> Result<Uuid> {
        if time < 0.0 || time > self.duration {
            return Err(StudioError::TimelineError(format!(
                "Time {} is outside timeline duration {}",
                time, self.duration
            )));
        }

        let keyframe = TimelineKeyframe::new(time, property.clone(), value);
        let keyframe_id = keyframe.id;

        // Create track if it doesn't exist
        if !self.tracks.contains_key(&property) {
            self.add_track(property.clone());
        }

        if let Some(track) = self.tracks.get_mut(&property) {
            track.add_keyframe(keyframe);
        }

        Ok(keyframe_id)
    }

    /// Remove keyframe from timeline
    pub fn remove_keyframe(&mut self, property: &AnimationProperty, keyframe_id: Uuid) -> bool {
        if let Some(track) = self.tracks.get_mut(property) {
            track.remove_keyframe(keyframe_id)
        } else {
            false
        }
    }

    /// Get current state at playhead time
    pub fn current_state(&self) -> Result<HashMap<AnimationProperty, AnimationValue>> {
        let mut state = HashMap::new();

        for (property, track) in &self.tracks {
            if track.enabled {
                let value = track.value_at(self.current_time)?;
                state.insert(property.clone(), value);
            }
        }

        Ok(state)
    }

    /// Seek to specific time
    pub fn seek(&mut self, time: f32) {
        self.current_time = time.clamp(0.0, self.duration);
    }

    /// Play timeline
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause timeline
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Update timeline (call every frame)
    pub fn update(
        &mut self,
        delta_time: f32,
    ) -> Result<HashMap<AnimationProperty, AnimationValue>> {
        if self.is_playing {
            self.current_time += delta_time;

            if self.current_time >= self.duration {
                if self.loop_enabled {
                    self.current_time = 0.0;
                } else {
                    self.current_time = self.duration;
                    self.is_playing = false;
                }
            }
        }

        self.current_state()
    }

    /// Get timeline duration
    pub fn duration(&self) -> f32 {
        self.duration
    }

    /// Set timeline duration
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration.max(0.1);
        // Clamp current time to new duration
        self.current_time = self.current_time.min(self.duration);
    }

    /// Get all keyframes in timeline
    pub fn keyframes(&self) -> Vec<&TimelineKeyframe> {
        let mut keyframes = Vec::new();
        for track in self.tracks.values() {
            keyframes.extend(track.keyframes());
        }
        keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        keyframes
    }
}

/// Timeline settings and preferences
#[derive(Debug, Clone)]
pub struct TimelineSettings {
    /// Timeline zoom level
    pub zoom: f32,
    /// Snap to grid
    pub snap_enabled: bool,
    /// Grid size in seconds
    pub grid_size: f32,
    /// Show frame numbers
    pub show_frame_numbers: bool,
    /// Framerate for frame display
    pub framerate: f32,
}

impl Default for TimelineSettings {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            snap_enabled: true,
            grid_size: 0.1,
            show_frame_numbers: false,
            framerate: 60.0,
        }
    }
}

/// Timeline Editor Component
#[component]
pub fn TimelineEditor(
    /// Timeline to edit
    #[prop(optional)]
    timeline: Option<Timeline3D>,

    /// Callback when timeline changes
    #[prop(optional)]
    on_timeline_change: Option<Callback<Timeline3D>>,

    /// Editor height in pixels
    #[prop(default = 300)]
    height: u32,
) -> impl IntoView {
    let (current_timeline, set_timeline) =
        create_signal(timeline.unwrap_or_else(|| Timeline3D::new("Timeline".to_string(), 10.0)));

    let (selected_keyframes, set_selected_keyframes) = create_signal(Vec::<Uuid>::new());
    let (dragging, set_dragging) = create_signal(false);

    // Handle timeline changes
    let handle_timeline_change = move |new_timeline: Timeline3D| {
        set_timeline.set(new_timeline.clone());
        if let Some(callback) = on_timeline_change {
            callback.call(new_timeline);
        }
    };

    view! {
        <div class="timeline-editor" style:height=format!("{}px", height)>
            <TimelineHeader timeline=current_timeline />
            <div class="timeline-editor__content">
                <TimelineRuler timeline=current_timeline />
                <TimelineTracks timeline=current_timeline />
                <TimelinePlayhead timeline=current_timeline />
            </div>
            <TimelineControls timeline=current_timeline />
        </div>
    }
}

/// Keyframe Editor Component
#[component]
pub fn KeyframeEditor(
    /// Keyframe to edit
    keyframe: TimelineKeyframe,

    /// Callback when keyframe changes
    #[prop(optional)]
    on_change: Option<Callback<TimelineKeyframe>>,
) -> impl IntoView {
    let (current_keyframe, set_keyframe) = create_signal(keyframe);

    view! {
        <div class="keyframe-editor">
            <div class="keyframe-editor__section">
                <label>"Time"</label>
                <input
                    type="number"
                    step="0.01"
                    min="0"
                    prop:value=move || current_keyframe.get().time
                    on:input=move |ev| {
                        let value: f32 = event_target_value(&ev).parse().unwrap_or(0.0);
                        let mut kf = current_keyframe.get();
                        kf.set_time(value);
                        set_keyframe.set(kf.clone());
                        if let Some(callback) = on_change {
                            callback.call(kf);
                        }
                    }
                />
            </div>

            <div class="keyframe-editor__section">
                <label>"Value"</label>
                <KeyframeValueEditor keyframe=current_keyframe />
            </div>

            <div class="keyframe-editor__section">
                <label>"Easing"</label>
                <EasingSelector keyframe=current_keyframe />
            </div>
        </div>
    }
}

/// Animation Timeline Component (simpler version)
#[component]
pub fn AnimationTimeline() -> impl IntoView {
    let (timeline, set_timeline) = create_signal(Timeline3D::new("Animation".to_string(), 5.0));

    view! {
        <div class="animation-timeline">
            <TimelineEditor
                timeline=Some(timeline.get())
                on_timeline_change=move |t| set_timeline.set(t)
            />
        </div>
    }
}

// Sub-components (simplified placeholders)
#[component]
fn TimelineHeader(timeline: ReadSignal<Timeline3D>) -> impl IntoView {
    view! {
        <div class="timeline-header">
            <h3>{move || timeline.get().name}</h3>
            <span class="timeline-duration">{move || format!("{:.1}s", timeline.get().duration)}</span>
        </div>
    }
}

#[component]
fn TimelineRuler(timeline: ReadSignal<Timeline3D>) -> impl IntoView {
    view! {
        <div class="timeline-ruler">
            <div class="timeline-ruler__ticks">
                // Timeline ruler ticks would be generated here
            </div>
        </div>
    }
}

#[component]
fn TimelineTracks(timeline: ReadSignal<Timeline3D>) -> impl IntoView {
    view! {
        <div class="timeline-tracks">
            // Track visualization would be implemented here
            <div class="timeline-track">
                <div class="timeline-track__header">"Transform"</div>
                <div class="timeline-track__content">
                    // Keyframes would be rendered here
                </div>
            </div>
        </div>
    }
}

#[component]
fn TimelinePlayhead(timeline: ReadSignal<Timeline3D>) -> impl IntoView {
    view! {
        <div
            class="timeline-playhead"
            style:left=move || {
                let t = timeline.get();
                let percent = (t.current_time / t.duration) * 100.0;
                format!("{}%", percent)
            }
        ></div>
    }
}

#[component]
fn TimelineControls(timeline: ReadSignal<Timeline3D>) -> impl IntoView {
    view! {
        <div class="timeline-controls">
            <button class="timeline-control-btn">"⏵"</button>
            <button class="timeline-control-btn">"⏸"</button>
            <button class="timeline-control-btn">"⏹"</button>
            <button class="timeline-control-btn">"🔄"</button>
        </div>
    }
}

#[component]
fn KeyframeValueEditor(keyframe: ReadSignal<TimelineKeyframe>) -> impl IntoView {
    view! {
        <div class="keyframe-value-editor">
            // Value editing controls based on value type
            <input type="text" placeholder="Value" />
        </div>
    }
}

#[component]
fn EasingSelector(keyframe: ReadSignal<TimelineKeyframe>) -> impl IntoView {
    view! {
        <select class="easing-selector">
            <option value="linear">"Linear"</option>
            <option value="ease-in">"Ease In"</option>
            <option value="ease-out">"Ease Out"</option>
            <option value="ease-in-out">"Ease In Out"</option>
        </select>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_timeline_keyframe_creation() {
        let keyframe = TimelineKeyframe::new(
            1.0,
            AnimationProperty::TranslateX,
            AnimationValue::Number(100.0),
        );

        assert_eq!(keyframe.time, 1.0);
        assert_eq!(keyframe.property, AnimationProperty::TranslateX);
        assert_eq!(keyframe.value, AnimationValue::Number(100.0));
    }

    #[test]
    fn test_animation_value_lerp() {
        let value1 = AnimationValue::Number(0.0);
        let value2 = AnimationValue::Number(100.0);

        let interpolated = value1.lerp(&value2, 0.5).unwrap();
        if let AnimationValue::Number(n) = interpolated {
            assert_relative_eq!(n, 50.0, epsilon = 1e-6);
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_color_interpolation() {
        let color1 = AnimationValue::Color([255, 0, 0, 255]);
        let color2 = AnimationValue::Color([0, 255, 0, 255]);

        let interpolated = color1.lerp(&color2, 0.5).unwrap();
        if let AnimationValue::Color(rgba) = interpolated {
            assert_eq!(rgba[0], 127); // Midpoint between 255 and 0
            assert_eq!(rgba[1], 127); // Midpoint between 0 and 255
            assert_eq!(rgba[2], 0);
            assert_eq!(rgba[3], 255);
        } else {
            panic!("Expected Color value");
        }
    }

    #[test]
    fn test_timeline_creation() {
        let timeline = Timeline3D::new("Test Timeline".to_string(), 10.0);
        assert_eq!(timeline.name, "Test Timeline");
        assert_eq!(timeline.duration, 10.0);
        assert_eq!(timeline.current_time, 0.0);
        assert!(!timeline.is_playing);
        assert!(!timeline.loop_enabled);
    }

    #[test]
    fn test_timeline_add_keyframe() {
        let mut timeline = Timeline3D::new("Test".to_string(), 10.0);

        let keyframe_id = timeline
            .add_keyframe(
                AnimationProperty::TranslateX,
                2.0,
                AnimationValue::Number(50.0),
            )
            .unwrap();

        assert!(!keyframe_id.is_nil());
        assert!(timeline.tracks.contains_key(&AnimationProperty::TranslateX));

        let track = timeline.tracks.get(&AnimationProperty::TranslateX).unwrap();
        assert_eq!(track.keyframe_count(), 1);
    }

    #[test]
    fn test_animation_track() {
        let mut track =
            AnimationTrack::new(AnimationProperty::Opacity, "Opacity Track".to_string());

        let keyframe1 =
            TimelineKeyframe::new(0.0, AnimationProperty::Opacity, AnimationValue::Number(0.0));
        let keyframe2 =
            TimelineKeyframe::new(2.0, AnimationProperty::Opacity, AnimationValue::Number(1.0));

        track.add_keyframe(keyframe1);
        track.add_keyframe(keyframe2);

        assert_eq!(track.keyframe_count(), 2);

        // Test interpolation
        let value_at_1s = track.value_at(1.0).unwrap();
        if let AnimationValue::Number(n) = value_at_1s {
            assert_relative_eq!(n, 0.5, epsilon = 1e-6);
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_timeline_playback() {
        let mut timeline = Timeline3D::new("Test".to_string(), 5.0);

        // Add keyframes
        let _ = timeline.add_keyframe(
            AnimationProperty::TranslateX,
            0.0,
            AnimationValue::Number(0.0),
        );
        let _ = timeline.add_keyframe(
            AnimationProperty::TranslateX,
            5.0,
            AnimationValue::Number(100.0),
        );

        // Test playback
        timeline.play();
        assert!(timeline.is_playing);

        // Update timeline
        let state = timeline.update(2.5).unwrap();
        assert_relative_eq!(timeline.current_time, 2.5, epsilon = 1e-6);

        if let Some(AnimationValue::Number(n)) = state.get(&AnimationProperty::TranslateX) {
            assert_relative_eq!(*n, 50.0, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_timeline_looping() {
        let mut timeline = Timeline3D::new("Test".to_string(), 2.0);
        timeline.loop_enabled = true;
        timeline.play();

        // Update past duration
        let _ = timeline.update(3.0);

        // Should loop back to beginning
        assert_relative_eq!(timeline.current_time, 1.0, epsilon = 1e-6); // 3.0 - 2.0
        assert!(timeline.is_playing);
    }

    #[test]
    fn test_css_value_conversion() {
        let number_value = AnimationValue::Number(50.0);
        let translate_css = number_value.to_css(&AnimationProperty::TranslateX);
        assert_eq!(translate_css, "50px");

        let rotate_css = number_value.to_css(&AnimationProperty::RotateZ);
        assert_eq!(rotate_css, "50rad");

        let color_value = AnimationValue::Color([255, 128, 64, 255]);
        let color_css = color_value.to_css(&AnimationProperty::BackgroundColor);
        assert_eq!(color_css, "rgba(255, 128, 64, 1)");
    }
}
