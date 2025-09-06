//! Test file to verify MotionDivFlexible API compatibility
//!
//! This test verifies that MotionDivFlexible can handle the different API patterns
//! found in the examples without requiring type conversions.

use leptos::prelude::*;
use leptos_motion_core::*;
use leptos_motion_dom::components::MotionDivFlexible;

#[component]
pub fn TestMotionDivFlexible() -> impl IntoView {
    view! {
        <div>
            // Test 1: Basic usage with &str class and style
            <MotionDivFlexible
                class="test-class"
                style="background: red; width: 100px; height: 100px;"
            >
                "Basic Test"
            </MotionDivFlexible>

            // Test 2: With bool drag
            <MotionDivFlexible
                class="drag-test"
                drag=true
            >
                "Drag Test"
            </MotionDivFlexible>

            // Test 3: With bool layout
            <MotionDivFlexible
                class="layout-test"
                layout=true
            >
                "Layout Test"
            </MotionDivFlexible>

            // Test 4: With direct AnimationTarget for while_hover
            <MotionDivFlexible
                class="hover-test"
                while_hover=motion_target!(
                    "scale" => AnimationValue::Number(1.1),
                    "opacity" => AnimationValue::Number(0.8)
                )
            >
                "Hover Test"
            </MotionDivFlexible>

            // Test 5: With String key
            <MotionDivFlexible
                class="key-test"
                key="test-key".to_string()
            >
                "Key Test"
            </MotionDivFlexible>

            // Test 6: With &str key
            <MotionDivFlexible
                class="key-test-2"
                key="test-key-2"
            >
                "Key Test 2"
            </MotionDivFlexible>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_motion_div_flexible_compiles() {
        // This test just verifies that the component compiles
        // with the different API patterns
        let _component = TestMotionDivFlexible;
        assert!(true); // If we get here, it compiled successfully
    }
}
