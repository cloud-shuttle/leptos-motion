//! Integration tests for Motion Studio

use leptos_motion_studio::*;
use leptos_motion_studio::{
    export::*, morphing::*, pooling::*, preview::*, project::*, studio::*, timeline::*,
    transforms::*, webgl::*,
};
use std::collections::HashMap;
use uuid::Uuid;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test Motion Studio end-to-end workflow
#[wasm_bindgen_test]
fn test_motion_studio_workflow() {
    // Create a new project
    let mut project = StudioProject::new("Integration Test Project");

    // Add an animation
    let animation_id = project.add_animation("Test Animation");

    // Create a timeline
    let mut timeline = Timeline3D::new("Test Timeline".to_string(), 5.0);

    // Add keyframes
    let keyframe_id = timeline
        .add_keyframe(
            AnimationProperty::TranslateX,
            0.0,
            AnimationValue::Number(0.0),
        )
        .unwrap();

    timeline
        .add_keyframe(
            AnimationProperty::TranslateX,
            2.5,
            AnimationValue::Number(100.0),
        )
        .unwrap();

    timeline
        .add_keyframe(
            AnimationProperty::TranslateX,
            5.0,
            AnimationValue::Number(0.0),
        )
        .unwrap();

    // Update project animation with timeline
    if let Some(animation) = project.get_animation_mut(animation_id) {
        animation.timeline = Some(timeline);
    }

    // Export to different formats
    let exporter = AnimationExporter::new(&project);

    // Test CSS export
    let css_result = exporter.export_css();
    assert!(css_result.is_ok());
    let css = css_result.unwrap();
    assert!(!css.content.is_empty());
    assert!(css.content.contains("@keyframes"));

    // Test WAAPI export
    let waapi_result = exporter.export_waapi();
    assert!(waapi_result.is_ok());
    let waapi = waapi_result.unwrap();
    assert!(!waapi.content.is_empty());
    assert!(waapi.content.contains("MotionAnimations"));

    // Test project serialization
    let mut manager = ProjectManager::new();
    let project_id = manager.load_project(project);
    let json = manager.save_project_json(project_id);
    assert!(json.is_ok());

    // Test project deserialization
    let loaded_id = manager.load_project_json(&json.unwrap());
    assert!(loaded_id.is_ok());

    let loaded_project = manager.get_project(loaded_id.unwrap());
    assert!(loaded_project.is_some());
    assert_eq!(loaded_project.unwrap().name, "Integration Test Project");
}

/// Test 3D transforms integration
#[wasm_bindgen_test]
fn test_3d_transforms_integration() {
    let mut timeline = Timeline3D::new("3D Test".to_string(), 3.0);

    // Add 3D transform keyframes
    timeline
        .add_keyframe(
            AnimationProperty::TranslateX,
            0.0,
            AnimationValue::Number(0.0),
        )
        .unwrap();

    timeline
        .add_keyframe(
            AnimationProperty::TranslateY,
            0.0,
            AnimationValue::Number(0.0),
        )
        .unwrap();

    timeline
        .add_keyframe(
            AnimationProperty::TranslateZ,
            0.0,
            AnimationValue::Number(0.0),
        )
        .unwrap();

    timeline
        .add_keyframe(AnimationProperty::RotateX, 0.0, AnimationValue::Number(0.0))
        .unwrap();

    // Add end keyframes
    timeline
        .add_keyframe(
            AnimationProperty::TranslateX,
            3.0,
            AnimationValue::Number(100.0),
        )
        .unwrap();

    timeline
        .add_keyframe(
            AnimationProperty::RotateX,
            3.0,
            AnimationValue::Number(std::f32::consts::PI / 2.0),
        )
        .unwrap();

    // Test timeline evaluation
    timeline.seek(1.5); // Midpoint
    let state = timeline.current_state().unwrap();

    // Should have interpolated values
    if let Some(AnimationValue::Number(x)) = state.get(&AnimationProperty::TranslateX) {
        assert!((*x - 50.0).abs() < 1.0); // Should be around 50 at midpoint
    }

    if let Some(AnimationValue::Number(rot)) = state.get(&AnimationProperty::RotateX) {
        assert!((*rot - std::f32::consts::PI / 4.0).abs() < 0.1); // Should be around PI/4 at midpoint
    }

    // Test Transform3D creation from values
    let transform = Transform3D::new();
    assert_eq!(transform.translation(), glam::Vec3::ZERO);
    assert_eq!(transform.rotation(), glam::Quat::IDENTITY);
    assert_eq!(transform.scale(), glam::Vec3::ONE);

    // Test CSS generation
    let css = transform.to_css();
    assert!(css.contains("translate3d"));
    assert!(css.contains("rotateX"));
    assert!(css.contains("scale3d"));
}

/// Test SVG morphing integration
#[wasm_bindgen_test]
fn test_svg_morphing_integration() {
    let path1 = "M 0 0 L 100 0 L 100 100 L 0 100 Z";
    let path2 = "M 50 50 L 150 50 L 100 150 L 0 150 Z";

    // Test path parsing
    let svg_path1 = SvgPath::from_data(path1);
    assert!(svg_path1.is_ok());
    let parsed1 = svg_path1.unwrap();
    assert_eq!(parsed1.commands.len(), 5); // M, L, L, L, Z

    // Test morphing
    let mut morpher = PathMorpher::new(path1, path2).unwrap();
    morpher.prepare().unwrap();

    // Test interpolation at different points
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        let interpolated = morpher.interpolate(t);
        assert!(interpolated.is_ok(), "Interpolation failed at t={}", t);

        let result = interpolated.unwrap();
        assert!(!result.to_data().is_empty());

        if t == 0.0 {
            // Should be close to original path
            assert!(result.to_data().contains("0"));
        } else if t == 1.0 {
            // Should be close to target path
            assert!(result.to_data().contains("50") || result.to_data().contains("150"));
        }
    }

    // Test SVG morphing animation
    let mut morphing = SvgMorphing::new(path1.to_string(), path2.to_string(), 2.0);
    morphing.initialize().unwrap();

    morphing.play();
    assert!(morphing.is_playing);

    let path_result = morphing.update(1.0); // Update by 1 second
    assert!(path_result.is_ok());
    assert_eq!(morphing.progress, 0.5); // Should be halfway through
}

/// Test animation pooling integration
#[wasm_bindgen_test]
fn test_animation_pooling_integration() {
    let pool = AnimationPool::new(20);

    // Pre-warm the pool
    let mut prealloc = HashMap::new();
    prealloc.insert(AnimationType::Transform, 5);
    prealloc.insert(AnimationType::Opacity, 3);
    pool.prewarm(&prealloc).unwrap();

    let stats = pool.memory_stats().unwrap();
    assert_eq!(stats.available_count, 8);
    assert_eq!(stats.total_allocated, 8);

    // Allocate animations
    let mut animations = Vec::new();

    for i in 0..10 {
        let animation =
            pool.allocate_with_type(AnimationType::Transform, std::time::Duration::from_secs(1));
        assert!(animation.is_ok());
        animations.push(animation.unwrap());
    }

    let stats_after_alloc = pool.memory_stats().unwrap();
    assert_eq!(stats_after_alloc.active_count, 10);

    // Test animation lifecycle
    let mut anim = animations.pop().unwrap();
    anim.start().unwrap();
    assert!(anim.is_active());

    anim.set_property("opacity".to_string(), AnimationValue::Number(0.8));
    assert!(anim.get_property("opacity").is_some());

    anim.update(0.5).unwrap();

    // Return animation to pool
    pool.deallocate(anim).unwrap();

    let final_stats = pool.memory_stats().unwrap();
    assert_eq!(final_stats.active_count, 9);
    assert!(final_stats.available_count > 0);

    // Test performance metrics
    let metrics = pool.performance_metrics().unwrap();
    assert!(metrics.cache_hits > 0 || metrics.cache_misses > 0);
}

/// Test WebGL integration (if available)
#[wasm_bindgen_test]
fn test_webgl_integration() {
    // Check if WebGL is available
    if !WebGLContext::is_available() {
        return; // Skip test if WebGL not available
    }

    // Test capability detection
    let capabilities = WebGLAcceleration::detect();
    if capabilities.is_err() {
        return; // Skip if detection fails
    }

    let caps = capabilities.unwrap();
    assert!(caps.webgl2_supported);
    assert!(caps.max_texture_size > 0);
    assert!(caps.max_vertex_attribs > 0);

    // Test GPU animation creation
    let mut gpu_animation = GPUAnimation::new();
    assert!(!gpu_animation.is_playing);
    assert_eq!(gpu_animation.current_frame, 0);

    // Test transform updates
    let transforms = vec![
        Transform3D::from_translation(glam::Vec3::new(0.0, 0.0, 0.0)),
        Transform3D::from_translation(glam::Vec3::new(10.0, 10.0, 0.0)),
        Transform3D::from_translation(glam::Vec3::new(20.0, 0.0, 10.0)),
    ];

    // Can't test WebGL context creation without actual canvas in test environment
    // but we can test the data structures and logic
    assert_eq!(gpu_animation.instance_matrices.len(), 0);

    gpu_animation.play();
    assert!(gpu_animation.is_playing);

    gpu_animation.update(1.0 / 60.0);
    assert_eq!(gpu_animation.current_frame, 1);
}

/// Test export formats integration
#[wasm_bindgen_test]
fn test_export_integration() {
    let mut project = StudioProject::new("Export Test");
    let animation_id = project.add_animation("Export Animation");

    // Add some transforms
    if let Some(animation) = project.get_animation_mut(animation_id) {
        animation.transforms = vec![
            Transform3D::from_translation(glam::Vec3::new(0.0, 0.0, 0.0)),
            Transform3D::from_translation(glam::Vec3::new(100.0, 50.0, 0.0)),
        ];
        animation.duration = 2.0;
    }

    let exporter = AnimationExporter::new(&project);
    let formats = exporter.supported_formats();

    // Test that all expected formats are supported
    assert!(formats.contains(&ExportFormat::CSS));
    assert!(formats.contains(&ExportFormat::WAAPI));
    assert!(formats.contains(&ExportFormat::LeptosMotion));
    assert!(formats.contains(&ExportFormat::FramerMotion));
    assert!(formats.contains(&ExportFormat::SVGAnimate));

    // Test CSS export with actual animation
    let css_config = ExportConfig {
        format: ExportFormat::CSS,
        settings: ExportSettings::CSS(CSSSettings::default()),
        optimization: OptimizationLevel::Basic,
        include_comments: true,
        minify: false,
    };

    let css_exporter = AnimationExporter::with_config(&project, css_config);
    let css_result = css_exporter.export();
    assert!(css_result.is_ok());

    let css = css_result.unwrap();
    assert!(css.content.contains("transform"));
    assert!(css.content.contains("translate3d"));
    assert_eq!(css.mime_type, "text/css");
    assert_eq!(css.file_extension, "css");

    // Test Leptos Motion export
    let leptos_result = exporter.export_leptos_motion();
    assert!(leptos_result.is_ok());

    let leptos = leptos_result.unwrap();
    assert!(leptos.content.contains("use leptos_motion::*"));
    assert!(leptos.content.contains("MotionDiv"));
    assert_eq!(leptos.mime_type, "text/x-rust");
}

/// Test live preview integration
#[wasm_bindgen_test]
fn test_preview_integration() {
    let mut project = StudioProject::new("Preview Test");
    let animation_id = project.add_animation("Preview Animation");

    // Create timeline with keyframes
    let mut timeline = Timeline3D::new("Preview Timeline".to_string(), 3.0);
    timeline
        .add_keyframe(AnimationProperty::Opacity, 0.0, AnimationValue::Number(0.0))
        .unwrap();

    timeline
        .add_keyframe(AnimationProperty::Opacity, 1.5, AnimationValue::Number(1.0))
        .unwrap();

    timeline
        .add_keyframe(AnimationProperty::Opacity, 3.0, AnimationValue::Number(0.0))
        .unwrap();

    if let Some(animation) = project.get_animation_mut(animation_id) {
        animation.timeline = Some(timeline);
    }

    // Test live preview creation
    let mut preview = LivePreview::new();
    assert!(preview.active_animations.is_empty());

    // Set project
    preview.set_project(project);
    assert!(!preview.active_animations.is_empty());

    // Test preview animation
    let project_animation = preview
        .project
        .as_ref()
        .unwrap()
        .animations
        .values()
        .next()
        .unwrap();
    let mut preview_animation = PreviewAnimation::from_project_animation(project_animation);

    assert_eq!(preview_animation.state, PreviewAnimationState::Ready);
    assert_eq!(preview_animation.duration, 3.0);

    preview_animation.play();
    assert_eq!(preview_animation.state, PreviewAnimationState::Playing);

    // Simulate time progression
    preview_animation.progress = 0.5;
    preview_animation.update(1.0 / 60.0).unwrap();

    // Test metrics
    let mut metrics = PreviewMetrics::default();
    metrics.update(1.0 / 60.0);
    assert_eq!(metrics.frame_count, 1);
    assert!(metrics.fps > 0.0);

    // Test settings
    let settings = PreviewSettings::default();
    assert!(settings.webgl_enabled);
    assert!(settings.gpu_acceleration);
    assert_eq!(settings.target_fps, 60.0);
}

/// Test complete Motion Studio API integration
#[wasm_bindgen_test]
fn test_complete_studio_api() {
    // This test would normally require DOM elements and can't run in basic unit test
    // Instead we test the data structures and logic

    let config = StudioConfig::default();
    assert!(config.webgl_enabled);
    assert!(config.transforms_3d);
    assert!(config.svg_morphing);
    assert_eq!(config.animation_pool_size, 50);

    let timeline_state = TimelineState::default();
    assert_eq!(timeline_state.current_time, 0.0);
    assert_eq!(timeline_state.zoom, 1.0);
    assert_eq!(timeline_state.duration, 10.0);

    let canvas_state = CanvasState::default();
    assert_eq!(
        canvas_state.camera_position,
        glam::Vec3::new(0.0, 0.0, 10.0)
    );
    assert_eq!(canvas_state.camera_zoom, 1.0);
    assert!(canvas_state.show_grid);

    // Test studio themes
    assert_eq!(StudioTheme::Dark.css_class(), "dark");
    assert_eq!(StudioTheme::Light.css_class(), "light");
    assert_eq!(StudioTheme::Auto.css_class(), "auto");
}

/// Test memory management and performance
#[wasm_bindgen_test]
fn test_memory_performance_integration() {
    // Test memory manager
    let mut memory_manager = MemoryManager::new(500);

    let pool1 = AnimationPool::new(100);
    let pool2 = AnimationPool::new(200);

    memory_manager.register_pool("main".to_string(), pool1);
    memory_manager.register_pool("effects".to_string(), pool2);

    assert!(memory_manager.get_pool("main").is_some());
    assert!(memory_manager.get_pool("effects").is_some());

    let pressure = memory_manager.check_memory_pressure();
    assert!(pressure >= 0.0);

    // Test handling memory pressure
    let result = memory_manager.handle_memory_pressure();
    assert!(result.is_ok());

    let global_stats = memory_manager.global_stats();
    assert_eq!(global_stats.active_count, 0); // No active animations yet

    // Allocate some animations to test memory usage
    if let Some(pool) = memory_manager.get_pool("main") {
        let _anim1 = pool.allocate();
        let _anim2 = pool.allocate();

        let stats = pool.memory_stats().unwrap();
        assert!(stats.active_count > 0);
    }
}

/// Performance benchmark test
#[wasm_bindgen_test]
fn test_performance_benchmarks() {
    const NUM_OPERATIONS: usize = 1000;

    // Benchmark Transform3D operations
    let start_time = js_sys::Date::now();

    for i in 0..NUM_OPERATIONS {
        let transform = Transform3D::from_translation(glam::Vec3::new(i as f32, i as f32, 0.0));
        let _matrix = transform.to_matrix();
        let _css = transform.to_css();
    }

    let transform_time = js_sys::Date::now() - start_time;

    // Benchmark timeline operations
    let timeline_start = js_sys::Date::now();
    let mut timeline = Timeline3D::new("Benchmark".to_string(), 10.0);

    for i in 0..NUM_OPERATIONS / 10 {
        let time = (i as f32 / (NUM_OPERATIONS / 10) as f32) * 10.0;
        timeline
            .add_keyframe(
                AnimationProperty::TranslateX,
                time,
                AnimationValue::Number(i as f32),
            )
            .unwrap();
    }

    let timeline_time = js_sys::Date::now() - timeline_start;

    // Benchmark animation pool
    let pool_start = js_sys::Date::now();
    let pool = AnimationPool::new(NUM_OPERATIONS);
    let mut animations = Vec::new();

    for _ in 0..NUM_OPERATIONS / 10 {
        if let Ok(anim) = pool.allocate() {
            animations.push(anim);
        }
    }

    for anim in animations {
        let _ = pool.deallocate(anim);
    }

    let pool_time = js_sys::Date::now() - pool_start;

    // Log performance metrics (in a real test, you'd assert these are within acceptable ranges)
    web_sys::console::log_1(
        &format!(
            "Transform3D operations ({} ops): {:.2}ms",
            NUM_OPERATIONS, transform_time
        )
        .into(),
    );
    web_sys::console::log_1(
        &format!(
            "Timeline operations ({} keyframes): {:.2}ms",
            NUM_OPERATIONS / 10,
            timeline_time
        )
        .into(),
    );
    web_sys::console::log_1(
        &format!(
            "Animation pool operations ({} ops): {:.2}ms",
            NUM_OPERATIONS / 10,
            pool_time
        )
        .into(),
    );

    // Basic performance assertions (these thresholds would be tuned based on requirements)
    assert!(
        transform_time < 1000.0,
        "Transform3D operations too slow: {:.2}ms",
        transform_time
    );
    assert!(
        timeline_time < 500.0,
        "Timeline operations too slow: {:.2}ms",
        timeline_time
    );
    assert!(
        pool_time < 200.0,
        "Animation pool operations too slow: {:.2}ms",
        pool_time
    );
}
