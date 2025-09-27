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
        ("3D Cube Rotation", "rotate_cube"),
        ("Particle System", "particles"),
        ("Morphing Shapes", "morphing"),
        ("Elastic Bounce", "bounce"),
        ("Wave Animation", "wave"),
        ("Gravity Physics", "gravity"),
    ];
    
    view! {
        <div style="
            min-height: 100vh;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            color: white;
            overflow-x: hidden;
        ">
            <div style="padding: 2rem; max-width: 1200px; margin: 0 auto;">
                <h1 style="
                    font-size: 3rem;
                    font-weight: 800;
                    text-align: center;
                    margin-bottom: 1rem;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1);
                    background-size: 200% 200%;
                    -webkit-background-clip: text;
                    -webkit-text-fill-color: transparent;
                    animation: gradient 3s ease infinite;
                ">
                    "🚀 Leptos Motion Showcase"
                </h1>
                
                <p style="
                    text-align: center;
                    font-size: 1.2rem;
                    margin-bottom: 3rem;
                    opacity: 0.9;
                ">
                    "High-performance animations powered by Rust + WASM"
                </p>
                
                // Demo selector
                <div style="
                    display: flex;
                    flex-wrap: wrap;
                    gap: 1rem;
                    justify-content: center;
                    margin-bottom: 3rem;
                ">
                    {demos.into_iter().enumerate().map(|(i, (name, _))| {
                        let is_active = move || current_demo.get() == i;
                        view! {
                            <button
                                on:click=move |_| set_current_demo.set(i)
                                style=move || format!("
                                    padding: 0.75rem 1.5rem;
                                    border: none;
                                    border-radius: 50px;
                                    background: {};
                                    color: white;
                                    font-weight: 600;
                                    cursor: pointer;
                                    transition: all 0.3s ease;
                                    transform: scale({});
                                ", 
                                if is_active() { "linear-gradient(45deg, #ff6b6b, #4ecdc4)" } else { "rgba(255,255,255,0.2)" },
                                if is_active() { "1.05" } else { "1.0" }
                                )
                            >
                                {name}
                            </button>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                
                // Play/Pause button
                <div style="text-align: center; margin-bottom: 2rem;">
                    <button
                        on:click=move |_| set_playing.update(|v| *v = !*v)
                        style="
                            padding: 1rem 2rem;
                            border: none;
                            border-radius: 50px;
                            background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                            color: white;
                            font-size: 1.1rem;
                            font-weight: 600;
                            cursor: pointer;
                            box-shadow: 0 4px 15px rgba(0,0,0,0.2);
                        "
                    >
                        {move || if is_playing.get() { "⏸️ Pause" } else { "▶️ Play" }}
                    </button>
                </div>
                
                // Demo content
                <div style="
                    background: rgba(255,255,255,0.1);
                    border-radius: 20px;
                    padding: 3rem;
                    backdrop-filter: blur(10px);
                    border: 1px solid rgba(255,255,255,0.2);
                    min-height: 500px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                ">
                    {move || match current_demo.get() {
                        0 => view! { <CubeRotationDemo is_playing=is_playing.get() /> }.into_any(),
                        1 => view! { <ParticleSystemDemo is_playing=is_playing.get() /> }.into_any(),
                        2 => view! { <MorphingShapesDemo is_playing=is_playing.get() /> }.into_any(),
                        3 => view! { <ElasticBounceDemo is_playing=is_playing.get() /> }.into_any(),
                        4 => view! { <WaveAnimationDemo is_playing=is_playing.get() /> }.into_any(),
                        5 => view! { <GravityPhysicsDemo is_playing=is_playing.get() /> }.into_any(),
                        _ => view! { <div>"Select a demo"</div> }.into_any(),
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn CubeRotationDemo(is_playing: bool) -> impl IntoView {
    let (rotation, set_rotation) = signal(0.0);
    let node_ref = NodeRef::new();
    
    // Create animation effect
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_rotation.update(|r| *r += 2.0);
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
        values.insert("rotateX".to_string(), AnimationValue::Degrees(rotation.get()));
        values.insert("rotateY".to_string(), AnimationValue::Degrees(rotation.get() * 0.7));
        values.insert("rotateZ".to_string(), AnimationValue::Degrees(rotation.get() * 0.3));
        values
    };
    
    view! {
        <div style="perspective: 1000px;">
            <MotionDiv
                animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                node_ref=node_ref
                style="
                    width: 200px;
                    height: 200px;
                    background: linear-gradient(45deg, #ff6b6b, #4ecdc4, #45b7d1, #96ceb4);
                    border-radius: 20px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 1.5rem;
                    font-weight: bold;
                    color: white;
                    box-shadow: 0 20px 40px rgba(0,0,0,0.3);
                    transform-style: preserve-3d;
                ".to_string()
            >
                "3D Cube"
            </MotionDiv>
        </div>
    }
}

#[component]
fn ParticleSystemDemo(is_playing: bool) -> impl IntoView {
    let (particles, set_particles) = signal(vec![]);
    
    Effect::new(move |_| {
        if is_playing {
            // Create 20 particles with random positions and velocities
            let mut new_particles = Vec::new();
            for i in 0..20 {
                new_particles.push((
                    (js_sys::Math::random() * 400.0) as f64,
                    (js_sys::Math::random() * 400.0) as f64,
                    (js_sys::Math::random() * 4.0 - 2.0) as f64,
                    (js_sys::Math::random() * 4.0 - 2.0) as f64,
                ));
            }
            set_particles.set(new_particles);
            
            let interval = set_interval_with_handle(move || {
                set_particles.update(|particles| {
                    for (x, y, vx, vy) in particles.iter_mut() {
                        *x += *vx;
                        *y += *vy;
                        
                        // Bounce off walls
                        if *x <= 0.0 || *x >= 400.0 { *vx *= -0.8; }
                        if *y <= 0.0 || *y >= 400.0 { *vy *= -0.8; }
                        
                        // Keep in bounds
                        *x = (*x).max(0.0).min(400.0);
                        *y = (*y).max(0.0).min(400.0);
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
        <div style="position: relative; width: 400px; height: 400px; border: 2px solid rgba(255,255,255,0.3); border-radius: 10px;">
            {move || particles.get().into_iter().enumerate().map(|(i, (x, y, _, _))| {
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
                            width: 8px;
                            height: 8px;
                            background: hsl({}, 70%, 60%);
                            border-radius: 50%;
                            left: 0;
                            top: 0;
                        ", (i as f64 * 18.0) % 360.0)
                    />
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
                set_morph_progress.update(|p| *p += 0.02);
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
        let scale = 1.0 + (progress * 2.0 * std::f64::consts::PI).sin() * 0.3;
        let rotate = progress * 360.0;
        
        let mut values = HashMap::new();
        values.insert("scale".to_string(), AnimationValue::Number(scale));
        values.insert("rotateZ".to_string(), AnimationValue::Degrees(rotate));
        values.insert("borderRadius".to_string(), AnimationValue::Pixels(50.0 + (progress * 2.0 * std::f64::consts::PI).sin() * 30.0));
        values
    };
    
    view! {
        <MotionDiv
            animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
            node_ref=node_ref
            style="
                width: 150px;
                height: 150px;
                background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                font-weight: bold;
                font-size: 1.2rem;
            ".to_string()
        >
            "Morphing"
        </MotionDiv>
    }
}

#[component]
fn ElasticBounceDemo(is_playing: bool) -> impl IntoView {
    let (bounce_height, set_bounce_height) = signal(0.0);
    let node_ref = NodeRef::new();
    
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_bounce_height.update(|h| {
                    let new_h = *h + 0.1;
                    if new_h > 100.0 { 0.0 } else { new_h }
                });
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    let animate_values = move || {
        let height = bounce_height.get();
        let scale = 1.0 + (height / 100.0) * 0.5;
        
        let mut values = HashMap::new();
        values.insert("y".to_string(), AnimationValue::Pixels(-height));
        values.insert("scale".to_string(), AnimationValue::Number(scale));
        values
    };
    
    view! {
        <MotionDiv
            animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
            node_ref=node_ref
            style="
                width: 100px;
                height: 100px;
                background: linear-gradient(45deg, #ff9a9e, #fecfef);
                border-radius: 50%;
                display: flex;
                align-items: center;
                justify-content: center;
                color: white;
                font-weight: bold;
            ".to_string()
        >
            "Bounce!"
        </MotionDiv>
    }
}

#[component]
fn WaveAnimationDemo(is_playing: bool) -> impl IntoView {
    let (wave_offset, set_wave_offset) = signal(0.0);
    
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_wave_offset.update(|o| *o += 0.1);
            }, std::time::Duration::from_millis(16));
            
            move || {
                if let Ok(handle) = interval {
                    handle.clear();
                }
            }
        }
    });
    
    view! {
        <div style="position: relative; width: 400px; height: 200px;">
            {move || (0..20).map(|i| {
                let animate_values = move || {
                    let x = i as f64 * 20.0;
                    let y = 100.0 + (wave_offset.get() + i as f64 * 0.3).sin() * 50.0;
                    
                    let mut values = HashMap::new();
                    values.insert("x".to_string(), AnimationValue::Pixels(x));
                    values.insert("y".to_string(), AnimationValue::Pixels(y));
                    values
                };
                
                let node_ref = NodeRef::new();
                
                view! {
                    <MotionDiv
                        animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                        node_ref=node_ref
                        style="
                            position: absolute;
                            width: 8px;
                            height: 8px;
                            background: hsl(200, 70%, 60%);
                            border-radius: 50%;
                            left: 0;
                            top: 0;
                        ".to_string()
                    />
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn GravityPhysicsDemo(is_playing: bool) -> impl IntoView {
    let (ball_y, set_ball_y) = signal(0.0);
    let (velocity, set_velocity) = signal(0.0);
    let node_ref = NodeRef::new();
    
    Effect::new(move |_| {
        if is_playing {
            let interval = set_interval_with_handle(move || {
                set_ball_y.update(|y| {
                    set_velocity.update(|v| {
                        let new_v = *v + 0.5; // gravity
                        *y + new_v
                    })
                });
                
                // Reset when ball hits bottom
                if ball_y.get() > 300.0 {
                    set_ball_y.set(0.0);
                    set_velocity.set(0.0);
                }
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
        values.insert("y".to_string(), AnimationValue::Pixels(ball_y.get()));
        values
    };
    
    view! {
        <div style="position: relative; width: 200px; height: 400px; border: 2px solid rgba(255,255,255,0.3); border-radius: 10px;">
            <MotionDiv
                animate=AnimateProp::Fn(std::rc::Rc::new(animate_values))
                node_ref=node_ref
                style="
                    position: absolute;
                    width: 30px;
                    height: 30px;
                    background: linear-gradient(45deg, #ff6b6b, #ffa726);
                    border-radius: 50%;
                    left: 85px;
                    top: 0;
                ".to_string()
            />
        </div>
    }
}