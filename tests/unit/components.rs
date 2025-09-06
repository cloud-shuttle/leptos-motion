use leptos::prelude::*;
use leptos_motion::*;
use leptos_testing::*;
use pretty_assertions::assert_eq;

#[test]
fn test_motion_div_renders() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv class="test-element">
            "Test Content"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Test Content").is_some());
}

#[test]
fn test_motion_div_with_animation_props() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(100.0)
            )
            transition=Transition {
                duration: Some(0.5),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        >
            "Animated Content"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    let element = test_app.query_selector(".test-element").unwrap();
    assert!(element.text_content().unwrap().contains("Animated Content"));
}

#[test]
fn test_motion_span_renders() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionSpan class="test-span">
            "Span Content"
        </MotionSpan>
    };

    test_app.mount(component);

    // Check that the span was rendered
    assert!(test_app.query_selector(".test-span").is_some());
    assert!(test_app.query_by_text("Span Content").is_some());
}

#[test]
fn test_motion_div_with_initial_props() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            initial=Some(motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "scale" => AnimationValue::Number(0.5)
            ))
        >
            "Initial State"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Initial State").is_some());
}

#[test]
fn test_motion_div_with_while_hover() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            while_hover=Some(motion_target!(
                "scale" => AnimationValue::Number(1.1),
                "rotate" => AnimationValue::Degrees(5.0)
            ))
        >
            "Hover Me"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Hover Me").is_some());
}

#[test]
fn test_motion_div_with_while_tap() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            while_tap=Some(motion_target!(
                "scale" => AnimationValue::Number(0.95)
            ))
        >
            "Tap Me"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Tap Me").is_some());
}

#[test]
fn test_motion_div_with_exit_props() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            exit=Some(motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "y" => AnimationValue::Pixels(-50.0)
            ))
        >
            "Exit Animation"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Exit Animation").is_some());
}

#[test]
fn test_motion_div_with_variants() {
    let test_app = TestApp::new();

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

    let component = view! {
        <MotionDiv
            class="test-element"
            variants=Some(variants)
        >
            "Variant Content"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Variant Content").is_some());
}

#[test]
fn test_motion_div_with_layout_prop() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            layout=Some(true)
        >
            "Layout Animation"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Layout Animation").is_some());
}

#[test]
fn test_motion_div_with_drag_prop() {
    let test_app = TestApp::new();

    let drag_config = DragConfig::new()
        .axis(DragAxis::Both)
        .constraints(DragConstraints {
            left: Some(-100.0),
            right: Some(100.0),
            top: Some(-100.0),
            bottom: Some(100.0),
        });

    let component = view! {
        <MotionDiv
            class="test-element"
            drag=Some(drag_config)
        >
            "Draggable"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Draggable").is_some());
}

#[test]
fn test_motion_div_with_style_prop() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            style=Some("background-color: red; width: 100px; height: 100px;".to_string())
        >
            "Styled Content"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Styled Content").is_some());
}

#[test]
fn test_motion_div_with_id_prop() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv
            class="test-element"
            id=Some("unique-id".to_string())
        >
            "ID Content"
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element was rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("ID Content").is_some());
}

#[test]
fn test_motion_div_nested_content() {
    let test_app = TestApp::new();

    let component = view! {
        <MotionDiv class="test-element">
            <h1>"Title"</h1>
            <p>"Paragraph content"</p>
            <button>"Click me"</button>
        </MotionDiv>
    };

    test_app.mount(component);

    // Check that the element and nested content were rendered
    assert!(test_app.query_selector(".test-element").is_some());
    assert!(test_app.query_by_text("Title").is_some());
    assert!(test_app.query_by_text("Paragraph content").is_some());
    assert!(test_app.query_by_text("Click me").is_some());
}

#[test]
fn test_motion_div_multiple_instances() {
    let test_app = TestApp::new();

    let component = view! {
        <div>
            <MotionDiv class="element-1">
                "First Element"
            </MotionDiv>
            <MotionDiv class="element-2">
                "Second Element"
            </MotionDiv>
            <MotionDiv class="element-3">
                "Third Element"
            </MotionDiv>
        </div>
    };

    test_app.mount(component);

    // Check that all elements were rendered
    assert!(test_app.query_selector(".element-1").is_some());
    assert!(test_app.query_selector(".element-2").is_some());
    assert!(test_app.query_selector(".element-3").is_some());

    assert!(test_app.query_by_text("First Element").is_some());
    assert!(test_app.query_by_text("Second Element").is_some());
    assert!(test_app.query_by_text("Third Element").is_some());
}
