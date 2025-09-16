// CSS Animation Alternative for Leptos
// This provides a working animation solution while leptos-motion is being fixed

use leptos::prelude::*;
use std::collections::HashMap;

/// CSS-based animated div component that works reliably
#[component]
pub fn CssAnimatedDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Animation duration in seconds
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Animation easing
    #[prop(optional, default = "ease-in-out".to_string())]
    easing: String,
    /// Whether the element should be animated
    #[prop(optional, default = true)]
    animated: bool,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let element_ref = NodeRef::<leptos::html::Div>::new();
    
    // Create dynamic style that includes transition
    let dynamic_style = move || {
        let mut styles = Vec::new();
        
        if animated {
            styles.push(format!("transition: all {}s {}", duration, easing));
        }
        
        if let Some(custom_style) = &style {
            styles.push(custom_style.clone());
        }
        
        styles.join("; ")
    };
    
    view! {
        <div
            ref=element_ref
            class=class
            style=dynamic_style()
        >
            {children()}
        </div>
    }
}

/// Reactive animated div that responds to signal changes
#[component]
pub fn ReactiveCssAnimatedDiv(
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Base CSS styles
    #[prop(optional)]
    style: Option<String>,
    /// Animation duration in seconds
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Animation easing
    #[prop(optional, default = "ease-in-out".to_string())]
    easing: String,
    /// Reactive style properties
    #[prop(optional)]
    reactive_styles: Option<ReadSignal<HashMap<String, String>>>,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let element_ref = NodeRef::<leptos::html::Div>::new();
    
    // Create reactive style that includes transitions
    let reactive_style = move || {
        let mut styles = Vec::new();
        
        // Add transition
        styles.push(format!("transition: all {}s {}", duration, easing));
        
        // Add reactive styles
        if let Some(reactive) = &reactive_styles {
            for (key, value) in reactive.get().iter() {
                styles.push(format!("{}: {}", key, value));
            }
        }
        
        // Add base styles
        if let Some(base_style) = &style {
            styles.push(base_style.clone());
        }
        
        styles.join("; ")
    };
    
    view! {
        <div
            ref=element_ref
            class=class
            style=reactive_style()
        >
            {children()}
        </div>
    }
}

/// Scale animation component
#[component]
pub fn ScaleAnimatedDiv(
    /// Scale factor signal
    scale: ReadSignal<f64>,
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Additional styles
    #[prop(optional)]
    style: Option<String>,
    /// Animation duration
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let reactive_styles = move || {
        let mut styles = HashMap::new();
        styles.insert("transform".to_string(), format!("scale({})", scale.get()));
        styles
    };
    
    let reactive_signal = create_memo(move |_| reactive_styles());
    
    view! {
        <ReactiveCssAnimatedDiv
            class=class
            style=style
            duration=duration
            reactive_styles=Some(reactive_signal.read_only())
        >
            {children()}
        </ReactiveCssAnimatedDiv>
    }
}

/// Opacity animation component
#[component]
pub fn OpacityAnimatedDiv(
    /// Opacity signal
    opacity: ReadSignal<f64>,
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Additional styles
    #[prop(optional)]
    style: Option<String>,
    /// Animation duration
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let reactive_styles = move || {
        let mut styles = HashMap::new();
        styles.insert("opacity".to_string(), format!("{}", opacity.get()));
        styles
    };
    
    let reactive_signal = create_memo(move |_| reactive_styles());
    
    view! {
        <ReactiveCssAnimatedDiv
            class=class
            style=style
            duration=duration
            reactive_styles=Some(reactive_signal.read_only())
        >
            {children()}
        </ReactiveCssAnimatedDiv>
    }
}

/// Transform animation component (position, rotation, scale)
#[component]
pub fn TransformAnimatedDiv(
    /// X position signal
    x: ReadSignal<f64>,
    /// Y position signal
    y: ReadSignal<f64>,
    /// Rotation signal (in degrees)
    rotation: ReadSignal<f64>,
    /// Scale signal
    scale: ReadSignal<f64>,
    /// CSS class name
    #[prop(optional)]
    class: Option<String>,
    /// Additional styles
    #[prop(optional)]
    style: Option<String>,
    /// Animation duration
    #[prop(optional, default = 0.3)]
    duration: f64,
    /// Children elements
    children: Children,
) -> impl IntoView {
    let reactive_styles = move || {
        let mut styles = HashMap::new();
        styles.insert("transform".to_string(), format!(
            "translate({}px, {}px) rotate({}deg) scale({})",
            x.get(),
            y.get(),
            rotation.get(),
            scale.get()
        ));
        styles
    };
    
    let reactive_signal = create_memo(move |_| reactive_styles());
    
    view! {
        <ReactiveCssAnimatedDiv
            class=class
            style=style
            duration=duration
            reactive_styles=Some(reactive_signal.read_only())
        >
            {children()}
        </ReactiveCssAnimatedDiv>
    }
}

/// Example usage component
#[component]
pub fn AnimationExample() -> impl IntoView {
    let (scale, set_scale) = create_signal(1.0);
    let (opacity, set_opacity) = create_signal(1.0);
    let (x, set_x) = create_signal(0.0);
    let (y, set_y) = create_signal(0.0);
    let (rotation, set_rotation) = create_signal(0.0);
    
    view! {
        <div style="padding: 20px; font-family: Arial, sans-serif;">
            <h2>"CSS Animation Alternative Demo"</h2>
            
            <div style="display: flex; gap: 20px; margin: 20px 0;">
                <div style="flex: 1;">
                    <h3>"Controls"</h3>
                    
                    <div style="margin: 10px 0;">
                        <label>"Scale: " {move || format!("{:.1}", scale.get())}</label>
                        <input
                            type="range"
                            min="0.5"
                            max="2.0"
                            step="0.1"
                            value=move || scale.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                set_scale.set(value);
                            }
                        />
                    </div>
                    
                    <div style="margin: 10px 0;">
                        <label>"Opacity: " {move || format!("{:.1}", opacity.get())}</label>
                        <input
                            type="range"
                            min="0.0"
                            max="1.0"
                            step="0.1"
                            value=move || opacity.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(1.0);
                                set_opacity.set(value);
                            }
                        />
                    </div>
                    
                    <div style="margin: 10px 0;">
                        <label>"X Position: " {move || format!("{:.0}px", x.get())}</label>
                        <input
                            type="range"
                            min="-200"
                            max="200"
                            step="10"
                            value=move || x.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_x.set(value);
                            }
                        />
                    </div>
                    
                    <div style="margin: 10px 0;">
                        <label>"Y Position: " {move || format!("{:.0}px", y.get())}</label>
                        <input
                            type="range"
                            min="-200"
                            max="200"
                            step="10"
                            value=move || y.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_y.set(value);
                            }
                        />
                    </div>
                    
                    <div style="margin: 10px 0;">
                        <label>"Rotation: " {move || format!("{:.0}°", rotation.get())}</label>
                        <input
                            type="range"
                            min="0"
                            max="360"
                            step="10"
                            value=move || rotation.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev).parse::<f64>().unwrap_or(0.0);
                                set_rotation.set(value);
                            }
                        />
                    </div>
                    
                    <button
                        on:click=move |_| {
                            set_scale.set(1.0);
                            set_opacity.set(1.0);
                            set_x.set(0.0);
                            set_y.set(0.0);
                            set_rotation.set(0.0);
                        }
                        style="margin-top: 20px; padding: 10px 20px; background: #667eea; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    >
                        "Reset"
                    </button>
                </div>
                
                <div style="flex: 1; border: 2px dashed #ccc; padding: 20px; min-height: 300px; display: flex; align-items: center; justify-content: center;">
                    <TransformAnimatedDiv
                        x=x
                        y=y
                        rotation=rotation
                        scale=scale
                        style="width: 100px; height: 100px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; box-shadow: 0 4px 8px rgba(0,0,0,0.2);"
                    >
                        "Animated!"
                    </TransformAnimatedDiv>
                </div>
            </div>
        </div>
    }
}
