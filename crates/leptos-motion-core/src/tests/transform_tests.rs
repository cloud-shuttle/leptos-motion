//! Unit tests for Transform type

use crate::types::*;

#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_transform_default() {
        let transform = Transform::default();
        assert_eq!(transform.x, None);
        assert_eq!(transform.y, None);
        assert_eq!(transform.z, None);
        assert_eq!(transform.scale_x, None);
        assert_eq!(transform.scale_y, None);
        assert_eq!(transform.scale_z, None);
        assert_eq!(transform.rotate_x, None);
        assert_eq!(transform.rotate_y, None);
        assert_eq!(transform.rotate_z, None);
        assert_eq!(transform.skew_x, None);
        assert_eq!(transform.skew_y, None);
    }

    #[test]
    fn test_transform_new() {
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

        assert_eq!(transform.x, Some(10.0));
        assert_eq!(transform.y, Some(20.0));
        assert_eq!(transform.z, Some(30.0));
        assert_eq!(transform.scale_x, Some(1.5));
        assert_eq!(transform.scale_y, Some(2.0));
        assert_eq!(transform.scale_z, Some(1.0));
        assert_eq!(transform.rotate_x, Some(45.0));
        assert_eq!(transform.rotate_y, Some(90.0));
        assert_eq!(transform.rotate_z, Some(180.0));
        assert_eq!(transform.skew_x, Some(10.0));
        assert_eq!(transform.skew_y, Some(20.0));
    }

    #[test]
    fn test_transform_equality() {
        let transform1 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform2 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform3 = Transform {
            x: Some(11.0),
            y: Some(20.0),
            ..Default::default()
        };

        assert_eq!(transform1, transform2);
        assert_ne!(transform1, transform3);
    }

    #[test]
    fn test_transform_clone() {
        let transform1 = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let transform2 = transform1.clone();
        assert_eq!(transform1, transform2);
    }

    #[test]
    fn test_transform_debug() {
        let transform = Transform {
            x: Some(10.0),
            y: Some(20.0),
            ..Default::default()
        };
        let debug_str = format!("{:?}", transform);
        assert!(debug_str.contains("Transform"));
    }
}
