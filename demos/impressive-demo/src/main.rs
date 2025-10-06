use leptos::prelude::*;
use leptos_motion_dom::{MotionDiv, AnimateProp, AnimationValue};
use std::collections::HashMap;

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let (is_playing, set_playing) = signal(false);
    let (current_demo, set_current_demo) = signal(0);
    
    // Demo configurations
    let demos = vec![
        ("🎨 Interactive Gallery", "gallery"),
        ("🌊 Fluid Physics", "physics"),
        ("🎭 Morphing Shapes", "morphing"),
        ("⚡ Particle Explosion", "particles"),
        ("🎪 3D Carousel", "carousel"),
        ("🌟 Constellation", "stars"),
    ];
    
    view! {
        <div style="
            min-height: 100vh;
            background: linear-gradient(135deg, #1e3c72 0%, #2a5298 100%);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            color: white;
            overflow-x: hidden;
        ">
            <div style="padding: 2rem; max-width: 1400px; margin: 0 auto;">
                <h1 style="
                    font-size: 4rem;
                    font-weight: 900;
                    text-align: center;
                    margin-bottom: 1rem;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4, #feca57);
                    background-size: 300% 300%;
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    animation: gradient 4s ease infinite;
                ">
                    "🚀 Leptos Motion Spectacular"
                </h1>
                
                <p style="
                    text-align: center;
                    font-size: 1.4rem;
                    margin-bottom: 3rem;
                    opacity: 0.9;
                    font-weight: 300;
                ">
                    "Next-generation animations powered by Rust + WASM"
                </p>
                
                // Demo selector
                <div style="
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
                    gap: 1rem;
                    margin-bottom: 3rem;
                ">
                    {demos.into_iter().enumerate().map(|(i, (name, _))| {
                        let is_active = move || current_demo.get() == i;
                        view! {
                            <button
                                on:click=move |_| set_current_demo.set(i)
                                style=move || format!("
                                    padding: 1.5rem;
                                    border: none;
                                    border-radius: 15px;
                                    background: {};
                                    color: white;
                                    font-weight: 600;
                                    cursor: pointer;
                                    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
                                    transform: scale({}) translateY({});
                                    box-shadow: 0 8px 25px rgba(0,0,0,{});
                                    font-size: 1.1rem;
                                ", 
                                if is_active() { 
                                    "linear-gradient(135deg, #667eea 0%, #764ba2 100%)" 
                                } else { 
                                    "rgba(255,255,255,0.1)" 
                                },
                                if is_active() { "1.05" } else { "1.0" },
                                if is_active() { "-5px" } else { "0px" },
                                if is_active() { "0.3" } else { "0.1" }
                                )
                            >
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                
                // Play/Pause button
                <div style="text-align: center; margin-bottom: 3rem;">
                    <button
                        on:click=move |_| set_playing.update(|v| *v = !*v)
                        style="
                            padding: 1.2rem 3rem;
                            border: none;
                            border-radius: 50px;
                            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                            color: white;
                            font-size: 1.3rem;
                            font-weight: 700;
                            cursor: pointer;
                            box-shadow: 0 10px 30px rgba(255, 107, 107, 0.4);
                            transition: all 0.3s ease;
                            text-transform: uppercase;
                            letter-spacing: 1px;
                        "
                    >
                        {move || if is_playing.get() { "⏸️ Pause Spectacle" } else { "▶️ Start Spectacle" }}
                    </button>
                </div>
                
                // Demo content
                <div style="
                    background: rgba(255,255,255,0.05);
                    border-radius: 25px;
                    padding: 4rem;
                    backdrop-filter: blur(20px);
                    border: 1px solid rgba(255,255,255,0.1);
                    min-height: 600px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    position: relative;
                    overflow: hidden;
                ">
                    {move || match current_demo.get() {
                        0 => view! { <InteractiveGalleryDemo is_playing=is_playing.get() /> }.into_any(),
                        1 => view! { <FluidPhysicsDemo is_playing=is_playing.get() /> }.into_any(),
                        2 => view! { <MorphingShapesDemo is_playing=is_playing.get() /> }.into_any(),
                        3 => view! { <ParticleExplosionDemo is_playing=is_playing.get() /> }.into_any(),
                        4 => view! { <Carousel3DDemo is_playing=is_playing.get() /> }.into_any(),
                        5 => view! { <ConstellationDemo is_playing=is_playing.get() /> }.into_any(),
                        _ => view! { <div>"Select a spectacular demo"</div> }.into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn InteractiveGalleryDemo(is_playing: bool) -> impl IntoView {
    let (selected_item, set_selected_item) = signal(0);
    let (hover_item, set_hover_item) = signal(None::<usize>);
    
    let gallery_items = vec![
        ("🎨", "Digital Art", "#ff6b6b"),
        ("🎵", "Music", "#4ecdc4"),
        ("🎮", "Gaming", "#45b7d1"),
        ("📱", "Mobile", "#96ceb4"),
        ("💻", "Code", "#feca57"),
        ("🚀", "Space", "#ff9ff3"),
    ];
    
    // Auto-rotate when playing
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_selected_item.update(|i| *i = (*i + 1) % gallery_items.len());
            }, std::time::Duration::from_millis(2000));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    view! {
        <div style="
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 2rem;
            width: 100%;
            max-width: 800px;
        ">
            {gallery_items.into_iter().enumerate().map(|(i, (emoji, title, color))| {
                let is_selected = move || selected_item.get() == i;
                let is_hovered = move || hover_item.get() == Some(i);
                
                let animate_values = move || {
                    let mut values = HashMap::new();
                    let scale = if is_selected() { 1.2 } else if is_hovered() { 1.1 } else { 1.0 };
                    let rotate = if is_selected() { 5.0 } else { 0.0 };
                    let y = if is_selected() { -20.0 } else { 0.0 };
                    
                    values.insert("scale".to_string(), AnimationValue::Number(scale));
                    values.insert("rotateZ".to_string(), AnimationValue::Degrees(rotate));
                    values.insert("y".to_string(), AnimationValue::Pixels(y));
                    values.insert("opacity".to_string(), AnimationValue::Number(if is_selected() { 1.0 } else { 0.8 }));
                    values
                };
                
                let node_ref = NodeRef::new();
                
                view! {
                    <MotionDiv
                        animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                        node_ref=node_ref
                        on:mouse_enter=move |_| set_hover_item.set(Some(i))
                        on:mouse_leave=move |_| set_hover_item.set(None)
                        on:click=move |_| set_selected_item.set(i)
                        style=format!("
                            background: linear-gradient(135deg, {}, rgba(255,255,255,0.1));
                            border-radius: 20px;
                            padding: 2rem;
                            text-align: center;
                            cursor: pointer;
                            border: 2px solid {};
                            box-shadow: 0 10px 30px rgba(0,0,0,0.2);
                            transition: all 0.3s ease;
                        ", color, if is_selected() { color } else { "transparent" })
                    >
                        <div style="font-size: 3rem; margin-bottom: 1rem;">{emoji}</div>
                        <div style="font-size: 1.2rem; font-weight: 600;">{title}</div>
                    </MotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn FluidPhysicsDemo(is_playing: bool) -> impl IntoView {
    let (fluid_waves, set_fluid_waves) = signal(vec![]);
    
    Effect::new(move |_| {
        if is_playing {
            // Initialize fluid particles
            let mut waves = Vec::new();
            for i in 0..50 {
                waves.push((
                    i as f64 * 8.0,
                    (js_sys::Math::random() * 200.0) as f64,
                    (js_sys::Math::random() * 2.0 - 1.0) as f64,
                    (js_sys::Math::random() * 0.1 + 0.05) as f64,
                ));
            }
            set_fluid_waves.set(waves);
            
            let interval = set_interval_with_handle(move || {
                set_fluid_waves.update(|waves| {
                    for (x, y, vx, vy) in waves.iter_mut() {
                        *x += *vx;
                        *y += *vy;
                        
                        // Fluid dynamics simulation
                        *vy += (js_sys::Math::random() * 0.2 - 0.1) as f64;
                        *vx += (js_sys::Math::random() * 0.1 - 0.05) as f64;
                        
                        // Boundary conditions
                        if *x < 0.0 || *x > 400.0 { *vx *= -0.8; }
                        if *y < 0.0 || *y > 300.0 { *vy *= -0.8; }
                        
                        *x = (*x).max(0.0).min(400.0);
                        *y = (*y).max(0.0).min(300.0);
                    }
                });
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    view! {
        <div style="
            position: relative;
            width: 400px;
            height: 300px;
            background: linear-gradient(135deg, #667eea, #764ba2);
            border-radius: 20px;
            overflow: hidden;
        ">
            {move || fluid_waves.get().into_iter().enumerate().map(|(i, (x, y, _, _))| {
                let animate_values = move || {
                    let mut values = HashMap::new();
                    values.insert("x".to_string(), AnimationValue::Pixels(x));
                    values.insert("y".to_string(), AnimationValue::Pixels(y));
                    values.insert("opacity".to_string(), AnimationValue::Number(0.7));
                    values
                };
                
                let node_ref = NodeRef::new();
                
                view! {
                    <MotionDiv
                        animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                        node_ref=node_ref
                        style=format!("
                            position: absolute;
                            width: 6px;
                            height: 6px;
                            background: hsl({}, 80%, 70%);
                            border-radius: 50%;
                            left: 0;
                            top: 0;
                            box-shadow: 0 0 10px hsl({}, 80%, 50%);
                        ", (i as f64 * 7.2) % 360.0, (i as f64 * 7.2) % 360.0)
                    >
                        ""
                    </MotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn MorphingShapesDemo(is_playing: bool) -> impl IntoView {
    let (morph_progress, set_morph_progress) = signal(0.0);
    let node_ref = NodeRef::new();
    
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_morph_progress.update(|p| *p += 0.01);
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    let animate_values = move || {
        let progress = morph_progress.get();
        let scale = 1.0 + (progress * 3.0 * std::f64::consts::PI).sin() * 0.4;
        let rotate = progress * 720.0;
        let skew_x = (progress * 2.0 * std::f64::consts::PI).sin() * 10.0;
        let skew_y = (progress * 1.5 * std::f64::consts::PI).cos() * 5.0;
        
        let mut values = HashMap::new();
        values.insert("scale".to_string(), AnimationValue::Number(scale));
        values.insert("rotateZ".to_string(), AnimationValue::Degrees(rotate));
        values.insert("skewX".to_string(), AnimationValue::Degrees(skew_x));
        values.insert("skewY".to_string(), AnimationValue::Degrees(skew_y));
        values.insert("borderRadius".to_string(), AnimationValue::Pixels(50.0 + (progress * 4.0 * std::f64::consts::PI).sin() * 40.0));
        values
    };
    
    view! {
        <MotionDiv
            animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
            node_ref=node_ref
            style="
                width: 200px;
                height: 200px;
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4);
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                font-weight: bold;
                font-size: 1.5rem;
                box-shadow: 0 20px 40px rgba(0,0,0,0.3);
            ".to_string()
        >
            "Morphing"
        </MotionDiv>
    }
}

#[component]
fn ParticleExplosionDemo(is_playing: bool) -> impl IntoView {
    let (particles, set_particles) = signal(vec![]);
    let (explosion_center, set_explosion_center) = signal((200.0, 150.0));
    
    Effect::new(move |_| {
        if is_playing {
            // Create explosion particles
            let mut new_particles = Vec::new();
            for i in 0..100 {
                let angle = (i as f64 / 100.0) * 2.0 * std::f64::consts::PI;
                let speed = js_sys::Math::random() * 5.0 + 2.0;
                new_particles.push((
                    explosion_center.get().0,
                    explosion_center.get().1,
                    angle.cos() * speed,
                    angle.sin() * speed,
                    js_sys::Math::random() * 360.0,
                ));
            }
            set_particles.set(new_particles);

            let interval = set_interval_with_handle(move || {
                set_particles.update(|particles| {
                    for (x, y, vx, vy, hue) in particles.iter_mut() {
                        *x += *vx;
                        *y += *vy;
                        *vx *= 0.98; // friction
                        *vy *= 0.98;
                        *vy += 0.1; // gravity
                    }
                });
            }, std::time::Duration::from_millis(16));

            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        } else {
            move || {
                // No cleanup needed
            }
        }
    });
    
    view! {
        <div style="
            position: relative;
            width: 400px;
            height: 300px;
            background: radial-gradient(circle, #1a1a2e, #16213e);
            border-radius: 20px;
            overflow: hidden;
        ">
            {move || particles.get().into_iter().enumerate().map(|(i, (x, y, _, _, hue))| {
                let animate_values = move || {
                    let mut values = HashMap::new();
                    values.insert("x".to_string(), AnimationValue::Pixels(x));
                    values.insert("y".to_string(), AnimationValue::Pixels(y));
                    values.insert("opacity".to_string(), AnimationValue::Number(0.8));
                    values
                };
                
                let node_ref = NodeRef::new();
                
                view! {
                    <MotionDiv
                        animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                        node_ref=node_ref
                        style=format!("
                            position: absolute;
                            width: 4px;
                            height: 4px;
                            background: hsl({}, 100%, 60%);
                            border-radius: 50%;
                            left: 0;
                            top: 0;
                            box-shadow: 0 0 8px hsl({}, 100%, 40%);
                        ", hue, hue)
                    >
                        ""
                    </MotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn Carousel3DDemo(is_playing: bool) -> impl IntoView {
    let (rotation, set_rotation) = signal(0.0);
    let node_ref = NodeRef::new();
    
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_rotation.update(|r| *r += 1.0);
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    let animate_values = move || {
        let mut values = HashMap::new();
        values.insert("rotateY".to_string(), AnimationValue::Degrees(rotation.get()));
        values.insert("rotateX".to_string(), AnimationValue::Degrees(rotation.get() * 0.5));
        values.insert("scale".to_string(), AnimationValue::Number(1.0 + (rotation.get() * 0.01).sin() * 0.1));
        values
    };
    
    view! {
        <div style="perspective: 1000px; transform-style: preserve-3d;">
            <MotionDiv
                animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                node_ref=node_ref
                style="
                    width: 300px;
                    height: 200px;
                    background: linear-gradient(45deg, #667eea, #764ba2);
                    border-radius: 20px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    font-size: 2rem;
                    box-shadow: 0 30px 60px rgba(0,0,0,0.4);
                    transform-style: preserve-3d;
                ".to_string()
            >
                "3D Carousel"
            </MotionDiv>
        </div>
    }
}

#[component]
fn ConstellationDemo(is_playing: bool) -> impl IntoView {
    let (stars, set_stars) = signal(vec![]);
    
    Effect::new(move |_| {
        if is_playing {
            // Create constellation
            let mut new_stars = Vec::new();
            for i in 0..30 {
                new_stars.push((
                    (js_sys::Math::random() * 400.0) as f64,
                    (js_sys::Math::random() * 300.0) as f64,
                    (js_sys::Math::random() * 2.0 + 1.0) as f64,
                    (js_sys::Math::random() * 360.0) as f64,
                ));
            }
            set_stars.set(new_stars);
            
            let interval = set_interval_with_handle(move || {
                set_stars.update(|stars| {
                    for (x, y, size, hue) in stars.iter_mut() {
                        *hue += 0.5;
                        if *hue > 360.0 { *hue = 0.0; }
                    }
                });
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    view! {
        <div style="
            position: relative;
            width: 400px;
            height: 300px;
            background: radial-gradient(circle, #0f0f23, #1a1a2e);
            border-radius: 20px;
            overflow: hidden;
        ">
            {move || stars.get().into_iter().enumerate().map(|(i, (x, y, size, hue))| {
                let animate_values = move || {
                    let mut values = HashMap::new();
                    values.insert("x".to_string(), AnimationValue::Pixels(x));
                    values.insert("y".to_string(), AnimationValue::Pixels(y));
                    values.insert("opacity".to_string(), AnimationValue::Number(0.9));
                    values
                };
                
                let node_ref = NodeRef::new();
                
                view! {
                    <MotionDiv
                        animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                        node_ref=node_ref
                        style=format!("
                            position: absolute;
                            width: {}px;
                            height: {}px;
                            background: hsl({}, 100%, 80%);
                            border-radius: 50%;
                            left: 0;
                            top: 0;
                            box-shadow: 0 0 {}px hsl({}, 100%, 60%);
                        ", size, size, hue, size * 3.0, hue)
                    >
                        ""
                    </MotionDiv>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}