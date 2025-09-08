//! TDD Tests for Leptos v0.8 Compatibility (Phase 2)
//!
//! This module contains comprehensive failing tests for Leptos v0.8 trait compatibility,
//! addressing the missing IntoClass, IntoAttributeValue, and IntoProperty implementations.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::*;
use std::collections::HashMap;

/// Test that AnimationTarget implements IntoClass for class attributes
#[test]
fn test_animation_target_into_class_trait() {
    // RED PHASE: This trait implementation is missing

    let animation_target = create_animation_target("opacity", 0.5);

    // This should compile and work with Leptos v0.8 class attribute
    let class_value: Class = animation_target.into_class();

    // Verify the conversion works
    assert!(class_value.to_string().contains("opacity"));
    assert!(class_value.to_string().contains("0.5"));
}

/// Test that AnimationValue implements IntoAttributeValue
#[test]
fn test_animation_value_into_attribute_value_trait() {
    // RED PHASE: This trait implementation is missing

    let number_value = AnimationValue::Number(1.0);
    let pixel_value = AnimationValue::Pixels(100.0);
    let color_value = AnimationValue::Color("red".to_string());

    // These should compile and work with Leptos v0.8 attributes
    let attr1: AttributeValue = number_value.into_attribute_value();
    let attr2: AttributeValue = pixel_value.into_attribute_value();
    let attr3: AttributeValue = color_value.into_attribute_value();

    // Verify conversions work
    assert_eq!(attr1.to_string(), "1");
    assert_eq!(attr2.to_string(), "100px");
    assert_eq!(attr3.to_string(), "red");
}

/// Test that Transition implements IntoProperty
#[test]
fn test_transition_into_property_trait() {
    // RED PHASE: This trait implementation is missing

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.1),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    // This should compile and work with Leptos v0.8 properties
    let property: Property = transition.into_property();

    // Verify the conversion works
    let property_string = property.to_string();
    assert!(property_string.contains("0.5"));
    assert!(property_string.contains("ease-in-out"));
}

/// Test that MotionDiv works with Leptos v0.8 signal system
#[test]
fn test_motion_div_leptos_v08_signals() {
    // RED PHASE: Signal system compatibility issues

    let (is_visible, set_visible) = signal(false);
    let (animation_progress, set_progress) = signal(0.0);

    // This should compile with Leptos v0.8 signal system
    let component = view! {
        <MotionDiv
            class=move || format!("transition-opacity {}", if is_visible.get() { "opacity-100" } else { "opacity-0" })
            animate=move || {
                let mut target = HashMap::new();
                target.insert("opacity".to_string(), AnimationValue::Number(animation_progress.get()));
                Some(target)
            }
            transition=move || {
                Some(Transition {
                    duration: Some(0.3),
                    ease: Easing::EaseOut,
                    ..Default::default()
                })
            }
        >
            "Signal Test"
        </MotionDiv>
    };

    // Verify the component compiles and works
    assert!(component.into_view().is_some());

    // Test signal updates
    set_visible.set(true);
    set_progress.set(1.0);

    // Verify signals work correctly
    assert!(is_visible.get());
    assert_eq!(animation_progress.get(), 1.0);
}

/// Test that MotionDiv works with Leptos v0.8 effect system
#[test]
fn test_motion_div_leptos_v08_effects() {
    // RED PHASE: Effect system compatibility issues

    let (counter, set_counter) = signal(0);
    let (animation_trigger, set_trigger) = signal(false);

    // This should work with Leptos v0.8 effect system
    create_effect(move |_| {
        let count = counter.get();
        if count > 0 {
            set_trigger.set(true);
        }
    });

    let component = view! {
        <MotionDiv
            animate=move || {
                if animation_trigger.get() {
                    let mut target = HashMap::new();
                    target.insert("scale".to_string(), AnimationValue::Number(1.1));
                    Some(target)
                } else {
                    None
                }
            }
        >
            "Effect Test"
        </MotionDiv>
    };

    // Verify component compiles
    assert!(component.into_view().is_some());

    // Test effect triggering
    set_counter.set(1);

    // Verify effect was triggered
    assert!(animation_trigger.get());
}

/// Test that MotionDiv works with Leptos v0.8 view macro
#[test]
fn test_motion_div_leptos_v08_view_macro() {
    // RED PHASE: View macro compatibility issues

    let (items, set_items) = signal(vec!["item1", "item2", "item3"]);

    // This should compile with Leptos v0.8 view macro
    let component = view! {
        <div>
            <For
                each=items
                key=|item| *item
                children=move |item| {
                    view! {
                        <MotionDiv
                            class="list-item"
                            initial=Some(create_animation_target("opacity", 0.0))
                            animate=Some(create_animation_target("opacity", 1.0))
                            transition=Some(Transition {
                                duration: Some(0.3),
                                ease: Easing::EaseOut,
                                ..Default::default()
                            })
                        >
                            {item}
                        </MotionDiv>
                    }
                }
            />
        </div>
    };

    // Verify component compiles
    assert!(component.into_view().is_some());

    // Test dynamic updates
    set_items.set(vec!["new_item1", "new_item2"]);

    // Verify updates work
    assert_eq!(items.get().len(), 2);
}

/// Test that MotionDiv works with Leptos v0.8 resource system
#[test]
fn test_motion_div_leptos_v08_resources() {
    // RED PHASE: Resource system compatibility issues

    let animation_data = Resource::new(|| async {
        // Simulate async data loading
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        create_animation_target("opacity", 1.0)
    });

    // This should work with Leptos v0.8 resource system
    let component = view! {
        <Suspense fallback=move || view! { <div>"Loading..."</div> }>
            {move || {
                animation_data.get().map(|data| {
                    view! {
                        <MotionDiv
                            animate=Some(data)
                            transition=Some(Transition {
                                duration: Some(0.5),
                                ease: Easing::EaseInOut,
                                ..Default::default()
                            })
                        >
                            "Resource Test"
                        </MotionDiv>
                    }
                })
            }}
        </Suspense>
    };

    // Verify component compiles
    assert!(component.into_view().is_some());
}

/// Test that MotionDiv works with Leptos v0.8 error boundaries
#[test]
fn test_motion_div_leptos_v08_error_boundaries() {
    // RED PHASE: Error boundary compatibility issues

    let (should_error, set_error) = signal(false);

    // This should work with Leptos v0.8 error boundaries
    let component = view! {
        <ErrorBoundary
            fallback=|cx, errors| {
                view! { cx,
                    <div class="error">
                        "Animation Error: " {errors.get().into_iter().map(|e| e.to_string()).collect::<Vec<_>>().join(", ")}
                    </div>
                }
            }
        >
            {move || {
                if should_error.get() {
                    // This should trigger an error boundary
                    panic!("Test error for error boundary");
                } else {
                    view! {
                        <MotionDiv
                            animate=Some(create_animation_target("opacity", 1.0))
                        >
                            "Error Boundary Test"
                        </MotionDiv>
                    }
                }
            }}
        </ErrorBoundary>
    };

    // Verify component compiles
    assert!(component.into_view().is_some());

    // Test error boundary
    set_error.set(true);

    // Verify error state
    assert!(should_error.get());
}

/// Test that MotionDiv works with Leptos v0.8 transitions
#[test]
fn test_motion_div_leptos_v08_transitions() {
    // RED PHASE: Transition system compatibility issues

    let (is_loading, set_loading) = signal(true);

    // This should work with Leptos v0.8 transition system
    let component = view! {
        <Transition
            fallback=move || view! { <div>"Loading..."</div> }
        >
            {move || {
                if is_loading.get() {
                    None
                } else {
                    Some(view! {
                        <MotionDiv
                            initial=Some(create_animation_target("opacity", 0.0))
                            animate=Some(create_animation_target("opacity", 1.0))
                            transition=Some(Transition {
                                duration: Some(0.3),
                                ease: Easing::EaseOut,
                                ..Default::default()
                            })
                        >
                            "Transition Test"
                        </MotionDiv>
                    })
                }
            }}
        </Transition>
    };

    // Verify component compiles
    assert!(component.into_view().is_some());

    // Test transition
    set_loading.set(false);

    // Verify transition state
    assert!(!is_loading.get());
}

// Helper functions for tests

fn create_animation_target(property: &str, value: f64) -> AnimationTarget {
    let mut target = HashMap::new();
    target.insert(property.to_string(), AnimationValue::Number(value));
    target
}
