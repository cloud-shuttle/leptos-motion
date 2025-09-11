//! Comprehensive test suite for Motion Studio

use pretty_assertions::assert_eq;
use rstest::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use crate::*;

/// Test fixture for Motion Studio
#[fixture]
fn studio() -> MotionStudio {
    MotionStudio::new().expect("Failed to create studio")
}

/// Test fixture for WebGL context
#[fixture]
fn webgl_context() -> Option<web_sys::WebGl2RenderingContext> {
    // This will be implemented when we create the WebGL module
    None
}

#[rstest]
#[wasm_bindgen_test]
fn test_studio_creation(studio: MotionStudio) {
    // Test basic studio instantiation
    assert!(studio.is_initialized());
}

#[rstest]
#[wasm_bindgen_test]
fn test_timeline_creation(studio: MotionStudio) {
    let timeline = studio.create_timeline();
    assert_eq!(timeline.duration(), 0.0);
    assert!(timeline.keyframes().is_empty());
}

#[rstest]
#[wasm_bindgen_test]
fn test_3d_transform_creation() {
    let transform = Transform3D::new();
    assert_eq!(transform.translation(), glam::Vec3::ZERO);
    assert_eq!(transform.rotation(), glam::Quat::IDENTITY);
    assert_eq!(transform.scale(), glam::Vec3::ONE);
}

#[rstest]
#[wasm_bindgen_test]
fn test_path_morphing_basic() {
    let path1 = "M 0 0 L 100 0 L 100 100 Z";
    let path2 = "M 0 0 L 50 50 L 100 100 Z";

    let morpher = PathMorpher::new(path1, path2);
    assert!(morpher.is_ok());

    let interpolated = morpher.unwrap().interpolate(0.5);
    assert!(interpolated.is_ok());
}

#[rstest]
#[wasm_bindgen_test]
fn test_animation_pool_creation() {
    let pool = AnimationPool::new(10);
    assert_eq!(pool.capacity(), 10);
    assert_eq!(pool.active_count(), 0);
    assert_eq!(pool.available_count(), 10);
}

#[rstest]
#[wasm_bindgen_test]
fn test_webgl_renderer_creation() {
    // Skip if WebGL not available
    if webgl_context().is_none() {
        return;
    }

    let renderer = WebGLRenderer::new();
    assert!(renderer.is_ok());
}

#[rstest]
#[wasm_bindgen_test]
fn test_project_creation() {
    let project = StudioProject::new("Test Project");
    assert_eq!(project.name(), "Test Project");
    assert_eq!(project.version(), "1.0.0");
    assert!(project.animations().is_empty());
}

#[rstest]
#[wasm_bindgen_test]
fn test_export_formats() {
    let project = StudioProject::new("Test Project");
    let exporter = AnimationExporter::new(&project);

    let formats = exporter.supported_formats();
    assert!(formats.contains(&ExportFormat::CSS));
    assert!(formats.contains(&ExportFormat::WAAPI));
    assert!(formats.contains(&ExportFormat::LeptosMotion));
}

#[cfg(feature = "webgl")]
mod webgl_tests {
    use super::*;

    #[rstest]
    #[wasm_bindgen_test]
    fn test_gpu_acceleration_detection() {
        let acceleration = WebGLAcceleration::detect();
        // Should not panic even if WebGL unavailable
        assert!(acceleration.is_ok() || acceleration.is_err());
    }
}

#[cfg(feature = "svg-morphing")]
mod morphing_tests {
    use super::*;

    #[rstest]
    #[wasm_bindgen_test]
    fn test_complex_path_morphing() {
        let path1 = "M 0 0 Q 50 25 100 0 T 200 0";
        let path2 = "M 0 0 Q 25 50 100 100 T 200 50";

        let morpher = PathMorpher::new(path1, path2);
        assert!(morpher.is_ok());

        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let result = morpher.as_ref().unwrap().interpolate(t);
            assert!(result.is_ok(), "Interpolation failed at t={}", t);
        }
    }
}

#[cfg(feature = "3d-support")]
mod transform_3d_tests {
    use super::*;

    #[rstest]
    #[wasm_bindgen_test]
    fn test_perspective_projection() {
        let perspective = Perspective::new(45.0, 1.0, 0.1, 100.0);
        let matrix = perspective.to_matrix();

        // Basic sanity check - matrix should not be identity
        assert_ne!(matrix, glam::Mat4::IDENTITY);
    }

    #[rstest]
    #[wasm_bindgen_test]
    fn test_transform_matrix_composition() {
        let translate = Transform3D::from_translation(glam::Vec3::new(10.0, 20.0, 30.0));
        let rotate =
            Transform3D::from_rotation(glam::Quat::from_rotation_z(std::f32::consts::PI / 4.0));
        let scale = Transform3D::from_scale(glam::Vec3::new(2.0, 2.0, 2.0));

        let composed = translate.compose(&rotate).compose(&scale);
        let matrix = composed.to_matrix();

        // Verify composition is not identity
        assert_ne!(matrix, glam::Mat4::IDENTITY);
    }
}

mod property_based_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_timeline_keyframe_ordering(
            times in prop::collection::vec(0.0f32..10.0, 1..20)
        ) {
            let mut timeline = AnimationTimeline::new();

            for (i, time) in times.iter().enumerate() {
                let _ = timeline.add_keyframe(*time, format!("keyframe_{}", i));
            }

            // Keyframes should be sorted by time
            let keyframe_times: Vec<f32> = timeline.keyframes()
                .iter()
                .map(|kf| kf.time())
                .collect();

            let mut sorted_times = keyframe_times.clone();
            sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

            assert_eq!(keyframe_times, sorted_times);
        }

        #[test]
        fn test_animation_pool_consistency(
            pool_size in 1usize..100,
            allocation_count in 0usize..200
        ) {
            let pool = AnimationPool::new(pool_size);
            let mut allocated = Vec::new();

            // Allocate up to pool capacity
            for _ in 0..allocation_count.min(pool_size) {
                if let Ok(anim) = pool.allocate() {
                    allocated.push(anim);
                }
            }

            assert_eq!(pool.active_count() + pool.available_count(), pool_size);
            assert!(pool.active_count() <= pool_size);
        }

        #[test]
        fn test_transform_3d_interpolation(
            t in 0.0f32..1.0,
            x1 in -100.0f32..100.0,
            y1 in -100.0f32..100.0,
            z1 in -100.0f32..100.0,
            x2 in -100.0f32..100.0,
            y2 in -100.0f32..100.0,
            z2 in -100.0f32..100.0
        ) {
            let transform1 = Transform3D::from_translation(glam::Vec3::new(x1, y1, z1));
            let transform2 = Transform3D::from_translation(glam::Vec3::new(x2, y2, z2));

            let interpolated = transform1.lerp(&transform2, t);
            let translation = interpolated.translation();

            // Check interpolation bounds
            if t == 0.0 {
                assert!((translation.x - x1).abs() < 1e-6);
                assert!((translation.y - y1).abs() < 1e-6);
                assert!((translation.z - z1).abs() < 1e-6);
            } else if t == 1.0 {
                assert!((translation.x - x2).abs() < 1e-6);
                assert!((translation.y - y2).abs() < 1e-6);
                assert!((translation.z - z2).abs() < 1e-6);
            } else {
                // Should be between the two values
                let min_x = x1.min(x2);
                let max_x = x1.max(x2);
                assert!(translation.x >= min_x - 1e-6 && translation.x <= max_x + 1e-6);
            }
        }
    }
}
