//! SVG morphing editor component

use leptos::*;
use leptos::prelude::*;
use super::*;

/// SVG morphing editor component
#[component]
pub fn SvgMorphingEditor(
    /// Source SVG path
    #[prop(optional)]
    source_path: Option<String>,
    /// Target SVG path
    #[prop(optional)]
    target_path: Option<String>,
    /// Morphing configuration
    #[prop(optional)]
    config: Option<MorphConfig>,
    /// Editor width
    #[prop(optional, default = 800.0)]
    width: f64,
    /// Editor height
    #[prop(optional, default = 600.0)]
    height: f64,
) -> impl IntoView {
    let (morphing_progress, set_morphing_progress) = signal(0.0);
    let (is_morphing, set_is_morphing) = signal(false);
    let (current_path, set_current_path) = signal(String::new());

    // Create default paths if not provided
    let default_source = source_path.unwrap_or_else(|| "M10,10 L50,10 L50,50 L10,50 Z".to_string());
    let default_target = target_path.unwrap_or_else(|| "M20,20 L60,20 L60,60 L20,60 Z".to_string());
    let default_config = config.unwrap_or_default();

    // Start morphing function
    let start_morphing = move |_| {
        set_is_morphing.set(true);
        set_morphing_progress.set(0.0);
        
        // In a real implementation, this would start the actual morphing animation
        // For now, we'll just simulate progress
        let _interval = set_interval(move || {
            let current_progress = morphing_progress.get() + 0.01;
            if current_progress >= 1.0 {
                set_is_morphing.set(false);
                set_morphing_progress.set(1.0);
            } else {
                set_morphing_progress.set(current_progress);
            }
        }, std::time::Duration::from_millis(16));
    };

    // Stop morphing function
    let stop_morphing = move |_| {
        set_is_morphing.set(false);
    };

    // Reset morphing function
    let reset_morphing = move |_| {
        set_is_morphing.set(false);
        set_morphing_progress.set(0.0);
    };

    view! {
        <div class="svg-morphing-editor" style=format!("width: {}px; height: {}px; border: 1px solid #ccc;", width, height)>
            <div class="editor-controls" style="padding: 10px; background: #f5f5f5; border-bottom: 1px solid #ccc;">
                <button
                    on:click=start_morphing
                    disabled=is_morphing
                    style="margin-right: 10px; padding: 8px 16px; background: #007bff; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    {if is_morphing.get() { "Morphing..." } else { "Start Morphing" }}
                </button>
                
                <button
                    on:click=stop_morphing
                    disabled=move || !is_morphing.get()
                    style="margin-right: 10px; padding: 8px 16px; background: #dc3545; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Stop"
                </button>
                
                <button
                    on:click=reset_morphing
                    style="padding: 8px 16px; background: #6c757d; color: white; border: none; border-radius: 4px; cursor: pointer;"
                >
                    "Reset"
                </button>
                
                <div style="margin-top: 10px;">
                    <label style="margin-right: 10px;">"Progress: "</label>
                    <span>{format!("{:.1}%", morphing_progress.get() * 100.0)}</span>
                </div>
            </div>
            
            <div class="editor-canvas" style="position: relative; width: 100%; height: calc(100% - 60px);">
                <svg width="100%" height="100%" viewBox="0 0 100 100" style="border: 1px solid #ddd;">
                    // Source path (faded)
                    <path
                        d=default_source.clone()
                        fill="none"
                        stroke="#999"
                        stroke-width="0.5"
                        opacity=0.3
                    />
                    
                    // Target path (faded)
                    <path
                        d=default_target.clone()
                        fill="none"
                        stroke="#999"
                        stroke-width="0.5"
                        opacity=0.3
                    />
                    
                    // Current morphed path
                    <path
                        d=current_path.get()
                        fill="none"
                        stroke="#007bff"
                        stroke-width="1"
                        opacity=0.8
                    />
                </svg>
            </div>
        </div>
    }
}
