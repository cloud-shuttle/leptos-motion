//! WebGL canvas component

use super::*;
use leptos::*;
use leptos::prelude::*;

/// WebGL canvas component
#[component]
pub fn WebGLCanvas(
    /// Canvas width
    #[prop(optional, default = 800)]
    width: u32,
    /// Canvas height
    #[prop(optional, default = 600)]
    height: u32,
    /// Whether to enable hardware acceleration
    #[prop(optional, default = true)]
    hardware_acceleration: bool,
    /// Whether to enable depth testing
    #[prop(optional, default = true)]
    depth_testing: bool,
    /// Whether to enable blending
    #[prop(optional, default = false)]
    blending: bool,
    /// Clear color (R, G, B, A)
    #[prop(optional, default = (0.0, 0.0, 0.0, 1.0))]
    clear_color: (f32, f32, f32, f32),
) -> impl IntoView {
    let canvas_ref = NodeRef::new::<leptos::html::Canvas>();
    let (renderer, set_renderer) = signal(None::<WebGLRenderer>);
    let (capabilities, set_capabilities) = signal(None::<WebGLCapabilities>);
    let (error, set_error) = signal(None::<String>);

    // Initialize WebGL context when canvas is mounted
    let _effect = Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            match WebGLContext::new(canvas) {
                Ok(context) => {
                    let caps = context.capabilities().clone();
                    set_capabilities.set(Some(caps));
                    
                    match WebGLRenderer::new(canvas) {
                        Ok(renderer) => {
                            set_renderer.set(Some(renderer));
                            set_error.set(None);
                        }
                        Err(e) => {
                            set_error.set(Some(format!("Failed to create renderer: {:?}", e)));
                        }
                    }
                }
                Err(e) => {
                    set_error.set(Some(format!("Failed to create WebGL context: {:?}", e)));
                }
            }
        }
    });

    view! {
        <div class="webgl-canvas-container" style="display: flex; flex-direction: column; align-items: center; gap: 20px;">
            <canvas
                node_ref=canvas_ref
                width=width
                height=height
                style=format!("border: 1px solid #ccc; background: rgba({}, {}, {}, {});", 
                    (clear_color.0 * 255.0) as u8,
                    (clear_color.1 * 255.0) as u8,
                    (clear_color.2 * 255.0) as u8,
                    clear_color.3
                )
            />
            
            {move || {
                if let Some(error) = error.get() {
                    view! {
                        <div class="error-message" style="color: red; padding: 10px; background: #ffe6e6; border-radius: 4px; border: 1px solid #ffcccc;">
                            <strong>"WebGL Error:"</strong> {error}
                        </div>
                    }.into_any()
                } else if let Some(caps) = capabilities.get() {
                    view! {
                        <div class="capabilities-info" style="background: #f0f8ff; padding: 15px; border-radius: 8px; border: 1px solid #b0d4f1; font-family: monospace; font-size: 12px;">
                            <h4 style="margin: 0 0 10px 0; color: #333;">"WebGL Capabilities"</h4>
                            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 10px;">
                                <div>
                                    <strong>"Version:"</strong> {caps.webgl_version}<br/>
                                    <strong>"Vendor:"</strong> {caps.vendor}<br/>
                                    <strong>"Renderer:"</strong> {caps.renderer}
                                </div>
                                <div>
                                    <strong>"Max Texture Size:"</strong> {caps.max_texture_size}<br/>
                                    <strong>"Max Vertex Attributes:"</strong> {caps.max_vertex_attributes}<br/>
                                    <strong>"Performance Score:"</strong> {format!("{:.1}%", caps.get_performance_score() * 100.0)}
                                </div>
                                <div>
                                    <strong>"Anisotropic Filtering:"</strong> {if caps.anisotropic_filtering { "✅" } else { "❌" }}<br/>
                                    <strong>"Depth Textures:"</strong> {if caps.depth_textures { "✅" } else { "❌" }}<br/>
                                    <strong>"Vertex Array Objects:"</strong> {if caps.vertex_array_objects { "✅" } else { "❌" }}
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="loading-message" style="color: #666; padding: 10px;">
                            "Initializing WebGL context..."
                        </div>
                    }.into_any()
                }
            }}
            
            {move || {
                if let Some(renderer) = renderer.get() {
                    view! {
                        <div class="renderer-controls" style="display: flex; gap: 10px; flex-wrap: wrap; justify-content: center;">
                            <button
                                style="padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| {
                                    // In a real implementation, this would start rendering
                                    web_sys::console::log_1(&"Start rendering".into());
                                }
                            >
                                "Start Rendering"
                            </button>
                            <button
                                style="padding: 8px 16px; background: #28a745; color: white; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| {
                                    // In a real implementation, this would stop rendering
                                    web_sys::console::log_1(&"Stop rendering".into());
                                }
                            >
                                "Stop Rendering"
                            </button>
                            <button
                                style="padding: 8px 16px; background: #ffc107; color: black; border: none; border-radius: 4px; cursor: pointer;"
                                on:click=move |_| {
                                    // In a real implementation, this would clear the canvas
                                    web_sys::console::log_1(&"Clear canvas".into());
                                }
                            >
                                "Clear Canvas"
                            </button>
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}
