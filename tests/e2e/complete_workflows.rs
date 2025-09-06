//! Complete End-to-End Workflow Tests
//!
//! This module tests complete user workflows that span multiple components
//! and interactions, ensuring the entire animation system works correctly
//! in real-world scenarios.

use leptos::prelude::*;
use leptos_motion::*;
use std::collections::HashMap;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Advanced E2E test helper with workflow-specific utilities
struct WorkflowTestHelper {
    document: web_sys::Document,
    window: web_sys::Window,
    test_app: web_sys::Element,
    interaction_log: Vec<String>,
}

impl WorkflowTestHelper {
    fn new() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // Create test app container
        let test_app = document.create_element("div").unwrap();
        test_app.set_id("workflow-test-app");
        test_app
            .set_attribute(
                "style",
                "width: 100vw; height: 100vh; position: relative; background: #f0f0f0;",
            )
            .unwrap();
        document.body().unwrap().append_child(&test_app).unwrap();

        Self {
            document,
            window,
            test_app,
            interaction_log: Vec::new(),
        }
    }

    fn log_interaction(&mut self, action: &str) {
        self.interaction_log.push(action.to_string());
        web_sys::console::log_1(&format!("E2E Interaction: {}", action).into());
    }

    fn create_element(&self, tag: &str, id: &str, styles: &str) -> web_sys::Element {
        let element = self.document.create_element(tag).unwrap();
        element.set_id(id);
        element.set_attribute("style", styles).unwrap();
        self.test_app.append_child(&element).unwrap();
        element
    }

    fn simulate_touch_gesture(&self, element: &web_sys::Element, touches: Vec<(f64, f64)>) {
        // Simulate multi-touch gesture
        for (i, (x, y)) in touches.iter().enumerate() {
            let touch_event = web_sys::TouchEvent::new("touchstart").unwrap();
            // Note: TouchEvent creation is complex in wasm-bindgen,
            // so we'll simulate with mouse events for testing
            self.simulate_mouse_event(element, "mousedown", *x, *y);
        }
    }

    fn simulate_mouse_event(&self, element: &web_sys::Element, event_type: &str, x: f64, y: f64) {
        let event = web_sys::MouseEvent::new(event_type).unwrap();
        event.init_mouse_event_with_bubbles_and_cancelable_and_view_and_detail_and_screen_x_and_screen_y_and_client_x_and_client_y_and_ctrl_key_and_alt_key_and_shift_key_and_meta_key_and_button_and_related_target(
            event_type, true, true, Some(&self.window), 0, x, y, x, y, false, false, false, false, 0, None
        ).unwrap();
        element.dispatch_event(&event).unwrap();
    }

    fn wait_for_animation_completion(
        &self,
        element: &web_sys::Element,
        property: &str,
        expected_value: &str,
        timeout_ms: u64,
    ) -> bool {
        let start_time = js_sys::Date::now();
        let timeout = timeout_ms as f64;

        while js_sys::Date::now() - start_time < timeout {
            let current_value = self.get_computed_style(element, property);
            if current_value.contains(expected_value) {
                return true;
            }
            // Small delay to prevent busy waiting
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        false
    }

    fn get_computed_style(&self, element: &web_sys::Element, property: &str) -> String {
        let computed_style = self.window.get_computed_style(element).unwrap().unwrap();
        computed_style
            .get_property_value(property)
            .unwrap_or_default()
    }

    fn assert_animation_completed(
        &self,
        element: &web_sys::Element,
        property: &str,
        expected: &str,
        timeout_ms: u64,
    ) {
        let completed = self.wait_for_animation_completion(element, property, expected, timeout_ms);
        assert!(
            completed,
            "Animation for property '{}' did not complete to '{}' within {}ms",
            property, expected, timeout_ms
        );
    }

    fn cleanup(&self) {
        if let Some(body) = self.document.body() {
            if let Some(test_app) = self.document.get_element_by_id("workflow-test-app") {
                body.remove_child(&test_app).unwrap();
            }
        }
    }
}

/// Test complete e-commerce product interaction workflow
#[wasm_bindgen_test]
async fn test_ecommerce_product_workflow() {
    let mut helper = WorkflowTestHelper::new();

    // Create product grid
    let product_grid = helper.create_element(
        "div",
        "product-grid",
        "display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; padding: 20px;",
    );

    // Create products
    let products: Vec<web_sys::Element> = (0..6)
        .map(|i| {
            let product = helper.create_element("div", &format!("product-{}", i),
                "background: white; border-radius: 8px; padding: 16px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); cursor: pointer;");

            let image = helper.document.create_element("div").unwrap();
            image.set_attribute("style", "width: 100%; height: 200px; background: #e0e0e0; border-radius: 4px; margin-bottom: 12px;").unwrap();
            product.append_child(&image).unwrap();

            let title = helper.document.create_element("h3").unwrap();
            title.set_text_content(Some(&format!("Product {}", i + 1)));
            title.set_attribute("style", "margin: 0 0 8px 0; font-size: 16px;").unwrap();
            product.append_child(&title).unwrap();

            let price = helper.document.create_element("p").unwrap();
            price.set_text_content(Some(&format!("${}", (i + 1) * 25)));
            price.set_attribute("style", "margin: 0; font-weight: bold; color: #2c3e50;").unwrap();
            product.append_child(&price).unwrap();

            product_grid.append_child(&product).unwrap();
            product
        })
        .collect();

    // Apply product hover animations
    for product in &products {
        let motion_product = view! {
            <MotionDiv
                node_ref=product.clone()
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.05),
                    "box-shadow" => AnimationValue::String("0 8px 25px rgba(0,0,0,0.15)".to_string())
                )
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(0.98)
                )
                transition=Transition {
                    duration: Some(0.2),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Test product hover workflow
    helper.log_interaction("Starting product hover workflow");

    for (i, product) in products.iter().enumerate() {
        // Hover over product
        helper.simulate_mouse_event(product, "mouseenter", 0.0, 0.0);
        helper.log_interaction(&format!("Hovered over product {}", i + 1));

        // Wait for hover animation
        helper.assert_animation_completed(product, "transform", "scale(1.05)", 300);

        // Click product
        helper.simulate_mouse_event(product, "mousedown", 0.0, 0.0);
        helper.simulate_mouse_event(product, "mouseup", 0.0, 0.0);
        helper.log_interaction(&format!("Clicked product {}", i + 1));

        // Wait for tap animation
        helper.assert_animation_completed(product, "transform", "scale(0.98)", 200);

        // Unhover
        helper.simulate_mouse_event(product, "mouseleave", 0.0, 0.0);
        helper.log_interaction(&format!("Unhovered product {}", i + 1));

        // Wait for return animation
        helper.assert_animation_completed(product, "transform", "scale(1)", 300);
    }

    helper.log_interaction("Product hover workflow completed successfully");
    helper.cleanup();
}

/// Test complete form validation and submission workflow
#[wasm_bindgen_test]
async fn test_form_validation_workflow() {
    let mut helper = WorkflowTestHelper::new();

    // Create form container
    let form_container = helper.create_element(
        "div",
        "form-container",
        "max-width: 500px; margin: 0 auto; padding: 20px; background: white; border-radius: 8px;",
    );

    // Create form
    let form = helper.document.create_element("form").unwrap();
    form.set_id("contact-form");
    form_container.append_child(&form).unwrap();

    // Create form fields
    let fields = vec![
        ("name", "text", "Full Name"),
        ("email", "email", "Email Address"),
        ("phone", "tel", "Phone Number"),
        ("message", "textarea", "Message"),
    ];

    let mut form_elements = HashMap::new();

    for (id, input_type, label) in fields {
        // Create field container
        let field_container = helper.document.create_element("div").unwrap();
        field_container
            .set_attribute("style", "margin-bottom: 20px;")
            .unwrap();

        // Create label
        let label_element = helper.document.create_element("label").unwrap();
        label_element.set_text_content(Some(label));
        label_element
            .set_attribute(
                "style",
                "display: block; margin-bottom: 5px; font-weight: bold;",
            )
            .unwrap();
        field_container.append_child(&label_element).unwrap();

        // Create input
        let input = helper
            .document
            .create_element(if input_type == "textarea" {
                "textarea"
            } else {
                "input"
            })
            .unwrap();
        input.set_id(id);
        input.set_attribute("style", "width: 100%; padding: 10px; border: 2px solid #ddd; border-radius: 4px; font-size: 14px;");
        if input_type != "textarea" {
            input.set_attribute("type", input_type).unwrap();
        }
        field_container.append_child(&input).unwrap();

        // Create error message
        let error = helper.document.create_element("div").unwrap();
        error.set_id(&format!("{}-error", id));
        error
            .set_attribute(
                "style",
                "color: red; font-size: 12px; margin-top: 5px; display: none;",
            )
            .unwrap();
        field_container.append_child(&error).unwrap();

        form.append_child(&field_container).unwrap();
        form_elements.insert(id.to_string(), (input, error));
    }

    // Create submit button
    let submit_button = helper.document.create_element("button").unwrap();
    submit_button.set_id("submit-btn");
    submit_button.set_text_content(Some("Submit Form"));
    submit_button.set_attribute("type", "submit").unwrap();
    submit_button.set_attribute("style", "width: 100%; padding: 12px; background: #3498db; color: white; border: none; border-radius: 4px; font-size: 16px; cursor: pointer;");
    form.append_child(&submit_button).unwrap();

    // Apply form validation animations
    for (field_id, (input, error)) in &form_elements {
        let motion_input = view! {
            <MotionInput
                node_ref=input.clone()
                while_focus=motion_target!(
                    "border-color" => AnimationValue::Color("#3498db".to_string()),
                    "box-shadow" => AnimationValue::String("0 0 0 3px rgba(52, 152, 219, 0.1)".to_string())
                )
                transition=Transition {
                    duration: Some(0.2),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };

        let motion_error = view! {
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
                    duration: Some(0.3),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Test form validation workflow
    helper.log_interaction("Starting form validation workflow");

    // Test empty form submission
    helper.simulate_mouse_event(&submit_button, "click", 0.0, 0.0);
    helper.log_interaction("Submitted empty form");

    // Wait for validation animations
    for (field_id, (input, error)) in &form_elements {
        helper.assert_animation_completed(error, "opacity", "1", 400);
        helper.log_interaction(&format!("Validation error shown for {}", field_id));
    }

    // Fill out form fields
    for (field_id, (input, _)) in &form_elements {
        let value = match field_id.as_str() {
            "name" => "John Doe",
            "email" => "john@example.com",
            "phone" => "555-1234",
            "message" => "This is a test message",
            _ => "test value",
        };

        input.set_attribute("value", value).unwrap();
        helper.log_interaction(&format!("Filled field: {}", field_id));
    }

    // Test focus animations
    for (field_id, (input, _)) in &form_elements {
        helper.simulate_mouse_event(input, "focus", 0.0, 0.0);
        helper.assert_animation_completed(input, "border-color", "rgb(52, 152, 219)", 300);
        helper.log_interaction(&format!("Focused field: {}", field_id));

        helper.simulate_mouse_event(input, "blur", 0.0, 0.0);
        helper.log_interaction(&format!("Blurred field: {}", field_id));
    }

    // Submit valid form
    helper.simulate_mouse_event(&submit_button, "click", 0.0, 0.0);
    helper.log_interaction("Submitted valid form");

    helper.log_interaction("Form validation workflow completed successfully");
    helper.cleanup();
}

/// Test complete image gallery workflow with transitions
#[wasm_bindgen_test]
async fn test_image_gallery_workflow() {
    let mut helper = WorkflowTestHelper::new();

    // Create gallery container
    let gallery = helper.create_element(
        "div",
        "image-gallery",
        "display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; padding: 20px;",
    );

    // Create thumbnail images
    let thumbnails: Vec<web_sys::Element> = (0..12)
        .map(|i| {
            let thumbnail = helper.create_element("div", &format!("thumb-{}", i),
                "width: 100%; height: 150px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; cursor: pointer; position: relative; overflow: hidden;");

            let overlay = helper.document.create_element("div").unwrap();
            overlay.set_attribute("style", "position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.3); display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;");
            overlay.set_text_content(Some(&format!("Image {}", i + 1)));
            thumbnail.append_child(&overlay).unwrap();

            gallery.append_child(&thumbnail).unwrap();
            thumbnail
        })
        .collect();

    // Create lightbox
    let lightbox = helper.create_element("div", "lightbox",
        "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.9); display: none; align-items: center; justify-content: center; z-index: 1000;");

    let lightbox_content = helper.document.create_element("div").unwrap();
    lightbox_content.set_attribute(
        "style",
        "position: relative; max-width: 90%; max-height: 90%;",
    );

    let lightbox_image = helper.document.create_element("div").unwrap();
    lightbox_image.set_id("lightbox-image");
    lightbox_image.set_attribute("style", "width: 600px; height: 400px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px;");

    let close_button = helper.document.create_element("button").unwrap();
    close_button.set_id("close-lightbox");
    close_button.set_text_content(Some("×"));
    close_button.set_attribute("style", "position: absolute; top: -40px; right: 0; background: none; border: none; color: white; font-size: 30px; cursor: pointer;");

    lightbox_content.append_child(&lightbox_image).unwrap();
    lightbox_content.append_child(&close_button).unwrap();
    lightbox.append_child(&lightbox_content).unwrap();
    helper.test_app.append_child(&lightbox).unwrap();

    // Apply thumbnail animations
    for thumbnail in &thumbnails {
        let motion_thumbnail = view! {
            <MotionDiv
                node_ref=thumbnail.clone()
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.1),
                    "z-index" => AnimationValue::Number(10.0)
                )
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(0.95)
                )
                transition=Transition {
                    duration: Some(0.2),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Apply lightbox animations
    let motion_lightbox = view! {
        <MotionDiv
            node_ref=lightbox.clone()
            initial=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "opacity" => AnimationValue::Number(1.0)
            )
            exit=motion_target!(
                "opacity" => AnimationValue::Number(0.0)
            )
            transition=Transition {
                duration: Some(0.3),
                ease: Easing::EaseInOut,
                ..Default::default()
            }
        />
    };

    let motion_lightbox_content = view! {
        <MotionDiv
            node_ref=lightbox_content.clone()
            initial=motion_target!(
                "scale" => AnimationValue::Number(0.8),
                "opacity" => AnimationValue::Number(0.0)
            )
            animate=motion_target!(
                "scale" => AnimationValue::Number(1.0),
                "opacity" => AnimationValue::Number(1.0)
            )
            exit=motion_target!(
                "scale" => AnimationValue::Number(0.8),
                "opacity" => AnimationValue::Number(0.0)
            )
            transition=Transition {
                duration: Some(0.3),
                ease: Easing::EaseOut,
                ..Default::default()
            }
        />
    };

    // Test gallery workflow
    helper.log_interaction("Starting image gallery workflow");

    // Test thumbnail interactions
    for (i, thumbnail) in thumbnails.iter().enumerate() {
        // Hover over thumbnail
        helper.simulate_mouse_event(thumbnail, "mouseenter", 0.0, 0.0);
        helper.log_interaction(&format!("Hovered thumbnail {}", i + 1));

        // Wait for hover animation
        helper.assert_animation_completed(thumbnail, "transform", "scale(1.1)", 300);

        // Click thumbnail to open lightbox
        helper.simulate_mouse_event(thumbnail, "click", 0.0, 0.0);
        helper.log_interaction(&format!("Clicked thumbnail {}", i + 1));

        // Show lightbox
        lightbox.set_attribute("style", "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.9); display: flex; align-items: center; justify-content: center; z-index: 1000;").unwrap();

        // Wait for lightbox animation
        helper.assert_animation_completed(&lightbox, "opacity", "1", 400);
        helper.assert_animation_completed(&lightbox_content, "transform", "scale(1)", 400);

        // Close lightbox
        helper.simulate_mouse_event(&close_button, "click", 0.0, 0.0);
        helper.log_interaction(&format!("Closed lightbox for thumbnail {}", i + 1));

        // Hide lightbox
        lightbox.set_attribute("style", "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.9); display: none; align-items: center; justify-content: center; z-index: 1000;").unwrap();

        // Unhover thumbnail
        helper.simulate_mouse_event(thumbnail, "mouseleave", 0.0, 0.0);
        helper.log_interaction(&format!("Unhovered thumbnail {}", i + 1));

        // Wait for return animation
        helper.assert_animation_completed(thumbnail, "transform", "scale(1)", 300);
    }

    helper.log_interaction("Image gallery workflow completed successfully");
    helper.cleanup();
}

/// Test complete dashboard interaction workflow
#[wasm_bindgen_test]
async fn test_dashboard_workflow() {
    let mut helper = WorkflowTestHelper::new();

    // Create dashboard layout
    let dashboard = helper.create_element("div", "dashboard",
        "display: grid; grid-template-columns: 250px 1fr; grid-template-rows: 60px 1fr; gap: 0; height: 100vh;");

    // Create header
    let header = helper.create_element("div", "dashboard-header",
        "grid-column: 1 / -1; background: #2c3e50; color: white; display: flex; align-items: center; padding: 0 20px;");

    let header_title = helper.document.create_element("h1").unwrap();
    header_title.set_text_content(Some("Dashboard"));
    header_title.set_attribute("style", "margin: 0; font-size: 24px;");
    header.append_child(&header_title).unwrap();

    // Create sidebar
    let sidebar = helper.create_element(
        "div",
        "sidebar",
        "background: #34495e; color: white; padding: 20px;",
    );

    let menu_items = vec!["Dashboard", "Analytics", "Users", "Settings", "Reports"];
    let mut sidebar_elements = Vec::new();

    for (i, item) in menu_items.iter().enumerate() {
        let menu_item = helper.create_element("div", &format!("menu-{}", i),
            "padding: 12px 16px; margin: 4px 0; border-radius: 4px; cursor: pointer; transition: all 0.2s ease;");
        menu_item.set_text_content(Some(item));
        sidebar.append_child(&menu_item).unwrap();
        sidebar_elements.push(menu_item);
    }

    // Create main content area
    let main_content = helper.create_element(
        "div",
        "main-content",
        "background: #ecf0f1; padding: 20px; overflow-y: auto;",
    );

    // Create content sections
    let content_sections = vec![
        (
            "dashboard-content",
            "Dashboard Overview",
            "Welcome to your dashboard",
        ),
        ("analytics-content", "Analytics", "View your analytics data"),
        ("users-content", "User Management", "Manage your users"),
        ("settings-content", "Settings", "Configure your settings"),
        ("reports-content", "Reports", "Generate and view reports"),
    ];

    let mut content_elements = HashMap::new();

    for (id, title, description) in content_sections {
        let section = helper.create_element("div", id,
            "background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);");

        let section_title = helper.document.create_element("h2").unwrap();
        section_title.set_text_content(Some(title));
        section_title.set_attribute("style", "margin: 0 0 10px 0; color: #2c3e50;");
        section.append_child(&section_title).unwrap();

        let section_desc = helper.document.create_element("p").unwrap();
        section_desc.set_text_content(Some(description));
        section_desc.set_attribute("style", "margin: 0; color: #7f8c8d;");
        section.append_child(&section_desc).unwrap();

        main_content.append_child(&section).unwrap();
        content_elements.insert(id.to_string(), section);
    }

    // Assemble dashboard
    dashboard.append_child(&header).unwrap();
    dashboard.append_child(&sidebar).unwrap();
    dashboard.append_child(&main_content).unwrap();

    // Apply sidebar animations
    for (i, menu_item) in sidebar_elements.iter().enumerate() {
        let motion_menu_item = view! {
            <MotionDiv
                node_ref=menu_item.clone()
                while_hover=motion_target!(
                    "background-color" => AnimationValue::Color("#3498db".to_string()),
                    "transform" => AnimationValue::String("translateX(5px)".to_string())
                )
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(0.95)
                )
                transition=Transition {
                    duration: Some(0.2),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Apply content section animations
    for (id, section) in &content_elements {
        let motion_section = view! {
            <MotionDiv
                node_ref=section.clone()
                initial=motion_target!(
                    "opacity" => AnimationValue::Number(0.0),
                    "y" => AnimationValue::Pixels(20.0)
                )
                animate=motion_target!(
                    "opacity" => AnimationValue::Number(1.0),
                    "y" => AnimationValue::Pixels(0.0)
                )
                transition=Transition {
                    duration: Some(0.4),
                    delay: Some(0.1),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Test dashboard workflow
    helper.log_interaction("Starting dashboard workflow");

    // Test sidebar navigation
    for (i, menu_item) in sidebar_elements.iter().enumerate() {
        // Hover over menu item
        helper.simulate_mouse_event(menu_item, "mouseenter", 0.0, 0.0);
        helper.log_interaction(&format!("Hovered menu item: {}", menu_items[i]));

        // Wait for hover animation
        helper.assert_animation_completed(menu_item, "background-color", "rgb(52, 152, 219)", 300);

        // Click menu item
        helper.simulate_mouse_event(menu_item, "click", 0.0, 0.0);
        helper.log_interaction(&format!("Clicked menu item: {}", menu_items[i]));

        // Wait for tap animation
        helper.assert_animation_completed(menu_item, "transform", "scale(0.95)", 200);

        // Unhover
        helper.simulate_mouse_event(menu_item, "mouseleave", 0.0, 0.0);
        helper.log_interaction(&format!("Unhovered menu item: {}", menu_items[i]));

        // Wait for return animation
        helper.assert_animation_completed(menu_item, "background-color", "rgba(0, 0, 0, 0)", 300);
    }

    // Test content section animations
    for (id, section) in &content_elements {
        helper.assert_animation_completed(section, "opacity", "1", 600);
        helper.log_interaction(&format!("Content section {} animated in", id));
    }

    helper.log_interaction("Dashboard workflow completed successfully");
    helper.cleanup();
}

/// Test complete mobile gesture workflow
#[wasm_bindgen_test]
async fn test_mobile_gesture_workflow() {
    let mut helper = WorkflowTestHelper::new();

    // Create mobile viewport
    let mobile_container = helper.create_element("div", "mobile-container",
        "width: 375px; height: 667px; margin: 0 auto; background: #f8f9fa; border-radius: 20px; overflow: hidden; position: relative;");

    // Create mobile header
    let mobile_header = helper.create_element("div", "mobile-header",
        "height: 60px; background: #007AFF; color: white; display: flex; align-items: center; justify-content: center; font-weight: bold;");
    mobile_header.set_text_content(Some("Mobile App"));

    // Create swipeable content
    let swipe_container = helper.create_element(
        "div",
        "swipe-container",
        "height: calc(100% - 60px); position: relative; overflow: hidden;",
    );

    let pages = vec![
        ("page-1", "#FF6B6B", "Page 1"),
        ("page-2", "#4ECDC4", "Page 2"),
        ("page-3", "#45B7D1", "Page 3"),
        ("page-4", "#96CEB4", "Page 4"),
    ];

    let mut page_elements = Vec::new();

    for (i, (id, color, title)) in pages.iter().enumerate() {
        let page = helper.create_element("div", id,
            &format!("position: absolute; top: 0; left: {}%; width: 100%; height: 100%; background: {}; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: bold;", i * 100, color));
        page.set_text_content(Some(title));
        swipe_container.append_child(&page).unwrap();
        page_elements.push(page);
    }

    // Create page indicators
    let indicators = helper.create_element("div", "page-indicators",
        "position: absolute; bottom: 20px; left: 50%; transform: translateX(-50%); display: flex; gap: 8px; z-index: 10;");

    for i in 0..pages.len() {
        let indicator = helper.create_element("div", &format!("indicator-{}", i),
            "width: 8px; height: 8px; border-radius: 50%; background: rgba(255,255,255,0.5); cursor: pointer;");
        indicators.append_child(&indicator).unwrap();
    }

    // Assemble mobile app
    mobile_container.append_child(&mobile_header).unwrap();
    mobile_container.append_child(&swipe_container).unwrap();
    mobile_container.append_child(&indicators).unwrap();

    // Apply page animations
    for (i, page) in page_elements.iter().enumerate() {
        let motion_page = view! {
            <MotionDiv
                node_ref=page.clone()
                initial=motion_target!(
                    "x" => AnimationValue::Pixels(i as f64 * 375.0)
                )
                animate=motion_target!(
                    "x" => AnimationValue::Pixels(0.0)
                )
                transition=Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Apply indicator animations
    for i in 0..pages.len() {
        let indicator = helper
            .document
            .get_element_by_id(&format!("indicator-{}", i))
            .unwrap();
        let motion_indicator = view! {
            <MotionDiv
                node_ref=indicator.clone()
                while_tap=motion_target!(
                    "scale" => AnimationValue::Number(1.2)
                )
                transition=Transition {
                    duration: Some(0.1),
                    ease: Easing::EaseOut,
                    ..Default::default()
                }
            />
        };
    }

    // Test mobile gesture workflow
    helper.log_interaction("Starting mobile gesture workflow");

    // Test page indicators
    for i in 0..pages.len() {
        let indicator = helper
            .document
            .get_element_by_id(&format!("indicator-{}", i))
            .unwrap();

        // Tap indicator
        helper.simulate_mouse_event(&indicator, "mousedown", 0.0, 0.0);
        helper.simulate_mouse_event(&indicator, "mouseup", 0.0, 0.0);
        helper.log_interaction(&format!("Tapped indicator {}", i + 1));

        // Wait for tap animation
        helper.assert_animation_completed(&indicator, "transform", "scale(1.2)", 150);

        // Wait for page animation
        if i > 0 {
            let page = &page_elements[i];
            helper.assert_animation_completed(page, "transform", "translateX(0px)", 400);
        }
    }

    // Test swipe gestures (simulated with mouse events)
    for i in 0..pages.len() - 1 {
        let current_page = &page_elements[i];
        let next_page = &page_elements[i + 1];

        // Simulate swipe left
        helper.simulate_mouse_event(current_page, "mousedown", 300.0, 300.0);
        helper.simulate_mouse_event(current_page, "mousemove", 100.0, 300.0);
        helper.simulate_mouse_event(current_page, "mouseup", 100.0, 300.0);
        helper.log_interaction(&format!("Swiped from page {} to page {}", i + 1, i + 2));

        // Wait for page transition
        helper.assert_animation_completed(next_page, "transform", "translateX(0px)", 400);
    }

    helper.log_interaction("Mobile gesture workflow completed successfully");
    helper.cleanup();
}
