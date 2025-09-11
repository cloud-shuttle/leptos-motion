//! Advanced timeline editor with keyframe-based animations

use crate::{Result, StudioError, transforms::*};
use leptos::*;
use leptos::prelude::{ElementChild, PropAttribute, NodeRefAttribute, StyleAttribute, OnAttribute, create_signal, ReadSignal, WriteSignal, Callback, event_target_value, Get, Set};
use leptos::attr::global::ClassAttribute;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;
use glam::{Vec2, Vec3, Quat};

/// Animation property types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum AnimationProperty {
    Translation,
    Rotation,
    Scale,
    Opacity,
    Color,
    Custom(String),
}

/// Animation value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimationValue {
    Vec3(Vec3),
    Quat(Quat),
    Float(f32),
    String(String),
}

/// Easing function types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bezier(f32, f32, f32, f32),
}

/// Handle type for bezier curves
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleType {
    Auto,
    Aligned,
    Free,
}

/// Animation track containing keyframes for a property
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationTrack {
    pub property: AnimationProperty,
    pub keyframes: Vec<TimelineKeyframe>,
}

/// Timeline settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineSettings {
    pub frame_rate: f32,
    pub auto_key: bool,
    pub snap_to_grid: bool,
    pub grid_size: f32,
}

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

impl Default for AnimationTrack {
    fn default() -> Self {
        Self {
            property: AnimationProperty::Translation,
            keyframes: Vec::new(),
        }
    }
}

impl AnimationTrack {
    /// Add keyframe to track
    pub fn add_keyframe(&mut self, keyframe: TimelineKeyframe) {
        self.keyframes.push(keyframe);
    }

    /// Remove keyframe from track
    pub fn remove_keyframe(&mut self, keyframe_id: Uuid) -> bool {
        if let Some(pos) = self.keyframes.iter().position(|kf| kf.id == keyframe_id) {
            self.keyframes.remove(pos);
            true
        } else {
            false
        }
    }

    /// Get value at specific time (with interpolation)
    pub fn value_at(&self, time: f32) -> Result<AnimationValue> {
        if self.keyframes.is_empty() {
            return Ok(self.property.default_value());
        }

        // Find surrounding keyframes
        let mut before = None;
        let mut after = None;

        for keyframe in &self.keyframes {
            if keyframe.time <= time {
                before = Some(keyframe);
            }
            if keyframe.time >= time && after.is_none() {
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
                    before_kf.interpolate_to(after_kf, t)
                }
            }
            (None, None) => Ok(self.property.default_value()),
        }
    }

    /// Get all keyframes as vector
    pub fn keyframes(&self) -> Vec<&TimelineKeyframe> {
        self.keyframes.iter().collect()
    }
}

impl Default for TimelineSettings {
    fn default() -> Self {
        Self {
            frame_rate: 60.0,
            auto_key: false,
            snap_to_grid: true,
            grid_size: 1.0,
        }
    }
}


impl AnimationProperty {
    /// Get property display name
    pub fn display_name(&self) -> &str {
        match self {
            Self::Translation => "Translation",
            Self::Rotation => "Rotation",
            Self::Scale => "Scale",
            Self::Opacity => "Opacity",
            Self::Color => "Color",
            Self::Custom(name) => name,
        }
    }

    /// Get default value for property
    pub fn default_value(&self) -> AnimationValue {
        match self {
            Self::Translation => AnimationValue::Vec3(Vec3::ZERO),
            Self::Rotation => AnimationValue::Quat(Quat::IDENTITY),
            Self::Scale => AnimationValue::Vec3(Vec3::ONE),
            Self::Opacity => AnimationValue::Float(1.0),
            Self::Color => AnimationValue::String("black".to_string()),
            Self::Custom(_) => AnimationValue::Float(0.0),
        }
    }
}


impl EasingFunction {
    /// Apply easing function to time value
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - 2.0 * (1.0 - t) * (1.0 - t)
                }
            }
            Self::Bezier(x1, y1, x2, y2) => {
                // Simple cubic bezier approximation
                let t2 = t * t;
                let t3 = t2 * t;
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let mt3 = mt2 * mt;
                
                3.0 * mt2 * t * y1 + 3.0 * mt * t2 * y2 + t3
            }
        }
    }
}

impl AnimationValue {
    /// Linear interpolation between values
    pub fn lerp(&self, other: &Self, t: f32) -> Result<Self> {
        let t = t.clamp(0.0, 1.0);

        match (self, other) {
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + (b - a) * t)),
            (Self::Vec3(a), Self::Vec3(b)) => Ok(Self::Vec3(a.lerp(*b, t))),
            (Self::Quat(a), Self::Quat(b)) => Ok(Self::Quat(a.slerp(*b, t))),
            (Self::String(a), Self::String(_)) => Ok(Self::String(a.clone())),
            _ => Err(StudioError::TimelineError(
                "Cannot interpolate between incompatible value types".to_string(),
            )),
        }
    }

    /// Convert to CSS value string
    pub fn to_css(&self, property: &AnimationProperty) -> String {
        match (self, property) {
            (Self::Float(n), AnimationProperty::Translation) => {
                format!("{}px", n)
            }
            (Self::Float(n), AnimationProperty::Rotation) => {
                format!("{}rad", n)
            }
            (Self::Float(n), AnimationProperty::Scale) => n.to_string(),
            (Self::Float(n), AnimationProperty::Opacity) => n.clamp(0.0, 1.0).to_string(),
            (Self::Vec3(v), AnimationProperty::Translation) => {
                format!("{}px {}px {}px", v.x, v.y, v.z)
            }
            (Self::Vec3(v), AnimationProperty::Scale) => {
                format!("{} {} {}", v.x, v.y, v.z)
            }
            (Self::String(s), _) => s.clone(),
            _ => "0".to_string(),
        }
    }
}





/// 3D Timeline for advanced keyframe animation
#[derive(Debug, Clone, Serialize, Deserialize)]
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
        let track = AnimationTrack::default();
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
            let value = track.value_at(self.current_time)?;
            state.insert(property.clone(), value);
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
        // Temporarily disabled until callback API is clarified
        // if let Some(callback) = on_timeline_change {
        //     callback(new_timeline);
        // }
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
                        // Temporarily disabled until callback API is clarified
                        // if let Some(callback) = on_change {
                        //     callback(kf);
                        // }
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
                timeline=timeline.get()
                on_timeline_change=Callback::new(move |t| set_timeline.set(t))
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
            AnimationProperty::Translation,
            AnimationValue::Float(100.0),
        );

        assert_eq!(keyframe.time, 1.0);
        assert_eq!(keyframe.property, AnimationProperty::Translation);
        assert_eq!(keyframe.value, AnimationValue::Float(100.0));
    }

    #[test]
    fn test_animation_value_lerp() {
        let value1 = AnimationValue::Float(0.0);
        let value2 = AnimationValue::Float(100.0);

        let interpolated = value1.lerp(&value2, 0.5).unwrap();
        if let AnimationValue::Float(n) = interpolated {
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
                AnimationProperty::Translation,
                2.0,
                AnimationValue::Float(50.0),
            )
            .unwrap();

        assert!(!keyframe_id.is_nil());
        assert!(timeline.tracks.contains_key(&AnimationProperty::Translation));

        let track = timeline.tracks.get(&AnimationProperty::Translation).unwrap();
        assert_eq!(track.keyframe_count(), 1);
    }

    #[test]
    fn test_animation_track() {
        let mut track =
            AnimationTrack::new(AnimationProperty::Opacity, "Opacity Track".to_string());

        let keyframe1 =
            TimelineKeyframe::new(0.0, AnimationProperty::Opacity, AnimationValue::Float(0.0));
        let keyframe2 =
            TimelineKeyframe::new(2.0, AnimationProperty::Opacity, AnimationValue::Float(1.0));

        track.add_keyframe(keyframe1);
        track.add_keyframe(keyframe2);

        assert_eq!(track.keyframe_count(), 2);

        // Test interpolation
        let value_at_1s = track.value_at(1.0).unwrap();
        if let AnimationValue::Float(n) = value_at_1s {
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
            AnimationProperty::Translation,
            0.0,
            AnimationValue::Float(0.0),
        );
        let _ = timeline.add_keyframe(
            AnimationProperty::Translation,
            5.0,
            AnimationValue::Float(100.0),
        );

        // Test playback
        timeline.play();
        assert!(timeline.is_playing);

        // Update timeline
        let state = timeline.update(2.5).unwrap();
        assert_relative_eq!(timeline.current_time, 2.5, epsilon = 1e-6);

        if let Some(AnimationValue::Float(n)) = state.get(&AnimationProperty::Translation) {
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
        let number_value = AnimationValue::Float(50.0);
        let translate_css = number_value.to_css(&AnimationProperty::Translation);
        assert_eq!(translate_css, "50px");

        let rotate_css = number_value.to_css(&AnimationProperty::Rotation);
        assert_eq!(rotate_css, "50rad");

        let color_value = AnimationValue::String("#ff8040".to_string());
        let color_css = color_value.to_css(&AnimationProperty::Color);
        assert_eq!(color_css, "rgba(255, 128, 64, 1)");
    }
}
