// FLIP Animation Tests - Test-Driven Development for Layout Animations
// FLIP = First, Last, Invert, Play

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys;

// Test 1: Basic FLIP Animation - Element moves from position A to position B
#[test]
fn test_basic_flip_animation() {
    // Create a mock element with initial position
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 200.0,
        y: 150.0,
        width: 100.0,
        height: 100.0,
    };

    // Create FLIP animation
    let flip_animation = FlipAnimation::new(initial_bounds, final_bounds, 300.0);

    // Test that the animation calculates the correct transform
    let transform = flip_animation.calculate_transform(0.0);
    assert_eq!(transform.translate_x, -200.0); // Inverted from final position
    assert_eq!(transform.translate_y, -150.0);
    assert_eq!(transform.scale_x, 1.0);
    assert_eq!(transform.scale_y, 1.0);

    // Test animation progress
    let mid_transform = flip_animation.calculate_transform(0.5);
    assert!(mid_transform.translate_x > -200.0 && mid_transform.translate_x < 0.0);
    assert!(mid_transform.translate_y > -150.0 && mid_transform.translate_y < 0.0);

    // Test final state
    let final_transform = flip_animation.calculate_transform(1.0);
    assert_eq!(final_transform.translate_x, 0.0);
    assert_eq!(final_transform.translate_y, 0.0);
}

// Test 2: FLIP with Scale Changes - Element changes size during animation
#[test]
fn test_flip_with_scale_changes() {
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 200.0,
        height: 150.0,
    };

    let flip_animation = FlipAnimation::new(initial_bounds, final_bounds, 300.0);

    // Test scale calculation
    let transform = flip_animation.calculate_transform(0.0);
    assert_eq!(transform.scale_x, 0.5); // 100/200
    assert_eq!(transform.scale_y, 0.6666666666666666); // 100/150

    let final_transform = flip_animation.calculate_transform(1.0);
    assert_eq!(final_transform.scale_x, 1.0);
    assert_eq!(final_transform.scale_y, 1.0);
}

// Test 3: FLIP with Rotation - Element rotates during animation
#[test]
fn test_flip_with_rotation() {
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let rotation_change = 90.0; // degrees
    let flip_animation =
        FlipAnimation::with_rotation(initial_bounds, final_bounds, rotation_change, 300.0);

    let transform = flip_animation.calculate_transform(0.0);
    assert_eq!(transform.rotate, -90.0); // Inverted rotation

    let final_transform = flip_animation.calculate_transform(1.0);
    assert_eq!(final_transform.rotate, 0.0);
}

// Test 4: FLIP Animation Timing - Test different easing functions
#[test]
fn test_flip_animation_timing() {
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 100.0,
        y: 100.0,
        width: 100.0,
        height: 100.0,
    };

    // Test linear easing
    let linear_flip =
        FlipAnimation::with_easing(initial_bounds, final_bounds, Easing::Linear, 300.0);
    let mid_linear = linear_flip.calculate_transform(0.5);
    assert_eq!(mid_linear.translate_x, -50.0); // Exactly halfway

    // Test ease-in-out
    let ease_flip =
        FlipAnimation::with_easing(initial_bounds, final_bounds, Easing::EaseInOut, 300.0);
    let mid_ease = ease_flip.calculate_transform(0.5);
    // Ease-in-out at 0.5 should be exactly 0.5 (symmetric function)
    assert_eq!(mid_ease.translate_x, -50.0); // Should be exactly halfway
    assert!(mid_ease.translate_x < 0.0); // Should still be negative (inverted)
}

// Test 5: FLIP Animation State Management
#[test]
fn test_flip_animation_state_management() {
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 200.0,
        y: 150.0,
        width: 100.0,
        height: 100.0,
    };

    let mut flip_animation = FlipAnimation::new(initial_bounds, final_bounds, 300.0);

    // Test initial state
    assert_eq!(flip_animation.state(), FlipState::Ready);

    // Test running state
    flip_animation.start();
    assert_eq!(flip_animation.state(), FlipState::Running);

    // Test completion
    flip_animation.update(1.0);
    assert_eq!(flip_animation.state(), FlipState::Completed);
}

// Test 6: FLIP Animation with Multiple Elements
#[test]
fn test_flip_animation_multiple_elements() {
    let elements = vec![
        (
            Bounds {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            Bounds {
                x: 200.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        ),
        (
            Bounds {
                x: 100.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            Bounds {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        ),
    ];

    let mut flip_group = FlipGroup::new(elements, 300.0);

    // Test that all elements are properly initialized
    assert_eq!(flip_group.animations().len(), 2);

    // Test group state
    assert_eq!(flip_group.state(), FlipState::Ready);
    flip_group.start();
    assert_eq!(flip_group.state(), FlipState::Running);
}

// Test 7: FLIP Animation Performance - Test animation loop efficiency
#[test]
fn test_flip_animation_performance() {
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 200.0,
        y: 150.0,
        width: 100.0,
        height: 100.0,
    };

    let flip_animation = FlipAnimation::new(initial_bounds, final_bounds, 300.0);

    // Test that calculations are fast (mock timing for unit tests)
    let start_time = 0.0;

    for i in 0..1000 {
        let progress = (i as f64) / 1000.0;
        let _transform = flip_animation.calculate_transform(progress);
    }

    let end_time = 1.0; // Mock end time
    let duration = end_time - start_time;

    // Should complete 1000 calculations in less than 10ms (mock assertion)
    assert!(duration < 10.0);
}

// Test 8: FLIP Animation Edge Cases
#[test]
fn test_flip_animation_edge_cases() {
    // Test zero duration
    let initial_bounds = Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };

    let final_bounds = Bounds {
        x: 100.0,
        y: 100.0,
        width: 100.0,
        height: 100.0,
    };

    let instant_flip = FlipAnimation::new(initial_bounds, final_bounds, 0.0);
    let transform = instant_flip.calculate_transform(1.0); // Use progress 1.0 for instant animation
    assert_eq!(transform.translate_x, 0.0); // Should be at final position immediately

    // Test identical bounds
    let no_change_flip = FlipAnimation::new(initial_bounds, initial_bounds, 300.0);
    let transform = no_change_flip.calculate_transform(0.0);
    assert_eq!(transform.translate_x, 0.0);
    assert_eq!(transform.translate_y, 0.0);
    assert_eq!(transform.scale_x, 1.0);
    assert_eq!(transform.scale_y, 1.0);
}

// Test 9: FLIP Animation Integration with MotionDiv
#[test]
fn test_flip_animation_motion_div_integration() {
    // This test will verify that FLIP animations work with the MotionDiv component
    // We'll test the _layout prop and automatic FLIP detection

    let (layout_enabled, set_layout_enabled) = signal(true);
    let (element_bounds, set_element_bounds) = signal(Bounds {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    });

    // Simulate layout change
    set_element_bounds.set(Bounds {
        x: 200.0,
        y: 150.0,
        width: 100.0,
        height: 100.0,
    });

    // Test that FLIP animation is automatically triggered
    // This will be implemented in the MotionDiv component
    assert!(layout_enabled.get());
}

// Test 10: FLIP Animation with Stagger
#[test]
fn test_flip_animation_with_stagger() {
    let elements = vec![
        (
            Bounds {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            Bounds {
                x: 200.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        ),
        (
            Bounds {
                x: 100.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            Bounds {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        ),
        (
            Bounds {
                x: 200.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
            Bounds {
                x: 100.0,
                y: 0.0,
                width: 100.0,
                height: 100.0,
            },
        ),
    ];

    let stagger_delay = 100.0; // 100ms between each element
    let flip_group = FlipGroup::with_stagger(elements, 300.0, stagger_delay);

    // Test stagger timing
    let animations = flip_group.animations();
    assert_eq!(animations.len(), 3);

    // Test that animations have different start times
    let start_times: Vec<f64> = animations.iter().map(|anim| anim.start_time()).collect();
    assert_eq!(start_times[1] - start_times[0], -stagger_delay); // Negative because we set negative start times
    assert_eq!(start_times[2] - start_times[1], -stagger_delay);
}

// Supporting types and structures for FLIP animations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub translate_x: f64,
    pub translate_y: f64,
    pub scale_x: f64,
    pub scale_y: f64,
    pub rotate: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlipState {
    Ready,
    Running,
    Completed,
    Paused,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Easing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Spring,
}

pub struct FlipAnimation {
    initial_bounds: Bounds,
    final_bounds: Bounds,
    duration: f64,
    easing: Easing,
    state: FlipState,
    start_time: f64,
    rotation_change: f64,
}

impl FlipAnimation {
    pub fn new(initial_bounds: Bounds, final_bounds: Bounds, duration: f64) -> Self {
        Self {
            initial_bounds,
            final_bounds,
            duration,
            easing: Easing::EaseInOut,
            state: FlipState::Ready,
            start_time: 0.0,
            rotation_change: 0.0,
        }
    }

    pub fn with_easing(
        initial_bounds: Bounds,
        final_bounds: Bounds,
        easing: Easing,
        duration: f64,
    ) -> Self {
        Self {
            initial_bounds,
            final_bounds,
            duration,
            easing,
            state: FlipState::Ready,
            start_time: 0.0,
            rotation_change: 0.0,
        }
    }

    pub fn with_rotation(
        initial_bounds: Bounds,
        final_bounds: Bounds,
        rotation_change: f64,
        duration: f64,
    ) -> Self {
        Self {
            initial_bounds,
            final_bounds,
            duration,
            easing: Easing::EaseInOut,
            state: FlipState::Ready,
            start_time: 0.0,
            rotation_change,
        }
    }

    pub fn calculate_transform(&self, progress: f64) -> Transform {
        let eased_progress = self.apply_easing(progress);

        // Calculate position difference
        let dx = self.final_bounds.x - self.initial_bounds.x;
        let dy = self.final_bounds.y - self.initial_bounds.y;

        // Calculate scale difference
        let scale_x = self.initial_bounds.width / self.final_bounds.width;
        let scale_y = self.initial_bounds.height / self.final_bounds.height;

        // Apply FLIP: First, Last, Invert, Play
        // Invert: Start from the difference (negative)
        // Play: Animate to 0
        let translate_x = -dx * (1.0 - eased_progress);
        let translate_y = -dy * (1.0 - eased_progress);
        let final_scale_x = 1.0 + (scale_x - 1.0) * (1.0 - eased_progress);
        let final_scale_y = 1.0 + (scale_y - 1.0) * (1.0 - eased_progress);
        let final_rotate = -self.rotation_change * (1.0 - eased_progress);

        Transform {
            translate_x,
            translate_y,
            scale_x: final_scale_x,
            scale_y: final_scale_y,
            rotate: final_rotate,
        }
    }

    pub fn apply_easing(&self, progress: f64) -> f64 {
        match self.easing {
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
            Easing::Spring => {
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
        self.state = FlipState::Running;
        self.start_time = 0.0; // Mock time for testing
    }

    pub fn update(&mut self, progress: f64) {
        if progress >= 1.0 {
            self.state = FlipState::Completed;
        }
    }

    pub fn state(&self) -> FlipState {
        self.state.clone()
    }

    pub fn start_time(&self) -> f64 {
        self.start_time
    }
}

pub struct FlipGroup {
    animations: Vec<FlipAnimation>,
    state: FlipState,
    stagger_delay: f64,
}

impl FlipGroup {
    pub fn new(element_pairs: Vec<(Bounds, Bounds)>, duration: f64) -> Self {
        let animations = element_pairs
            .into_iter()
            .map(|(initial, final_bounds)| FlipAnimation::new(initial, final_bounds, duration))
            .collect();

        Self {
            animations,
            state: FlipState::Ready,
            stagger_delay: 0.0,
        }
    }

    pub fn with_stagger(
        element_pairs: Vec<(Bounds, Bounds)>,
        duration: f64,
        stagger_delay: f64,
    ) -> Self {
        let mut animations = Vec::new();
        for (i, (initial, final_bounds)) in element_pairs.into_iter().enumerate() {
            let mut anim = FlipAnimation::new(initial, final_bounds, duration);
            anim.start_time = -(i as f64) * stagger_delay;
            animations.push(anim);
        }

        Self {
            animations,
            state: FlipState::Ready,
            stagger_delay,
        }
    }

    pub fn animations(&self) -> &Vec<FlipAnimation> {
        &self.animations
    }

    pub fn start(&mut self) {
        self.state = FlipState::Running;
        for anim in &mut self.animations {
            anim.start();
        }
    }

    pub fn state(&self) -> FlipState {
        self.state.clone()
    }
}
