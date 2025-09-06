//! Visual regression tests for animation consistency

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test helper for creating consistent test elements
struct VisualTestHelper {
    document: web_sys::Document,
}

impl VisualTestHelper {
    fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Self { document }
    }

    fn create_test_element(&self, id: &str, class: &str) -> web_sys::Element {
        let element = self.document.create_element("div").unwrap();
        element.set_id(id);
        element.set_class_name(class);
        element
            .set_attribute(
                "style",
                "width: 100px; height: 100px; background: red; position: absolute;",
            )
            .unwrap();
        element
    }

    fn get_computed_style(&self, element: &web_sys::Element, property: &str) -> String {
        let window = web_sys::window().unwrap();
        let computed_style = window.get_computed_style(element).unwrap().unwrap();
        computed_style
            .get_property_value(property)
            .unwrap_or_default()
    }

    fn get_transform_matrix(&self, element: &web_sys::Element) -> String {
        self.get_computed_style(element, "transform")
    }

    fn get_opacity(&self, element: &web_sys::Element) -> f64 {
        self.get_computed_style(element, "opacity")
            .parse()
            .unwrap_or(1.0)
    }

    fn get_position(&self, element: &web_sys::Element) -> (f64, f64) {
        let left = self
            .get_computed_style(element, "left")
            .replace("px", "")
            .parse()
            .unwrap_or(0.0);
        let top = self
            .get_computed_style(element, "top")
            .replace("px", "")
            .parse()
            .unwrap_or(0.0);
        (left, top)
    }
}

#[wasm_bindgen_test]
async fn test_fade_in_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("fade-test", "fade-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Initial state
    let initial_opacity = helper.get_opacity(&element);
    assert_eq!(initial_opacity, 1.0);

    // Apply fade-in animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Final state
    let final_opacity = helper.get_opacity(&element);
    assert!(
        (final_opacity - 1.0).abs() < 0.01,
        "Opacity should be 1.0, got {}",
        final_opacity
    );
}

#[wasm_bindgen_test]
async fn test_slide_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("slide-test", "slide-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Initial position
    let (initial_x, initial_y) = helper.get_position(&element);

    // Apply slide animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "x" => AnimationValue::Pixels(-100.0)
            )
            animate=motion_target!(
                "x" => AnimationValue::Pixels(100.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Final position
    let (final_x, final_y) = helper.get_position(&element);
    assert!(
        (final_x - (initial_x + 200.0)).abs() < 1.0,
        "X position should be {} + 200, got {}",
        initial_x,
        final_x
    );
    assert_eq!(final_y, initial_y, "Y position should remain unchanged");
}

#[wasm_bindgen_test]
async fn test_scale_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("scale-test", "scale-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Initial transform
    let initial_transform = helper.get_transform_matrix(&element);

    // Apply scale animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "scale" => AnimationValue::Number(0.5)
            )
            animate=motion_target!(
                "scale" => AnimationValue::Number(2.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Final transform
    let final_transform = helper.get_transform_matrix(&element);
    assert!(
        final_transform.contains("scale(2)"),
        "Transform should contain scale(2), got {}",
        final_transform
    );
}

#[wasm_bindgen_test]
async fn test_rotation_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("rotate-test", "rotate-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Apply rotation animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "rotate" => AnimationValue::Degrees(0.0)
            )
            animate=motion_target!(
                "rotate" => AnimationValue::Degrees(360.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Final transform
    let final_transform = helper.get_transform_matrix(&element);
    assert!(
        final_transform.contains("rotate(360deg)"),
        "Transform should contain rotate(360deg), got {}",
        final_transform
    );
}

#[wasm_bindgen_test]
async fn test_complex_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("complex-test", "complex-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Apply complex animation (fade + slide + scale)
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-50.0),
                "scale" => AnimationValue::Number(0.5)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(50.0),
                "scale" => AnimationValue::Number(1.5)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Check final state
    let final_opacity = helper.get_opacity(&element);
    let (final_x, _) = helper.get_position(&element);
    let final_transform = helper.get_transform_matrix(&element);

    assert!(
        (final_opacity - 1.0).abs() < 0.01,
        "Opacity should be 1.0, got {}",
        final_opacity
    );
    assert!(
        final_transform.contains("scale(1.5)"),
        "Transform should contain scale(1.5), got {}",
        final_transform
    );
}

#[wasm_bindgen_test]
async fn test_spring_animation_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("spring-test", "spring-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Apply spring animation
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            initial=motion_target!(
                "x" => AnimationValue::Pixels(0.0)
            )
            animate=motion_target!(
                "x" => AnimationValue::Pixels(100.0)
            )
            transition=Transition {
                duration: None,
                ease: Easing::Spring(SpringConfig {
                    stiffness: 100.0,
                    damping: 10.0,
                    mass: 1.0,
                }),
                ..Default::default()
            }
        />
    };

    // Wait for spring to settle
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Final position should be close to target
    let (final_x, _) = helper.get_position(&element);
    assert!(
        (final_x - 100.0).abs() < 5.0,
        "Spring should settle near 100px, got {}",
        final_x
    );
}

#[wasm_bindgen_test]
async fn test_animation_interruption_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("interrupt-test", "interrupt-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Start first animation
    let motion_div1 = view! {
        <MotionDiv
            node_ref=element.clone()
            animate=motion_target!(
                "x" => AnimationValue::Pixels(200.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait a bit, then start second animation
    std::thread::sleep(std::time::Duration::from_millis(100));

    let motion_div2 = view! {
        <MotionDiv
            node_ref=element.clone()
            animate=motion_target!(
                "x" => AnimationValue::Pixels(0.0)
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for second animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Should end at 0, not 200
    let (final_x, _) = helper.get_position(&element);
    assert!(
        (final_x - 0.0).abs() < 1.0,
        "Should end at 0px, got {}",
        final_x
    );
}

#[wasm_bindgen_test]
async fn test_animation_presence_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("presence-test", "presence-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Test AnimatePresence
    let (is_visible, set_is_visible) = create_signal(true);

    let presence_div = view! {
        <AnimatePresence>
            {move || if is_visible.get() {
                view! {
                    <MotionDiv
                        node_ref=element.clone()
                        initial=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "scale" => AnimationValue::Number(0.0)
                        )
                        animate=motion_target!(
                            "opacity" => AnimationValue::Number(1.0),
                            "scale" => AnimationValue::Number(1.0)
                        )
                        exit=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "scale" => AnimationValue::Number(0.0)
                        )
                        transition=Transition {
                            duration: Some(0.1),
                            ease: Easing::Linear,
                            ..Default::default()
                        }
                    />
                }
            } else {
                view! { <div></div> }
            }}
        </AnimatePresence>
    };

    // Wait for enter animation
    std::thread::sleep(std::time::Duration::from_millis(150));

    let enter_opacity = helper.get_opacity(&element);
    assert!(
        (enter_opacity - 1.0).abs() < 0.01,
        "Enter opacity should be 1.0, got {}",
        enter_opacity
    );

    // Trigger exit
    set_is_visible.set(false);

    // Wait for exit animation
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Element should be removed from DOM or have opacity 0
    let exit_opacity = helper.get_opacity(&element);
    assert!(
        (exit_opacity - 0.0).abs() < 0.01,
        "Exit opacity should be 0.0, got {}",
        exit_opacity
    );
}

#[wasm_bindgen_test]
async fn test_animation_variants_visual_consistency() {
    let helper = VisualTestHelper::new();
    let element = helper.create_test_element("variants-test", "variants-element");

    // Add element to DOM
    let body = helper.document.body().unwrap();
    body.append_child(&element).unwrap();

    // Define variants
    let variants = Variants::new()
        .variant(
            "hidden",
            motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-100.0)
            ),
        )
        .variant(
            "visible",
            motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(0.0)
            ),
        );

    // Apply variants
    let motion_div = view! {
        <MotionDiv
            node_ref=element.clone()
            variants=variants
            initial="hidden"
            animate="visible"
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };

    // Wait for animation to complete
    std::thread::sleep(std::time::Duration::from_millis(150));

    // Check final state
    let final_opacity = helper.get_opacity(&element);
    let (final_x, _) = helper.get_position(&element);

    assert!(
        (final_opacity - 1.0).abs() < 0.01,
        "Opacity should be 1.0, got {}",
        final_opacity
    );
    assert!(
        (final_x - 0.0).abs() < 1.0,
        "X should be 0px, got {}",
        final_x
    );
}

#[wasm_bindgen_test]
async fn test_animation_stagger_visual_consistency() {
    let helper = VisualTestHelper::new();

    // Create multiple elements
    let elements: Vec<web_sys::Element> = (0..3)
        .map(|i| {
            let element = helper.create_test_element(&format!("stagger-{}", i), "stagger-element");
            helper
                .document
                .body()
                .unwrap()
                .append_child(&element)
                .unwrap();
            element
        })
        .collect();

    // Apply staggered animation
    let stagger_delay = 0.05;
    for (i, element) in elements.iter().enumerate() {
        let motion_div = view! {
            <MotionDiv
                node_ref=element.clone()
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "y" => AnimationValue::Pixels(50.0)
                )
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "y" => AnimationValue::Pixels(0.0)
                )
                transition=Transition {
                    duration: Some(0.1),
                    delay: Some(stagger_delay * i as f64),
                    ease: Easing::Linear,
                    ..Default::default()
                }
            />
        };
    }

    // Wait for all animations to complete
    std::thread::sleep(std::time::Duration::from_millis(300));

    // Check final states
    for element in &elements {
        let final_opacity = helper.get_opacity(element);
        let (_, final_y) = helper.get_position(element);

        assert!(
            (final_opacity - 1.0).abs() < 0.01,
            "Opacity should be 1.0, got {}",
            final_opacity
        );
        assert!(
            (final_y - 0.0).abs() < 1.0,
            "Y should be 0px, got {}",
            final_y
        );
    }
}
