//! Tests to verify compatibility fixes work correctly

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_cubic_bezier_type_exists() {
        // Test that CubicBezier type exists and can be created
        let cubic_bezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
        assert_eq!(cubic_bezier.0, 0.4);
        assert_eq!(cubic_bezier.1, 0.0);
        assert_eq!(cubic_bezier.2, 0.2);
        assert_eq!(cubic_bezier.3, 1.0);
    }

    #[test]
    fn test_cubic_bezier_easing_variant() {
        // Test that CubicBezier variant exists in Easing enum
        let cubic_bezier = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
        let easing = Easing::CubicBezier(cubic_bezier);
        
        // Test evaluation
        let result = easing.basic_evaluate(0.5);
        assert!(result >= 0.0 && result <= 1.0);
    }

    #[test]
    fn test_bezier_variant_still_works() {
        // Test that the original Bezier variant still works
        let easing = Easing::Bezier(0.4, 0.0, 0.2, 1.0);
        let result = easing.basic_evaluate(0.5);
        assert!(result >= 0.0 && result <= 1.0);
    }

    #[test]
    fn test_from_cubic_bezier_for_easing() {
        // Test that From<CubicBezier> for Easing works
        let cb = CubicBezier::new(0.25, 0.1, 0.25, 1.0);
        let easing: Easing = cb.into();
        
        // Should be equivalent to Bezier variant (not CubicBezier)
        match easing {
            Easing::Bezier(x1, y1, x2, y2) => {
                assert_eq!(x1, 0.25);
                assert_eq!(y1, 0.1);
                assert_eq!(x2, 0.25);
                assert_eq!(y2, 1.0);
            }
            _ => panic!("Expected Bezier variant"),
        }
    }

    #[test]
    fn test_cubic_bezier_evaluation_consistency() {
        // Test that CubicBezier and Bezier variants produce the same results
        let cb = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
        let easing_cb = Easing::CubicBezier(cb);
        let easing_bezier = Easing::Bezier(0.4, 0.0, 0.2, 1.0);
        
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result_cb = easing_cb.basic_evaluate(t);
            let result_bezier = easing_bezier.basic_evaluate(t);
            assert!((result_cb - result_bezier).abs() < 1e-10, 
                "Results differ at t={}: {} vs {}", t, result_cb, result_bezier);
        }
    }

    #[test]
    fn test_cubic_bezier_edge_cases() {
        // Test edge cases for CubicBezier
        let cb = CubicBezier::new(0.0, 0.0, 1.0, 1.0); // Linear
        let easing = Easing::CubicBezier(cb);
        
        // At t=0, should be 0
        assert_eq!(easing.basic_evaluate(0.0), 0.0);
        
        // At t=1, should be 1
        assert_eq!(easing.basic_evaluate(1.0), 1.0);
        
        // At t=0.5, should be 0.5 for linear curve
        assert!((easing.basic_evaluate(0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_cubic_bezier_clamping() {
        // Test that evaluation works with edge case values
        let cb = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
        let easing = Easing::CubicBezier(cb);
        
        // Test that evaluation works with negative values (no clamping in basic_evaluate)
        let result_negative = easing.basic_evaluate(-0.5);
        // Should still produce a valid result (not panic)
        assert!(result_negative.is_finite());
        
        // Test that evaluation works with values > 1 (no clamping in basic_evaluate)
        let result_positive = easing.basic_evaluate(1.5);
        // Should still produce a valid result (not panic)
        assert!(result_positive.is_finite());
        
        // Test normal range values
        let result_normal = easing.basic_evaluate(0.5);
        assert!(result_normal >= 0.0 && result_normal <= 1.0);
    }

    #[test]
    fn test_cubic_bezier_copy_trait() {
        // Test that CubicBezier implements Copy
        let cb1 = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
        let cb2 = cb1; // This should work because of Copy trait
        
        assert_eq!(cb1.0, cb2.0);
        assert_eq!(cb1.1, cb2.1);
        assert_eq!(cb1.2, cb2.2);
        assert_eq!(cb1.3, cb2.3);
    }

    #[test]
    fn test_cubic_bezier_serde_support() {
        // Test that CubicBezier can be serialized/deserialized (if serde feature is enabled)
        #[cfg(feature = "serde-support")]
        {
            let cb = CubicBezier::new(0.4, 0.0, 0.2, 1.0);
            let json = serde_json::to_string(&cb).expect("Serialization should work");
            let deserialized: CubicBezier = serde_json::from_str(&json).expect("Deserialization should work");
            
            assert_eq!(cb, deserialized);
        }
    }
}
