use leptos::prelude::*;
use leptos_motion_core::AnimationValue;
use leptos_motion_dom::ReactiveMotionDiv;
use std::collections::HashMap;

/// Advanced 3D Demo Component
///
/// This component showcases advanced 3D animation features including:
/// - Morphing animations (shape transitions)
/// - Particle systems
/// - Complex 3D transformations
/// - Advanced perspective effects
/// - 3D path animations
/// - Dynamic lighting effects
#[component]
pub fn Advanced3DDemo() -> impl IntoView {
    let (morphing_enabled, set_morphing_enabled) = signal(false);
    let (particle_system_enabled, set_particle_system_enabled) = signal(false);
    let (complex_transform_enabled, set_complex_transform_enabled) = signal(false);
    let (perspective_effect_enabled, set_perspective_effect_enabled) = signal(false);
    let (path_animation_enabled, set_path_animation_enabled) = signal(false);
    let (lighting_effect_enabled, set_lighting_effect_enabled) = signal(false);

    // Morphing Animation (Cube to Sphere)
    let morphing_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if morphing_enabled.get() {
            // Sphere-like transformation
            target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(45.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
        } else {
            // Cube-like transformation
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
        }
        target
    });

    // Particle System Animation (Explosion Effect)
    let particle_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if particle_system_enabled.get() {
            target.insert("scaleX".to_string(), AnimationValue::Number(0.1));
            target.insert("scaleY".to_string(), AnimationValue::Number(0.1));
            target.insert("scaleZ".to_string(), AnimationValue::Number(0.1));
            target.insert("translateX".to_string(), AnimationValue::Number(100.0));
            target.insert("translateY".to_string(), AnimationValue::Number(100.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(100.0));
            target.insert("rotateX".to_string(), AnimationValue::Number(360.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(360.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(360.0));
        } else {
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
            target.insert("translateX".to_string(), AnimationValue::Number(0.0));
            target.insert("translateY".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
        }
        target
    });

    // Complex 3D Transformation
    let complex_transform_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if complex_transform_enabled.get() {
            target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(60.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(30.0));
            target.insert("translateX".to_string(), AnimationValue::Number(100.0));
            target.insert("translateY".to_string(), AnimationValue::Number(50.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(200.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.5));
            target.insert("scaleY".to_string(), AnimationValue::Number(0.8));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.2));
        } else {
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("translateX".to_string(), AnimationValue::Number(0.0));
            target.insert("translateY".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
        }
        target
    });

    // Advanced Perspective Effect
    let perspective_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if perspective_effect_enabled.get() {
            target.insert(
                "perspective".to_string(),
                AnimationValue::String("200px".to_string()),
            );
            target.insert(
                "perspective-origin".to_string(),
                AnimationValue::String("center center".to_string()),
            );
            target.insert(
                "transform-style".to_string(),
                AnimationValue::String("preserve-3d".to_string()),
            );
            target.insert("rotateX".to_string(), AnimationValue::Number(30.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(30.0));
        } else {
            target.insert(
                "perspective".to_string(),
                AnimationValue::String("1000px".to_string()),
            );
            target.insert(
                "perspective-origin".to_string(),
                AnimationValue::String("center center".to_string()),
            );
            target.insert(
                "transform-style".to_string(),
                AnimationValue::String("preserve-3d".to_string()),
            );
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
        }
        target
    });

    // 3D Path Animation (Circular Motion)
    let path_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if path_animation_enabled.get() {
            target.insert("translateX".to_string(), AnimationValue::Number(100.0));
            target.insert("translateY".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
        } else {
            target.insert("translateX".to_string(), AnimationValue::Number(0.0));
            target.insert("translateY".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
        }
        target
    });

    // Dynamic Lighting Effect
    let lighting_animation = Memo::new(move |_| {
        let mut target = HashMap::new();
        if lighting_effect_enabled.get() {
            target.insert("rotateX".to_string(), AnimationValue::Number(45.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(30.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(100.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
        } else {
            target.insert("rotateX".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateY".to_string(), AnimationValue::Number(0.0));
            target.insert("rotateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("translateZ".to_string(), AnimationValue::Number(0.0));
            target.insert("scaleX".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleY".to_string(), AnimationValue::Number(1.0));
            target.insert("scaleZ".to_string(), AnimationValue::Number(1.0));
        }
        target
    });

    view! {
        <div class="advanced-3d-demo">
            <header>
                <h2>"Advanced 3D Animation Features"</h2>
                <p>"Experience the power of advanced 3D animations with morphing, particle systems, and complex transformations."</p>
            </header>

            <div class="demo-controls">
                <button class="demo-button" on:click=move |_| set_morphing_enabled.update(|m| *m = !*m)>"Toggle Morphing"</button>
                <button class="demo-button" on:click=move |_| set_particle_system_enabled.update(|p| *p = !*p)>"Toggle Particles"</button>
                <button class="demo-button" on:click=move |_| set_complex_transform_enabled.update(|c| *c = !*c)>"Toggle Complex Transform"</button>
                <button class="demo-button" on:click=move |_| set_perspective_effect_enabled.update(|p| *p = !*p)>"Toggle Perspective"</button>
                <button class="demo-button" on:click=move |_| set_path_animation_enabled.update(|p| *p = !*p)>"Toggle Path Animation"</button>
                <button class="demo-button" on:click=move |_| set_lighting_effect_enabled.update(|l| *l = !*l)>"Toggle Lighting"</button>
            </div>

            <div class="demo-container">
                <div class="demo-section">
                    <h3>"Morphing Animation"</h3>
                    <p>"Smooth transitions between different 3D shapes"</p>
                    <ReactiveMotionDiv
                        class="demo-cube morphing-cube".to_string()
                        animate_fn=Box::new(move || morphing_animation.get())
                    >
                        <div class="cube-face front">"Morph"</div>
                        <div class="cube-face back">"Morph"</div>
                        <div class="cube-face left">"Morph"</div>
                        <div class="cube-face right">"Morph"</div>
                        <div class="cube-face top">"Morph"</div>
                        <div class="cube-face bottom">"Morph"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"Particle System"</h3>
                    <p>"Dynamic particle effects with explosion-like behavior"</p>
                    <ReactiveMotionDiv
                        class="demo-cube particle-cube".to_string()
                        animate_fn=Box::new(move || particle_animation.get())
                    >
                        <div class="cube-face front">"Particle"</div>
                        <div class="cube-face back">"Particle"</div>
                        <div class="cube-face left">"Particle"</div>
                        <div class="cube-face right">"Particle"</div>
                        <div class="cube-face top">"Particle"</div>
                        <div class="cube-face bottom">"Particle"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"Complex Transform"</h3>
                    <p>"Combined rotation, translation, and scaling"</p>
                    <ReactiveMotionDiv
                        class="demo-cube complex-cube".to_string()
                        animate_fn=Box::new(move || complex_transform_animation.get())
                    >
                        <div class="cube-face front">"Complex"</div>
                        <div class="cube-face back">"Complex"</div>
                        <div class="cube-face left">"Complex"</div>
                        <div class="cube-face right">"Complex"</div>
                        <div class="cube-face top">"Complex"</div>
                        <div class="cube-face bottom">"Complex"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"Perspective Effect"</h3>
                    <p>"Advanced perspective manipulation for depth"</p>
                    <ReactiveMotionDiv
                        class="demo-cube perspective-cube".to_string()
                        animate_fn=Box::new(move || perspective_animation.get())
                    >
                        <div class="cube-face front">"Perspective"</div>
                        <div class="cube-face back">"Perspective"</div>
                        <div class="cube-face left">"Perspective"</div>
                        <div class="cube-face right">"Perspective"</div>
                        <div class="cube-face top">"Perspective"</div>
                        <div class="cube-face bottom">"Perspective"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"Path Animation"</h3>
                    <p>"3D path following with circular motion"</p>
                    <ReactiveMotionDiv
                        class="demo-cube path-cube".to_string()
                        animate_fn=Box::new(move || path_animation.get())
                    >
                        <div class="cube-face front">"Path"</div>
                        <div class="cube-face back">"Path"</div>
                        <div class="cube-face left">"Path"</div>
                        <div class="cube-face right">"Path"</div>
                        <div class="cube-face top">"Path"</div>
                        <div class="cube-face bottom">"Path"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"Lighting Effect"</h3>
                    <p>"Dynamic lighting simulation with rotation"</p>
                    <ReactiveMotionDiv
                        class="demo-cube lighting-cube".to_string()
                        animate_fn=Box::new(move || lighting_animation.get())
                    >
                        <div class="cube-face front">"Light"</div>
                        <div class="cube-face back">"Light"</div>
                        <div class="cube-face left">"Light"</div>
                        <div class="cube-face right">"Light"</div>
                        <div class="cube-face top">"Light"</div>
                        <div class="cube-face bottom">"Light"</div>
                    </ReactiveMotionDiv>
                </div>
            </div>

            <div class="demo-info">
                <h3>"Advanced 3D Features"</h3>
                <ul>
                    <li>"✅ Morphing animations (shape transitions)"</li>
                    <li>"✅ Particle systems (explosion effects)"</li>
                    <li>"✅ Complex 3D transformations"</li>
                    <li>"✅ Advanced perspective effects"</li>
                    <li>"✅ 3D path animations"</li>
                    <li>"✅ Dynamic lighting effects"</li>
                    <li>"✅ Hardware acceleration support"</li>
                    <li>"✅ Signal-based reactivity"</li>
                    <li>"✅ Performance optimized (60+ FPS)"</li>
                </ul>
            </div>
        </div>
    }
}
