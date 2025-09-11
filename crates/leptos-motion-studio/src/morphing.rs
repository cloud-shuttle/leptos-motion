//! SVG path morphing functionality for Motion Studio

use crate::{Result, StudioError};
use leptos::*;
use leptos::prelude::{ElementChild, PropAttribute, CustomAttribute, NodeRefAttribute, StyleAttribute, OnAttribute, create_signal, create_memo, event_target_value, Callback, Get, Set};
use leptos::attr::global::ClassAttribute;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "svg-morphing")]
use lyon::path::Path as LyonPath;
#[cfg(feature = "svg-morphing")]
use lyon::path::builder::*;

/// SVG path data structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SvgPath {
    /// Path ID
    pub id: Uuid,
    /// Path data string (d attribute)
    pub data: String,
    /// Parsed path commands
    pub commands: Vec<PathCommand>,
    /// Path bounding box
    pub bounds: Option<BoundingBox>,
    /// Path length (approximate)
    pub length: f32,
}

impl SvgPath {
    /// Create SVG path from data string
    pub fn from_data(data: &str) -> Result<Self> {
        let commands = Self::parse_path_data(data)?;
        let bounds = Self::calculate_bounds(&commands);
        let length = Self::calculate_length(&commands);

        Ok(Self {
            id: Uuid::new_v4(),
            data: data.to_string(),
            commands,
            bounds: Some(bounds),
            length,
        })
    }

    /// Parse SVG path data into commands
    fn parse_path_data(data: &str) -> Result<Vec<PathCommand>> {
        let mut commands = Vec::new();
        let mut chars = data.chars().peekable();
        let mut current_pos = Point::new(0.0, 0.0);
        let mut path_start = Point::new(0.0, 0.0);

        while let Some(&ch) = chars.peek() {
            if ch.is_whitespace() || ch == ',' {
                chars.next();
                continue;
            }

            if ch.is_ascii_alphabetic() {
                let command_char = chars.next().unwrap();
                match command_char {
                    'M' | 'm' => {
                        let (x, y) = Self::parse_coordinate_pair(&mut chars)?;
                        let absolute = command_char.is_ascii_uppercase();
                        if absolute {
                            current_pos = Point::new(x, y);
                        } else {
                            current_pos = Point::new(current_pos.x + x, current_pos.y + y);
                        }
                        path_start = current_pos;
                        commands.push(PathCommand::MoveTo {
                            point: current_pos,
                            absolute,
                        });
                    }
                    'L' | 'l' => {
                        let (x, y) = Self::parse_coordinate_pair(&mut chars)?;
                        let absolute = command_char.is_ascii_uppercase();
                        if absolute {
                            current_pos = Point::new(x, y);
                        } else {
                            current_pos = Point::new(current_pos.x + x, current_pos.y + y);
                        }
                        commands.push(PathCommand::LineTo {
                            point: current_pos,
                            absolute,
                        });
                    }
                    'C' | 'c' => {
                        let (x1, y1) = Self::parse_coordinate_pair(&mut chars)?;
                        let (x2, y2) = Self::parse_coordinate_pair(&mut chars)?;
                        let (x, y) = Self::parse_coordinate_pair(&mut chars)?;
                        let absolute = command_char.is_ascii_uppercase();

                        let (cp1, cp2, end_point) = if absolute {
                            (Point::new(x1, y1), Point::new(x2, y2), Point::new(x, y))
                        } else {
                            (
                                Point::new(current_pos.x + x1, current_pos.y + y1),
                                Point::new(current_pos.x + x2, current_pos.y + y2),
                                Point::new(current_pos.x + x, current_pos.y + y),
                            )
                        };

                        current_pos = end_point;
                        commands.push(PathCommand::CurveTo {
                            control1: cp1,
                            control2: cp2,
                            point: end_point,
                            absolute,
                        });
                    }
                    'Q' | 'q' => {
                        let (x1, y1) = Self::parse_coordinate_pair(&mut chars)?;
                        let (x, y) = Self::parse_coordinate_pair(&mut chars)?;
                        let absolute = command_char.is_ascii_uppercase();

                        let (control, end_point) = if absolute {
                            (Point::new(x1, y1), Point::new(x, y))
                        } else {
                            (
                                Point::new(current_pos.x + x1, current_pos.y + y1),
                                Point::new(current_pos.x + x, current_pos.y + y),
                            )
                        };

                        current_pos = end_point;
                        commands.push(PathCommand::QuadTo {
                            control,
                            point: end_point,
                            absolute,
                        });
                    }
                    'Z' | 'z' => {
                        current_pos = path_start;
                        commands.push(PathCommand::ClosePath);
                    }
                    _ => {
                        return Err(StudioError::MorphingError(format!(
                            "Unsupported path command: {}",
                            command_char
                        )));
                    }
                }
            } else {
                return Err(StudioError::MorphingError(
                    "Invalid path data format".to_string(),
                ));
            }
        }

        Ok(commands)
    }

    /// Parse coordinate pair from character iterator
    fn parse_coordinate_pair(
        chars: &mut std::iter::Peekable<std::str::Chars>,
    ) -> Result<(f32, f32)> {
        // Skip whitespace and commas
        while chars
            .peek()
            .map_or(false, |&c| c.is_whitespace() || c == ',')
        {
            chars.next();
        }

        // Parse X coordinate
        let mut x_str = String::new();
        while chars
            .peek()
            .map_or(false, |&c| c.is_ascii_digit() || c == '.' || c == '-')
        {
            x_str.push(chars.next().unwrap());
        }

        // Skip whitespace and commas
        while chars
            .peek()
            .map_or(false, |&c| c.is_whitespace() || c == ',')
        {
            chars.next();
        }

        // Parse Y coordinate
        let mut y_str = String::new();
        while chars
            .peek()
            .map_or(false, |&c| c.is_ascii_digit() || c == '.' || c == '-')
        {
            y_str.push(chars.next().unwrap());
        }

        let x = x_str
            .parse::<f32>()
            .map_err(|_| StudioError::MorphingError("Invalid X coordinate".to_string()))?;

        let y = y_str
            .parse::<f32>()
            .map_err(|_| StudioError::MorphingError("Invalid Y coordinate".to_string()))?;

        Ok((x, y))
    }

    /// Calculate path bounding box
    fn calculate_bounds(commands: &[PathCommand]) -> BoundingBox {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for command in commands {
            match command {
                PathCommand::MoveTo { point, .. }
                | PathCommand::LineTo { point, .. }
                | PathCommand::QuadTo { point, .. }
                | PathCommand::CurveTo { point, .. } => {
                    min_x = min_x.min(point.x);
                    min_y = min_y.min(point.y);
                    max_x = max_x.max(point.x);
                    max_y = max_y.max(point.y);
                }
                PathCommand::ClosePath => {}
            }
        }

        BoundingBox {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Calculate approximate path length
    fn calculate_length(commands: &[PathCommand]) -> f32 {
        let mut length = 0.0;
        let mut current_pos = Point::new(0.0, 0.0);

        for command in commands {
            match command {
                PathCommand::MoveTo { point, .. } => {
                    current_pos = *point;
                }
                PathCommand::LineTo { point, .. } => {
                    length += current_pos.distance_to(*point);
                    current_pos = *point;
                }
                PathCommand::QuadTo { control, point, .. } => {
                    // Approximate quadratic bezier length
                    let d1 = current_pos.distance_to(*control);
                    let d2 = control.distance_to(*point);
                    length += (d1 + d2) * 0.75; // Rough approximation
                    current_pos = *point;
                }
                PathCommand::CurveTo {
                    control1,
                    control2,
                    point,
                    ..
                } => {
                    // Approximate cubic bezier length
                    let d1 = current_pos.distance_to(*control1);
                    let d2 = control1.distance_to(*control2);
                    let d3 = control2.distance_to(*point);
                    length += (d1 + d2 + d3) * 0.6; // Rough approximation
                    current_pos = *point;
                }
                PathCommand::ClosePath => {}
            }
        }

        length
    }

    /// Convert back to SVG path data string
    pub fn to_data(&self) -> String {
        let mut data = String::new();

        for command in &self.commands {
            match command {
                PathCommand::MoveTo { point, absolute } => {
                    let cmd = if *absolute { "M" } else { "m" };
                    data.push_str(&format!("{} {:.2} {:.2}", cmd, point.x, point.y));
                }
                PathCommand::LineTo { point, absolute } => {
                    let cmd = if *absolute { "L" } else { "l" };
                    data.push_str(&format!(" {} {:.2} {:.2}", cmd, point.x, point.y));
                }
                PathCommand::CurveTo {
                    control1,
                    control2,
                    point,
                    absolute,
                } => {
                    let cmd = if *absolute { "C" } else { "c" };
                    data.push_str(&format!(
                        " {} {:.2} {:.2} {:.2} {:.2} {:.2} {:.2}",
                        cmd, control1.x, control1.y, control2.x, control2.y, point.x, point.y
                    ));
                }
                PathCommand::QuadTo {
                    control,
                    point,
                    absolute,
                } => {
                    let cmd = if *absolute { "Q" } else { "q" };
                    data.push_str(&format!(
                        " {} {:.2} {:.2} {:.2} {:.2}",
                        cmd, control.x, control.y, point.x, point.y
                    ));
                }
                PathCommand::ClosePath => {
                    data.push_str(" Z");
                }
            }
        }

        data.trim().to_string()
    }
}

/// SVG path command types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PathCommand {
    MoveTo {
        point: Point,
        absolute: bool,
    },
    LineTo {
        point: Point,
        absolute: bool,
    },
    CurveTo {
        control1: Point,
        control2: Point,
        point: Point,
        absolute: bool,
    },
    QuadTo {
        control: Point,
        point: Point,
        absolute: bool,
    },
    ClosePath,
}

/// 2D point for path coordinates
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn distance_to(self, other: Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
        }
    }
}

/// Bounding box for paths
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

impl BoundingBox {
    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }

    pub fn center(&self) -> Point {
        Point::new(
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }
}

/// Path morphing engine
#[derive(Debug, Clone, PartialEq)]
pub struct PathMorpher {
    /// Source path
    pub source: SvgPath,
    /// Target path
    pub target: SvgPath,
    /// Morphing configuration
    pub config: MorphConfig,
    /// Cached normalized paths
    pub normalized_source: Option<Vec<PathCommand>>,
    pub normalized_target: Option<Vec<PathCommand>>,
}

impl PathMorpher {
    /// Create new path morpher
    pub fn new(source_data: &str, target_data: &str) -> Result<Self> {
        let source = SvgPath::from_data(source_data)?;
        let target = SvgPath::from_data(target_data)?;

        Ok(Self {
            source,
            target,
            config: MorphConfig::default(),
            normalized_source: None,
            normalized_target: None,
        })
    }

    /// Prepare paths for morphing by normalizing command counts
    pub fn prepare(&mut self) -> Result<()> {
        // Normalize paths to have same number of commands
        let (norm_source, norm_target) = self.normalize_paths()?;
        self.normalized_source = Some(norm_source);
        self.normalized_target = Some(norm_target);
        Ok(())
    }

    /// Interpolate between source and target paths
    pub fn interpolate(&self, t: f32) -> Result<SvgPath> {
        let t = t.clamp(0.0, 1.0);

        if t == 0.0 {
            return Ok(self.source.clone());
        } else if t == 1.0 {
            return Ok(self.target.clone());
        }

        // Use normalized paths if available, otherwise prepare them
        let (source_commands, target_commands) =
            if let (Some(src), Some(tgt)) = (&self.normalized_source, &self.normalized_target) {
                (src.clone(), tgt.clone())
            } else {
                self.normalize_paths()?
            };

        if source_commands.len() != target_commands.len() {
            return Err(StudioError::MorphingError(
                "Path normalization failed - command count mismatch".to_string(),
            ));
        }

        // Interpolate each command
        let mut interpolated_commands = Vec::new();
        for (src_cmd, tgt_cmd) in source_commands.iter().zip(target_commands.iter()) {
            let interpolated = self.interpolate_commands(src_cmd, tgt_cmd, t)?;
            interpolated_commands.push(interpolated);
        }

        // Create interpolated path
        let mut interpolated_path = SvgPath {
            id: Uuid::new_v4(),
            data: String::new(),
            commands: interpolated_commands,
            bounds: None,
            length: 0.0,
        };

        // Update derived properties
        interpolated_path.data = interpolated_path.to_data();
        interpolated_path.bounds = Some(SvgPath::calculate_bounds(&interpolated_path.commands));
        interpolated_path.length = SvgPath::calculate_length(&interpolated_path.commands);

        Ok(interpolated_path)
    }

    /// Normalize two paths to have the same number of commands
    fn normalize_paths(&self) -> Result<(Vec<PathCommand>, Vec<PathCommand>)> {
        let mut source_commands = self.source.commands.clone();
        let mut target_commands = self.target.commands.clone();

        // Find the maximum number of commands
        let max_commands = source_commands.len().max(target_commands.len());

        // Pad shorter path with intermediate points
        while source_commands.len() < max_commands {
            source_commands = self.subdivide_path(source_commands)?;
            if source_commands.len() >= max_commands {
                source_commands.truncate(max_commands);
                break;
            }
        }

        while target_commands.len() < max_commands {
            target_commands = self.subdivide_path(target_commands)?;
            if target_commands.len() >= max_commands {
                target_commands.truncate(max_commands);
                break;
            }
        }

        // Ensure both paths have exactly the same command types
        let (normalized_source, normalized_target) =
            self.align_command_types(source_commands, target_commands)?;

        Ok((normalized_source, normalized_target))
    }

    /// Subdivide path by adding intermediate points
    fn subdivide_path(&self, commands: Vec<PathCommand>) -> Result<Vec<PathCommand>> {
        let mut subdivided = Vec::new();

        for (i, command) in commands.iter().enumerate() {
            subdivided.push(command.clone());

            // Add intermediate point for curves
            if i < commands.len() - 1 {
                if let (
                    PathCommand::LineTo { point: start, .. },
                    PathCommand::LineTo { point: end, .. },
                ) = (command, &commands[i + 1])
                {
                    let mid_point = start.lerp(*end, 0.5);
                    subdivided.push(PathCommand::LineTo {
                        point: mid_point,
                        absolute: true,
                    });
                }
            }
        }

        Ok(subdivided)
    }

    /// Align command types between two paths
    fn align_command_types(
        &self,
        mut source: Vec<PathCommand>,
        mut target: Vec<PathCommand>,
    ) -> Result<(Vec<PathCommand>, Vec<PathCommand>)> {
        // Ensure both paths have the same length
        let len = source.len().min(target.len());
        source.truncate(len);
        target.truncate(len);

        // Convert incompatible command types
        for i in 0..len {
            match (&source[i], &target[i]) {
                (PathCommand::MoveTo { .. }, PathCommand::LineTo { point, absolute }) => {
                    target[i] = PathCommand::MoveTo {
                        point: *point,
                        absolute: *absolute,
                    };
                }
                (PathCommand::LineTo { point, absolute }, PathCommand::MoveTo { .. }) => {
                    source[i] = PathCommand::MoveTo {
                        point: *point,
                        absolute: *absolute,
                    };
                }
                // Add more alignment cases as needed
                _ => {}
            }
        }

        Ok((source, target))
    }

    /// Interpolate between two path commands
    fn interpolate_commands(
        &self,
        source: &PathCommand,
        target: &PathCommand,
        t: f32,
    ) -> Result<PathCommand> {
        match (source, target) {
            (
                PathCommand::MoveTo {
                    point: p1,
                    absolute: a1,
                },
                PathCommand::MoveTo {
                    point: p2,
                    absolute: a2,
                },
            ) => {
                Ok(PathCommand::MoveTo {
                    point: p1.lerp(*p2, t),
                    absolute: *a1 || *a2, // Prefer absolute
                })
            }
            (
                PathCommand::LineTo {
                    point: p1,
                    absolute: a1,
                },
                PathCommand::LineTo {
                    point: p2,
                    absolute: a2,
                },
            ) => Ok(PathCommand::LineTo {
                point: p1.lerp(*p2, t),
                absolute: *a1 || *a2,
            }),
            (
                PathCommand::CurveTo {
                    control1: c1_1,
                    control2: c1_2,
                    point: p1,
                    absolute: a1,
                },
                PathCommand::CurveTo {
                    control1: c2_1,
                    control2: c2_2,
                    point: p2,
                    absolute: a2,
                },
            ) => Ok(PathCommand::CurveTo {
                control1: c1_1.lerp(*c2_1, t),
                control2: c1_2.lerp(*c2_2, t),
                point: p1.lerp(*p2, t),
                absolute: *a1 || *a2,
            }),
            (
                PathCommand::QuadTo {
                    control: c1,
                    point: p1,
                    absolute: a1,
                },
                PathCommand::QuadTo {
                    control: c2,
                    point: p2,
                    absolute: a2,
                },
            ) => Ok(PathCommand::QuadTo {
                control: c1.lerp(*c2, t),
                point: p1.lerp(*p2, t),
                absolute: *a1 || *a2,
            }),
            (PathCommand::ClosePath, PathCommand::ClosePath) => Ok(PathCommand::ClosePath),
            // Convert between different command types
            (source_cmd, target_cmd) => {
                // Convert to line segments for incompatible types
                let source_point = self.extract_end_point(source_cmd);
                let target_point = self.extract_end_point(target_cmd);

                Ok(PathCommand::LineTo {
                    point: source_point.lerp(target_point, t),
                    absolute: true,
                })
            }
        }
    }

    /// Extract end point from any path command
    fn extract_end_point(&self, command: &PathCommand) -> Point {
        match command {
            PathCommand::MoveTo { point, .. }
            | PathCommand::LineTo { point, .. }
            | PathCommand::QuadTo { point, .. }
            | PathCommand::CurveTo { point, .. } => *point,
            PathCommand::ClosePath => Point::zero(),
        }
    }
}

/// Morphing configuration
#[derive(Debug, Clone, PartialEq)]
pub struct MorphConfig {
    /// Smoothing factor (0.0 to 1.0)
    pub smoothing: f32,
    /// Whether to optimize for performance
    pub optimize: bool,
    /// Maximum subdivisions for path normalization
    pub max_subdivisions: usize,
    /// Tolerance for path simplification
    pub tolerance: f32,
}

impl Default for MorphConfig {
    fn default() -> Self {
        Self {
            smoothing: 0.5,
            optimize: true,
            max_subdivisions: 100,
            tolerance: 1.0,
        }
    }
}

/// SVG morphing animation
#[derive(Debug, Clone)]
pub struct SvgMorphing {
    /// Animation ID
    pub id: Uuid,
    /// Source path
    pub from_path: String,
    /// Target path
    pub to_path: String,
    /// Animation duration
    pub duration: f32,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Easing function
    pub easing: crate::transforms::EasingFunction,
    /// Path morpher
    pub morpher: Option<PathMorpher>,
    /// Animation state
    pub is_playing: bool,
}

impl SvgMorphing {
    /// Create new SVG morphing animation
    pub fn new(from_path: String, to_path: String, duration: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            from_path,
            to_path,
            duration,
            progress: 0.0,
            easing: crate::transforms::EasingFunction::EaseInOut,
            morpher: None,
            is_playing: false,
        }
    }

    /// Initialize morphing engine
    pub fn initialize(&mut self) -> Result<()> {
        let mut morpher = PathMorpher::new(&self.from_path, &self.to_path)?;
        morpher.prepare()?;
        self.morpher = Some(morpher);
        Ok(())
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) -> Result<String> {
        if !self.is_playing {
            return self.current_path();
        }

        self.progress += delta_time / self.duration;
        self.progress = self.progress.clamp(0.0, 1.0);

        if self.progress >= 1.0 {
            self.is_playing = false;
        }

        self.current_path()
    }

    /// Get current interpolated path
    pub fn current_path(&self) -> Result<String> {
        if let Some(morpher) = &self.morpher {
            let t = self.easing.apply(self.progress);
            let interpolated = morpher.interpolate(t)?;
            Ok(interpolated.to_data())
        } else {
            Ok(self.from_path.clone())
        }
    }

    /// Play animation
    pub fn play(&mut self) {
        self.is_playing = true;
    }

    /// Pause animation
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Reset animation
    pub fn reset(&mut self) {
        self.progress = 0.0;
        self.is_playing = false;
    }
}

/// Morph transition for timeline integration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MorphTransition {
    /// Transition ID
    pub id: Uuid,
    /// Start path
    pub start_path: String,
    /// End path
    pub end_path: String,
    /// Transition duration
    pub duration: f32,
    /// Easing function
    pub easing: crate::transforms::EasingFunction,
}

impl MorphTransition {
    /// Create new morph transition
    pub fn new(start_path: String, end_path: String, duration: f32) -> Self {
        Self {
            id: Uuid::new_v4(),
            start_path,
            end_path,
            duration,
            easing: crate::transforms::EasingFunction::EaseInOut,
        }
    }

    /// Evaluate transition at given time
    pub fn evaluate(&self, time: f32) -> Result<String> {
        let t = (time / self.duration).clamp(0.0, 1.0);
        let eased_t = self.easing.apply(t);

        let morpher = PathMorpher::new(&self.start_path, &self.end_path)?;
        let interpolated = morpher.interpolate(eased_t)?;
        Ok(interpolated.to_data())
    }
}

/// SVG Morphing Editor Component
#[component]
pub fn SvgMorphingEditor(
    /// Source path
    #[prop(optional)]
    from_path: Option<String>,

    /// Target path
    #[prop(optional)]
    to_path: Option<String>,

    /// Callback when paths change
    #[prop(optional)]
    on_change: Option<Callback<(String, String)>>,
) -> impl IntoView {
    let (source_path, set_source_path) =
        create_signal(from_path.unwrap_or_else(|| "M 50 50 L 150 50 L 100 150 Z".to_string()));
    let (target_path, set_target_path) =
        create_signal(to_path.unwrap_or_else(|| "M 50 100 L 150 100 L 100 50 Z".to_string()));
    let (morph_progress, set_morph_progress) = create_signal(0.0);

    // Create morpher when paths change
    let current_morpher =
        create_memo(move |_| PathMorpher::new(&source_path.get(), &target_path.get()).ok());

    // Get current interpolated path
    let interpolated_path = create_memo(move |_| {
        if let Some(morpher) = current_morpher.get() {
            morpher
                .interpolate(morph_progress.get())
                .unwrap_or_else(|_| {
                    SvgPath::from_data(&source_path.get()).unwrap_or_else(|_| SvgPath {
                        id: Uuid::new_v4(),
                        data: source_path.get(),
                        commands: vec![],
                        bounds: None,
                        length: 0.0,
                    })
                })
                .to_data()
        } else {
            source_path.get()
        }
    });

    view! {
        <div class="svg-morphing-editor">
            <div class="svg-morphing-editor__controls">
                <div class="path-input-group">
                    <label>"Source Path:"</label>
                    <textarea
                        prop:value=source_path
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_source_path.set(value.clone());
                    // Temporarily disabled until callback API is clarified
                    // if let Some(callback) = on_change {
                    //     callback((value, target_path.get()));
                    // }
                        }
                    ></textarea>
                </div>

                <div class="path-input-group">
                    <label>"Target Path:"</label>
                    <textarea
                        prop:value=target_path
                        on:input=move |ev| {
                            let value = event_target_value(&ev);
                            set_target_path.set(value.clone());
                    // Temporarily disabled until callback API is clarified
                    // if let Some(callback) = on_change {
                    //     callback((source_path.get(), value));
                    // }
                        }
                    ></textarea>
                </div>

                <div class="morph-controls">
                    <label>"Morph Progress:"</label>
                    <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        prop:value=morph_progress
                        on:input=move |ev| {
                            let value: f32 = event_target_value(&ev).parse().unwrap_or(0.0);
                            set_morph_progress.set(value);
                        }
                    />
                    <span>{move || format!("{:.2}", morph_progress.get())}</span>
                </div>
            </div>

            <div class="svg-morphing-editor__preview">
                <svg
                    width="300"
                    height="300"
                    viewBox="0 0 200 200"
                    class="morph-preview"
                >
                    <path
                        d=move || interpolated_path.get()
                        fill="rgba(100, 150, 255, 0.5)"
                        stroke="rgb(100, 150, 255)"
                        stroke-width="2"
                    />

                    // Show source path in background
                    <path
                        d=source_path
                        fill="none"
                        stroke="rgba(255, 100, 100, 0.3)"
                        stroke-width="1"
                        stroke-dasharray="5,5"
                    />

                    // Show target path in background
                    <path
                        d=target_path
                        fill="none"
                        stroke="rgba(100, 255, 100, 0.3)"
                        stroke-width="1"
                        stroke-dasharray="5,5"
                    />
                </svg>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_svg_path_parsing() {
        let path_data = "M 10 10 L 90 90 Z";
        let path = SvgPath::from_data(path_data).unwrap();

        assert_eq!(path.commands.len(), 3);
        assert!(matches!(path.commands[0], PathCommand::MoveTo { .. }));
        assert!(matches!(path.commands[1], PathCommand::LineTo { .. }));
        assert!(matches!(path.commands[2], PathCommand::ClosePath));
    }

    #[test]
    fn test_path_to_data() {
        let path_data = "M 10 10 L 90 90 Z";
        let path = SvgPath::from_data(path_data).unwrap();
        let regenerated = path.to_data();

        // Should be equivalent (allowing for formatting differences)
        let reparsed = SvgPath::from_data(&regenerated).unwrap();
        assert_eq!(path.commands.len(), reparsed.commands.len());
    }

    #[test]
    fn test_point_interpolation() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(100.0, 100.0);
        let lerped = p1.lerp(p2, 0.5);

        assert_relative_eq!(lerped.x, 50.0, epsilon = 1e-6);
        assert_relative_eq!(lerped.y, 50.0, epsilon = 1e-6);
    }

    #[test]
    fn test_simple_path_morphing() {
        let path1 = "M 0 0 L 100 0 L 100 100 Z";
        let path2 = "M 0 0 L 50 50 L 100 100 Z";

        let morpher = PathMorpher::new(path1, path2);
        assert!(morpher.is_ok());

        let morpher = morpher.unwrap();
        let interpolated = morpher.interpolate(0.5);
        assert!(interpolated.is_ok());

        let result = interpolated.unwrap();
        assert!(!result.data.is_empty());
    }

    #[test]
    fn test_bounding_box_calculation() {
        let commands = vec![
            PathCommand::MoveTo {
                point: Point::new(10.0, 10.0),
                absolute: true,
            },
            PathCommand::LineTo {
                point: Point::new(90.0, 90.0),
                absolute: true,
            },
        ];

        let bounds = SvgPath::calculate_bounds(&commands);
        assert_relative_eq!(bounds.min_x, 10.0, epsilon = 1e-6);
        assert_relative_eq!(bounds.min_y, 10.0, epsilon = 1e-6);
        assert_relative_eq!(bounds.max_x, 90.0, epsilon = 1e-6);
        assert_relative_eq!(bounds.max_y, 90.0, epsilon = 1e-6);
        assert_relative_eq!(bounds.width(), 80.0, epsilon = 1e-6);
        assert_relative_eq!(bounds.height(), 80.0, epsilon = 1e-6);
    }

    #[test]
    fn test_svg_morphing_animation() {
        let mut morphing = SvgMorphing::new(
            "M 0 0 L 100 0 Z".to_string(),
            "M 0 0 L 0 100 Z".to_string(),
            1.0,
        );

        assert!(morphing.initialize().is_ok());

        morphing.play();
        assert!(morphing.is_playing);

        let result = morphing.update(0.5);
        assert!(result.is_ok());
        assert_relative_eq!(morphing.progress, 0.5, epsilon = 1e-6);
    }

    #[test]
    fn test_morph_transition() {
        let transition = MorphTransition::new(
            "M 0 0 L 100 0 Z".to_string(),
            "M 0 0 L 0 100 Z".to_string(),
            2.0,
        );

        let result_at_start = transition.evaluate(0.0);
        assert!(result_at_start.is_ok());

        let result_at_mid = transition.evaluate(1.0);
        assert!(result_at_mid.is_ok());

        let result_at_end = transition.evaluate(2.0);
        assert!(result_at_end.is_ok());
    }

    #[test]
    fn test_complex_path_parsing() {
        let path_data = "M 10 10 C 20 20 30 30 40 40 Q 50 50 60 60 L 70 70 Z";
        let path = SvgPath::from_data(path_data);
        assert!(path.is_ok());

        let path = path.unwrap();
        assert_eq!(path.commands.len(), 5);
        assert!(matches!(path.commands[0], PathCommand::MoveTo { .. }));
        assert!(matches!(path.commands[1], PathCommand::CurveTo { .. }));
        assert!(matches!(path.commands[2], PathCommand::QuadTo { .. }));
        assert!(matches!(path.commands[3], PathCommand::LineTo { .. }));
        assert!(matches!(path.commands[4], PathCommand::ClosePath));
    }
}
