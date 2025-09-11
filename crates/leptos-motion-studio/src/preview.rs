//! Live preview functionality for Motion Studio

use crate::{
    // webgl::WebGLRenderer, // Temporarily disabled
    Result,
    StudioError,
    pooling::AnimationPool,
    project::StudioProject,
    timeline::{AnimationProperty, AnimationValue, Timeline3D},
    transforms::Transform3D,
};
use leptos::attr::global::ClassAttribute;
use leptos::prelude::{
    ElementChild, Get, NodeRefAttribute, OnAttribute, Set, StyleAttribute, create_effect,
    create_node_ref, create_signal,
};
use leptos::*;
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

/// Live preview renderer for animations
#[derive(Debug)]
pub struct LivePreview {
    /// Preview ID
    pub id: Uuid,
    /// Target element for preview
    pub target_element: Option<HtmlElement>,
    /// Current project
    pub project: Option<StudioProject>,
    /// Active animations
    pub active_animations: HashMap<Uuid, PreviewAnimation>,
    /// Preview settings
    pub settings: PreviewSettings,
    /// WebGL renderer (optional)
    // pub webgl_renderer: Option<WebGLRenderer>, // Temporarily disabled
    /// Animation pool
    pub animation_pool: AnimationPool,
    /// Performance metrics
    pub metrics: PreviewMetrics,
    /// Animation frame ID
    pub animation_frame_id: Option<i32>,
    /// Last frame time
    pub last_frame_time: f64,
}

impl LivePreview {
    /// Create new live preview
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            target_element: None,
            project: None,
            active_animations: HashMap::new(),
            settings: PreviewSettings::default(),
            animation_pool: AnimationPool::new(100),
            metrics: PreviewMetrics::default(),
            animation_frame_id: None,
            last_frame_time: 0.0,
        }
    }

    /// Set target element for preview
    pub fn set_target(&mut self, element: HtmlElement) -> Result<()> {
        self.target_element = Some(element);

        // Initialize WebGL renderer if enabled
        if self.settings.webgl_enabled {
            if let Some(canvas) = self
                .target_element
                .as_ref()
                .and_then(|el| el.query_selector("canvas").ok().flatten())
                .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            {
                // self.webgl_renderer = WebGLRenderer::initialize(&canvas).ok(); // Temporarily disabled
            }
        }

        Ok(())
    }

    /// Set project for preview
    pub fn set_project(&mut self, project: StudioProject) {
        self.project = Some(project);
        self.refresh_animations();
    }

    /// Refresh animations from current project
    fn refresh_animations(&mut self) {
        self.active_animations.clear();

        if let Some(project) = &self.project {
            for (id, animation) in &project.animations {
                if animation.enabled {
                    let preview_anim = PreviewAnimation::from_project_animation(animation);
                    self.active_animations.insert(*id, preview_anim);
                }
            }
        }
    }

    /// Start preview playback
    pub fn play(&mut self) -> Result<()> {
        if self.animation_frame_id.is_some() {
            return Ok(()); // Already playing
        }

        self.start_animation_loop()?;

        // Start all animations
        for animation in self.active_animations.values_mut() {
            animation.play();
        }

        Ok(())
    }

    /// Pause preview playback
    pub fn pause(&mut self) {
        if let Some(id) = self.animation_frame_id.take() {
            web_sys::window().unwrap().cancel_animation_frame(id).ok();
        }

        // Pause all animations
        for animation in self.active_animations.values_mut() {
            animation.pause();
        }
    }

    /// Stop preview playback
    pub fn stop(&mut self) {
        self.pause();

        // Reset all animations
        for animation in self.active_animations.values_mut() {
            animation.reset();
        }
    }

    /// Start animation loop
    fn start_animation_loop(&mut self) -> Result<()> {
        let window = web_sys::window()
            .ok_or_else(|| StudioError::WebGLError("Window not available".to_string()))?;

        // Create callback for animation frame
        let callback = self.create_animation_callback()?;

        let id = window
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .map_err(|_| {
                StudioError::WebGLError("Failed to request animation frame".to_string())
            })?;

        self.animation_frame_id = Some(id);
        callback.forget(); // Prevent callback from being dropped

        Ok(())
    }

    /// Create animation frame callback
    fn create_animation_callback(&self) -> Result<Closure<dyn FnMut(f64)>> {
        let callback = Closure::wrap(Box::new(move |timestamp: f64| {
            // This would need to be handled differently in a real implementation
            // due to Rust's ownership rules with closures
            web_sys::console::log_1(&format!("Animation frame: {}", timestamp).into());
        }) as Box<dyn FnMut(f64)>);

        Ok(callback)
    }

    /// Update preview frame
    pub fn update_frame(&mut self, timestamp: f64) -> Result<()> {
        let delta_time = if self.last_frame_time > 0.0 {
            (timestamp - self.last_frame_time) / 1000.0 // Convert to seconds
        } else {
            1.0 / 60.0 // Assume 60fps for first frame
        };

        self.last_frame_time = timestamp;

        // Update performance metrics
        self.metrics.update(delta_time as f32);

        // Update all active animations
        let mut completed_animations = Vec::new();

        for (id, animation) in &mut self.active_animations {
            animation.update(delta_time as f32)?;

            if animation.is_completed() {
                completed_animations.push(*id);
            }
        }

        // Remove completed animations if not looping
        for id in completed_animations {
            if !self.settings.loop_enabled {
                self.active_animations.remove(&id);
            }
        }

        // Render frame
        self.render_frame()?;

        // Continue animation loop if there are active animations
        if !self.active_animations.is_empty() || self.settings.loop_enabled {
            self.start_animation_loop()?;
        }

        Ok(())
    }

    /// Render current frame
    fn render_frame(&mut self) -> Result<()> {
        // WebGL rendering temporarily disabled
        if false {
            // Render with WebGL
            let delta_time = if self.metrics.frame_count > 0 {
                1.0 / self.metrics.fps
            } else {
                1.0 / 60.0
            };

            // renderer.render(delta_time)?; // Temporarily disabled
        } else {
            // Render with CSS/DOM
            self.render_dom()?;
        }

        Ok(())
    }

    /// Render using DOM/CSS
    fn render_dom(&self) -> Result<()> {
        if let Some(target) = &self.target_element {
            for (id, animation) in &self.active_animations {
                let properties = animation.get_current_properties();

                // Apply CSS transforms and properties
                for (property, value) in properties {
                    let css_property = self.animation_property_to_css(&property);
                    let css_value = value.to_css(&property);

                    let style = target.style();
                    style.set_property(&css_property, &css_value).ok();
                }
            }
        }

        Ok(())
    }

    /// Convert animation property to CSS property name
    fn animation_property_to_css(&self, property: &AnimationProperty) -> String {
        match property {
            AnimationProperty::Translation => "transform".to_string(),
            AnimationProperty::Rotation => "transform".to_string(),
            AnimationProperty::Scale => "transform".to_string(),
            AnimationProperty::Opacity => "opacity".to_string(),
            AnimationProperty::Color => "color".to_string(),
            AnimationProperty::Custom(name) => name.clone(),
        }
    }

    /// Get current preview metrics
    pub fn get_metrics(&self) -> &PreviewMetrics {
        &self.metrics
    }

    /// Set preview settings
    pub fn set_settings(&mut self, settings: PreviewSettings) {
        self.settings = settings;

        // Update WebGL renderer if needed
        if self.settings.webgl_enabled {
            if let Some(target) = &self.target_element {
                if let Some(canvas) = target
                    .query_selector("canvas")
                    .ok()
                    .flatten()
                    .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
                {
                    // self.webgl_renderer = WebGLRenderer::initialize(&canvas).ok(); // Temporarily disabled
                }
            }
        }
    }
}

impl Default for LivePreview {
    fn default() -> Self {
        Self::new()
    }
}

/// Preview-specific animation wrapper
#[derive(Debug, Clone)]
pub struct PreviewAnimation {
    /// Animation ID
    pub id: Uuid,
    /// Timeline reference
    pub timeline: Option<Timeline3D>,
    /// Current properties
    pub current_properties: HashMap<AnimationProperty, AnimationValue>,
    /// Animation state
    pub state: PreviewAnimationState,
    /// Start time
    pub start_time: Option<f64>,
    /// Current progress (0.0 to 1.0)
    pub progress: f32,
    /// Duration
    pub duration: f32,
}

impl PreviewAnimation {
    /// Create from project animation
    pub fn from_project_animation(animation: &crate::project::ProjectAnimation) -> Self {
        Self {
            id: animation.id,
            timeline: animation.timeline.clone(),
            current_properties: HashMap::new(),
            state: PreviewAnimationState::Ready,
            start_time: None,
            progress: 0.0,
            duration: animation.duration,
        }
    }

    /// Play animation
    pub fn play(&mut self) {
        self.state = PreviewAnimationState::Playing;
        self.start_time = Some(js_sys::Date::now());
        self.progress = 0.0;
    }

    /// Pause animation
    pub fn pause(&mut self) {
        if self.state == PreviewAnimationState::Playing {
            self.state = PreviewAnimationState::Paused;
        }
    }

    /// Reset animation
    pub fn reset(&mut self) {
        self.state = PreviewAnimationState::Ready;
        self.start_time = None;
        self.progress = 0.0;
        self.current_properties.clear();
    }

    /// Update animation
    pub fn update(&mut self, delta_time: f32) -> Result<()> {
        if self.state != PreviewAnimationState::Playing {
            return Ok(());
        }

        if let Some(start_time) = self.start_time {
            let elapsed = (js_sys::Date::now() - start_time) / 1000.0; // Convert to seconds
            self.progress = (elapsed as f32 / self.duration).clamp(0.0, 1.0);

            if self.progress >= 1.0 {
                self.state = PreviewAnimationState::Completed;
            }
        }

        // Update current properties from timeline
        if let Some(timeline) = &self.timeline {
            let current_time = self.progress * self.duration;
            for property in [
                AnimationProperty::Translation,
                AnimationProperty::Rotation,
                AnimationProperty::Scale,
                AnimationProperty::Opacity,
            ] {
                if let Some(track) = timeline.get_track(&property) {
                    if let Ok(value) = track.value_at(current_time) {
                        self.current_properties.insert(property, value);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get current properties
    pub fn get_current_properties(&self) -> &HashMap<AnimationProperty, AnimationValue> {
        &self.current_properties
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.state == PreviewAnimationState::Completed
    }
}

/// Preview animation state
#[derive(Debug, Clone, PartialEq)]
pub enum PreviewAnimationState {
    Ready,
    Playing,
    Paused,
    Completed,
}

/// Preview settings
#[derive(Debug, Clone)]
pub struct PreviewSettings {
    /// Enable WebGL rendering
    pub webgl_enabled: bool,
    /// Enable GPU acceleration
    pub gpu_acceleration: bool,
    /// Loop animations
    pub loop_enabled: bool,
    /// Show performance metrics
    pub show_metrics: bool,
    /// Target framerate
    pub target_fps: f32,
    /// Preview quality
    pub quality: PreviewQuality,
    /// Background color
    pub background_color: [u8; 4],
}

impl Default for PreviewSettings {
    fn default() -> Self {
        Self {
            webgl_enabled: true,
            gpu_acceleration: true,
            loop_enabled: true,
            show_metrics: false,
            target_fps: 60.0,
            quality: PreviewQuality::High,
            background_color: [30, 30, 30, 255],
        }
    }
}

/// Preview quality levels
#[derive(Debug, Clone, PartialEq)]
pub enum PreviewQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// Preview performance metrics
#[derive(Debug, Clone, Default)]
pub struct PreviewMetrics {
    /// Current FPS
    pub fps: f32,
    /// Average frame time (ms)
    pub avg_frame_time: f32,
    /// Frame count
    pub frame_count: u64,
    /// Memory usage (estimated)
    pub memory_usage: usize,
    /// GPU memory usage (if available)
    pub gpu_memory_usage: Option<usize>,
    /// Active animations count
    pub active_animations: usize,
}

impl PreviewMetrics {
    /// Update metrics with new frame data
    pub fn update(&mut self, delta_time: f32) {
        self.frame_count += 1;

        let frame_time_ms = delta_time * 1000.0;

        // Update FPS using exponential moving average
        if self.frame_count == 1 {
            self.fps = 1.0 / delta_time;
            self.avg_frame_time = frame_time_ms;
        } else {
            let alpha = 0.1; // Smoothing factor
            self.fps = (1.0 - alpha) * self.fps + alpha * (1.0 / delta_time);
            self.avg_frame_time = (1.0 - alpha) * self.avg_frame_time + alpha * frame_time_ms;
        }
    }

    /// Reset metrics
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Preview renderer component
#[derive(Debug)]
pub struct PreviewRenderer {
    /// Live preview instance
    pub preview: LivePreview,
    /// Render target
    pub target: Option<HtmlElement>,
}

impl PreviewRenderer {
    /// Create new preview renderer
    pub fn new() -> Self {
        Self {
            preview: LivePreview::new(),
            target: None,
        }
    }

    /// Set render target
    pub fn set_target(&mut self, element: HtmlElement) -> Result<()> {
        self.target = Some(element.clone());
        self.preview.set_target(element)
    }

    /// Start rendering
    pub fn start(&mut self) -> Result<()> {
        self.preview.play()
    }

    /// Stop rendering
    pub fn stop(&mut self) {
        self.preview.stop();
    }
}

impl Default for PreviewRenderer {
    fn default() -> Self {
        Self::new()
    }
}

/// Live Preview Component
#[component]
pub fn LivePreviewComponent(
    /// Project to preview
    #[prop(optional)]
    project: Option<StudioProject>,

    /// Preview settings
    #[prop(default = PreviewSettings::default())]
    settings: PreviewSettings,

    /// Width of preview area
    #[prop(default = 800)]
    width: u32,

    /// Height of preview area
    #[prop(default = 600)]
    height: u32,
) -> impl IntoView {
    // Temporarily disable preview renderer signal due to thread safety issues
    // let (preview_renderer, set_preview_renderer) = create_signal(None::<PreviewRenderer>);
    let (is_playing, set_is_playing) = create_signal(false);
    let (metrics, set_metrics) = create_signal(PreviewMetrics::default());

    let preview_ref = create_node_ref::<leptos::html::Div>();

    // Initialize preview when element mounts
    create_effect(move |_| {
        if let Some(element) = preview_ref.get() {
            let mut renderer = PreviewRenderer::new();
            if renderer.set_target(element.unchecked_into()).is_ok() {
                if let Some(proj) = project.clone() {
                    renderer.preview.set_project(proj);
                }
                // Temporarily disabled until WebGL is re-enabled
                // renderer.preview.set_settings(settings.clone());
                // set_preview_renderer.set(Some(renderer));
            }
        }
    });

    // Play/pause controls
    let handle_play_pause = move |_| {
        // if let Some(mut renderer) = preview_renderer.get() {
        //     if is_playing.get() {
        //         renderer.stop();
        //         set_is_playing.set(false);
        //     } else {
        //         if renderer.start().is_ok() {
        //             set_is_playing.set(true);
        //         }
        //     }
        //     set_preview_renderer.set(Some(renderer));
        // }
        // Temporarily disabled until WebGL is re-enabled
        if is_playing.get() {
            set_is_playing.set(false);
        } else {
            set_is_playing.set(true);
        }
    };

    view! {
        <div class="live-preview-container">
            <div class="preview-controls">
                <button
                    class="preview-control-btn"
                    on:click=handle_play_pause
                >
                    {move || if is_playing.get() { "⏸" } else { "▶" }}
                </button>

                <button
                    class="preview-control-btn"
                    on:click=move |_| {
                        // if let Some(mut renderer) = preview_renderer.get() {
                        //     renderer.stop();
                        //     set_is_playing.set(false);
                        //     set_preview_renderer.set(Some(renderer));
                        // }
                        set_is_playing.set(false);
                    }
                >
                    "⏹"
                </button>

                {move || settings.show_metrics.then(|| view! {
                    <div class="preview-metrics">
                        <span>"FPS: " {move || format!("{:.1}", metrics.get().fps)}</span>
                        <span>"Frame Time: " {move || format!("{:.1}ms", metrics.get().avg_frame_time)}</span>
                        <span>"Animations: " {move || metrics.get().active_animations}</span>
                    </div>
                })}
            </div>

            <div
                node_ref=preview_ref
                class="preview-viewport"
                style:width=format!("{}px", width)
                style:height=format!("{}px", height)
                style:background_color="rgba(0, 0, 0, 0.1)"
            >
                {move || if settings.webgl_enabled {
                    view! {
                        <canvas
                            width=width
                            height=height
                            class="preview-canvas"
                        ></canvas>
                    }.into_view()
                } else {
                    view! {
                        <canvas
                            width=width
                            height=height
                            class="preview-canvas"
                        ></canvas>
                    }.into_view()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_live_preview_creation() {
        let preview = LivePreview::new();
        assert!(preview.target_element.is_none());
        assert!(preview.project.is_none());
        assert!(preview.active_animations.is_empty());
    }

    #[test]
    fn test_preview_settings() {
        let settings = PreviewSettings::default();
        assert!(settings.webgl_enabled);
        assert!(settings.gpu_acceleration);
        assert!(settings.loop_enabled);
        assert_eq!(settings.target_fps, 60.0);
    }

    #[test]
    fn test_preview_metrics() {
        let mut metrics = PreviewMetrics::default();
        assert_eq!(metrics.frame_count, 0);
        assert_eq!(metrics.fps, 0.0);

        metrics.update(1.0 / 60.0); // 60 FPS frame
        assert_eq!(metrics.frame_count, 1);
        assert!(metrics.fps > 0.0);
    }

    #[test]
    fn test_preview_animation_lifecycle() {
        let project_anim = crate::project::ProjectAnimation::new("Test Animation");
        let mut preview_anim = PreviewAnimation::from_project_animation(&project_anim);

        assert_eq!(preview_anim.state, PreviewAnimationState::Ready);
        assert!(!preview_anim.is_completed());

        preview_anim.play();
        assert_eq!(preview_anim.state, PreviewAnimationState::Playing);

        preview_anim.pause();
        assert_eq!(preview_anim.state, PreviewAnimationState::Paused);

        preview_anim.reset();
        assert_eq!(preview_anim.state, PreviewAnimationState::Ready);
        assert_eq!(preview_anim.progress, 0.0);
    }

    #[test]
    fn test_preview_renderer() {
        let mut renderer = PreviewRenderer::new();
        assert!(renderer.target.is_none());

        // Can't test DOM interaction in unit tests, but we can test structure
        assert!(renderer.preview.active_animations.is_empty());
    }
}
