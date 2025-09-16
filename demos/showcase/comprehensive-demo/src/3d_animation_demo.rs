use leptos::prelude::*;
use leptos_motion_dom::animation_3d_implementation::{
    Animation3D, Perspective3D, Transform3D, utils,
};
use leptos_motion_dom::{
    Easing, ReactiveMotionDiv, RepeatConfig, StaggerConfig, StaggerFrom, Transition,
};

/// 3D Animation Demo Component
///
/// This component showcases the new 3D animation capabilities of Leptos Motion.
/// It demonstrates 3D transforms, perspective, and advanced 3D animations.
#[component]
pub fn Animation3DDemo() -> impl IntoView {
    let (is_rotating, set_is_rotating) = signal(false);
    let (is_scaling, set_is_scaling) = signal(false);
    let (is_translating, set_is_translating) = signal(false);
    let (is_perspective, set_is_perspective) = signal(false);
    let (is_matrix3d, set_is_matrix3d) = signal(false);

    // 3D Rotation Animation
    let rotation_animation = Memo::new(move |_| {
        if is_rotating.get() {
            Animation3D::rotate_3d(360.0, 180.0, 90.0)
                .transition(Transition {
                    duration: Some(2.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Infinite,
                    stagger: None,
                })
                .to_animation_target()
        } else {
            Animation3D::rotate_3d(0.0, 0.0, 0.0)
                .transition(Transition {
                    duration: Some(1.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                .to_animation_target()
        }
    });

    // 3D Scale Animation
    let scale_animation = Memo::new(move |_| {
        if is_scaling.get() {
            Animation3D::scale_3d(1.5, 1.5, 1.5)
                .transition(Transition {
                    duration: Some(1.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Count(3),
                    stagger: None,
                })
                .to_animation_target()
        } else {
            Animation3D::scale_3d(1.0, 1.0, 1.0)
                .transition(Transition {
                    duration: Some(0.5),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                .to_animation_target()
        }
    });

    // 3D Translation Animation
    let translation_animation = Memo::new(move |_| {
        if is_translating.get() {
            Animation3D::translate_3d(100.0, 100.0, 100.0)
                .transition(Transition {
                    duration: Some(1.5),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::InfiniteReverse,
                    stagger: None,
                })
                .to_animation_target()
        } else {
            Animation3D::translate_3d(0.0, 0.0, 0.0)
                .transition(Transition {
                    duration: Some(1.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                .to_animation_target()
        }
    });

    // 3D Perspective Animation
    let perspective_animation = Memo::new(move |_| {
        if is_perspective.get() {
            Animation3D::with_perspective("500px")
                .transform(Transform3D::new().rotate_x(45.0).rotate_y(45.0))
                .transition(Transition {
                    duration: Some(2.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                .to_animation_target()
        } else {
            Animation3D::with_perspective("1000px")
                .transform(Transform3D::new().rotate_x(0.0).rotate_y(0.0))
                .transition(Transition {
                    duration: Some(1.0),
                    delay: Some(0.0),
                    ease: Easing::EaseInOut,
                    repeat: RepeatConfig::Never,
                    stagger: None,
                })
                .to_animation_target()
        }
    });

    // Matrix3D Animation
    let matrix3d_animation = Memo::new(move |_| {
        if is_matrix3d.get() {
            utils::matrix3d_animation_target(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 50.0, 50.0, 50.0, 1.0,
            )
        } else {
            utils::matrix3d_animation_target(
                1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            )
        }
    });

    view! {
        <div class="3d-animation-demo">
            <header>
                <h2>"3D Animation System"</h2>
                <p>"Experience the power of 3D animations with Leptos Motion"</p>
            </header>

            <div class="demo-controls">
                <button
                    class="demo-button"
                    on:click=move |_| set_is_rotating.update(|v| *v = !*v)
                >
                    {move || if is_rotating.get() { "Stop Rotation" } else { "Start Rotation" }}
                </button>

                <button
                    class="demo-button"
                    on:click=move |_| set_is_scaling.update(|v| *v = !*v)
                >
                    {move || if is_scaling.get() { "Stop Scaling" } else { "Start Scaling" }}
                </button>

                <button
                    class="demo-button"
                    on:click=move |_| set_is_translating.update(|v| *v = !*v)
                >
                    {move || if is_translating.get() { "Stop Translation" } else { "Start Translation" }}
                </button>

                <button
                    class="demo-button"
                    on:click=move |_| set_is_perspective.update(|v| *v = !*v)
                >
                    {move || if is_perspective.get() { "Reset Perspective" } else { "Apply Perspective" }}
                </button>

                <button
                    class="demo-button"
                    on:click=move |_| set_is_matrix3d.update(|v| *v = !*v)
                >
                    {move || if is_matrix3d.get() { "Reset Matrix3D" } else { "Apply Matrix3D" }}
                </button>
            </div>

            <div class="demo-container">
                <div class="demo-section">
                    <h3>"3D Rotation"</h3>
                    <ReactiveMotionDiv
                        class="demo-cube rotation-cube"
                        animate=Some(rotation_animation)
                    >
                        <div class="cube-face front">"Front"</div>
                        <div class="cube-face back">"Back"</div>
                        <div class="cube-face left">"Left"</div>
                        <div class="cube-face right">"Right"</div>
                        <div class="cube-face top">"Top"</div>
                        <div class="cube-face bottom">"Bottom"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"3D Scaling"</h3>
                    <ReactiveMotionDiv
                        class="demo-cube scale-cube"
                        animate=Some(scale_animation)
                    >
                        <div class="cube-face front">"Scale"</div>
                        <div class="cube-face back">"Scale"</div>
                        <div class="cube-face left">"Scale"</div>
                        <div class="cube-face right">"Scale"</div>
                        <div class="cube-face top">"Scale"</div>
                        <div class="cube-face bottom">"Scale"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"3D Translation"</h3>
                    <ReactiveMotionDiv
                        class="demo-cube translation-cube"
                        animate=Some(translation_animation)
                    >
                        <div class="cube-face front">"Move"</div>
                        <div class="cube-face back">"Move"</div>
                        <div class="cube-face left">"Move"</div>
                        <div class="cube-face right">"Move"</div>
                        <div class="cube-face top">"Move"</div>
                        <div class="cube-face bottom">"Move"</div>
                    </ReactiveMotionDiv>
                </div>

                <div class="demo-section">
                    <h3>"3D Perspective"</h3>
                    <ReactiveMotionDiv
                        class="demo-cube perspective-cube"
                        animate=Some(perspective_animation)
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
                    <h3>"Matrix3D Transform"</h3>
                    <ReactiveMotionDiv
                        class="demo-cube matrix3d-cube"
                        animate=Some(matrix3d_animation)
                    >
                        <div class="cube-face front">"Matrix3D"</div>
                        <div class="cube-face back">"Matrix3D"</div>
                        <div class="cube-face left">"Matrix3D"</div>
                        <div class="cube-face right">"Matrix3D"</div>
                        <div class="cube-face top">"Matrix3D"</div>
                        <div class="cube-face bottom">"Matrix3D"</div>
                    </ReactiveMotionDiv>
                </div>
            </div>

            <div class="demo-info">
                <h3>"3D Animation Features"</h3>
                <ul>
                    <li>"✅ 3D Transform Properties (translateX, translateY, translateZ)"</li>
                    <li>"✅ 3D Rotation (rotateX, rotateY, rotateZ)"</li>
                    <li>"✅ 3D Scaling (scaleX, scaleY, scaleZ)"</li>
                    <li>"✅ 3D Perspective and Camera Controls"</li>
                    <li>"✅ Matrix3D Transformations"</li>
                    <li>"✅ Hardware-Accelerated 3D Animations"</li>
                    <li>"✅ Signal-Based 3D Animation Reactivity"</li>
                    <li>"✅ Performance-Optimized 3D Rendering"</li>
                </ul>
            </div>
        </div>
    }
}
