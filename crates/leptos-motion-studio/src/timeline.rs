//! Timeline and keyframe management for animations

use std::collections::HashMap;

/// Animation property types that can be animated
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq)]
pub enum AnimationValue {
    /// Numeric value
    Number(f64),
    /// String value
    String(String),
    /// Boolean value
    Bool(bool),
}

/// A keyframe in the animation timeline
#[derive(Debug, Clone)]
pub struct Keyframe {
    /// The property being animated
    pub property: AnimationProperty,
    /// Time position in the timeline
    pub time: f64,
    /// The value at this keyframe
    pub value: AnimationValue,
}

/// 3D animation timeline
#[derive(Debug, Clone)]
pub struct Timeline3D {
    /// Timeline name
    pub name: String,
    /// Total duration
    pub duration: f64,
    /// Keyframes in the timeline
    pub keyframes: Vec<Keyframe>,
}

impl Timeline3D {
    /// Create a new 3D timeline
    pub fn new(name: String, duration: f64) -> Self {
        Self {
            name,
            duration,
            keyframes: Vec::new(),
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
            .or_insert_with(Vec::new)
            .push(keyframe);

        Ok(())
    }

    /// Get keyframes for a property
    pub fn get_keyframes(&self, property: &AnimationProperty) -> Option<&Vec<Keyframe>> {
        self.properties.get(property)
    }
}
