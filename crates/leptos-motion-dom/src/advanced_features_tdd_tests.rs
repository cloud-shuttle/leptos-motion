//! TDD Tests for Advanced Features
//!
//! This module contains comprehensive tests for advanced animation features:
//! - SVG path morphing animations
//! - Shared element transitions
//! - Animation orchestration and sequencing

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::{ReactiveMotionDiv, reactive_animate};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::{console_error, console_log};
use web_sys::{Element, SvgElement};

// ============================================================================
// SVG PATH MORPHING ANIMATIONS
// ============================================================================

/// SVG path morphing configuration
#[derive(Clone, Debug)]
pub struct PathMorphConfig {
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: Easing,
    /// Whether to preserve aspect ratio
    pub preserve_aspect_ratio: bool,
    /// Number of interpolation points
    pub interpolation_points: usize,
    /// Whether to use cubic bezier interpolation
    pub use_cubic_bezier: bool,
    /// Path precision (number of decimal places)
    pub precision: usize,
}

impl Default for PathMorphConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            easing: Easing::EaseInOut,
            preserve_aspect_ratio: true,
            interpolation_points: 100,
            use_cubic_bezier: true,
            precision: 2,
        }
    }
}

/// SVG path data structure
#[derive(Clone, Debug, PartialEq)]
pub struct PathData {
    /// Raw path string (e.g., "M10,10 L20,20 C30,30 40,40 50,50")
    pub raw: String,
    /// Parsed path commands
    pub commands: Vec<PathCommand>,
    /// Bounding box
    pub bounds: PathBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathCommand {
    pub command_type: PathCommandType,
    pub points: Vec<PathPoint>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathCommandType {
    MoveTo,
    LineTo,
    CurveTo,
    QuadraticCurveTo,
    Arc,
    ClosePath,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathPoint {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PathBounds {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub width: f64,
    pub height: f64,
}

/// SVG path morphing manager
#[derive(Clone)]
pub struct PathMorphManager {
    config: PathMorphConfig,
    start_path: Option<PathData>,
    end_path: Option<PathData>,
    current_path: Option<PathData>,
    progress: f64,
    is_animating: bool,
    animation_id: Option<i32>,
}

impl PathMorphManager {
    /// Create a new path morphing manager
    pub fn new(config: PathMorphConfig) -> Self {
        Self {
            config,
            start_path: None,
            end_path: None,
            current_path: None,
            progress: 0.0,
            is_animating: false,
            animation_id: None,
        }
    }

    /// Set the start path
    pub fn set_start_path(&mut self, path_string: &str) -> std::result::Result<(), String> {
        self.start_path = Some(Self::parse_path(path_string)?);
        Ok(())
    }

    /// Set the end path
    pub fn set_end_path(&mut self, path_string: &str) -> std::result::Result<(), String> {
        self.end_path = Some(Self::parse_path(path_string)?);
        Ok(())
    }

    /// Start the path morphing animation
    pub fn start_animation(&mut self) -> std::result::Result<(), String> {
        if self.start_path.is_none() || self.end_path.is_none() {
            return Err("Both start and end paths must be set".to_string());
        }

        self.is_animating = true;
        self.progress = 0.0;

        // Start the animation loop
        self.animate_frame()?;

        Ok(())
    }

    /// Stop the animation
    pub fn stop_animation(&mut self) {
        self.is_animating = false;
        if let Some(_id) = self.animation_id {
            // Cancel animation frame
            self.animation_id = None;
        }
    }

    /// Get current path data
    pub fn get_current_path(&self) -> Option<&PathData> {
        self.current_path.as_ref()
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        self.progress
    }

    /// Check if currently animating
    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    /// Parse SVG path string into PathData
    fn parse_path(path_string: &str) -> std::result::Result<PathData, String> {
        let mut commands = Vec::new();
        let mut current_command = None;
        let mut current_points = Vec::new();

        // Simple path parsing - in real implementation, use proper SVG path parser
        let tokens: Vec<&str> = path_string.split_whitespace().collect();
        let mut i = 0;

        while i < tokens.len() {
            let token = tokens[i];

            if let Some(command_type) = Self::parse_command_type(token) {
                // Save previous command if exists
                if let Some(cmd_type) = current_command {
                    commands.push(PathCommand {
                        command_type: cmd_type,
                        points: current_points.clone(),
                    });
                }

                // Start new command
                current_command = Some(command_type);
                current_points.clear();
            } else if let Some(point) = Self::parse_point(token) {
                current_points.push(point);
            }

            i += 1;
        }

        // Add final command
        if let Some(cmd_type) = current_command {
            commands.push(PathCommand {
                command_type: cmd_type,
                points: current_points,
            });
        }

        // Calculate bounds
        let bounds = Self::calculate_bounds(&commands);

        Ok(PathData {
            raw: path_string.to_string(),
            commands,
            bounds,
        })
    }

    /// Parse command type from token
    fn parse_command_type(token: &str) -> Option<PathCommandType> {
        match token {
            "M" | "m" => Some(PathCommandType::MoveTo),
            "L" | "l" => Some(PathCommandType::LineTo),
            "C" | "c" => Some(PathCommandType::CurveTo),
            "Q" | "q" => Some(PathCommandType::QuadraticCurveTo),
            "A" | "a" => Some(PathCommandType::Arc),
            "Z" | "z" => Some(PathCommandType::ClosePath),
            _ => None,
        }
    }

    /// Parse point from token
    fn parse_point(token: &str) -> Option<PathPoint> {
        if let Some(comma_pos) = token.find(',') {
            let x_str = &token[..comma_pos];
            let y_str = &token[comma_pos + 1..];

            if let (Ok(x), Ok(y)) = (x_str.parse::<f64>(), y_str.parse::<f64>()) {
                return Some(PathPoint { x, y });
            }
        }
        None
    }

    /// Calculate path bounds
    fn calculate_bounds(commands: &[PathCommand]) -> PathBounds {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for command in commands {
            for point in &command.points {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }

        PathBounds {
            min_x,
            min_y,
            max_x,
            max_y,
            width: max_x - min_x,
            height: max_y - min_y,
        }
    }

    /// Interpolate between two paths
    fn interpolate_paths(start: &PathData, end: &PathData, progress: f64) -> PathData {
        let mut interpolated_commands = Vec::new();

        // Ensure both paths have the same number of commands
        let max_commands = start.commands.len().max(end.commands.len());

        for i in 0..max_commands {
            let start_cmd = start.commands.get(i);
            let end_cmd = end.commands.get(i);

            if let (Some(start_cmd), Some(end_cmd)) = (start_cmd, end_cmd) {
                let interpolated_cmd = Self::interpolate_command(start_cmd, end_cmd, progress);
                interpolated_commands.push(interpolated_cmd);
            } else if let Some(cmd) = start_cmd {
                interpolated_commands.push(cmd.clone());
            } else if let Some(cmd) = end_cmd {
                interpolated_commands.push(cmd.clone());
            }
        }

        let bounds = Self::calculate_bounds(&interpolated_commands);
        let raw = Self::commands_to_string(&interpolated_commands);

        PathData {
            raw,
            commands: interpolated_commands,
            bounds,
        }
    }

    /// Interpolate between two path commands
    fn interpolate_command(start: &PathCommand, end: &PathCommand, progress: f64) -> PathCommand {
        let mut interpolated_points = Vec::new();
        let max_points = start.points.len().max(end.points.len());

        for i in 0..max_points {
            let start_point = start.points.get(i);
            let end_point = end.points.get(i);

            if let (Some(start_point), Some(end_point)) = (start_point, end_point) {
                let interpolated_point = PathPoint {
                    x: start_point.x + (end_point.x - start_point.x) * progress,
                    y: start_point.y + (end_point.y - start_point.y) * progress,
                };
                interpolated_points.push(interpolated_point);
            } else if let Some(point) = start_point {
                interpolated_points.push(point.clone());
            } else if let Some(point) = end_point {
                interpolated_points.push(point.clone());
            }
        }

        PathCommand {
            command_type: start.command_type.clone(),
            points: interpolated_points,
        }
    }

    /// Convert commands back to path string
    fn commands_to_string(commands: &[PathCommand]) -> String {
        let mut result = String::new();

        for command in commands {
            let cmd_char = match command.command_type {
                PathCommandType::MoveTo => "M",
                PathCommandType::LineTo => "L",
                PathCommandType::CurveTo => "C",
                PathCommandType::QuadraticCurveTo => "Q",
                PathCommandType::Arc => "A",
                PathCommandType::ClosePath => "Z",
            };

            result.push_str(cmd_char);

            for point in &command.points {
                result.push_str(&format!("{},{} ", point.x, point.y));
            }
        }

        result.trim().to_string()
    }

    /// Animate a single frame
    fn animate_frame(&mut self) -> std::result::Result<(), String> {
        if !self.is_animating {
            return Ok(());
        }

        let start_path = self.start_path.as_ref().ok_or("No start path")?;
        let end_path = self.end_path.as_ref().ok_or("No end path")?;

        // Apply easing to progress
        let eased_progress = self.apply_easing(self.progress);

        // Interpolate paths
        self.current_path = Some(Self::interpolate_paths(
            start_path,
            end_path,
            eased_progress,
        ));

        // Update progress
        self.progress += 1.0 / (self.config.duration * 60.0); // Assuming 60 FPS

        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.is_animating = false;
        }

        Ok(())
    }

    /// Apply easing function to progress
    fn apply_easing(&self, progress: f64) -> f64 {
        match self.config.easing {
            Easing::Linear => progress,
            Easing::EaseIn => progress * progress,
            Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            _ => progress, // Handle other easing types with linear fallback
        }
    }
}

/// SVG Path MotionDiv component
#[component]
pub fn PathMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Start path string
    #[prop(optional)]
    start_path: Option<String>,
    /// End path string
    #[prop(optional)]
    end_path: Option<String>,
    /// Path morphing configuration
    #[prop(optional)]
    path_config: Option<PathMorphConfig>,
    /// Whether to start animation automatically
    #[prop(optional)]
    auto_start: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let config = path_config.unwrap_or_default();
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let auto_start = auto_start.unwrap_or(false);

    // Create path morphing manager
    let (path_manager, set_path_manager) = signal(None::<PathMorphManager>);
    let (current_path_string, set_current_path_string) = signal(String::new());

    // Initialize path manager
    Effect::new(move |_| {
        if let (Some(start), Some(end)) = (start_path.clone(), end_path.clone()) {
            let mut manager = PathMorphManager::new(config.clone());

            if let Err(e) = manager.set_start_path(&start) {
                console_error!("Failed to set start path: {}", e);
                return;
            }

            if let Err(e) = manager.set_end_path(&end) {
                console_error!("Failed to set end path: {}", e);
                return;
            }

            set_path_manager.set(Some(manager));

            if auto_start {
                // Start animation automatically
                if let Some(mut mgr) = path_manager.get() {
                    if let Err(e) = mgr.start_animation() {
                        console_error!("Failed to start path animation: {}", e);
                    } else {
                        set_path_manager.set(Some(mgr));
                    }
                }
            }
        }
    });

    // Update path string when manager changes
    Effect::new(move |_| {
        if let Some(manager) = path_manager.get() {
            if let Some(path_data) = manager.get_current_path() {
                set_current_path_string.set(path_data.raw.clone());
            }
        }
    });

    view! {
        <div
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            node_ref=node_ref.unwrap_or_else(NodeRef::new)
        >
            <svg width="100%" height="100%" viewBox="0 0 100 100">
                <path
                    d=current_path_string
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                />
            </svg>
            {children()}
        </div>
    }
}

// ============================================================================
// SHARED ELEMENT TRANSITIONS
// ============================================================================

/// Shared element configuration
#[derive(Clone, Debug)]
pub struct SharedElementConfig {
    /// Unique identifier for the shared element
    pub id: String,
    /// Animation duration in seconds
    pub duration: f64,
    /// Easing function
    pub easing: Easing,
    /// Whether to animate position
    pub animate_position: bool,
    /// Whether to animate size
    pub animate_size: bool,
    /// Whether to animate opacity
    pub animate_opacity: bool,
    /// Z-index for the shared element
    pub z_index: i32,
}

impl Default for SharedElementConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            duration: 0.5,
            easing: Easing::EaseInOut,
            animate_position: true,
            animate_size: true,
            animate_opacity: true,
            z_index: 1000,
        }
    }
}

/// Shared element state
#[derive(Clone, Debug, PartialEq)]
pub enum SharedElementState {
    /// Element is in its original position
    Original,
    /// Element is transitioning
    Transitioning,
    /// Element is in its shared position
    Shared,
    /// Element is returning to original
    Returning,
}

/// Shared element manager
#[derive(Clone)]
pub struct SharedElementManager {
    config: SharedElementConfig,
    state: SharedElementState,
    original_bounds: Option<ElementBounds>,
    shared_bounds: Option<ElementBounds>,
    current_bounds: Option<ElementBounds>,
    progress: f64,
    is_animating: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElementBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub opacity: f64,
}

impl SharedElementManager {
    /// Create a new shared element manager
    pub fn new(config: SharedElementConfig) -> Self {
        Self {
            config,
            state: SharedElementState::Original,
            original_bounds: None,
            shared_bounds: None,
            current_bounds: None,
            progress: 0.0,
            is_animating: false,
        }
    }

    /// Set original element bounds
    pub fn set_original_bounds(&mut self, bounds: ElementBounds) {
        self.original_bounds = Some(bounds);
    }

    /// Set shared element bounds
    pub fn set_shared_bounds(&mut self, bounds: ElementBounds) {
        self.shared_bounds = Some(bounds);
    }

    /// Start transition to shared state
    pub fn start_shared_transition(&mut self) -> std::result::Result<(), String> {
        if self.original_bounds.is_none() || self.shared_bounds.is_none() {
            return Err("Both original and shared bounds must be set".to_string());
        }

        self.state = SharedElementState::Transitioning;
        self.is_animating = true;
        self.progress = 0.0;

        Ok(())
    }

    /// Start transition back to original state
    pub fn start_return_transition(&mut self) -> std::result::Result<(), String> {
        if self.original_bounds.is_none() || self.shared_bounds.is_none() {
            return Err("Both original and shared bounds must be set".to_string());
        }

        self.state = SharedElementState::Returning;
        self.is_animating = true;
        self.progress = 0.0;

        Ok(())
    }

    /// Get current element bounds
    pub fn get_current_bounds(&self) -> Option<&ElementBounds> {
        self.current_bounds.as_ref()
    }

    /// Get current state
    pub fn get_state(&self) -> &SharedElementState {
        &self.state
    }

    /// Check if currently animating
    pub fn is_animating(&self) -> bool {
        self.is_animating
    }

    /// Update animation progress
    pub fn update_progress(&mut self, delta_time: f64) {
        if !self.is_animating {
            return;
        }

        self.progress += delta_time / self.config.duration;

        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.is_animating = false;

            match self.state {
                SharedElementState::Transitioning => {
                    self.state = SharedElementState::Shared;
                }
                SharedElementState::Returning => {
                    self.state = SharedElementState::Original;
                }
                _ => {}
            }
        }

        // Update current bounds
        self.update_current_bounds();
    }

    /// Update current bounds based on progress
    fn update_current_bounds(&mut self) {
        if let (Some(original), Some(shared)) = (&self.original_bounds, &self.shared_bounds) {
            let eased_progress = self.apply_easing(self.progress);

            let current = match self.state {
                SharedElementState::Transitioning => ElementBounds {
                    x: original.x + (shared.x - original.x) * eased_progress,
                    y: original.y + (shared.y - original.y) * eased_progress,
                    width: original.width + (shared.width - original.width) * eased_progress,
                    height: original.height + (shared.height - original.height) * eased_progress,
                    opacity: original.opacity
                        + (shared.opacity - original.opacity) * eased_progress,
                },
                SharedElementState::Returning => ElementBounds {
                    x: shared.x + (original.x - shared.x) * eased_progress,
                    y: shared.y + (original.y - shared.y) * eased_progress,
                    width: shared.width + (original.width - shared.width) * eased_progress,
                    height: shared.height + (original.height - shared.height) * eased_progress,
                    opacity: shared.opacity + (original.opacity - shared.opacity) * eased_progress,
                },
                _ => original.clone(),
            };

            self.current_bounds = Some(current);
        }
    }

    /// Apply easing function to progress
    fn apply_easing(&self, progress: f64) -> f64 {
        match self.config.easing {
            Easing::Linear => progress,
            Easing::EaseIn => progress * progress,
            Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            _ => progress, // Handle other easing types with linear fallback
        }
    }
}

/// Shared Element MotionDiv component
#[component]
pub fn SharedElementMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Shared element configuration
    #[prop(optional)]
    shared_config: Option<SharedElementConfig>,
    /// Whether this element should be in shared state
    #[prop(optional)]
    is_shared: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let config = shared_config.unwrap_or_default();
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let is_shared = is_shared.unwrap_or(false);

    // Create shared element manager
    let (shared_manager, set_shared_manager) = signal(None::<SharedElementManager>);
    let (current_style, set_current_style) = signal(String::new());

    // Initialize shared element manager
    Effect::new(move |_| {
        if let Some(element) = node_ref.get() {
            let mut manager = SharedElementManager::new(config.clone());

            // Get original bounds
            let rect = element.get_bounding_client_rect();
            let original_bounds = ElementBounds {
                x: rect.left(),
                y: rect.top(),
                width: rect.width(),
                height: rect.height(),
                opacity: 1.0,
            };

            manager.set_original_bounds(original_bounds);
            set_shared_manager.set(Some(manager));
        }
    });

    // Handle shared state changes
    Effect::new(move |_| {
        if let Some(mut manager) = shared_manager.get() {
            if is_shared {
                // Set shared bounds (in real implementation, get from target element)
                let shared_bounds = ElementBounds {
                    x: 100.0,
                    y: 100.0,
                    width: 200.0,
                    height: 200.0,
                    opacity: 0.8,
                };

                manager.set_shared_bounds(shared_bounds);

                if let Err(e) = manager.start_shared_transition() {
                    console_error!("Failed to start shared transition: {}", e);
                } else {
                    set_shared_manager.set(Some(manager));
                }
            } else {
                if let Err(e) = manager.start_return_transition() {
                    console_error!("Failed to start return transition: {}", e);
                } else {
                    set_shared_manager.set(Some(manager));
                }
            }
        }
    });

    // Update style based on current bounds
    Effect::new(move |_| {
        if let Some(manager) = shared_manager.get() {
            if let Some(bounds) = manager.get_current_bounds() {
                let style = format!(
                    "position: absolute; left: {}px; top: {}px; width: {}px; height: {}px; opacity: {}; z-index: {};",
                    bounds.x, bounds.y, bounds.width, bounds.height, bounds.opacity, config.z_index
                );
                set_current_style.set(style);
            }
        }
    });

    view! {
        <div
            class=class.unwrap_or_default()
            style=current_style
            node_ref=node_ref.unwrap_or_else(NodeRef::new)
        >
            {children()}
        </div>
    }
}

// ============================================================================
// ANIMATION ORCHESTRATION
// ============================================================================

/// Animation orchestration configuration
#[derive(Clone, Debug)]
pub struct OrchestrationConfig {
    /// Animation sequence
    pub sequence: Vec<AnimationStep>,
    /// Whether to loop the sequence
    pub loop_sequence: bool,
    /// Delay between steps
    pub step_delay: f64,
    /// Whether to reverse the sequence on loop
    pub reverse_on_loop: bool,
}

#[derive(Clone, Debug)]
pub struct AnimationStep {
    /// Step identifier
    pub id: String,
    /// Animation properties
    pub properties: HashMap<String, AnimationValue>,
    /// Step duration
    pub duration: f64,
    /// Step easing
    pub easing: Easing,
    /// Delay before this step
    pub delay: f64,
}

impl Default for OrchestrationConfig {
    fn default() -> Self {
        Self {
            sequence: Vec::new(),
            loop_sequence: false,
            step_delay: 0.1,
            reverse_on_loop: false,
        }
    }
}

/// Animation orchestration manager
#[derive(Clone)]
pub struct OrchestrationManager {
    config: OrchestrationConfig,
    current_step: usize,
    current_progress: f64,
    is_playing: bool,
    is_reversed: bool,
    current_properties: HashMap<String, AnimationValue>,
}

impl OrchestrationManager {
    /// Create a new orchestration manager
    pub fn new(config: OrchestrationConfig) -> Self {
        Self {
            config,
            current_step: 0,
            current_progress: 0.0,
            is_playing: false,
            is_reversed: false,
            current_properties: HashMap::new(),
        }
    }

    /// Start the orchestration
    pub fn start(&mut self) {
        self.is_playing = true;
        self.current_step = 0;
        self.current_progress = 0.0;
        self.is_reversed = false;
    }

    /// Stop the orchestration
    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    /// Pause the orchestration
    pub fn pause(&mut self) {
        self.is_playing = false;
    }

    /// Resume the orchestration
    pub fn resume(&mut self) {
        self.is_playing = true;
    }

    /// Update orchestration progress
    pub fn update(&mut self, delta_time: f64) {
        if !self.is_playing || self.config.sequence.is_empty() {
            return;
        }

        let current_step = &self.config.sequence[self.current_step];
        self.current_progress += delta_time / current_step.duration;

        if self.current_progress >= 1.0 {
            self.current_progress = 1.0;
            self.next_step();
        }

        // Update current properties
        self.update_current_properties();
    }

    /// Move to next step
    fn next_step(&mut self) {
        if self.is_reversed {
            if self.current_step > 0 {
                self.current_step -= 1;
            } else {
                // Reached beginning
                if self.config.loop_sequence {
                    if self.config.reverse_on_loop {
                        self.is_reversed = false;
                        self.current_step = 0;
                    } else {
                        self.current_step = self.config.sequence.len() - 1;
                    }
                } else {
                    self.is_playing = false;
                }
            }
        } else {
            if self.current_step < self.config.sequence.len() - 1 {
                self.current_step += 1;
            } else {
                // Reached end
                if self.config.loop_sequence {
                    if self.config.reverse_on_loop {
                        self.is_reversed = true;
                        self.current_step = self.config.sequence.len() - 1;
                    } else {
                        self.current_step = 0;
                    }
                } else {
                    self.is_playing = false;
                }
            }
        }

        self.current_progress = 0.0;
    }

    /// Update current properties based on current step and progress
    fn update_current_properties(&mut self) {
        if self.current_step >= self.config.sequence.len() {
            return;
        }

        let step = &self.config.sequence[self.current_step];
        let eased_progress = self.apply_easing(self.current_progress, step.easing.clone());

        self.current_properties = step.properties.clone();

        // Apply progress to animated properties
        for (_key, value) in &mut self.current_properties {
            match value {
                AnimationValue::Number(num) => {
                    *num *= eased_progress;
                }
                AnimationValue::Pixels(pixels) => {
                    *pixels *= eased_progress;
                }
                _ => {}
            }
        }
    }

    /// Apply easing function to progress
    fn apply_easing(&self, progress: f64, easing: Easing) -> f64 {
        match easing {
            Easing::Linear => progress,
            Easing::EaseIn => progress * progress,
            Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            Easing::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            _ => progress, // Handle other easing types with linear fallback
        }
    }

    /// Get current animation properties
    pub fn get_current_properties(&self) -> &HashMap<String, AnimationValue> {
        &self.current_properties
    }

    /// Get current step index
    pub fn get_current_step(&self) -> usize {
        self.current_step
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_current_progress(&self) -> f64 {
        self.current_progress
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing
    }
}

/// Orchestrated MotionDiv component
#[component]
pub fn OrchestratedMotionDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Node reference for the element
    #[prop(optional)]
    node_ref: Option<NodeRef<leptos::html::Div>>,
    /// Orchestration configuration
    #[prop(optional)]
    orchestration_config: Option<OrchestrationConfig>,
    /// Whether to start automatically
    #[prop(optional)]
    auto_start: Option<bool>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let config = orchestration_config.unwrap_or_default();
    let node_ref = node_ref.unwrap_or_else(|| NodeRef::new());
    let auto_start = auto_start.unwrap_or(false);

    // Create orchestration manager
    let (orchestration_manager, set_orchestration_manager) = signal(None::<OrchestrationManager>);
    let (current_properties, set_current_properties) = signal(HashMap::new());

    // Initialize orchestration manager
    Effect::new(move |_| {
        let mut manager = OrchestrationManager::new(config.clone());

        if auto_start {
            manager.start();
        }

        set_orchestration_manager.set(Some(manager));
    });

    // Update properties when manager changes
    Effect::new(move |_| {
        if let Some(manager) = orchestration_manager.get() {
            set_current_properties.set(manager.get_current_properties().clone());
        }
    });

    // Create reactive animation target
    let animation_target = move || current_properties.get();

    view! {
        <ReactiveMotionDiv
            class=class.unwrap_or_default()
            style=style.unwrap_or_default()
            node_ref=node_ref.unwrap_or_else(NodeRef::new)
            animate=reactive_animate(animation_target)
        >
            {children()}
        </ReactiveMotionDiv>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // ============================================================================
    // SVG PATH MORPHING TESTS
    // ============================================================================

    #[wasm_bindgen_test]
    fn test_path_morph_config_default() {
        let config = PathMorphConfig::default();

        assert_eq!(config.duration, 1.0);
        assert_eq!(config.easing, Easing::EaseInOut);
        assert!(config.preserve_aspect_ratio);
        assert_eq!(config.interpolation_points, 100);
        assert!(config.use_cubic_bezier);
        assert_eq!(config.precision, 2);
    }

    #[wasm_bindgen_test]
    fn test_path_parsing() {
        let path_string = "M10,10 L20,20 C30,30 40,40 50,50";
        let path_data = PathMorphManager::parse_path(path_string).unwrap();

        assert_eq!(path_data.raw, path_string);
        assert!(!path_data.commands.is_empty());
        assert!(path_data.bounds.width > 0.0);
        assert!(path_data.bounds.height > 0.0);
    }

    #[wasm_bindgen_test]
    fn test_path_morph_manager_creation() {
        let config = PathMorphConfig::default();
        let manager = PathMorphManager::new(config);

        assert!(!manager.is_animating());
        assert_eq!(manager.get_progress(), 0.0);
        assert!(manager.get_current_path().is_none());
    }

    #[wasm_bindgen_test]
    fn test_path_morph_animation() {
        let config = PathMorphConfig::default();
        let mut manager = PathMorphManager::new(config);

        manager.set_start_path("M10,10 L20,20").unwrap();
        manager.set_end_path("M30,30 L40,40").unwrap();

        assert!(manager.start_animation().is_ok());
        assert!(manager.is_animating());
    }

    #[wasm_bindgen_test]
    fn test_path_motion_div_component() {
        mount_to_body(|| {
            view! {
                <PathMotionDiv
                    class="test-path".to_string()
                    start_path="M10,10 L20,20".to_string()
                    end_path="M30,30 L40,40".to_string()
                    auto_start=true
                >
                    "Path Animation Test"
                </PathMotionDiv>
            }
        });
    }

    // ============================================================================
    // SHARED ELEMENT TESTS
    // ============================================================================

    #[wasm_bindgen_test]
    fn test_shared_element_config_default() {
        let config = SharedElementConfig::default();

        assert_eq!(config.duration, 0.5);
        assert_eq!(config.easing, Easing::EaseInOut);
        assert!(config.animate_position);
        assert!(config.animate_size);
        assert!(config.animate_opacity);
        assert_eq!(config.z_index, 1000);
    }

    #[wasm_bindgen_test]
    fn test_shared_element_manager_creation() {
        let config = SharedElementConfig::default();
        let manager = SharedElementManager::new(config);

        assert_eq!(manager.get_state(), &SharedElementState::Original);
        assert!(!manager.is_animating());
        assert!(manager.get_current_bounds().is_none());
    }

    #[wasm_bindgen_test]
    fn test_shared_element_transition() {
        let config = SharedElementConfig::default();
        let mut manager = SharedElementManager::new(config);

        let original_bounds = ElementBounds {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
            opacity: 1.0,
        };

        let shared_bounds = ElementBounds {
            x: 100.0,
            y: 100.0,
            width: 200.0,
            height: 200.0,
            opacity: 0.8,
        };

        manager.set_original_bounds(original_bounds);
        manager.set_shared_bounds(shared_bounds);

        assert!(manager.start_shared_transition().is_ok());
        assert_eq!(manager.get_state(), &SharedElementState::Transitioning);
        assert!(manager.is_animating());
    }

    #[wasm_bindgen_test]
    fn test_shared_element_motion_div_component() {
        mount_to_body(|| {
            view! {
                <SharedElementMotionDiv
                    class="test-shared".to_string()
                    is_shared=false
                >
                    "Shared Element Test"
                </SharedElementMotionDiv>
            }
        });
    }

    // ============================================================================
    // ORCHESTRATION TESTS
    // ============================================================================

    #[wasm_bindgen_test]
    fn test_orchestration_config_default() {
        let config = OrchestrationConfig::default();

        assert!(config.sequence.is_empty());
        assert!(!config.loop_sequence);
        assert_eq!(config.step_delay, 0.1);
        assert!(!config.reverse_on_loop);
    }

    #[wasm_bindgen_test]
    fn test_orchestration_manager_creation() {
        let config = OrchestrationConfig::default();
        let manager = OrchestrationManager::new(config);

        assert!(!manager.is_playing());
        assert_eq!(manager.get_current_step(), 0);
        assert_eq!(manager.get_current_progress(), 0.0);
    }

    #[wasm_bindgen_test]
    fn test_orchestration_sequence() {
        let mut sequence = Vec::new();
        sequence.push(AnimationStep {
            id: "step1".to_string(),
            properties: HashMap::from([("x".to_string(), AnimationValue::Pixels(100.0))]),
            duration: 1.0,
            easing: Easing::EaseOut,
            delay: 0.0,
        });

        let config = OrchestrationConfig {
            sequence,
            loop_sequence: false,
            step_delay: 0.1,
            reverse_on_loop: false,
        };

        let mut manager = OrchestrationManager::new(config);
        manager.start();

        assert!(manager.is_playing());
        assert_eq!(manager.get_current_step(), 0);
    }

    #[wasm_bindgen_test]
    fn test_orchestrated_motion_div_component() {
        let mut sequence = Vec::new();
        sequence.push(AnimationStep {
            id: "step1".to_string(),
            properties: HashMap::from([("x".to_string(), AnimationValue::Pixels(100.0))]),
            duration: 1.0,
            easing: Easing::EaseOut,
            delay: 0.0,
        });

        let config = OrchestrationConfig {
            sequence,
            loop_sequence: false,
            step_delay: 0.1,
            reverse_on_loop: false,
        };

        mount_to_body(|| {
            view! {
                <OrchestratedMotionDiv
                    class="test-orchestrated".to_string()
                    orchestration_config=config
                    auto_start=true
                >
                    "Orchestrated Animation Test"
                </OrchestratedMotionDiv>
            }
        });
    }

    // ============================================================================
    // INTEGRATION TESTS
    // ============================================================================

    #[wasm_bindgen_test]
    fn test_advanced_features_integration() {
        mount_to_body(|| {
            view! {
                <div>
                    <PathMotionDiv
                        class="path-test".to_string()
                        start_path="M10,10 L20,20".to_string()
                        end_path="M30,30 L40,40".to_string()
                    >
                        "Path Animation"
                    </PathMotionDiv>

                    <SharedElementMotionDiv
                        class="shared-test".to_string()
                        is_shared=false
                    >
                        "Shared Element"
                    </SharedElementMotionDiv>

                    <OrchestratedMotionDiv
                        class="orchestrated-test".to_string()
                    >
                        "Orchestrated Animation"
                    </OrchestratedMotionDiv>
                </div>
            }
        });
    }

    #[wasm_bindgen_test]
    fn test_complex_orchestration_sequence() {
        let mut sequence = Vec::new();

        // Step 1: Move right
        sequence.push(AnimationStep {
            id: "move_right".to_string(),
            properties: HashMap::from([("x".to_string(), AnimationValue::Pixels(100.0))]),
            duration: 0.5,
            easing: Easing::EaseOut,
            delay: 0.0,
        });

        // Step 2: Scale up
        sequence.push(AnimationStep {
            id: "scale_up".to_string(),
            properties: HashMap::from([("scale".to_string(), AnimationValue::Number(1.5))]),
            duration: 0.3,
            easing: Easing::EaseInOut,
            delay: 0.1,
        });

        // Step 3: Rotate
        sequence.push(AnimationStep {
            id: "rotate".to_string(),
            properties: HashMap::from([("rotate".to_string(), AnimationValue::Number(180.0))]),
            duration: 0.4,
            easing: Easing::EaseIn,
            delay: 0.1,
        });

        let config = OrchestrationConfig {
            sequence,
            loop_sequence: true,
            step_delay: 0.2,
            reverse_on_loop: true,
        };

        mount_to_body(|| {
            view! {
                <OrchestratedMotionDiv
                    class="complex-orchestration".to_string()
                    orchestration_config=config
                    auto_start=true
                >
                    "Complex Orchestration"
                </OrchestratedMotionDiv>
            }
        });
    }
}
