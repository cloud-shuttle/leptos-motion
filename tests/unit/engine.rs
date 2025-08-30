use leptos_motion_core::*;
use pretty_assertions::assert_eq;
use test_case::test_case;

#[test]
fn test_interpolation_linear() {
    let from = AnimationValue::Number(0.0);
    let to = AnimationValue::Number(100.0);
    
    let interpolated = from.interpolate(&to, 0.5);
    assert_eq!(interpolated, AnimationValue::Number(50.0));
}

#[test]
fn test_interpolation_pixels() {
    let from = AnimationValue::Pixels(0.0);
    let to = AnimationValue::Pixels(200.0);
    
    let interpolated = from.interpolate(&to, 0.25);
    assert_eq!(interpolated, AnimationValue::Pixels(50.0));
}

#[test]
fn test_interpolation_degrees() {
    let from = AnimationValue::Degrees(0.0);
    let to = AnimationValue::Degrees(180.0);
    
    let interpolated = from.interpolate(&to, 0.5);
    assert_eq!(interpolated, AnimationValue::Degrees(90.0));
}

#[test]
fn test_spring_physics() {
    let config = SpringConfig {
        stiffness: 100.0,
        damping: 10.0,
        mass: 1.0,
        ..Default::default()
    };
    
    let mut simulator = SpringSimulator::new(config);
    let mut state = SpringState {
        position: 0.0,
        velocity: 0.0,
    };
    
    // Simulate spring animation
    for _ in 0..100 {
        state = simulator.step(state, 100.0, 1.0 / 60.0);
    }
    
    // Should converge to target
    assert!((state.position - 100.0).abs() < 1.0);
    assert!(state.velocity.abs() < 1.0);
}

#[test_case(0.0 => 0.0)]
#[test_case(0.5 => 0.5)]
#[test_case(1.0 => 1.0)]
fn test_easing_linear(t: f64) -> f64 {
    Easing::Linear.evaluate(t)
}

#[test_case(0.0 => 0.0)]
#[test_case(0.5 => 0.25)]
#[test_case(1.0 => 1.0)]
fn test_easing_ease_in(t: f64) -> f64 {
    Easing::EaseIn.evaluate(t)
}

#[test_case(0.0 => 0.0)]
#[test_case(0.5 => 0.75)]
#[test_case(1.0 => 1.0)]
fn test_easing_ease_out(t: f64) -> f64 {
    Easing::EaseOut.evaluate(t)
}

#[test_case(0.0 => 0.0)]
#[test_case(0.5 => 0.5)]
#[test_case(1.0 => 1.0)]
fn test_easing_ease_in_out(t: f64) -> f64 {
    Easing::EaseInOut.evaluate(t)
}

#[test]
fn test_transition_default() {
    let transition = Transition::default();
    assert_eq!(transition.duration, None);
    assert_eq!(transition.delay, None);
    assert_eq!(transition.ease, Easing::Linear);
}

#[test]
fn test_transition_custom() {
    let transition = Transition {
        duration: Some(1.0),
        delay: Some(0.5),
        ease: Easing::Spring(SpringConfig::default()),
        ..Default::default()
    };
    
    assert_eq!(transition.duration, Some(1.0));
    assert_eq!(transition.delay, Some(0.5));
    assert!(matches!(transition.ease, Easing::Spring(_)));
}

#[test]
fn test_animation_target_creation() {
    let mut target = AnimationTarget::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target.insert("x".to_string(), AnimationValue::Pixels(100.0));
    
    assert_eq!(target.get("opacity"), Some(&AnimationValue::Number(1.0)));
    assert_eq!(target.get("x"), Some(&AnimationValue::Pixels(100.0)));
    assert_eq!(target.get("y"), None);
}

#[test]
fn test_motion_value_creation() {
    let motion_value = MotionValue::new(42.0);
    assert_eq!(motion_value.get(), 42.0);
}

#[test]
fn test_motion_value_set() {
    let motion_value = MotionValue::new(0.0);
    motion_value.set(100.0);
    assert_eq!(motion_value.get(), 100.0);
}

#[test]
fn test_motion_value_velocity() {
    let motion_value = MotionValue::new(0.0);
    assert_eq!(motion_value.get_velocity(), 0.0);
    
    // Simulate some velocity
    motion_value.set_velocity(50.0);
    assert_eq!(motion_value.get_velocity(), 50.0);
}

// Property-based tests using proptest
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_easing_bounded(t in 0.0..=1.0) {
            let result = Easing::EaseInOut.evaluate(t);
            assert!(result >= 0.0 && result <= 1.0);
        }
        
        #[test]
        fn test_interpolation_identity(from in -1000.0..1000.0) {
            let value = AnimationValue::Number(from);
            let interpolated = value.interpolate(&value, 0.5);
            assert_eq!(interpolated, value);
        }
        
        #[test]
        fn test_interpolation_endpoints(from in -1000.0..1000.0, to in -1000.0..1000.0) {
            let from_val = AnimationValue::Number(from);
            let to_val = AnimationValue::Number(to);
            
            let at_start = from_val.interpolate(&to_val, 0.0);
            let at_end = from_val.interpolate(&to_val, 1.0);
            
            assert_eq!(at_start, from_val);
            assert_eq!(at_end, to_val);
        }
    }
}
