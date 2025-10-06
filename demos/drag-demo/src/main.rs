use leptos::prelude::*;
use leptos_motion::*;
use leptos_motion::{EventDragAxis as DragAxis, EventDragConstraints as DragConstraints};
use std::collections::HashMap;

#[component]
fn DragDemo() -> impl IntoView {
    let (count, set_count) = signal(0);

    // Drag animations - scale up while dragging
    let while_drag = HashMap::from([
        ("scale".to_string(), AnimationValue::Number(1.1)),
        ("rotate".to_string(), AnimationValue::Degrees(5.0)),
        ("boxShadow".to_string(), AnimationValue::String("0 10px 25px rgba(0,0,0,0.3)".to_string())),
    ]);

    view! {
        <div style="padding: 40px; font-family: system-ui;">
            <h1 style="color: #333; margin-bottom: 20px;">"🎯 Drag Demo - Leptos Motion"</h1>

            <div style="margin-bottom: 20px;">
                <button
                    style="padding: 10px 20px; background: #4CAF50; color: white; border: none; border-radius: 5px; cursor: pointer;"
                    on:click=move |_| set_count.update(|c| *c += 1)
                >
                    "Click to animate: " {move || count.get()}
                </button>
            </div>

            <div style="display: flex; flex-wrap: wrap; gap: 20px;">
                // Basic draggable element
                <MotionDiv
                    node_ref=NodeRef::new()
                    drag=DragConfig {
                        axis: Some(DragAxis::Both),
                        constraints: Some(DragConstraints {
                            min_x: Some(-200.0),
                            max_x: Some(200.0),
                            min_y: Some(-100.0),
                            max_y: Some(100.0),
                        }),
                        elastic: Some(0.2),
                        momentum: Some(true),
                    }
                    while_drag=while_drag.clone()
                    style="width: 120px; height: 120px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: grab; user-select: none;".to_string()
                >
                    "Drag Me!"
                </MotionDiv>

                // Another draggable element with different constraints
                <MotionDiv
                    node_ref=NodeRef::new()
                    drag=DragConfig {
                        axis: Some(DragAxis::X), // Only horizontal
                        constraints: Some(DragConstraints {
                            min_x: Some(-100.0),
                            max_x: Some(100.0),
                            min_y: None,
                            max_y: None,
                        }),
                        elastic: Some(0.5),
                        momentum: Some(false),
                    }
                    while_drag=while_drag.clone()
                    style="width: 120px; height: 120px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: grab; user-select: none;".to_string()
                >
                    "Horizontal Only"
                </MotionDiv>

                // Vertical only drag
                <MotionDiv
                    node_ref=NodeRef::new()
                    drag=DragConfig {
                        axis: Some(DragAxis::Y), // Only vertical
                        constraints: Some(DragConstraints {
                            min_x: None,
                            max_x: None,
                            min_y: Some(-50.0),
                            max_y: Some(50.0),
                        }),
                        elastic: Some(0.1),
                        momentum: Some(true),
                    }
                    while_drag=while_drag.clone()
                    style="width: 120px; height: 120px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); border-radius: 10px; display: flex; align-items: center; justify-content: center; color: white; font-weight: bold; cursor: grab; user-select: none;".to_string()
                >
                    "Vertical Only"
                </MotionDiv>
            </div>

            <div style="margin-top: 40px; padding: 20px; background: #f5f5f5; border-radius: 10px;">
                <h3 style="margin-top: 0; color: #333;">"Instructions:"</h3>
                <ul style="color: #666;">
                    <li>"Click and drag the colored squares to move them around"</li>
                    <li>"Each square has different drag constraints and behaviors"</li>
                    <li>"Notice the scaling and rotation effects while dragging"</li>
                    <li>"Squares will bounce back when hitting boundaries"</li>
                </ul>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(DragDemo)
}
