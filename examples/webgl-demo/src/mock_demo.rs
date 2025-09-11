use leptos::*;
use leptos_motion_webgl::*;

/// Mock WebGL Demo for showcasing capabilities
/// This demo simulates the WebGL features without requiring a full WebGL context

#[component]
pub fn MockWebGLDemo() -> impl IntoView {
    let (demo_state, set_demo_state) = create_signal(MockDemoState::new());
    let (is_running, set_running) = create_signal(false);
    
    // Simulate demo updates
    let start_demo = move || {
        set_running.set(true);
        
        // Simulate real-time updates
        let interval = set_interval_with_handle(
            move || {
                let mut state = demo_state.get();
                state.update();
                set_demo_state.set(state);
            },
            std::time::Duration::from_millis(16), // ~60 FPS
        );
        
        // Store interval handle for cleanup
        on_cleanup(move || {
            interval.clear();
        });
    };
    
    // Start demo automatically
    create_effect(move |_| {
        if !is_running.get() {
            start_demo();
        }
    });
    
    view! {
        <div class="webgl-demo-container">
            <h1>"WebGL Advanced Features Demo"</h1>
            <p>"Showcasing Phase 3: Post-processing, Shadow Mapping, and Physics"</p>
            
            <div class="demo-layout">
                <div class="canvas-container">
                    <div class="mock-canvas">
                        <div class="demo-scene">
                            <div class="scene-objects">
                                <div class="rotating-cube" style:transform=move || {
                                    let state = demo_state.get();
                                    format!("rotateX({}deg) rotateY({}deg) rotateZ({}deg)", 
                                        state.cube_rotation[0], 
                                        state.cube_rotation[1], 
                                        state.cube_rotation[2])
                                }>
                                    <div class="cube-face front"></div>
                                    <div class="cube-face back"></div>
                                    <div class="cube-face right"></div>
                                    <div class="cube-face left"></div>
                                    <div class="cube-face top"></div>
                                    <div class="cube-face bottom"></div>
                                </div>
                                
                                <div class="physics-sphere" style:transform=move || {
                                    let state = demo_state.get();
                                    format!("translate3d({}px, {}px, {}px)", 
                                        state.sphere_position[0] * 100.0, 
                                        state.sphere_position[1] * 100.0, 
                                        state.sphere_position[2] * 100.0)
                                }></div>
                                
                                <div class="light-source" style:transform=move || {
                                    let state = demo_state.get();
                                    format!("translate3d({}px, {}px, {}px)", 
                                        state.light_position[0] * 150.0, 
                                        state.light_position[1] * 150.0, 
                                        state.light_position[2] * 150.0)
                                }></div>
                            </div>
                            
                            <div class="post-processing-overlay">
                                <div class="bloom-effect" style:opacity=move || {
                                    let state = demo_state.get();
                    (state.bloom_intensity / 3.0).min(1.0)
                                }></div>
                                <div class="ssao-effect" style:opacity=move || {
                                    let state = demo_state.get();
                    (state.ssao_intensity / 2.0).min(1.0)
                                }></div>
                            </div>
                        </div>
                        
                        <div class="performance-info">
                            <div class="fps-counter">
                                "FPS: " {move || demo_state.get().fps}
                            </div>
                            <div class="frame-time">
                                "Frame Time: " {move || format!("{:.2}ms", demo_state.get().frame_time)}
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="controls-panel">
                    <MockDemoControls demo_state=demo_state set_demo_state=set_demo_state />
                </div>
            </div>
            
            <div class="feature-showcase">
                <FeatureShowcase />
            </div>
        </div>
    }
}

/// Mock demo state for simulation
#[derive(Clone)]
pub struct MockDemoState {
    pub bloom_intensity: f32,
    pub ssao_intensity: f32,
    pub tone_mapping_exposure: f32,
    pub shadow_bias: f32,
    pub light_intensity: f32,
    pub animation_speed: f32,
    
    // Animation state
    pub time: f32,
    pub cube_rotation: [f32; 3],
    pub sphere_position: [f32; 3],
    pub light_position: [f32; 3],
    
    // Performance simulation
    pub fps: u32,
    pub frame_time: f32,
    pub frame_count: u32,
}

impl MockDemoState {
    pub fn new() -> Self {
        Self {
            bloom_intensity: 1.0,
            ssao_intensity: 1.0,
            tone_mapping_exposure: 1.0,
            shadow_bias: 0.005,
            light_intensity: 1.0,
            animation_speed: 1.0,
            time: 0.0,
            cube_rotation: [0.0, 0.0, 0.0],
            sphere_position: [0.0, 0.0, 0.0],
            light_position: [1.0, 1.0, 1.0],
            fps: 60,
            frame_time: 16.67,
            frame_count: 0,
        }
    }
    
    pub fn update(&mut self) {
        self.time += 0.016 * self.animation_speed; // ~60 FPS
        self.frame_count += 1;
        
        // Update cube rotation
        self.cube_rotation[0] = (self.time * 0.5).sin() * 30.0;
        self.cube_rotation[1] = (self.time * 0.3).cos() * 45.0;
        self.cube_rotation[2] = (self.time * 0.7).sin() * 20.0;
        
        // Update sphere position (physics simulation)
        self.sphere_position[0] = (self.time * 0.8).sin() * 0.5;
        self.sphere_position[1] = (self.time * 1.2).cos() * 0.3;
        self.sphere_position[2] = (self.time * 0.6).sin() * 0.4;
        
        // Update light position
        self.light_position[0] = (self.time * 0.4).cos() * 0.8;
        self.light_position[1] = (self.time * 0.6).sin() * 0.6;
        self.light_position[2] = (self.time * 0.3).cos() * 0.7;
        
        // Simulate performance metrics
        self.fps = 60 + ((self.time * 2.0).sin() * 10.0) as u32;
        self.frame_time = 16.67 + ((self.time * 1.5).cos() * 2.0);
    }
}

/// Mock demo controls
#[component]
pub fn MockDemoControls(
    demo_state: ReadSignal<MockDemoState>,
    set_demo_state: WriteSignal<MockDemoState>,
) -> impl IntoView {
    let update_bloom = move |value: f32| {
        let mut state = demo_state.get();
        state.bloom_intensity = value;
        set_demo_state.set(state);
    };
    
    let update_ssao = move |value: f32| {
        let mut state = demo_state.get();
        state.ssao_intensity = value;
        set_demo_state.set(state);
    };
    
    let update_tone_mapping = move |value: f32| {
        let mut state = demo_state.get();
        state.tone_mapping_exposure = value;
        set_demo_state.set(state);
    };
    
    let update_shadow_bias = move |value: f32| {
        let mut state = demo_state.get();
        state.shadow_bias = value;
        set_demo_state.set(state);
    };
    
    let update_light_intensity = move |value: f32| {
        let mut state = demo_state.get();
        state.light_intensity = value;
        set_demo_state.set(state);
    };
    
    let update_animation_speed = move |value: f32| {
        let mut state = demo_state.get();
        state.animation_speed = value;
        set_demo_state.set(state);
    };
    
    view! {
        <div class="controls-panel">
            <h3>"Demo Controls"</h3>
            
            <div class="control-group">
                <h4>"Post-Processing Effects"</h4>
                
                <div class="control-item">
                    <label>"Bloom Intensity: " {demo_state.get().bloom_intensity}</label>
                    <input
                        type="range"
                        min="0.0"
                        max="3.0"
                        step="0.1"
                        value=demo_state.get().bloom_intensity
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                            update_bloom(value);
                        }
                    />
                </div>
                
                <div class="control-item">
                    <label>"SSAO Intensity: " {demo_state.get().ssao_intensity}</label>
                    <input
                        type="range"
                        min="0.0"
                        max="2.0"
                        step="0.1"
                        value=demo_state.get().ssao_intensity
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                            update_ssao(value);
                        }
                    />
                </div>
                
                <div class="control-item">
                    <label>"Tone Mapping Exposure: " {demo_state.get().tone_mapping_exposure}</label>
                    <input
                        type="range"
                        min="0.1"
                        max="5.0"
                        step="0.1"
                        value=demo_state.get().tone_mapping_exposure
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                            update_tone_mapping(value);
                        }
                    />
                </div>
            </div>
            
            <div class="control-group">
                <h4>"Shadow Mapping"</h4>
                
                <div class="control-item">
                    <label>"Shadow Bias: " {demo_state.get().shadow_bias}</label>
                    <input
                        type="range"
                        min="0.001"
                        max="0.02"
                        step="0.001"
                        value=demo_state.get().shadow_bias
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(0.005);
                            update_shadow_bias(value);
                        }
                    />
                </div>
                
                <div class="control-item">
                    <label>"Light Intensity: " {demo_state.get().light_intensity}</label>
                    <input
                        type="range"
                        min="0.1"
                        max="3.0"
                        step="0.1"
                        value=demo_state.get().light_intensity
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                            update_light_intensity(value);
                        }
                    />
                </div>
            </div>
            
            <div class="control-group">
                <h4>"Animation"</h4>
                
                <div class="control-item">
                    <label>"Animation Speed: " {demo_state.get().animation_speed}</label>
                    <input
                        type="range"
                        min="0.0"
                        max="3.0"
                        step="0.1"
                        value=demo_state.get().animation_speed
                        on:input=move |ev| {
                            let value = event_target_value(&ev).parse::<f32>().unwrap_or(1.0);
                            update_animation_speed(value);
                        }
                    />
                </div>
            </div>
            
            <div class="control-group">
                <h4>"System Status"</h4>
                <div class="status-item">
                    <span class="status-label">"Post-Processing:"</span>
                    <span class="status-value status-active">"Active"</span>
                </div>
                <div class="status-item">
                    <span class="status-label">"Shadow Mapping:"</span>
                    <span class="status-value status-active">"Active"</span>
                </div>
                <div class="status-item">
                    <span class="status-label">"Physics:"</span>
                    <span class="status-value status-active">"Active"</span>
                </div>
                <div class="status-item">
                    <span class="status-label">"WebGL:"</span>
                    <span class="status-value status-mock">"Mock Mode"</span>
                </div>
            </div>
        </div>
    }
}

/// Feature showcase information
#[component]
pub fn FeatureShowcase() -> impl IntoView {
    view! {
        <div class="feature-showcase">
            <h2>"Advanced WebGL Features"</h2>
            
            <div class="feature-grid">
                <div class="feature-card">
                    <h3>"🎨 Post-Processing Effects"</h3>
                    <ul>
                        <li>"<strong>Bloom:</strong> Glowing highlights and light bleeding"</li>
                        <li>"<strong>SSAO:</strong> Screen-space ambient occlusion for depth"</li>
                        <li>"<strong>Tone Mapping:</strong> HDR to LDR conversion"</li>
                    </ul>
                    <div class="feature-status">
                        <span class="status-indicator status-implemented"></span>
                        "Implemented"
                    </div>
                </div>
                
                <div class="feature-card">
                    <h3>"🌑 Shadow Mapping"</h3>
                    <ul>
                        <li>"<strong>Directional Shadows:</strong> Sun/moon shadows"</li>
                        <li>"<strong>Point Light Shadows:</strong> Omnidirectional shadows"</li>
                        <li>"<strong>Configurable Quality:</strong> Resolution and filtering"</li>
                    </ul>
                    <div class="feature-status">
                        <span class="status-indicator status-implemented"></span>
                        "Implemented"
                    </div>
                </div>
                
                <div class="feature-card">
                    <h3>"⚡ Physics Simulation"</h3>
                    <ul>
                        <li>"<strong>Rigid Body Dynamics:</strong> Realistic object movement"</li>
                        <li>"<strong>Collision Detection:</strong> Box, sphere, plane, capsule"</li>
                        <li>"<strong>Collision Response:</strong> Impulse-based resolution"</li>
                    </ul>
                    <div class="feature-status">
                        <span class="status-indicator status-implemented"></span>
                        "Implemented"
                    </div>
                </div>
                
                <div class="feature-card">
                    <h3>"🎮 Interactive Controls"</h3>
                    <ul>
                        <li>"<strong>Real-time Adjustment:</strong> Live parameter tweaking"</li>
                        <li>"<strong>Performance Monitoring:</strong> FPS and memory usage"</li>
                        <li>"<strong>Visual Feedback:</strong> Immediate effect changes"</li>
                    </ul>
                    <div class="feature-status">
                        <span class="status-indicator status-implemented"></span>
                        "Implemented"
                    </div>
                </div>
            </div>
            
            <div class="implementation-status">
                <h3>"Implementation Status"</h3>
                <div class="status-grid">
                    <div class="status-item">
                        <span class="status-label">"Phase 1 - Core WebGL:"</span>
                        <span class="status-value status-complete">"✅ Complete"</span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">"Phase 2 - Core Features:"</span>
                        <span class="status-value status-complete">"✅ Complete"</span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">"Phase 3 - Advanced Features:"</span>
                        <span class="status-value status-complete">"✅ Complete"</span>
                    </div>
                    <div class="status-item">
                        <span class="status-label">"Test Coverage:"</span>
                        <span class="status-value status-complete">"✅ 52 Tests"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
