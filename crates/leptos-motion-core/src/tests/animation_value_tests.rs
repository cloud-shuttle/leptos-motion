//! Unit tests for AnimationValue type

use crate::types::*;

#[cfg(test)]
mod animation_value_tests {
    use super::*;

    #[test]
    fn test_animation_value_number() {
        let value = AnimationValue::Number(42.5);
        assert_eq!(value.to_string_value(), "42.5");
    }

    #[test]
    fn test_animation_value_pixels() {
        let value = AnimationValue::Pixels(100.0);
        assert_eq!(value.to_string_value(), "100px");
    }

    #[test]
    fn test_animation_value_percentage() {
        let value = AnimationValue::Percentage(50.0);
        assert_eq!(value.to_string_value(), "50%");
    }

    #[test]
    fn test_animation_value_degrees() {
        let value = AnimationValue::Degrees(90.0);
        assert_eq!(value.to_string_value(), "90deg");
    }

    #[test]
    fn test_animation_value_radians() {
        let value = AnimationValue::Radians(3.14159);
        assert_eq!(value.to_string_value(), "3.14159rad");
    }

    #[test]
    fn test_animation_value_color() {
        let value = AnimationValue::Color("#ff0000".to_string());
        assert_eq!(value.to_string_value(), "#ff0000");
    }

    #[test]
    fn test_animation_value_string() {
        let value = AnimationValue::String("auto".to_string());
        assert_eq!(value.to_string_value(), "auto");
    }

    #[test]
    fn test_animation_value_transform() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            z: Some(30.0),
            scale_x: Some(1.5),
            scale_y: Some(2.0),
            scale_z: Some(1.0),
            rotate_x: Some(45.0),
            rotate_y: Some(90.0),
            rotate_z: Some(180.0),
            skew_x: Some(10.0),
            skew_y: Some(20.0),
        };
        let value = AnimationValue::Transform(transform);
        let result = value.to_string_value();

        assert!(result.contains("translateX(10px)"));
        assert!(result.contains("translateY(20px)"));
        assert!(result.contains("translateZ(30px)"));
        assert!(result.contains("scaleX(1.5)"));
        assert!(result.contains("scaleY(2)"));
        assert!(result.contains("scaleZ(1)"));
        assert!(result.contains("rotateX(45deg)"));
        assert!(result.contains("rotateY(90deg)"));
        assert!(result.contains("rotateZ(180deg)"));
        assert!(result.contains("skewX(10deg)"));
        assert!(result.contains("skewY(20deg)"));
    }

    #[test]
    fn test_animation_value_complex() {
        let complex = ComplexValue {
            values: vec![
                AnimationValue::Number(1.0),
                AnimationValue::Number(2.0),
                AnimationValue::Number(3.0),
            ],
            interpolation: None,
        };
        let value = AnimationValue::Complex(complex);
        let result = value.to_string_value();
        assert_eq!(result, "1,2,3");
    }

    #[test]
    fn test_animation_value_equality() {
        let value1 = AnimationValue::Number(42.0);
        let value2 = AnimationValue::Number(42.0);
        let value3 = AnimationValue::Number(43.0);

        assert_eq!(value1, value2);
        assert_ne!(value1, value3);
    }

    #[test]
    fn test_animation_value_clone() {
        let value1 = AnimationValue::Number(42.0);
        let value2 = value1.clone();
        assert_eq!(value1, value2);
    }

    #[test]
    fn test_animation_value_debug() {
        let value = AnimationValue::Number(42.0);
        let debug_str = format!("{:?}", value);
        assert!(debug_str.contains("Number"));
        assert!(debug_str.contains("42"));
    }
}
