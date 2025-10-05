//! Unit tests for ComplexValue type

use crate::types::*;

#[cfg(test)]
mod complex_value_tests {
    use super::*;

    #[test]
    fn test_complex_value_new() {
        let values = vec![
            AnimationValue::Number(1.0),
            AnimationValue::Number(2.0),
            AnimationValue::Number(3.0),
        ];
        let complex = ComplexValue {
            values: values.clone(),
            interpolation: None,
        };

        assert_eq!(complex.values, values);
        assert_eq!(complex.interpolation, None);
    }

    #[test]
    fn test_complex_value_with_interpolation() {
        let values = vec![AnimationValue::Number(1.0)];
        let complex = ComplexValue {
            values,
            interpolation: Some("linear".to_string()),
        };

        assert_eq!(complex.interpolation, Some("linear".to_string()));
    }

    #[test]
    fn test_complex_value_equality() {
        let complex1 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex2 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex3 = ComplexValue {
            values: vec![AnimationValue::Number(2.0)],
            interpolation: None,
        };

        assert_eq!(complex1, complex2);
        assert_ne!(complex1, complex3);
    }

    #[test]
    fn test_complex_value_clone() {
        let complex1 = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let complex2 = complex1.clone();
        assert_eq!(complex1, complex2);
    }

    #[test]
    fn test_complex_value_debug() {
        let complex = ComplexValue {
            values: vec![AnimationValue::Number(1.0)],
            interpolation: None,
        };
        let debug_str = format!("{:?}", complex);
        assert!(debug_str.contains("ComplexValue"));
    }
}
