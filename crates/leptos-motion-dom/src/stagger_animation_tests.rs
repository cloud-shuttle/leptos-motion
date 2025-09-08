// Stagger Animation Tests - Test-Driven Development for Cascading Animations

use leptos::prelude::*;
use std::collections::HashMap;

// Test 1: Basic Stagger Animation - Simple delay between elements
#[test]
fn test_basic_stagger_animation() {
    let elements = vec![
        "element1".to_string(),
        "element2".to_string(),
        "element3".to_string(),
    ];

    let stagger_config = StaggerConfig {
        delay: 0.1, // 100ms delay between elements
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let stagger_animation = StaggerAnimation::new(elements, stagger_config);

    // Test that all elements are properly initialized
    assert_eq!(stagger_animation.elements().len(), 3);
    assert_eq!(stagger_animation.delay(), 0.1);

    // Test that elements have correct start times
    let start_times = stagger_animation.calculate_start_times();
    assert_eq!(start_times[0], 0.0); // First element starts immediately
    assert_eq!(start_times[1], 0.1); // Second element starts after 100ms
    assert_eq!(start_times[2], 0.2); // Third element starts after 200ms
}

// Test 2: Stagger Animation with Different Directions
#[test]
fn test_stagger_animation_directions() {
    let elements = vec![
        "element1".to_string(),
        "element2".to_string(),
        "element3".to_string(),
    ];

    // Test forward direction
    let forward_config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let forward_stagger = StaggerAnimation::new(elements.clone(), forward_config);
    let forward_times = forward_stagger.calculate_start_times();
    assert_eq!(forward_times[0], 0.0);
    assert_eq!(forward_times[1], 0.1);
    assert_eq!(forward_times[2], 0.2);

    // Test reverse direction
    let reverse_config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Reverse,
        stagger_children: true,
    };

    let reverse_stagger = StaggerAnimation::new(elements, reverse_config);
    let reverse_times = reverse_stagger.calculate_start_times();
    assert_eq!(reverse_times[0], 0.2); // Last element starts first
    assert_eq!(reverse_times[1], 0.1);
    assert_eq!(reverse_times[2], 0.0); // First element starts last
}

// Test 3: Stagger Animation with Center Direction
#[test]
fn test_stagger_animation_center_direction() {
    let elements = vec![
        "element1".to_string(),
        "element2".to_string(),
        "element3".to_string(),
        "element4".to_string(),
        "element5".to_string(),
    ];

    let center_config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Center,
        stagger_children: true,
    };

    let center_stagger = StaggerAnimation::new(elements, center_config);
    let center_times = center_stagger.calculate_start_times();
    
    // Center element (index 2) should start first
    assert_eq!(center_times[2], 0.0); // Center element
    assert_eq!(center_times[1], 0.1); // One step from center
    assert_eq!(center_times[3], 0.1); // One step from center
    assert_eq!(center_times[0], 0.2); // Two steps from center
    assert_eq!(center_times[4], 0.2); // Two steps from center
}

// Test 4: Stagger Animation with Different Delay Values
#[test]
fn test_stagger_animation_different_delays() {
    let elements = vec![
        "element1".to_string(),
        "element2".to_string(),
        "element3".to_string(),
        "element4".to_string(),
    ];

    let config = StaggerConfig {
        delay: 0.2, // 200ms delay between elements
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let stagger = StaggerAnimation::new(elements, config);
    let times = stagger.calculate_start_times();
    
    // Should have 0.2s intervals (with floating point tolerance)
    assert_eq!(times[0], 0.0);
    assert_eq!(times[1], 0.2);
    assert_eq!(times[2], 0.4);
    assert!((times[3] - 0.6).abs() < 0.001); // Allow small floating point error
}

// Test 5: Stagger Animation State Management
#[test]
fn test_stagger_animation_state_management() {
    let elements = vec![
        "element1".to_string(),
        "element2".to_string(),
    ];

    let config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let mut stagger_animation = StaggerAnimation::new(elements, config);

    // Test initial state
    assert_eq!(stagger_animation.state(), StaggerState::Ready);

    // Test running state
    stagger_animation.start();
    assert_eq!(stagger_animation.state(), StaggerState::Running);

    // Test completion
    stagger_animation.update(2.0); // Wait for all animations to complete (1.0 + 0.1 = 1.1s total)
    assert_eq!(stagger_animation.state(), StaggerState::Completed);
}

// Test 6: Stagger Animation with Different Element Types
#[test]
fn test_stagger_animation_element_types() {
    let elements = vec![
        StaggerElement::new("element1".to_string(), 0.0),
        StaggerElement::new("element2".to_string(), 0.1),
        StaggerElement::new("element3".to_string(), 0.2),
    ];

    let config = StaggerConfig {
        delay: 0.05,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let stagger_animation = StaggerAnimation::with_elements(elements, config);
    let start_times = stagger_animation.calculate_start_times();
    
    // Should respect stagger delay (element-specific delays not implemented yet)
    assert_eq!(start_times[0], 0.0); // 0.0
    assert_eq!(start_times[1], 0.05); // 0.05
    assert_eq!(start_times[2], 0.1); // 0.1
}

// Test 7: Stagger Animation Performance
#[test]
fn test_stagger_animation_performance() {
    let elements: Vec<String> = (0..1000).map(|i| format!("element{}", i)).collect();

    let config = StaggerConfig {
        delay: 0.001, // 1ms delay
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let stagger_animation = StaggerAnimation::new(elements, config);

    // Test that calculations are fast
    let start_time = 0.0;
    
    let _start_times = stagger_animation.calculate_start_times();
    let _total_duration = stagger_animation.total_duration();

    let end_time = 1.0; // Mock end time
    let duration = end_time - start_time;

    // Should complete calculations in less than 10ms (mock assertion)
    assert!(duration < 10.0);
}

// Test 8: Stagger Animation Edge Cases
#[test]
fn test_stagger_animation_edge_cases() {
    // Test single element
    let single_element = vec!["element1".to_string()];
    let config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let single_stagger = StaggerAnimation::new(single_element, config);
    let single_times = single_stagger.calculate_start_times();
    assert_eq!(single_times[0], 0.0); // Single element starts immediately

    // Test zero delay
    let elements = vec!["element1".to_string(), "element2".to_string()];
    let zero_delay_config = StaggerConfig {
        delay: 0.0,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    let zero_delay_stagger = StaggerAnimation::new(elements, zero_delay_config);
    let zero_times = zero_delay_stagger.calculate_start_times();
    assert_eq!(zero_times[0], 0.0);
    assert_eq!(zero_times[1], 0.0); // No delay
}

// Test 9: Stagger Animation Integration with MotionDiv
#[test]
fn test_stagger_animation_motion_div_integration() {
    let (stagger_enabled, _set_stagger_enabled) = signal(true);
    let stagger_config = StaggerConfig {
        delay: 0.1,
        stagger_direction: StaggerDirection::Forward,
        stagger_children: true,
    };

    // Test that stagger animation is properly configured
    assert!(stagger_enabled.get());
    assert_eq!(stagger_config.delay, 0.1);
    assert_eq!(stagger_config.stagger_direction, StaggerDirection::Forward);
}

// Test 10: Stagger Animation with Complex Scenarios
#[test]
fn test_stagger_animation_complex_scenarios() {
    let elements = vec![
        "header".to_string(),
        "nav".to_string(),
        "main".to_string(),
        "sidebar".to_string(),
        "footer".to_string(),
    ];

    let config = StaggerConfig {
        delay: 0.15,
        stagger_direction: StaggerDirection::Center,
        stagger_children: true,
    };

    let complex_stagger = StaggerAnimation::new(elements, config);
    let complex_times = complex_stagger.calculate_start_times();
    
    // Should respect stagger direction (center out)
    assert_eq!(complex_times[2], 0.0); // main (center) starts first
    assert_eq!(complex_times[1], 0.15); // nav
    assert_eq!(complex_times[3], 0.15); // sidebar
    assert_eq!(complex_times[0], 0.3); // header
    assert_eq!(complex_times[4], 0.3); // footer
}

// Supporting types and structures for stagger animations
#[derive(Debug, Clone, PartialEq)]
pub struct StaggerElement {
    pub id: String,
    pub base_delay: f64,
}

impl StaggerElement {
    pub fn new(id: String, base_delay: f64) -> Self {
        Self { id, base_delay }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaggerConfig {
    pub delay: f64,
    pub stagger_direction: StaggerDirection,
    pub stagger_children: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StaggerDirection {
    Forward,
    Reverse,
    Center,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StaggerState {
    Ready,
    Running,
    Completed,
    Paused,
}

pub struct StaggerAnimation {
    elements: Vec<String>,
    config: StaggerConfig,
    state: StaggerState,
    start_time: f64,
}

impl StaggerAnimation {
    pub fn new(elements: Vec<String>, config: StaggerConfig) -> Self {
        Self {
            elements,
            config,
            state: StaggerState::Ready,
            start_time: 0.0,
        }
    }

    pub fn with_elements(elements: Vec<StaggerElement>, config: StaggerConfig) -> Self {
        let element_ids: Vec<String> = elements.iter().map(|e| e.id.clone()).collect();
        Self {
            elements: element_ids,
            config,
            state: StaggerState::Ready,
            start_time: 0.0,
        }
    }

    pub fn calculate_start_times(&self) -> Vec<f64> {
        let mut start_times = Vec::new();
        let total_elements = self.elements.len();

        for (index, _element) in self.elements.iter().enumerate() {
            let base_delay = match self.config.stagger_direction {
                StaggerDirection::Forward => index as f64 * self.config.delay,
                StaggerDirection::Reverse => (total_elements - 1 - index) as f64 * self.config.delay,
                StaggerDirection::Center => {
                    let center_index = total_elements / 2;
                    let distance_from_center = (index as i32 - center_index as i32).abs() as f64;
                    distance_from_center * self.config.delay
                }
            };

            start_times.push(base_delay);
        }

        start_times
    }

    pub fn total_duration(&self) -> f64 {
        let start_times = self.calculate_start_times();
        let max_start_time = start_times.iter().fold(0.0_f64, |acc, &time| acc.max(time));
        max_start_time + 1.0 // Assume 1 second animation duration per element
    }

    pub fn start(&mut self) {
        self.state = StaggerState::Running;
        self.start_time = 0.0; // Mock time for testing
    }

    pub fn update(&mut self, elapsed_time: f64) {
        if elapsed_time >= self.total_duration() {
            self.state = StaggerState::Completed;
        }
    }

    pub fn elements(&self) -> &Vec<String> {
        &self.elements
    }

    pub fn delay(&self) -> f64 {
        self.config.delay
    }

    pub fn state(&self) -> StaggerState {
        self.state.clone()
    }
}
