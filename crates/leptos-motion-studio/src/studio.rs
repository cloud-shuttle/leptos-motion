//! Main Motion Studio component and API

use crate::{Result, StudioError, project::*, timeline::*, transforms::*};
use leptos::*;
use leptos::prelude::{ElementChild, NodeRefAttribute, StyleAttribute, OnAttribute, create_signal, provide_context, create_effect, expect_context, create_node_ref, ReadSignal, WriteSignal, Callback, event_target_value, Get, Set};
use leptos::attr::global::ClassAttribute;
use std::collections::HashMap;
use uuid::Uuid;

/// Main Motion Studio component for visual animation editing
#[component]
pub fn MotionStudio(
    /// Project to edit
    #[prop(optional)]
    project: Option<StudioProject>,

    /// Studio configuration
    #[prop(default = StudioConfig::default())]
    config: StudioConfig,

    /// Callback when project changes
    #[prop(optional)]
    on_project_change: Option<Callback<StudioProject>>,

    /// Studio theme
    #[prop(default = StudioTheme::Dark)]
    theme: StudioTheme,
) -> impl IntoView {
    let (current_project, set_current_project) =
        create_signal(project.unwrap_or_else(|| StudioProject::new("Untitled Project")));

    let (selected_animation, set_selected_animation) = create_signal(None::<Uuid>);
    let (timeline_state, set_timeline_state) = create_signal(TimelineState::default());
    let (canvas_state, set_canvas_state) = create_signal(CanvasState::default());
    let (studio_mode, set_studio_mode) = create_signal(StudioMode::Timeline);

    // Create studio context
    let studio_context = StudioContext::new(
        current_project,
        set_current_project,
        selected_animation,
        set_selected_animation,
        timeline_state,
        set_timeline_state,
        canvas_state,
        set_canvas_state,
        studio_mode,
        set_studio_mode,
        config.clone(),
    );

    provide_context(studio_context);

    // Handle project changes
    create_effect(move |_| {
        // Temporarily disabled until callback API is clarified
        // if let Some(callback) = on_project_change {
        //     callback(current_project.get());
        // }
    });

    view! {
        <div class=move || format!("motion-studio motion-studio--{}", theme.css_class())>
            <StudioHeader />
            <div class="motion-studio__main">
                <StudioSidebar />
                <div class="motion-studio__workspace">
                    <StudioCanvas />
                    <StudioTimeline />
                </div>
                <StudioProperties />
            </div>
            <StudioStatusBar />
        </div>
    }
}

/// Studio configuration
#[derive(Clone, Debug, PartialEq)]
pub struct StudioConfig {
    /// Enable WebGL acceleration
    pub webgl_enabled: bool,
    /// Enable 3D transforms
    pub transforms_3d: bool,
    /// Enable SVG morphing
    pub svg_morphing: bool,
    /// Animation pool size
    pub animation_pool_size: usize,
    /// Performance monitoring
    pub performance_monitoring: bool,
    /// Auto-save interval in seconds
    pub auto_save_interval: Option<u32>,
}

impl Default for StudioConfig {
    fn default() -> Self {
        Self {
            webgl_enabled: true,
            transforms_3d: true,
            svg_morphing: true,
            animation_pool_size: 50,
            performance_monitoring: true,
            auto_save_interval: Some(30),
        }
    }
}

/// Studio visual themes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StudioTheme {
    Light,
    Dark,
    Auto,
}

impl StudioTheme {
    fn css_class(&self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::Auto => "auto",
        }
    }
}

/// Studio operating modes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StudioMode {
    Timeline,
    Transform3D,
    Morphing,
    Code,
    Preview,
}

/// Timeline state management
#[derive(Clone, Debug, PartialEq)]
pub struct TimelineState {
    /// Current playhead time
    pub current_time: f32,
    /// Timeline zoom level
    pub zoom: f32,
    /// Timeline scroll position
    pub scroll_x: f32,
    /// Selected keyframes
    pub selected_keyframes: Vec<Uuid>,
    /// Timeline duration
    pub duration: f32,
    /// Playback state
    pub is_playing: bool,
    /// Loop enabled
    pub loop_enabled: bool,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            current_time: 0.0,
            zoom: 1.0,
            scroll_x: 0.0,
            selected_keyframes: Vec::new(),
            duration: 10.0,
            is_playing: false,
            loop_enabled: false,
        }
    }
}

/// Canvas state for 3D viewport
#[derive(Clone, Debug, PartialEq)]
pub struct CanvasState {
    /// Camera position
    pub camera_position: glam::Vec3,
    /// Camera rotation
    pub camera_rotation: glam::Quat,
    /// Camera zoom/scale
    pub camera_zoom: f32,
    /// Grid visibility
    pub show_grid: bool,
    /// Axis visibility
    pub show_axes: bool,
    /// Wireframe mode
    pub wireframe_mode: bool,
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            camera_position: glam::Vec3::new(0.0, 0.0, 10.0),
            camera_rotation: glam::Quat::IDENTITY,
            camera_zoom: 1.0,
            show_grid: true,
            show_axes: true,
            wireframe_mode: false,
        }
    }
}

/// Studio context for components communication
#[derive(Clone, Debug)]
pub struct StudioContext {
    pub current_project: ReadSignal<StudioProject>,
    pub set_current_project: WriteSignal<StudioProject>,
    pub selected_animation: ReadSignal<Option<Uuid>>,
    pub set_selected_animation: WriteSignal<Option<Uuid>>,
    pub timeline_state: ReadSignal<TimelineState>,
    pub set_timeline_state: WriteSignal<TimelineState>,
    pub canvas_state: ReadSignal<CanvasState>,
    pub set_canvas_state: WriteSignal<CanvasState>,
    pub studio_mode: ReadSignal<StudioMode>,
    pub set_studio_mode: WriteSignal<StudioMode>,
    pub config: StudioConfig,
}

impl StudioContext {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        current_project: ReadSignal<StudioProject>,
        set_current_project: WriteSignal<StudioProject>,
        selected_animation: ReadSignal<Option<Uuid>>,
        set_selected_animation: WriteSignal<Option<Uuid>>,
        timeline_state: ReadSignal<TimelineState>,
        set_timeline_state: WriteSignal<TimelineState>,
        canvas_state: ReadSignal<CanvasState>,
        set_canvas_state: WriteSignal<CanvasState>,
        studio_mode: ReadSignal<StudioMode>,
        set_studio_mode: WriteSignal<StudioMode>,
        config: StudioConfig,
    ) -> Self {
        Self {
            current_project,
            set_current_project,
            selected_animation,
            set_selected_animation,
            timeline_state,
            set_timeline_state,
            canvas_state,
            set_canvas_state,
            studio_mode,
            set_studio_mode,
            config,
        }
    }
}

/// Studio header component
#[component]
fn StudioHeader() -> impl IntoView {
    let context = expect_context::<StudioContext>();

    view! {
        <header class="motion-studio__header">
            <div class="motion-studio__logo">
                <h1>"Motion Studio"</h1>
            </div>
            <nav class="motion-studio__nav">
                <button
                    class="motion-studio__nav-item"
                    class:active=move || context.studio_mode.get() == StudioMode::Timeline
                    on:click=move |_| context.set_studio_mode.set(StudioMode::Timeline)
                >
                    "Timeline"
                </button>
                <button
                    class="motion-studio__nav-item"
                    class:active=move || context.studio_mode.get() == StudioMode::Transform3D
                    on:click=move |_| context.set_studio_mode.set(StudioMode::Transform3D)
                >
                    "3D"
                </button>
                <button
                    class="motion-studio__nav-item"
                    class:active=move || context.studio_mode.get() == StudioMode::Morphing
                    on:click=move |_| context.set_studio_mode.set(StudioMode::Morphing)
                >
                    "Morphing"
                </button>
                <button
                    class="motion-studio__nav-item"
                    class:active=move || context.studio_mode.get() == StudioMode::Code
                    on:click=move |_| context.set_studio_mode.set(StudioMode::Code)
                >
                    "Code"
                </button>
                <button
                    class="motion-studio__nav-item"
                    class:active=move || context.studio_mode.get() == StudioMode::Preview
                    on:click=move |_| context.set_studio_mode.set(StudioMode::Preview)
                >
                    "Preview"
                </button>
            </nav>
            <div class="motion-studio__actions">
                <button class="motion-studio__button motion-studio__button--primary">
                    "Export"
                </button>
            </div>
        </header>
    }
}

/// Studio sidebar component
#[component]
fn StudioSidebar() -> impl IntoView {
    let context = expect_context::<StudioContext>();

    view! {
        <aside class="motion-studio__sidebar">
            <div class="motion-studio__section">
                <h3>"Animations"</h3>
                <AnimationsList />
            </div>
            <div class="motion-studio__section">
                <h3>"Assets"</h3>
                <AssetsList />
            </div>
        </aside>
    }
}

/// Studio canvas component
#[component]
fn StudioCanvas() -> impl IntoView {
    let context = expect_context::<StudioContext>();
    let canvas_ref = create_node_ref::<leptos::html::Canvas>();

    // Initialize WebGL context when canvas mounts
    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            // Initialize WebGL renderer
            if context.config.webgl_enabled {
                // let _ = crate::webgl::WebGLRenderer::initialize(&canvas); // Temporarily disabled
            }
        }
    });

    view! {
        <div class="motion-studio__canvas-container">
            <canvas
                node_ref=canvas_ref
                class="motion-studio__canvas"
                width="800"
                height="600"
            ></canvas>
            <StudioViewportControls />
        </div>
    }
}

/// Studio timeline component
#[component]
fn StudioTimeline() -> impl IntoView {
    let context = expect_context::<StudioContext>();

    view! {
        <div class="motion-studio__timeline-container">
            <TimelineControls />
            <TimelineEditor />
        </div>
    }
}

/// Studio properties panel
#[component]
fn StudioProperties() -> impl IntoView {
    let context = expect_context::<StudioContext>();

    view! {
        <aside class="motion-studio__properties">
            <div class="motion-studio__section">
                <h3>"Properties"</h3>
                <PropertiesEditor />
            </div>
        </aside>
    }
}

/// Studio status bar
#[component]
fn StudioStatusBar() -> impl IntoView {
    let context = expect_context::<StudioContext>();

    view! {
        <footer class="motion-studio__statusbar">
            <div class="motion-studio__status-left">
                <span>"Ready"</span>
            </div>
            <div class="motion-studio__status-right">
                <span>"FPS: 60"</span>
                <span>"Memory: 45MB"</span>
            </div>
        </footer>
    }
}

// Placeholder components - to be implemented
#[component]
fn AnimationsList() -> impl IntoView {
    view! { <div>"Animations list"</div> }
}

#[component]
fn AssetsList() -> impl IntoView {
    view! { <div>"Assets list"</div> }
}

#[component]
fn StudioViewportControls() -> impl IntoView {
    view! { <div class="motion-studio__viewport-controls">"Viewport controls"</div> }
}

#[component]
fn TimelineControls() -> impl IntoView {
    view! { <div class="motion-studio__timeline-controls">"Timeline controls"</div> }
}

#[component]
fn PropertiesEditor() -> impl IntoView {
    view! { <div>"Properties editor"</div> }
}

/// Core studio API for programmatic control
#[derive(Clone)]
pub struct MotionStudioAPI {
    context: StudioContext,
}

impl MotionStudioAPI {
    /// Create new studio API instance
    pub fn new() -> Result<Self> {
        // This would be called from within a Leptos context
        Err(StudioError::ProjectError(
            "Studio not initialized".to_string(),
        ))
    }

    /// Get current project
    pub fn current_project(&self) -> StudioProject {
        self.context.current_project.get()
    }

    /// Set current project
    pub fn set_project(&self, project: StudioProject) {
        self.context.set_current_project.set(project);
    }

    /// Create new animation in current project
    pub fn create_animation(&self, name: &str) -> Result<Uuid> {
        let mut project = self.current_project();
        let animation_id = project.add_animation(name);
        self.set_project(project);
        Ok(animation_id)
    }

    /// Delete animation from current project
    pub fn delete_animation(&self, animation_id: Uuid) -> Result<()> {
        let mut project = self.current_project();
        project.remove_animation(animation_id)?;
        self.set_project(project);
        Ok(())
    }

    /// Play timeline
    pub fn play(&self) {
        let mut state = self.context.timeline_state.get();
        state.is_playing = true;
        self.context.set_timeline_state.set(state);
    }

    /// Pause timeline
    pub fn pause(&self) {
        let mut state = self.context.timeline_state.get();
        state.is_playing = false;
        self.context.set_timeline_state.set(state);
    }

    /// Seek to time
    pub fn seek(&self, time: f32) {
        let mut state = self.context.timeline_state.get();
        state.current_time = time.clamp(0.0, state.duration);
        self.context.set_timeline_state.set(state);
    }

    /// Set timeline duration
    pub fn set_duration(&self, duration: f32) {
        let mut state = self.context.timeline_state.get();
        state.duration = duration.max(0.1);
        self.context.set_timeline_state.set(state);
    }
}

impl Default for MotionStudioAPI {
    fn default() -> Self {
        Self::new().expect("Failed to create studio API")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_studio_config_default() {
        let config = StudioConfig::default();
        assert!(config.webgl_enabled);
        assert!(config.transforms_3d);
        assert!(config.svg_morphing);
        assert_eq!(config.animation_pool_size, 50);
        assert!(config.performance_monitoring);
        assert_eq!(config.auto_save_interval, Some(30));
    }

    #[test]
    fn test_timeline_state_default() {
        let state = TimelineState::default();
        assert_eq!(state.current_time, 0.0);
        assert_eq!(state.zoom, 1.0);
        assert_eq!(state.duration, 10.0);
        assert!(!state.is_playing);
        assert!(!state.loop_enabled);
    }

    #[test]
    fn test_canvas_state_default() {
        let state = CanvasState::default();
        assert_eq!(state.camera_position, glam::Vec3::new(0.0, 0.0, 10.0));
        assert_eq!(state.camera_rotation, glam::Quat::IDENTITY);
        assert_eq!(state.camera_zoom, 1.0);
        assert!(state.show_grid);
        assert!(state.show_axes);
        assert!(!state.wireframe_mode);
    }

    #[test]
    fn test_studio_themes() {
        assert_eq!(StudioTheme::Light.css_class(), "light");
        assert_eq!(StudioTheme::Dark.css_class(), "dark");
        assert_eq!(StudioTheme::Auto.css_class(), "auto");
    }
}
