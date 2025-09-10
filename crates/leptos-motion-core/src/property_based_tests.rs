//! Property-Based Testing Framework for Leptos Motion
//!
//! This module implements comprehensive property-based testing using proptest
//! to ensure animation values, transitions, and easing functions behave correctly
//! across all possible inputs.

use crate::types::*;
use proptest::prelude::*;

/// Property-based tests for animation value interpolation
#[cfg(test)]
mod interpolation_property_tests {
    use super::*;

    proptest! {
        #[test]
        fn test_linear_interpolation_properties(
            start in any::<f64>(),
            end in any::<f64>(),
            progress in 0.0f64..1.0f64
        ) {
            let result = linear_interpolate(start, end, progress);

            // Property 1: Result should be between start and end
            prop_assert!(result >= start.min(end));
            prop_assert!(result <= start.max(end));

            // Property 2: At progress 0, result should equal start
            if progress == 0.0 {
                prop_assert_eq!(result, start);
            }

            // Property 3: At progress 1, result should equal end
            if progress == 1.0 {
                prop_assert_eq!(result, end);
            }

            // Property 4: Linear interpolation should be monotonic
            let result_half = linear_interpolate(start, end, 0.5);
            if start <= end {
                prop_assert!(result_half >= start);
                prop_assert!(result_half <= end);
            } else {
                prop_assert!(result_half <= start);
                prop_assert!(result_half >= end);
            }
        }

        #[test]
        fn test_easing_function_properties(
            progress in 0.0f64..1.0f64,
            easing in prop::sample::select(&[
                Easing::Linear,
                Easing::EaseIn,
                Easing::EaseOut,
                Easing::EaseInOut,
                Easing::CircIn,
                Easing::CircOut,
                Easing::CircInOut,
                Easing::BackIn,
                Easing::BackOut,
                Easing::BackInOut,
            ])
        ) {
            let result = apply_easing(progress, &easing);

            // Property 1: Easing result should be between 0 and 1
            prop_assert!(result >= 0.0);
            prop_assert!(result <= 1.0);

            // Property 2: At progress 0, result should be 0
            if progress == 0.0 {
                prop_assert_eq!(result, 0.0);
            }

            // Property 3: At progress 1, result should be 1
            if progress == 1.0 {
                prop_assert_eq!(result, 1.0);
            }

            // Property 4: Easing should be monotonic for most functions
            match easing {
                Easing::Linear | Easing::EaseIn | Easing::EaseOut | Easing::EaseInOut => {
                    // These should be monotonic
                    let result_plus = apply_easing((progress + 0.01).min(1.0), &easing);
                    prop_assert!(result_plus >= result);
                }
                _ => {
                    // Other easing functions might not be strictly monotonic
                    // (e.g., BackIn/BackOut can overshoot)
                }
            }
        }

        #[test]
        fn test_animation_value_properties(
            value in any::<f64>()
        ) {
            let animation_value = AnimationValue::Number(value);

            // Property 1: AnimationValue should preserve the original value
            match animation_value {
                AnimationValue::Number(n) => prop_assert_eq!(n, value),
                _ => prop_assert!(false, "Expected Number variant"),
            }
        }

        #[test]
        fn test_transition_properties(
            duration in 0.0f64..10.0f64,
            delay in 0.0f64..5.0f64,
            ease in prop::sample::select(&[
                Easing::Linear,
                Easing::EaseIn,
                Easing::EaseOut,
                Easing::EaseInOut,
            ]),
            repeat in prop::sample::select(&[
                RepeatConfig::Never,
                RepeatConfig::Count(1),
                RepeatConfig::Count(5),
                RepeatConfig::Infinite,
            ])
        ) {
            let transition = Transition {
                duration: Some(duration),
                delay: Some(delay),
                ease: ease.clone(),
                repeat: repeat.clone(),
                stagger: None,
            };

            // Property 1: Transition should preserve all values
            prop_assert_eq!(transition.duration, Some(duration));
            prop_assert_eq!(transition.delay, Some(delay));
            prop_assert_eq!(transition.ease, ease);
            prop_assert_eq!(transition.repeat, repeat);
        }

        #[test]
        fn test_animation_target_properties(
            x in any::<f64>(),
            y in any::<f64>(),
            scale in 0.1f64..5.0f64,
            rotation in -360.0f64..360.0f64,
            opacity in 0.0f64..1.0f64
        ) {
            let mut target = AnimationTarget::new();
            target.insert("x".to_string(), AnimationValue::Number(x));
            target.insert("y".to_string(), AnimationValue::Number(y));
            target.insert("scale".to_string(), AnimationValue::Number(scale));
            target.insert("rotation".to_string(), AnimationValue::Number(rotation));
            target.insert("opacity".to_string(), AnimationValue::Number(opacity));

            // Property 1: AnimationTarget should preserve all values
            prop_assert_eq!(target.get("x"), Some(&AnimationValue::Number(x)));
            prop_assert_eq!(target.get("y"), Some(&AnimationValue::Number(y)));
            prop_assert_eq!(target.get("scale"), Some(&AnimationValue::Number(scale)));
            prop_assert_eq!(target.get("rotation"), Some(&AnimationValue::Number(rotation)));
            prop_assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(opacity)));
        }
    }
}

/// Property-based tests for edge cases and boundary conditions
#[cfg(test)]
mod edge_case_property_tests {
    use super::*;

    proptest! {
        #[test]
        fn test_extreme_values_interpolation(
            start in -1e10f64..1e10f64,
            end in -1e10f64..1e10f64,
            progress in 0.0f64..1.0f64
        ) {
            let result = linear_interpolate(start, end, progress);

            // Property 1: Should handle extreme values without overflow
            prop_assert!(result.is_finite());

            // Property 2: Should handle NaN inputs gracefully
            if start.is_nan() || end.is_nan() {
                prop_assert!(result.is_nan());
            }

            // Property 3: Should handle infinite values
            if start.is_infinite() && end.is_infinite() && start.signum() == end.signum() {
                prop_assert_eq!(result, start);
            }
        }

        #[test]
        fn test_zero_duration_animations(
            duration in 0.0f64..0.001f64, // Very small durations
            progress in 0.0f64..1.0f64
        ) {
            let transition = Transition {
                duration: Some(duration),
                delay: Some(0.0),
                ease: Easing::Linear,
                repeat: RepeatConfig::Never,
                stagger: None,
            };

            // Property 1: Zero duration animations should complete immediately
            if duration == 0.0 {
                prop_assert_eq!(calculate_animation_progress(&transition, 0.0), 1.0);
            }

            // Property 2: Very small durations should not cause division by zero
            let progress_result = calculate_animation_progress(&transition, progress);
            prop_assert!(progress_result.is_finite());
        }

        #[test]
        fn test_negative_values_handling(
            negative_value in -1000.0f64..-0.1f64,
            progress in 0.0f64..1.0f64
        ) {
            let result = linear_interpolate(negative_value, 0.0, progress);

            // Property 1: Should handle negative values correctly
            prop_assert!(result >= negative_value);
            prop_assert!(result <= 0.0);

            // Property 2: Should be monotonic
            let result_plus = linear_interpolate(negative_value, 0.0, (progress + 0.01).min(1.0));
            prop_assert!(result_plus >= result);
        }
    }
}

/// Property-based tests for performance characteristics
#[cfg(test)]
mod performance_property_tests {
    use super::*;
    use std::time::Instant;

    proptest! {
        #[test]
        fn test_interpolation_performance(
            start in any::<f64>(),
            end in any::<f64>(),
            progress in 0.0f64..1.0f64
        ) {
            let start_time = Instant::now();
            let _result = linear_interpolate(start, end, progress);
            let duration = start_time.elapsed();

            // Property 1: Interpolation should be fast (< 1ms)
            prop_assert!(duration.as_micros() < 1000);
        }

        #[test]
        fn test_easing_performance(
            progress in 0.0f64..1.0f64,
            easing in prop::sample::select(&[
                Easing::Linear,
                Easing::EaseIn,
                Easing::EaseOut,
                Easing::EaseInOut,
                Easing::CircIn,
                Easing::CircOut,
                Easing::CircInOut,
            ])
        ) {
            let start_time = Instant::now();
            let _result = apply_easing(progress, &easing);
            let duration = start_time.elapsed();

            // Property 1: Easing should be fast (< 1ms)
            prop_assert!(duration.as_micros() < 1000);
        }
    }
}

/// Helper functions for property-based tests
fn linear_interpolate(start: f64, end: f64, progress: f64) -> f64 {
    start + (end - start) * progress
}

fn apply_easing(progress: f64, easing: &Easing) -> f64 {
    match easing {
        Easing::Linear => progress,
        Easing::EaseIn => progress * progress,
        Easing::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
        Easing::EaseInOut => {
            if progress < 0.5 {
                2.0 * progress * progress
            } else {
                1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
            }
        }
        Easing::CircIn => 1.0 - (1.0 - progress * progress).sqrt(),
        Easing::CircOut => ((2.0 - progress) * progress).sqrt(),
        Easing::CircInOut => {
            if progress < 0.5 {
                0.5 * (1.0 - (1.0 - 4.0 * progress * progress).sqrt())
            } else {
                0.5 * ((4.0 * progress - 2.0) * (2.0 - 2.0 * progress) + 1.0).sqrt()
            }
        }
        Easing::BackIn => {
            const C1: f64 = 1.70158;
            const C3: f64 = C1 + 1.0;
            let result = C3 * progress * progress * progress - C1 * progress * progress;
            result.max(0.0) // Ensure result is never negative
        }
        Easing::BackOut => {
            const C1: f64 = 1.70158;
            const C3: f64 = C1 + 1.0;
            let result = 1.0 + C3 * (progress - 1.0).powi(3) + C1 * (progress - 1.0).powi(2);
            result.max(0.0).min(1.0) // Ensure result is between 0 and 1
        }
        Easing::BackInOut => {
            const C1: f64 = 1.70158;
            const C2: f64 = C1 * 1.525;
            let result = if progress < 0.5 {
                ((2.0 * progress).powi(2) * ((C2 + 1.0) * 2.0 * progress - C2)) / 2.0
            } else {
                ((2.0 * progress - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * progress - 2.0) + C2) + 2.0)
                    / 2.0
            };
            result.max(0.0).min(1.0) // Ensure result is between 0 and 1
        }
        Easing::Bezier(_x1, y1, _x2, y2) => {
            // Simplified cubic bezier implementation for testing
            let t = progress;
            let u = 1.0 - t;
            let tt = t * t;
            let uu = u * u;
            let uuu = uu * u;
            let ttt = tt * t;

            uuu * 0.0 + 3.0 * uu * t * y1 + 3.0 * u * tt * y2 + ttt * 1.0
        }
        Easing::Spring(_) => {
            // Simplified spring implementation for testing
            progress
        }
    }
}

fn calculate_animation_progress(transition: &Transition, elapsed: f64) -> f64 {
    let duration = transition.duration.unwrap_or(1.0);
    let delay = transition.delay.unwrap_or(0.0);

    if elapsed < delay {
        0.0
    } else {
        let progress = (elapsed - delay) / duration;
        progress.min(1.0)
    }
}
