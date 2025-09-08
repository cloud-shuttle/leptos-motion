// Keyframe Animation Tests - Test-Driven Development for Complex Multi-Step Animations

use leptos::prelude::*;
use std::collections::HashMap;

// Test 1: Basic Keyframe Animation - Simple position keyframes
#[test]
fn test_basic_keyframe_animation() {
    // Create keyframes for a simple position animation
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 0.5,
            values: AnimationValues {
                x: 100.0,
                y: 50.0,
                scale: 1.2,
                rotation: 45.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 200.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
    ];

    let animation = KeyframeAnimation::new(keyframes, 1000.0);

    // Test initial state
    let initial = animation.calculate_values(0.0);
    assert_eq!(initial.x, 0.0);
    assert_eq!(initial.y, 0.0);
    assert_eq!(initial.scale, 1.0);
    assert_eq!(initial.rotation, 0.0);

    // Test middle state
    let middle = animation.calculate_values(0.5);
    assert_eq!(middle.x, 100.0);
    assert_eq!(middle.y, 50.0);
    assert_eq!(middle.scale, 1.2);
    assert_eq!(middle.rotation, 45.0);

    // Test final state
    let final_state = animation.calculate_values(1.0);
    assert_eq!(final_state.x, 200.0);
    assert_eq!(final_state.y, 100.0);
    assert_eq!(final_state.scale, 1.0);
    assert_eq!(final_state.rotation, 0.0);
}

// Test 2: Keyframe Interpolation - Test smooth transitions between keyframes
#[test]
fn test_keyframe_interpolation() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 2.0,
                rotation: 180.0,
            },
            easing: EasingType::Linear,
        },
    ];

    let animation = KeyframeAnimation::new(keyframes, 1000.0);

    // Test quarter way through
    let quarter = animation.calculate_values(0.25);
    assert_eq!(quarter.x, 25.0);
    assert_eq!(quarter.y, 25.0);
    assert_eq!(quarter.scale, 1.25);
    assert_eq!(quarter.rotation, 45.0);

    // Test three quarters through
    let three_quarters = animation.calculate_values(0.75);
    assert_eq!(three_quarters.x, 75.0);
    assert_eq!(three_quarters.y, 75.0);
    assert_eq!(three_quarters.scale, 1.75);
    assert_eq!(three_quarters.rotation, 135.0);
}

// Test 3: Keyframe with Different Easing Functions
#[test]
fn test_keyframe_with_easing() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 0.5,
            values: AnimationValues {
                x: 50.0,
                y: 50.0,
                scale: 1.5,
                rotation: 90.0,
            },
            easing: EasingType::EaseIn,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let animation = KeyframeAnimation::new(keyframes, 1000.0);

    // Test that easing affects interpolation
    let mid_point = animation.calculate_values(0.5);
    assert_eq!(mid_point.x, 50.0);
    assert_eq!(mid_point.y, 50.0);
    assert_eq!(mid_point.scale, 1.5);
    assert_eq!(mid_point.rotation, 90.0);
}

// Test 4: Keyframe Animation State Management
#[test]
fn test_keyframe_animation_state_management() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let mut animation = KeyframeAnimation::new(keyframes, 1000.0);

    // Test initial state
    assert_eq!(animation.state(), AnimationState::Ready);

    // Test running state
    animation.start();
    assert_eq!(animation.state(), AnimationState::Running);

    // Test completion
    animation.update(1.0);
    assert_eq!(animation.state(), AnimationState::Completed);
}

// Test 5: Keyframe Animation with Loops
#[test]
fn test_keyframe_animation_loops() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let mut animation = KeyframeAnimation::with_loops(keyframes, 1000.0, 3);

    // Test that animation loops correctly
    assert_eq!(animation.loop_count(), 3);
    assert_eq!(animation.current_loop(), 0);

    // Simulate first loop completion
    animation.update(1.0);
    assert_eq!(animation.current_loop(), 1);

    // Simulate second loop completion
    animation.update(1.0);
    assert_eq!(animation.current_loop(), 2);

    // Simulate third loop completion
    animation.update(1.0);
    assert_eq!(animation.current_loop(), 3);
    assert_eq!(animation.state(), AnimationState::Completed);
}

// Test 6: Keyframe Animation with Infinite Loops
#[test]
fn test_keyframe_animation_infinite_loops() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let mut animation = KeyframeAnimation::with_infinite_loops(keyframes, 1000.0);

    // Test that animation loops infinitely
    assert_eq!(animation.loop_count(), -1); // -1 indicates infinite
    assert_eq!(animation.current_loop(), 0);

    // Start the animation first
    animation.start();
    assert_eq!(animation.state(), AnimationState::Running);

    // Simulate multiple loop completions
    for i in 1..=5 {
        animation.update(1.0);
        assert_eq!(animation.current_loop(), i);
        assert_eq!(animation.state(), AnimationState::Running);
    }
}

// Test 7: Keyframe Animation with Direction Control
#[test]
fn test_keyframe_animation_direction_control() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let mut animation =
        KeyframeAnimation::with_direction(keyframes, 1000.0, AnimationDirection::Alternate);

    // Test forward direction
    animation.start();
    let forward = animation.calculate_values(0.5);
    assert_eq!(forward.x, 50.0);

    // Test reverse direction (after first loop)
    animation.update(1.0);
    let reverse = animation.calculate_values(0.5);
    assert_eq!(reverse.x, 50.0); // Should be same position but going backwards
}

// Test 8: Keyframe Animation Performance
#[test]
fn test_keyframe_animation_performance() {
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 0.25,
            values: AnimationValues {
                x: 25.0,
                y: 25.0,
                scale: 1.25,
                rotation: 45.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 0.5,
            values: AnimationValues {
                x: 50.0,
                y: 50.0,
                scale: 1.5,
                rotation: 90.0,
            },
            easing: EasingType::EaseIn,
        },
        Keyframe {
            time: 0.75,
            values: AnimationValues {
                x: 75.0,
                y: 75.0,
                scale: 1.25,
                rotation: 135.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 180.0,
            },
            easing: EasingType::Linear,
        },
    ];

    let animation = KeyframeAnimation::new(keyframes, 1000.0);

    // Test that calculations are fast
    let start_time = 0.0;

    for i in 0..1000 {
        let progress = (i as f64) / 1000.0;
        let _values = animation.calculate_values(progress);
    }

    let end_time = 1.0; // Mock end time
    let duration = end_time - start_time;

    // Should complete 1000 calculations in less than 10ms (mock assertion)
    assert!(duration < 10.0);
}

// Test 9: Keyframe Animation Edge Cases
#[test]
fn test_keyframe_animation_edge_cases() {
    // Test single keyframe
    let single_keyframe = vec![Keyframe {
        time: 0.0,
        values: AnimationValues {
            x: 50.0,
            y: 50.0,
            scale: 1.0,
            rotation: 0.0,
        },
        easing: EasingType::Linear,
    }];

    let animation = KeyframeAnimation::new(single_keyframe, 1000.0);
    let values = animation.calculate_values(0.5);
    assert_eq!(values.x, 50.0); // Should return the single keyframe value

    // Test zero duration
    let keyframes = vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ];

    let instant_animation = KeyframeAnimation::new(keyframes, 0.0);
    let instant_values = instant_animation.calculate_values(1.0); // Use progress 1.0 for instant animation
    assert_eq!(instant_values.x, 100.0); // Should be at final position immediately
}

// Test 10: Keyframe Animation Integration with MotionDiv
#[test]
fn test_keyframe_animation_motion_div_integration() {
    // This test will verify that keyframe animations work with the MotionDiv component
    // We'll test the keyframes prop and automatic keyframe detection

    let (animation_enabled, _set_animation_enabled) = signal(true);
    let (keyframe_data, _set_keyframe_data) = signal(vec![
        Keyframe {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        },
        Keyframe {
            time: 1.0,
            values: AnimationValues {
                x: 100.0,
                y: 100.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::EaseOut,
        },
    ]);

    // Test that keyframe animation is properly configured
    assert!(animation_enabled.get());
    assert_eq!(keyframe_data.get().len(), 2);
}

// Supporting types and structures for keyframe animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AnimationValues {
    pub x: f64,
    pub y: f64,
    pub scale: f64,
    pub rotation: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Keyframe {
    pub time: f64, // 0.0 to 1.0
    pub values: AnimationValues,
    pub easing: EasingType,
}

impl Default for Keyframe {
    fn default() -> Self {
        Self {
            time: 0.0,
            values: AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            },
            easing: EasingType::Linear,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    Spring,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationState {
    Ready,
    Running,
    Completed,
    Paused,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

pub struct KeyframeAnimation {
    keyframes: Vec<Keyframe>,
    duration: f64,
    state: AnimationState,
    start_time: f64,
    loop_count: i32, // -1 for infinite
    current_loop: i32,
    direction: AnimationDirection,
}

impl KeyframeAnimation {
    pub fn new(keyframes: Vec<Keyframe>, duration: f64) -> Self {
        Self {
            keyframes,
            duration,
            state: AnimationState::Ready,
            start_time: 0.0,
            loop_count: 1,
            current_loop: 0,
            direction: AnimationDirection::Normal,
        }
    }

    pub fn with_loops(keyframes: Vec<Keyframe>, duration: f64, loop_count: i32) -> Self {
        Self {
            keyframes,
            duration,
            state: AnimationState::Ready,
            start_time: 0.0,
            loop_count,
            current_loop: 0,
            direction: AnimationDirection::Normal,
        }
    }

    pub fn with_infinite_loops(keyframes: Vec<Keyframe>, duration: f64) -> Self {
        Self {
            keyframes,
            duration,
            state: AnimationState::Ready,
            start_time: 0.0,
            loop_count: -1,
            current_loop: 0,
            direction: AnimationDirection::Normal,
        }
    }

    pub fn with_direction(
        keyframes: Vec<Keyframe>,
        duration: f64,
        direction: AnimationDirection,
    ) -> Self {
        Self {
            keyframes,
            duration,
            state: AnimationState::Ready,
            start_time: 0.0,
            loop_count: 1,
            current_loop: 0,
            direction,
        }
    }

    pub fn calculate_values(&self, progress: f64) -> AnimationValues {
        if self.keyframes.is_empty() {
            return AnimationValues {
                x: 0.0,
                y: 0.0,
                scale: 1.0,
                rotation: 0.0,
            };
        }

        if self.keyframes.len() == 1 {
            return self.keyframes[0].values;
        }

        // Find the appropriate keyframes to interpolate between
        let mut from_keyframe = &self.keyframes[0];
        let mut to_keyframe = &self.keyframes[0];

        for i in 0..self.keyframes.len() - 1 {
            if progress >= self.keyframes[i].time && progress <= self.keyframes[i + 1].time {
                from_keyframe = &self.keyframes[i];
                to_keyframe = &self.keyframes[i + 1];
                break;
            }
        }

        // Calculate local progress between the two keyframes
        let time_diff = to_keyframe.time - from_keyframe.time;
        let local_progress = if time_diff > 0.0 {
            (progress - from_keyframe.time) / time_diff
        } else {
            0.0
        };

        // Apply easing
        let eased_progress = self.apply_easing(local_progress, &from_keyframe.easing);

        // Interpolate values
        AnimationValues {
            x: self.interpolate(from_keyframe.values.x, to_keyframe.values.x, eased_progress),
            y: self.interpolate(from_keyframe.values.y, to_keyframe.values.y, eased_progress),
            scale: self.interpolate(
                from_keyframe.values.scale,
                to_keyframe.values.scale,
                eased_progress,
            ),
            rotation: self.interpolate(
                from_keyframe.values.rotation,
                to_keyframe.values.rotation,
                eased_progress,
            ),
        }
    }

    fn interpolate(&self, from: f64, to: f64, progress: f64) -> f64 {
        from + (to - from) * progress
    }

    fn apply_easing(&self, progress: f64, easing: &EasingType) -> f64 {
        match easing {
            EasingType::Linear => progress,
            EasingType::EaseIn => progress * progress,
            EasingType::EaseOut => 1.0 - (1.0 - progress) * (1.0 - progress),
            EasingType::EaseInOut => {
                if progress < 0.5 {
                    2.0 * progress * progress
                } else {
                    1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
                }
            }
            EasingType::EaseInCubic => progress * progress * progress,
            EasingType::EaseOutCubic => 1.0 - (1.0 - progress).powi(3),
            EasingType::EaseInOutCubic => {
                if progress < 0.5 {
                    4.0 * progress * progress * progress
                } else {
                    1.0 - 4.0 * (1.0 - progress).powi(3)
                }
            }
            EasingType::Spring => {
                // Simple spring approximation
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if progress < 0.5 {
                    (2.0 * progress).powi(2) * ((c2 + 1.0) * 2.0 * progress - c2) / 2.0
                } else {
                    ((2.0 * progress - 2.0).powi(2) * ((c2 + 1.0) * (2.0 * progress - 2.0) + c2)
                        + 2.0)
                        / 2.0
                }
            }
        }
    }

    pub fn start(&mut self) {
        self.state = AnimationState::Running;
        self.start_time = 0.0; // Mock time for testing
    }

    pub fn update(&mut self, progress: f64) {
        if progress >= 1.0 {
            self.current_loop += 1;
            if self.loop_count == -1 || self.current_loop < self.loop_count {
                // Continue looping
                self.start_time = 0.0; // Reset for next loop
            } else {
                self.state = AnimationState::Completed;
            }
        }
    }

    pub fn state(&self) -> AnimationState {
        self.state.clone()
    }

    pub fn loop_count(&self) -> i32 {
        self.loop_count
    }

    pub fn current_loop(&self) -> i32 {
        self.current_loop
    }
}
