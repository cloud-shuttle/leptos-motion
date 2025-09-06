use leptos::*;
use leptos_motion::*;

#[component]
fn App() -> impl IntoView {
    view! {
            <div>
                <header style="
                background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                color: white;
                padding: 2rem;
                text-align: center;
            ">
                    <h1 style="font-size: 3rem; margin-bottom: 1rem;">"🎭 Leptos Motion"</h1>
                    <p style="font-size: 1.2rem; opacity: 0.9;">
                        "Animation library for Leptos with Motion-inspired API"
                    </p>
                </header>

                <main style="max-width: 1200px; margin: 0 auto; padding: 2rem;">
                    <section style="margin-bottom: 4rem;">
                        <h2 style="text-align: center; margin-bottom: 2rem; color: #333;">
                            "Interactive Examples"
                        </h2>

                        <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 2rem;">
                            <InteractiveButton />
                            <AnimatedCard />
                            <SimpleAnimation />
                        </div>
                    </section>

                    <section style="margin-bottom: 4rem;">
                        <h2 style="text-align: center; margin-bottom: 2rem; color: #333;">
                            "Installation"
                        </h2>

                        <div style="
                        background: #f8f9fa;
                        border-radius: 8px;
                        padding: 2rem;
                        font-family: 'Monaco', 'Menlo', monospace;
                        font-size: 0.9rem;
                        overflow-x: auto;
                    ">
                            <pre><code># Add to your Cargo.toml
    [dependencies]
    leptos = "0.7"
    leptos-motion = "0.1.0-alpha"

    # Basic usage
    use leptos::prelude::*;
    use leptos_motion::*;

    #[component]
    fn MyComponent() -> impl IntoView {
        view! {
            <MotionDiv
                animate=motion_target!(
                    "scale" => AnimationValue::Number(1.1)
                )
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.2)
                )
            >
                "Hover me!"
            </MotionDiv>
        }
    }</code></pre>
                        </div>
                    </section>

                    <section style="text-align: center;">
                        <h2 style="margin-bottom: 1rem; color: #333;">
                            "Ready to get started?"
                        </h2>
                        <p style="margin-bottom: 2rem; color: #666;">
                            "Check out our documentation and examples to learn more."
                        </p>
                        <div style="display: flex; gap: 1rem; justify-content: center;">
                            <a href="https://github.com/cloud-shuttle/leptos-motion"
                               style="
                               background: #3b82f6;
                               color: white;
                               padding: 0.75rem 1.5rem;
                               border-radius: 6px;
                               text-decoration: none;
                               font-weight: 600;
                           "
                               target="_blank">
                                "View on GitHub"
                            </a>
                            <a href="https://crates.io/crates/leptos-motion"
                               style="
                               background: #10b981;
                               color: white;
                               padding: 0.75rem 1.5rem;
                               border-radius: 6px;
                               text-decoration: none;
                               font-weight: 600;
                           "
                               target="_blank">
                                "Crates.io"
                            </a>
                        </div>
                    </section>
                </main>

                <footer style="
                background: #1f2937;
                color: white;
                text-align: center;
                padding: 2rem;
                margin-top: 4rem;
            ">
                    <p>"Built with ❤️ for the Rust community"</p>
                    <p style="opacity: 0.8; margin-top: 0.5rem;">"Inspired by Framer Motion"</p>
                </footer>
            </div>
        }
}

#[component]
fn InteractiveButton() -> impl IntoView {
    let (is_pressed, set_is_pressed) = create_signal(false);

    view! {
        <div style="
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            text-align: center;
        ">
            <h3 style="margin-bottom: 1rem; color: #333;">"Interactive Button"</h3>
            <MotionDiv
                on:click=move |_| set_is_pressed.update(|p| *p = !*p)
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.05),
                    "y" => AnimationValue::Pixels(-2.0)
                )
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(0.95)
                )
                style=move || {
                    if is_pressed.get() {
                        "padding: 1rem 2rem; background-color: #10b981; color: white; border-radius: 8px; cursor: pointer; user-select: none; font-weight: 600; border: none; font-size: 1rem;".to_string()
                    } else {
                        "padding: 1rem 2rem; background-color: #3b82f6; color: white; border-radius: 8px; cursor: pointer; user-select: none; font-weight: 600; border: none; font-size: 1rem;".to_string()
                    }
                }
            >
                {move || if is_pressed.get() { "Pressed! 🎉" } else { "Click me! ✨" }}
            </MotionDiv>
        </div>
    }
}

#[component]
fn AnimatedCard() -> impl IntoView {
    view! {
        <div style="
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            text-align: center;
        ">
            <h3 style="margin-bottom: 1rem; color: #333;">"Animated Card"</h3>
            <MotionDiv
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "y" => AnimationValue::Pixels(30.0)
                )
                while_in_view=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "y" => AnimationValue::Pixels(0.0)
                )
                while_hover=motion_target!(
                    "y" => AnimationValue::Pixels(-10.0),
                    "scale" => AnimationValue::Number(1.02)
                )
                style="
                    padding: 2rem;
                    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
                    color: white;
                    border-radius: 8px;
                    cursor: pointer;
                ".to_string()
            >
                <h4 style="margin-bottom: 0.5rem;">"Hover Effect"</h4>
                <p style="opacity: 0.9;">"This card animates in and lifts on hover"</p>
            </MotionDiv>
        </div>
    }
}

#[component]
fn SimpleAnimation() -> impl IntoView {
    view! {
        <div style="
            background: white;
            border-radius: 12px;
            padding: 2rem;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
            text-align: center;
        ">
            <h3 style="margin-bottom: 1rem; color: #333;">"Simple Animation"</h3>
            <MotionDiv
                animate=motion_target!(
                    "x" => AnimationValue::Pixels(100.0),
                    "rotate" => AnimationValue::Degrees(360.0)
                )
                transition=Transition {
                    duration: Some(2.0),
                    ease: Easing::EaseInOut,
                    repeat: Some(Repeat::Infinitely),
                    ..Default::default()
                }
                style="
                    width: 60px;
                    height: 60px;
                    background-color: #ec4899;
                    border-radius: 50%;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    color: white;
                    font-weight: bold;
                    font-size: 1.2rem;
                ".to_string()
            >
                "🎯"
            </MotionDiv>
            <p style="margin-top: 1rem; color: #666; font-size: 0.9rem;">
                "Continuous animation with rotation"
            </p>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}
