//! Simple unit tests for gesture system that can run in native environment
//!
//! These tests focus on basic functionality that doesn't require WASM/browser environment

use crate::*;

#[cfg(test)]
mod gesture_config_tests {
    use super::*;

    #[test]
    fn test_gesture_config_default() {
        let config = GestureConfig::default();
        assert!(config.basic_gestures);
        assert!(config.multi_touch);
        assert!(config.pinch_to_zoom);
        assert!(config.rotation);
        assert_eq!(config.sensitivity, 0.5);
        assert_eq!(config.min_distance, 10.0);
        assert_eq!(config.max_touches, 5);
        assert_eq!(config.timeout_ms, 300);
    }

    #[test]
    fn test_gesture_config_creation() {
        let config = GestureConfig {
            basic_gestures: false,
            multi_touch: true,
            pinch_to_zoom: false,
            rotation: true,
            sensitivity: 0.8,
            min_distance: 5.0,
            max_touches: 3,
            timeout_ms: 500,
        };
        
        assert!(!config.basic_gestures);
        assert!(config.multi_touch);
        assert!(!config.pinch_to_zoom);
        assert!(config.rotation);
        assert_eq!(config.sensitivity, 0.8);
        assert_eq!(config.min_distance, 5.0);
        assert_eq!(config.max_touches, 3);
        assert_eq!(config.timeout_ms, 500);
    }

    #[test]
    fn test_gesture_config_clone() {
        let config1 = GestureConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.basic_gestures, config2.basic_gestures);
        assert_eq!(config1.multi_touch, config2.multi_touch);
        assert_eq!(config1.sensitivity, config2.sensitivity);
    }

    #[test]
    fn test_gesture_config_debug() {
        let config = GestureConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("GestureConfig"));
        assert!(debug_str.contains("basic_gestures"));
        assert!(debug_str.contains("multi_touch"));
    }

    #[test]
    fn test_gesture_config_fluent_api() {
        let config = GestureConfig::default()
            .basic_only()
            .enable_multi_touch()
            .sensitivity(0.8)
            .min_distance(5.0)
            .max_touches(3)
            .timeout(500);

        // basic_only() keeps basic_gestures as true (default) but disables multi-touch initially
        // then enable_multi_touch() re-enables it
        assert!(config.basic_gestures);
        assert!(config.multi_touch);
        assert_eq!(config.sensitivity, 0.8);
        assert_eq!(config.min_distance, 5.0);
        assert_eq!(config.max_touches, 3);
        assert_eq!(config.timeout_ms, 500);
    }
}

#[cfg(test)]
mod touch_point_tests {
    use super::*;

    #[test]
    fn test_touch_point_creation() {
        let touch = TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 0.5,
            timestamp: 1234567890,
        };

        assert_eq!(touch.id, 1);
        assert_eq!(touch.x, 100.0);
        assert_eq!(touch.y, 200.0);
        assert_eq!(touch.pressure, 0.5);
        assert_eq!(touch.timestamp, 1234567890);
    }

    #[test]
    fn test_touch_point_clone() {
        let touch1 = TouchPoint {
            id: 2,
            x: 50.0,
            y: 75.0,
            pressure: 0.8,
            timestamp: 9876543210,
        };
        let touch2 = touch1.clone();
        assert_eq!(touch1.id, touch2.id);
        assert_eq!(touch1.x, touch2.x);
        assert_eq!(touch1.y, touch2.y);
        assert_eq!(touch1.pressure, touch2.pressure);
        assert_eq!(touch1.timestamp, touch2.timestamp);
    }

    #[test]
    fn test_touch_point_debug() {
        let touch = TouchPoint {
            id: 0,
            x: 0.0,
            y: 0.0,
            pressure: 0.0,
            timestamp: 0,
        };
        let debug_str = format!("{:?}", touch);
        assert!(debug_str.contains("TouchPoint"));
        assert!(debug_str.contains("id"));
        assert!(debug_str.contains("x"));
        assert!(debug_str.contains("y"));
    }
}

#[cfg(test)]
mod multi_touch_state_tests {
    use super::*;

    #[test]
    fn test_multi_touch_state_default() {
        let state = MultiTouchState::default();
        assert!(state.touches.is_empty());
        assert_eq!(state.center, (0.0, 0.0));
        assert_eq!(state.average_distance, 0.0);
        assert_eq!(state.scale, 1.0);
        assert_eq!(state.rotation, 0.0);
        assert!(!state.active);
        assert_eq!(state.gesture_type, MultiTouchGestureType::None);
    }

    #[test]
    fn test_multi_touch_state_creation() {
        let mut touches = HashMap::new();
        touches.insert(1, TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 0.5,
            timestamp: 1234567890,
        });

        let state = MultiTouchState {
            touches,
            center: (100.0, 200.0),
            average_distance: 50.0,
            scale: 1.5,
            rotation: 0.5,
            active: true,
            gesture_type: MultiTouchGestureType::Pinch,
        };

        assert_eq!(state.touches.len(), 1);
        assert_eq!(state.center, (100.0, 200.0));
        assert_eq!(state.average_distance, 50.0);
        assert_eq!(state.scale, 1.5);
        assert_eq!(state.rotation, 0.5);
        assert!(state.active);
        assert_eq!(state.gesture_type, MultiTouchGestureType::Pinch);
    }

    #[test]
    fn test_multi_touch_state_clone() {
        let state1 = MultiTouchState::default();
        let state2 = state1.clone();
        assert_eq!(state1.touches.len(), state2.touches.len());
        assert_eq!(state1.center, state2.center);
        assert_eq!(state1.scale, state2.scale);
        assert_eq!(state1.rotation, state2.rotation);
        assert_eq!(state1.active, state2.active);
        assert_eq!(state1.gesture_type, state2.gesture_type);
    }

    #[test]
    fn test_multi_touch_state_debug() {
        let state = MultiTouchState::default();
        let debug_str = format!("{:?}", state);
        assert!(debug_str.contains("MultiTouchState"));
        assert!(debug_str.contains("touches"));
        assert!(debug_str.contains("center"));
    }
}

#[cfg(test)]
mod multi_touch_gesture_type_tests {
    use super::*;

    #[test]
    fn test_multi_touch_gesture_type_variants() {
        let none = MultiTouchGestureType::None;
        let pinch = MultiTouchGestureType::Pinch;
        let rotation = MultiTouchGestureType::Rotation;
        let pinch_and_rotate = MultiTouchGestureType::PinchAndRotate;
        let multi_tap = MultiTouchGestureType::MultiTap;
        let multi_swipe = MultiTouchGestureType::MultiSwipe;

        // Test that all variants can be created
        assert!(matches!(none, MultiTouchGestureType::None));
        assert!(matches!(pinch, MultiTouchGestureType::Pinch));
        assert!(matches!(rotation, MultiTouchGestureType::Rotation));
        assert!(matches!(pinch_and_rotate, MultiTouchGestureType::PinchAndRotate));
        assert!(matches!(multi_tap, MultiTouchGestureType::MultiTap));
        assert!(matches!(multi_swipe, MultiTouchGestureType::MultiSwipe));
    }

    #[test]
    fn test_multi_touch_gesture_type_clone() {
        let gesture1 = MultiTouchGestureType::Pinch;
        let gesture2 = gesture1.clone();
        assert_eq!(gesture1, gesture2);
    }

    #[test]
    fn test_multi_touch_gesture_type_debug() {
        let gesture = MultiTouchGestureType::Rotation;
        let debug_str = format!("{:?}", gesture);
        assert!(debug_str.contains("Pinch") || debug_str.contains("Rotation") || debug_str.contains("None"));
    }

    #[test]
    fn test_multi_touch_gesture_type_partial_eq() {
        let gesture1 = MultiTouchGestureType::Pinch;
        let gesture2 = MultiTouchGestureType::Pinch;
        let gesture3 = MultiTouchGestureType::Rotation;

        assert_eq!(gesture1, gesture2);
        assert_ne!(gesture1, gesture3);
    }
}

#[cfg(test)]
mod gesture_event_tests {
    use super::*;

    #[test]
    fn test_gesture_event_touch_start() {
        let touches = vec![TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 0.5,
            timestamp: 1234567890,
        }];

        let event = GestureEvent::TouchStart { touches: touches.clone() };
        
        match event {
            GestureEvent::TouchStart { touches: event_touches } => {
                assert_eq!(event_touches.len(), 1);
                assert_eq!(event_touches[0].id, 1);
                assert_eq!(event_touches[0].x, 100.0);
                assert_eq!(event_touches[0].y, 200.0);
            }
            _ => panic!("Expected TouchStart event"),
        }
    }

    #[test]
    fn test_gesture_event_touch_move() {
        let touches = vec![TouchPoint {
            id: 1,
            x: 150.0,
            y: 250.0,
            pressure: 0.7,
            timestamp: 1234567891,
        }];

        let event = GestureEvent::TouchMove { touches: touches.clone() };
        
        match event {
            GestureEvent::TouchMove { touches: event_touches } => {
                assert_eq!(event_touches.len(), 1);
                assert_eq!(event_touches[0].x, 150.0);
                assert_eq!(event_touches[0].y, 250.0);
            }
            _ => panic!("Expected TouchMove event"),
        }
    }

    #[test]
    fn test_gesture_event_touch_end() {
        let touches = vec![TouchPoint {
            id: 1,
            x: 200.0,
            y: 300.0,
            pressure: 0.0,
            timestamp: 1234567892,
        }];

        let event = GestureEvent::TouchEnd { touches: touches.clone() };
        
        match event {
            GestureEvent::TouchEnd { touches: event_touches } => {
                assert_eq!(event_touches.len(), 1);
                assert_eq!(event_touches[0].x, 200.0);
                assert_eq!(event_touches[0].y, 300.0);
            }
            _ => panic!("Expected TouchEnd event"),
        }
    }

    #[test]
    fn test_gesture_event_gesture_recognized() {
        let gesture = MultiTouchState::default();
        let event = GestureEvent::GestureRecognized { gesture: gesture.clone() };
        
        match event {
            GestureEvent::GestureRecognized { gesture: event_gesture } => {
                assert_eq!(event_gesture.gesture_type, MultiTouchGestureType::None);
                assert!(!event_gesture.active);
            }
            _ => panic!("Expected GestureRecognized event"),
        }
    }

    #[test]
    fn test_gesture_event_clone() {
        let touches = vec![TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 0.5,
            timestamp: 1234567890,
        }];

        let event1 = GestureEvent::TouchStart { touches: touches.clone() };
        let event2 = event1.clone();
        
        match (event1, event2) {
            (GestureEvent::TouchStart { touches: touches1 }, GestureEvent::TouchStart { touches: touches2 }) => {
                assert_eq!(touches1.len(), touches2.len());
                assert_eq!(touches1[0].id, touches2[0].id);
            }
            _ => panic!("Expected TouchStart events"),
        }
    }

    #[test]
    fn test_gesture_event_debug() {
        let touches = vec![TouchPoint {
            id: 1,
            x: 100.0,
            y: 200.0,
            pressure: 0.5,
            timestamp: 1234567890,
        }];

        let event = GestureEvent::TouchStart { touches };
        let debug_str = format!("{:?}", event);
        assert!(debug_str.contains("TouchStart") || debug_str.contains("TouchMove") || debug_str.contains("TouchEnd"));
    }
}

#[cfg(test)]
mod gesture_result_tests {
    use super::*;

    #[test]
    fn test_gesture_result_default() {
        let result = GestureResult::default();
        assert!(!result.recognized);
        assert_eq!(result.gesture_type, MultiTouchGestureType::None);
        assert!(result.data.is_none());
        assert_eq!(result.confidence, 0.0);
    }

    #[test]
    fn test_gesture_result_creation() {
        let gesture_data = MultiTouchState {
            touches: HashMap::new(),
            center: (100.0, 200.0),
            average_distance: 50.0,
            scale: 1.5,
            rotation: 0.5,
            active: true,
            gesture_type: MultiTouchGestureType::Pinch,
        };

        let result = GestureResult {
            recognized: true,
            gesture_type: MultiTouchGestureType::Pinch,
            data: Some(gesture_data),
            confidence: 0.95,
        };

        assert!(result.recognized);
        assert_eq!(result.gesture_type, MultiTouchGestureType::Pinch);
        assert!(result.data.is_some());
        assert_eq!(result.confidence, 0.95);
    }

    #[test]
    fn test_gesture_result_clone() {
        let result1 = GestureResult {
            recognized: true,
            gesture_type: MultiTouchGestureType::Rotation,
            data: Some(MultiTouchState::default()),
            confidence: 0.8,
        };
        let result2 = result1.clone();
        assert_eq!(result1.recognized, result2.recognized);
        assert_eq!(result1.gesture_type, result2.gesture_type);
        assert_eq!(result1.confidence, result2.confidence);
    }

    #[test]
    fn test_gesture_result_debug() {
        let result = GestureResult::default();
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("GestureResult"));
        assert!(debug_str.contains("recognized"));
        assert!(debug_str.contains("confidence"));
    }
}