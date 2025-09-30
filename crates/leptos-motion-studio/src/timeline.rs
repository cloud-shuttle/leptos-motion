//! Timeline and keyframe management for animations

use std::collections::HashMap;

/// Animation property types that can be animated
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum AnimationProperty {
    /// Translation along X axis
    TranslateX,
    /// Translation along Y axis
    TranslateY,
    /// Translation along Z axis
    TranslateZ,
    /// Translation in 3D space
    Translation,
    /// Rotation around X axis
    RotationX,
    /// Rotation around Y axis
    RotationY,
    /// Rotation around Z axis
    RotationZ,
    /// Rotation in 3D space
    Rotation,
    /// Scale along X axis
    ScaleX,
    /// Scale along Y axis
    ScaleY,
    /// Scale along Z axis
    ScaleZ,
    /// Scale in 3D space
    Scale,
    /// Opacity
    Opacity,
    /// Color
    Color,
    /// Custom property
    Custom(String),
}

/// Animation value types
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AnimationValue {
    /// Numeric value
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Bool(bool),
    /// Color value (RGBA)
    Color([u8; 4]),
}

impl AnimationValue {
    /// Convert animation value to CSS string
    pub fn to_css(&self, property: &AnimationProperty) -> String {
        match (self, property) {
            (AnimationValue::Number(n), AnimationProperty::TranslateX) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::TranslateY) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::TranslateZ) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::ScaleX) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::ScaleY) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::ScaleZ) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::RotationX) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::RotationY) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::RotationZ) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::Opacity) => n.to_string(),
            (AnimationValue::String(s), AnimationProperty::Color) => s.clone(),
            (AnimationValue::String(s), AnimationProperty::Custom(_)) => s.clone(),
            (AnimationValue::Color([r, g, b, a]), AnimationProperty::Color) => {
                format!("rgba({}, {}, {}, {})", r, g, b, *a as f64 / 255.0)
            }
            (AnimationValue::Number(n), _) => n.to_string(),
            (AnimationValue::String(s), _) => s.clone(),
            (AnimationValue::Bool(b), _) => b.to_string(),
            (AnimationValue::Color([r, g, b, a]), _) => {
                format!("rgba({}, {}, {}, {})", r, g, b, *a as f64 / 255.0)
            }
        }
    }
}

/// A keyframe in the animation timeline
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Keyframe {
    /// The property being animated
    pub property: AnimationProperty,
    /// Time position in the timeline
    pub time: f64,
    /// The value at this keyframe
    pub value: AnimationValue,
}

/// 3D animation timeline
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Timeline3D {
    /// Timeline name
    pub name: String,
    /// Total duration
    pub duration: f64,
    /// Keyframes in the timeline
    pub keyframes: Vec<Keyframe>,
    /// Current playback time
    pub current_time: f64,
    /// Whether the timeline is currently playing
    pub is_playing: bool,
    /// Whether looping is enabled
    pub loop_enabled: bool,
}

impl Timeline3D {
    /// Create a new 3D timeline
    pub fn new(name: String, duration: f64) -> Self {
        Self {
            name,
            duration,
            keyframes: Vec::new(),
            current_time: 0.0,
            is_playing: false,
            loop_enabled: false,
        }
    }

    /// Add a keyframe to the timeline
    pub fn add_keyframe(
        &mut self,
        property: AnimationProperty,
        time: f64,
        value: AnimationValue,
    ) -> Result<(), String> {
        if time < 0.0 || time > self.duration {
            return Err("Keyframe time is out of timeline bounds".to_string());
        }

        let keyframe = Keyframe {
            property,
            time,
            value,
        };

        self.keyframes.push(keyframe);
        Ok(())
    }

    /// Get keyframes for a specific property
    pub fn get_keyframes(&self, property: &AnimationProperty) -> Vec<&Keyframe> {
        self.keyframes
            .iter()
            .filter(|kf| &kf.property == property)
            .collect()
    }

    /// Get all keyframes
    pub fn keyframes(&self) -> &Vec<Keyframe> {
        &self.keyframes
    }

    /// Get timeline duration
    pub fn duration(&self) -> f64 {
        self.duration
    }

    /// Get track for a specific property
    pub fn get_track(&self, property: &AnimationProperty) -> Option<Vec<&Keyframe>> {
        let keyframes = self.get_keyframes(property);
        if keyframes.is_empty() {
            None
        } else {
            Some(keyframes)
        }
    }

    /// Get current state of the timeline
    pub fn current_state(&self) -> HashMap<AnimationProperty, AnimationValue> {
        let mut state = HashMap::new();
        
        for keyframe in &self.keyframes {
            if keyframe.time <= self.current_time {
                state.insert(keyframe.property.clone(), keyframe.value.clone());
            }
        }
        
        state
    }

    /// Seek to a specific time
    pub fn seek(&mut self, time: f64) {
        self.current_time = time.max(0.0).min(self.duration);
    }

    /// Play the timeline
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause the timeline
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Update the timeline by delta time
    pub fn update(&mut self, delta_time: f64) {
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
    }
}

/// Extension trait for keyframe tracks
pub trait KeyframeTrack {
    /// Get interpolated value at a specific time
    fn value_at(&self, time: f64) -> Result<AnimationValue, String>;
}

impl KeyframeTrack for Vec<&Keyframe> {
    fn value_at(&self, time: f64) -> Result<AnimationValue, String> {
        if self.is_empty() {
            return Err("No keyframes available".to_string());
        }

        // Sort keyframes by time
        let mut sorted_keyframes = self.to_vec();
        sorted_keyframes.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        // Find the appropriate keyframes for interpolation
        if time <= sorted_keyframes[0].time {
            return Ok(sorted_keyframes[0].value.clone());
        }

        if time >= sorted_keyframes.last().unwrap().time {
            return Ok(sorted_keyframes.last().unwrap().value.clone());
        }

        // Find the two keyframes to interpolate between
        for i in 0..sorted_keyframes.len() - 1 {
            let current = &sorted_keyframes[i];
            let next = &sorted_keyframes[i + 1];

            if time >= current.time && time <= next.time {
                // Linear interpolation
                let t = (time - current.time) / (next.time - current.time);
                
                match (&current.value, &next.value) {
                    (AnimationValue::Number(a), AnimationValue::Number(b)) => {
                        let interpolated = a + (b - a) * t;
                        return Ok(AnimationValue::Number(interpolated));
                    }
                    (AnimationValue::String(a), AnimationValue::String(b)) => {
                        if a == b {
                            return Ok(AnimationValue::String(a.clone()));
                        } else {
                            // For strings, just return the current value
                            return Ok(AnimationValue::String(a.clone()));
                        }
                    }
                    (AnimationValue::Bool(a), AnimationValue::Bool(b)) => {
                        if a == b {
                            return Ok(AnimationValue::Bool(*a));
                        } else {
                            // For booleans, return the current value
                            return Ok(AnimationValue::Bool(*a));
                        }
                    }
                    _ => {
                        // Type mismatch, return current value
                        return Ok(current.value.clone());
                    }
                }
            }
        }

        Err("Could not interpolate value".to_string())
    }
}

/// Keyframe editor for managing animation keyframes
#[derive(Debug, Clone)]
pub struct KeyframeEditor {
    /// The timeline being edited
    pub timeline: Timeline3D,
}

impl KeyframeEditor {
    /// Create a new keyframe editor
    pub fn new(timeline: Timeline3D) -> Self {
        Self { timeline }
    }

    /// Add a keyframe
    pub fn add_keyframe(
        &mut self,
        property: AnimationProperty,
        time: f64,
        value: AnimationValue,
    ) -> Result<(), String> {
        self.timeline.add_keyframe(property, time, value)
    }

    /// Remove a keyframe
    pub fn remove_keyframe(&mut self, property: &AnimationProperty, time: f64) {
        self.timeline
            .keyframes
            .retain(|kf| !(&kf.property == property && (kf.time - time).abs() < f64::EPSILON));
    }
}

/// Animation timeline for managing multiple properties
#[derive(Debug, Clone)]
pub struct AnimationTimeline {
    /// Timeline name
    pub name: String,
    /// Total duration
    pub duration: f64,
    /// Properties and their keyframes
    pub properties: HashMap<AnimationProperty, Vec<Keyframe>>,
}

impl AnimationTimeline {
    /// Create a new animation timeline
    pub fn new(name: String, duration: f64) -> Self {
        Self {
            name,
            duration,
            properties: HashMap::new(),
        }
    }

    /// Add a keyframe for a property
    pub fn add_keyframe(
        &mut self,
        property: AnimationProperty,
        time: f64,
        value: AnimationValue,
    ) -> Result<(), String> {
        if time < 0.0 || time > self.duration {
            return Err("Keyframe time is out of timeline bounds".to_string());
        }

        let keyframe = Keyframe {
            property: property.clone(),
            time,
            value,
        };

        self.properties
            .entry(property)
            .or_default()
            .push(keyframe);

        Ok(())
    }

    /// Get keyframes for a property
    pub fn get_keyframes(&self, property: &AnimationProperty) -> Option<&Vec<Keyframe>> {
        self.properties.get(property)
    }

}
