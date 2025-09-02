//! End-to-end browser tests for real user scenarios

use leptos::prelude::*;
use leptos_motion::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// E2E test helper for simulating real user interactions
struct E2ETestHelper {
    document: web_sys::Document,
    window: web_sys::Window,
}

impl E2ETestHelper {
    fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Self { document, window }
    }
    
    fn create_test_app(&self) -> web_sys::Element {
        let app = self.document.create_element("div").unwrap();
        app.set_id("test-app");
        app.set_attribute("style", "width: 800px; height: 600px; position: relative;").unwrap();
        self.document.body().unwrap().append_child(&app).unwrap();
        app
    }
    
    fn simulate_click(&self, element: &web_sys::Element) {
        let click_event = web_sys::MouseEvent::new("click").unwrap();
        element.dispatch_event(&click_event).unwrap();
    }
    
    fn simulate_hover(&self, element: &web_sys::Element) {
        let mouseenter_event = web_sys::MouseEvent::new("mouseenter").unwrap();
        element.dispatch_event(&mouseenter_event).unwrap();
    }
    
    fn simulate_unhover(&self, element: &web_sys::Element) {
        let mouseleave_event = web_sys::MouseEvent::new("mouseleave").unwrap();
        element.dispatch_event(&mouseleave_event).unwrap();
    }
    
    fn simulate_drag(&self, element: &web_sys::Element, start_x: f64, start_y: f64, end_x: f64, end_y: f64) {
        // Mouse down
        let mousedown_event = web_sys::MouseEvent::new("mousedown").unwrap();
        mousedown_event.init_mouse_event_with_bubbles_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
            "mousedown", true, true, Some(&self.window), 0, start_x, start_y, start_x, start_y, false, false, false, false, 0, None
        ).unwrap();
        element.dispatch_event(&mousedown_event).unwrap();
        
        // Mouse move
        let mousemove_event = web_sys::MouseEvent::new("mousemove").unwrap();
        mousemove_event.init_mouse_event_with_bubbles_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
            "mousemove", true, true, Some(&self.window), 0, end_x, end_y, end_x, end_y, false, false, false, false, 0, None
        ).unwrap();
        element.dispatch_event(&mousemove_event).unwrap();
        
        // Mouse up
        let mouseup_event = web_sys::MouseEvent::new("mouseup").unwrap();
        mouseup_event.init_mouse_event_with_bubbles_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
            "mouseup", true, true, Some(&self.window), 0, end_x, end_y, end_x, end_y, false, false, false, false, 0, None
        ).unwrap();
        element.dispatch_event(&mouseup_event).unwrap();
    }
    
    fn wait_for_animation(&self, duration_ms: u64) {
        // In a real E2E test, we would wait for animation completion events
        // For now, we'll use a simple timeout
        std::thread::sleep(std::time::Duration::from_millis(duration_ms));
    }
    
    fn get_computed_style(&self, element: &web_sys::Element, property: &str) -> String {
        let computed_style = self.window.get_computed_style(element).unwrap().unwrap();
        computed_style.get_property_value(property).unwrap_or_default()
    }
    
    fn assert_style_equals(&self, element: &web_sys::Element, property: &str, expected: &str) {
        let actual = self.get_computed_style(element, property);
        assert_eq!(actual, expected, "Style property '{}' should be '{}', got '{}'", property, expected, actual);
    }
    
    fn assert_style_contains(&self, element: &web_sys::Element, property: &str, expected: &str) {
        let actual = self.get_computed_style(element, property);
        assert!(actual.contains(expected), "Style property '{}' should contain '{}', got '{}'", property, expected, actual);
    }
}

#[wasm_bindgen_test]
async fn test_button_hover_animation_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create button with hover animation
    let button = helper.document.create_element("button").unwrap();
    button.set_id("hover-button");
    button.set_text_content(Some("Hover Me"));
    button.set_attribute("style", "padding: 10px; background: blue; color: white; border: none;").unwrap();
    app.append_child(&button).unwrap();
    
    // Apply hover animation
    let motion_button = view! {
        <MotionButton
            node_ref=button.clone()
            while_hover=motion_target!(
                "scale" => AnimationValue::Number(1.1),
                "background-color" => AnimationValue::Color("red".to_string())
            )
            transition=Transition {
                duration: Some(0.1),
                ease: Easing::Linear,
                ..Default::default()
            }
        />
    };
    
    // Simulate hover
    helper.simulate_hover(&button);
    helper.wait_for_animation(150);
    
    // Check hover state
    helper.assert_style_contains(&button, "transform", "scale(1.1)");
    helper.assert_style_equals(&button, "background-color", "rgb(255, 0, 0)");
    
    // Simulate unhover
    helper.simulate_unhover(&button);
    helper.wait_for_animation(150);
    
    // Check normal state
    helper.assert_style_equals(&button, "background-color", "rgb(0, 0, 255)");
}

#[wasm_bindgen_test]
async fn test_draggable_element_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create draggable element
    let draggable = helper.document.create_element("div").unwrap();
    draggable.set_id("draggable");
    draggable.set_text_content(Some("Drag Me"));
    draggable.set_attribute("style", "width: 100px; height: 100px; background: green; position: absolute; cursor: grab;").unwrap();
    app.append_child(&draggable).unwrap();
    
    // Apply drag functionality
    let motion_div = view! {
        <MotionDiv
            node_ref=draggable.clone()
            drag=true
            drag_constraints=DragConstraints::Bounded {
                left: -100.0,
                right: 100.0,
                top: -100.0,
                bottom: 100.0,
            }
            while_drag=motion_target!(
                "scale" => AnimationValue::Number(1.05),
                "cursor" => AnimationValue::String("grabbing".to_string())
            )
        />
    };
    
    // Get initial position
    let initial_transform = helper.get_computed_style(&draggable, "transform");
    
    // Simulate drag
    helper.simulate_drag(&draggable, 0.0, 0.0, 50.0, 50.0);
    helper.wait_for_animation(100);
    
    // Check drag state
    let drag_transform = helper.get_computed_style(&draggable, "transform");
    assert_ne!(drag_transform, initial_transform, "Element should have moved during drag");
    helper.assert_style_contains(&drag_transform, "scale(1.05)");
}

#[wasm_bindgen_test]
async fn test_modal_animation_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create modal trigger button
    let trigger = helper.document.create_element("button").unwrap();
    trigger.set_id("modal-trigger");
    trigger.set_text_content(Some("Open Modal"));
    app.append_child(&trigger).unwrap();
    
    // Create modal
    let modal = helper.document.create_element("div").unwrap();
    modal.set_id("modal");
    modal.set_text_content(Some("Modal Content"));
    modal.set_attribute("style", "position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%); background: white; padding: 20px; border: 1px solid black;").unwrap();
    app.append_child(&modal).unwrap();
    
    // Create modal state
    let (is_open, set_is_open) = create_signal(false);
    
    // Apply modal animation
    let modal_motion = view! {
        <AnimatePresence>
            {move || if is_open.get() {
                view! {
                    <MotionDiv
                        node_ref=modal.clone()
                        initial=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "scale" => AnimationValue::Number(0.8)
                        )
                        animate=motion_target!(
                            "opacity" => AnimationValue::Number(1.0),
                            "scale" => AnimationValue::Number(1.0)
                        )
                        exit=motion_target!(
                            "opacity" => AnimationValue::Number(0.0),
                            "scale" => AnimationValue::Number(0.8)
                        )
                        transition=Transition {
                            duration: Some(0.2),
                            ease: Easing::EaseOut,
                            ..Default::default()
                        }
                    />
                }
            } else {
                view! { <div></div> }
            }}
        </AnimatePresence>
    };
    
    // Initially modal should be hidden
    helper.assert_style_equals(&modal, "opacity", "0");
    
    // Open modal
    helper.simulate_click(&trigger);
    set_is_open.set(true);
    helper.wait_for_animation(250);
    
    // Check open state
    helper.assert_style_equals(&modal, "opacity", "1");
    helper.assert_style_contains(&modal, "transform", "scale(1)");
    
    // Close modal
    helper.simulate_click(&trigger);
    set_is_open.set(false);
    helper.wait_for_animation(250);
    
    // Check closed state
    helper.assert_style_equals(&modal, "opacity", "0");
}

#[wasm_bindgen_test]
async fn test_list_animation_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create list container
    let list = helper.document.create_element("ul").unwrap();
    list.set_id("animated-list");
    list.set_attribute("style", "list-style: none; padding: 0;").unwrap();
    app.append_child(&list).unwrap();
    
    // Create list items
    let items: Vec<web_sys::Element> = (0..3)
        .map(|i| {
            let item = helper.document.create_element("li").unwrap();
            item.set_id(&format!("item-{}", i));
            item.set_text_content(Some(&format!("Item {}", i + 1)));
            item.set_attribute("style", "padding: 10px; margin: 5px; background: lightblue;").unwrap();
            list.append_child(&item).unwrap();
            item
        })
        .collect();
    
    // Apply staggered list animation
    for (i, item) in items.iter().enumerate() {
        let motion_item = view! {
            <MotionLi
                node_ref=item.clone()
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "x" => AnimationValue::Pixels(-50.0)
                )
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "x" => AnimationValue::Pixels(0.0)
                )
                transition=Transition {
                    duration: Some(0.3),
                    delay: Some(0.1 * i as f64),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }
    
    // Wait for all animations to complete
    helper.wait_for_animation(600);
    
    // Check final states
    for item in &items {
        helper.assert_style_equals(item, "opacity", "1");
        helper.assert_style_contains(&helper.get_computed_style(item, "transform"), "translateX(0px)");
    }
}

#[wasm_bindgen_test]
async fn test_form_validation_animation_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create form
    let form = helper.document.create_element("form").unwrap();
    form.set_id("test-form");
    app.append_child(&form).unwrap();
    
    // Create input field
    let input = helper.document.create_element("input").unwrap();
    input.set_id("test-input");
    input.set_attribute("type", "text").unwrap();
    input.set_attribute("required", "").unwrap();
    input.set_attribute("style", "padding: 10px; border: 2px solid #ccc;").unwrap();
    form.append_child(&input).unwrap();
    
    // Create error message
    let error = helper.document.create_element("div").unwrap();
    error.set_id("error-message");
    error.set_text_content(Some("This field is required"));
    error.set_attribute("style", "color: red; font-size: 12px; display: none;").unwrap();
    form.append_child(&error).unwrap();
    
    // Create submit button
    let submit = helper.document.create_element("button").unwrap();
    submit.set_id("submit-btn");
    submit.set_text_content(Some("Submit"));
    submit.set_attribute("type", "submit").unwrap();
    form.append_child(&submit).unwrap();
    
    // Apply validation animation
    let error_motion = view! {
        <MotionDiv
            node_ref=error.clone()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "height" => AnimationValue::Pixels(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "height" => AnimationValue::Pixels(20.0)
            )
            transition=Transition {
                duration: Some(0.2),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        />
    };
    
    // Initially error should be hidden
    helper.assert_style_equals(&error, "opacity", "0");
    
    // Submit form without input
    helper.simulate_click(&submit);
    helper.wait_for_animation(250);
    
    // Error should be visible
    helper.assert_style_equals(&error, "opacity", "1");
    helper.assert_style_equals(&error, "height", "20px");
    
    // Input should have error styling
    helper.assert_style_equals(&input, "border-color", "rgb(255, 0, 0)");
}

#[wasm_bindgen_test]
async fn test_page_transition_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create navigation
    let nav = helper.document.create_element("nav").unwrap();
    nav.set_id("navigation");
    app.append_child(&nav).unwrap();
    
    // Create page content
    let page = helper.document.create_element("main").unwrap();
    page.set_id("page-content");
    page.set_text_content(Some("Page Content"));
    page.set_attribute("style", "padding: 20px;").unwrap();
    app.append_child(&page).unwrap();
    
    // Create navigation buttons
    let (current_page, set_current_page) = create_signal(0);
    
    for i in 0..3 {
        let button = helper.document.create_element("button").unwrap();
        button.set_id(&format!("nav-{}", i));
        button.set_text_content(Some(&format!("Page {}", i + 1)));
        nav.append_child(&button).unwrap();
        
        // Add click handler
        let set_page = set_current_page.clone();
        let page_num = i;
        button.set_onclick(Some(wasm_bindgen::closure::Closure::wrap(Box::new(move || {
            set_page.set(page_num);
        }) as Box<dyn FnMut()>).into_js_value().unchecked_ref()));
    }
    
    // Apply page transition animation
    let page_motion = view! {
        <MotionMain
            node_ref=page.clone()
            key=move || current_page.get()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(100.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0),
                "x" => AnimationValue::Pixels(0.0)
            )
            exit=motion_target!(
                "opacity" => AnimationValue::Number(0.0),
                "x" => AnimationValue::Pixels(-100.0)
            )
            transition=Transition {
                duration: Some(0.3),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        />
    };
    
    // Navigate to different pages
    for i in 0..3 {
        let nav_button = helper.document.get_element_by_id(&format!("nav-{}", i)).unwrap();
        helper.simulate_click(&nav_button);
        helper.wait_for_animation(350);
        
        // Check that page is visible
        helper.assert_style_equals(&page, "opacity", "1");
        helper.assert_style_contains(&helper.get_computed_style(&page, "transform"), "translateX(0px)");
    }
}

#[wasm_bindgen_test]
async fn test_performance_under_load_e2e() {
    let helper = E2ETestHelper::new();
    let app = helper.create_test_app();
    
    // Create many animated elements to test performance
    let container = helper.document.create_element("div").unwrap();
    container.set_id("performance-container");
    container.set_attribute("style", "display: grid; grid-template-columns: repeat(10, 1fr); gap: 10px;").unwrap();
    app.append_child(&container).unwrap();
    
    // Create 100 animated elements
    let elements: Vec<web_sys::Element> = (0..100)
        .map(|i| {
            let element = helper.document.create_element("div").unwrap();
            element.set_id(&format!("perf-{}", i));
            element.set_text_content(Some(&format!("{}", i)));
            element.set_attribute("style", "width: 50px; height: 50px; background: purple; display: flex; align-items: center; justify-content: center; color: white;").unwrap();
            container.append_child(&element).unwrap();
            element
        })
        .collect();
    
    // Apply animations to all elements
    for element in &elements {
        let motion_div = view! {
            <MotionDiv
                node_ref=element.clone()
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.2),
                    "background-color" => AnimationValue::Color("orange".to_string())
                )
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(0.9)
                )
                transition=Transition {
                    duration: Some(0.1),
                    ease: Easing::Linear,
                    ..Default::default()
                }
            />
        };
    }
    
    // Simulate rapid interactions
    for i in 0..10 {
        let element = &elements[i * 10];
        
        // Hover
        helper.simulate_hover(element);
        helper.wait_for_animation(50);
        
        // Click
        helper.simulate_click(element);
        helper.wait_for_animation(50);
        
        // Unhover
        helper.simulate_unhover(element);
        helper.wait_for_animation(50);
    }
    
    // All elements should still be responsive
    for element in &elements {
        let transform = helper.get_computed_style(element, "transform");
        assert!(transform.contains("scale(1)") || transform.contains("scale(1.2)") || transform.contains("scale(0.9)"));
    }
}
