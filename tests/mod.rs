// Test module configuration for Leptos Motion

pub mod unit;
pub mod integration;
pub mod performance;

// Re-export commonly used test utilities
pub use leptos_motion_core::*;
pub use leptos_motion::*;

// Test utilities
pub mod utils {
    use leptos::prelude::*;
    use std::time::Duration;
    
    /// Wait for a specified duration (useful for async tests)
    pub async fn wait_for(duration: Duration) {
        tokio::time::sleep(duration).await;
    }
    
    /// Wait for a short duration (100ms)
    pub async fn wait_short() {
        wait_for(Duration::from_millis(100)).await;
    }
    
    /// Wait for a medium duration (500ms)
    pub async fn wait_medium() {
        wait_for(Duration::from_millis(500)).await;
    }
    
    /// Wait for a long duration (1s)
    pub async fn wait_long() {
        wait_for(Duration::from_millis(1000)).await;
    }
    
    /// Create a simple test component
    pub fn create_test_component() -> impl IntoView {
        view! {
            <div class="test-container">
                <MotionDiv class="test-element">
                    "Test Content"
                </MotionDiv>
            </div>
        }
    }
    
    /// Create a test component with animation
    pub fn create_animated_test_component() -> impl IntoView {
        view! {
            <div class="test-container">
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
            </div>
        }
    }
}

// Test configuration
pub mod config {
    use std::time::Duration;
    
    /// Default test timeout
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Short test timeout
    pub const SHORT_TIMEOUT: Duration = Duration::from_secs(5);
    
    /// Animation test timeout
    pub const ANIMATION_TIMEOUT: Duration = Duration::from_secs(10);
    
    /// Performance test timeout
    pub const PERFORMANCE_TIMEOUT: Duration = Duration::from_secs(60);
}

// Test assertions
pub mod assertions {
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;
    
    /// Assert that two animation values are approximately equal
    pub fn assert_animation_value_approx(
        actual: &AnimationValue,
        expected: &AnimationValue,
        tolerance: f64,
    ) {
        match (actual, expected) {
            (AnimationValue::Number(a), AnimationValue::Number(e)) => {
                assert!((a - e).abs() < tolerance, "Expected {}, got {}", e, a);
            }
            (AnimationValue::Pixels(a), AnimationValue::Pixels(e)) => {
                assert!((a - e).abs() < tolerance, "Expected {}, got {}", e, a);
            }
            (AnimationValue::Degrees(a), AnimationValue::Degrees(e)) => {
                assert!((a - e).abs() < tolerance, "Expected {}, got {}", e, a);
            }
            (AnimationValue::Color(a), AnimationValue::Color(e)) => {
                assert_eq!(a, e, "Expected {}, got {}", e, a);
            }
            _ => {
                assert_eq!(actual, expected, "Animation value types don't match");
            }
        }
    }
    
    /// Assert that an animation target contains expected values
    pub fn assert_animation_target_contains(
        target: &AnimationTarget,
        expected_values: &HashMap<String, AnimationValue>,
    ) {
        for (key, expected_value) in expected_values {
            let actual_value = target.get(key);
            assert!(
                actual_value.is_some(),
                "Expected key '{}' not found in animation target",
                key
            );
            assert_eq!(actual_value.unwrap(), expected_value);
        }
    }
    
    /// Assert that a motion value is within a range
    pub fn assert_motion_value_in_range(
        motion_value: &MotionValue<f64>,
        min: f64,
        max: f64,
    ) {
        let value = motion_value.get();
        assert!(
            value >= min && value <= max,
            "Expected value between {} and {}, got {}",
            min,
            max,
            value
        );
    }
}
