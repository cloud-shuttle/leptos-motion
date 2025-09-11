//! Tests for post-processing system

use crate::post_processing::*;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};

wasm_bindgen_test_configure!(run_in_browser);

/// Test post-processing effect enum
#[wasm_bindgen_test]
fn test_post_processing_effect_enum() {
    // Test all effect types
    let effects = vec![
        PostProcessingEffect::Bloom,
        PostProcessingEffect::SSAO,
        PostProcessingEffect::ToneMapping,
        PostProcessingEffect::GaussianBlur,
        PostProcessingEffect::EdgeDetection,
        PostProcessingEffect::ColorGrading,
        PostProcessingEffect::Vignette,
        PostProcessingEffect::ChromaticAberration,
    ];

    for effect in effects {
        // Test that effects can be compared
        assert_eq!(effect, effect);
    }
}

/// Test post-processing configuration creation
#[wasm_bindgen_test]
fn test_post_processing_config_creation() {
    let config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    
    assert_eq!(config.effect, PostProcessingEffect::Bloom);
    assert_eq!(config.intensity, 0.5);
    assert!(config.enabled);
    assert!(config.parameters.is_empty());
}

/// Test post-processing configuration intensity clamping
#[wasm_bindgen_test]
fn test_post_processing_config_intensity_clamping() {
    let mut config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.5);
    assert_eq!(config.intensity, 1.0); // Should be clamped to 1.0
    
    config.set_intensity(-0.5);
    assert_eq!(config.intensity, 0.0); // Should be clamped to 0.0
    
    config.set_intensity(0.7);
    assert_eq!(config.intensity, 0.7); // Should remain unchanged
}

/// Test post-processing configuration parameters
#[wasm_bindgen_test]
fn test_post_processing_config_parameters() {
    let mut config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    
    // Test setting parameters
    config.set_parameter("threshold", 1.2);
    config.set_parameter("blur_radius", 2.5);
    
    // Test getting parameters
    assert_eq!(config.get_parameter("threshold"), Some(1.2));
    assert_eq!(config.get_parameter("blur_radius"), Some(2.5));
    assert_eq!(config.get_parameter("nonexistent"), None);
    
    // Test parameter count
    assert_eq!(config.parameters.len(), 2);
}

/// Test post-processing configuration enable/disable
#[wasm_bindgen_test]
fn test_post_processing_config_enable_disable() {
    let mut config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    
    assert!(config.enabled);
    
    config.disable();
    assert!(!config.enabled);
    
    config.enable();
    assert!(config.enabled);
}

/// Test bloom configuration default
#[wasm_bindgen_test]
fn test_bloom_config_default() {
    let config = BloomConfig::default();
    
    assert_eq!(config.threshold, 1.0);
    assert_eq!(config.intensity, 0.5);
    assert_eq!(config.blur_passes, 5);
    assert_eq!(config.blur_radius, 1.0);
}

/// Test SSAO configuration default
#[wasm_bindgen_test]
fn test_ssao_config_default() {
    let config = SSAOConfig::default();
    
    assert_eq!(config.sample_radius, 0.5);
    assert_eq!(config.sample_count, 16);
    assert_eq!(config.bias, 0.025);
    assert_eq!(config.intensity, 1.0);
}

/// Test tone mapping configuration default
#[wasm_bindgen_test]
fn test_tone_mapping_config_default() {
    let config = ToneMappingConfig::default();
    
    assert_eq!(config.exposure, 1.0);
    assert_eq!(config.white_point, 11.2);
    assert_eq!(config.operator, ToneMappingOperator::ACES);
}

/// Test tone mapping operators
#[wasm_bindgen_test]
fn test_tone_mapping_operators() {
    let operators = vec![
        ToneMappingOperator::Linear,
        ToneMappingOperator::Reinhard,
        ToneMappingOperator::ACES,
        ToneMappingOperator::Uncharted2,
    ];

    for operator in operators {
        assert_eq!(operator, operator);
    }
}

/// Test post-processing framebuffer creation
#[wasm_bindgen_test]
fn test_post_processing_framebuffer_creation() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let framebuffer = PostProcessingFramebuffer::new(&context, 512, 512);
    assert!(framebuffer.is_ok());
    
    let framebuffer = framebuffer.unwrap();
    assert_eq!(framebuffer.width, 512);
    assert_eq!(framebuffer.height, 512);
}

/// Test post-processing framebuffer binding
#[wasm_bindgen_test]
fn test_post_processing_framebuffer_binding() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let framebuffer = PostProcessingFramebuffer::new(&context, 256, 256).unwrap();
    
    // Test binding
    framebuffer.bind(&context);
    
    // Test unbinding
    framebuffer.unbind(&context);
}

/// Test post-processing framebuffer resize
#[wasm_bindgen_test]
fn test_post_processing_framebuffer_resize() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut framebuffer = PostProcessingFramebuffer::new(&context, 256, 256).unwrap();
    
    // Test resize
    let result = framebuffer.resize(&context, 512, 512);
    assert!(result.is_ok());
    
    assert_eq!(framebuffer.width, 512);
    assert_eq!(framebuffer.height, 512);
}

/// Test post-processing pipeline creation
#[wasm_bindgen_test]
fn test_post_processing_pipeline_creation() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let pipeline = PostProcessingPipeline::new(context, 512, 512);
    assert!(pipeline.is_ok());
    
    let pipeline = pipeline.unwrap();
    assert_eq!(pipeline.get_effect_count(), 0);
    assert!(pipeline.is_empty());
}

/// Test post-processing pipeline effects
#[wasm_bindgen_test]
fn test_post_processing_pipeline_effects() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Test adding effects
    let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    let ssao_config = PostProcessingConfig::new(PostProcessingEffect::SSAO, 0.8);
    
    pipeline.add_effect(bloom_config);
    pipeline.add_effect(ssao_config);
    
    assert_eq!(pipeline.get_effect_count(), 2);
    assert!(!pipeline.is_empty());
    
    // Test getting effects
    let effect = pipeline.get_effect(0);
    assert!(effect.is_some());
    assert_eq!(effect.unwrap().effect, PostProcessingEffect::Bloom);
    
    let effect = pipeline.get_effect(1);
    assert!(effect.is_some());
    assert_eq!(effect.unwrap().effect, PostProcessingEffect::SSAO);
    
    // Test getting non-existent effect
    let effect = pipeline.get_effect(2);
    assert!(effect.is_none());
}

/// Test post-processing pipeline effect modification
#[wasm_bindgen_test]
fn test_post_processing_pipeline_effect_modification() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    let mut bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    bloom_config.set_parameter("threshold", 1.2);
    pipeline.add_effect(bloom_config);
    
    // Test modifying effect
    if let Some(effect) = pipeline.get_effect_mut(0) {
        effect.set_intensity(0.8);
        effect.set_parameter("blur_radius", 2.0);
        effect.disable();
    }
    
    let effect = pipeline.get_effect(0).unwrap();
    assert_eq!(effect.intensity, 0.8);
    assert_eq!(effect.get_parameter("blur_radius"), Some(2.0));
    assert!(!effect.enabled);
}

/// Test post-processing pipeline effect removal
#[wasm_bindgen_test]
fn test_post_processing_pipeline_effect_removal() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Add multiple effects
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::SSAO, 0.8));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0));
    
    assert_eq!(pipeline.get_effect_count(), 3);
    
    // Remove middle effect
    let removed = pipeline.remove_effect(1);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().effect, PostProcessingEffect::SSAO);
    assert_eq!(pipeline.get_effect_count(), 2);
    
    // Remove non-existent effect
    let removed = pipeline.remove_effect(5);
    assert!(removed.is_none());
    assert_eq!(pipeline.get_effect_count(), 2);
}

/// Test post-processing pipeline clear effects
#[wasm_bindgen_test]
fn test_post_processing_pipeline_clear_effects() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Add effects
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::SSAO, 0.8));
    
    assert_eq!(pipeline.get_effect_count(), 2);
    
    // Clear all effects
    pipeline.clear_effects();
    assert_eq!(pipeline.get_effect_count(), 0);
    assert!(pipeline.is_empty());
}

/// Test post-processing pipeline resize
#[wasm_bindgen_test]
fn test_post_processing_pipeline_resize() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 256, 256).unwrap();
    
    // Test resize
    let result = pipeline.resize(512, 512);
    assert!(result.is_ok());
}

/// Test post-processing pipeline rendering workflow
#[wasm_bindgen_test]
fn test_post_processing_pipeline_rendering_workflow() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Add effects
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0));
    
    // Test rendering workflow
    pipeline.begin();
    
    // Simulate rendering to framebuffer
    // (In a real implementation, this would render the scene)
    
    let result = pipeline.end();
    assert!(result.is_ok());
}

/// Test post-processing pipeline with disabled effects
#[wasm_bindgen_test]
fn test_post_processing_pipeline_disabled_effects() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Add effects with one disabled
    let mut bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    bloom_config.disable();
    pipeline.add_effect(bloom_config);
    
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0));
    
    // Test rendering workflow
    pipeline.begin();
    let result = pipeline.end();
    assert!(result.is_ok());
}

/// Test post-processing pipeline empty effects
#[wasm_bindgen_test]
fn test_post_processing_pipeline_empty_effects() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Test rendering workflow with no effects
    pipeline.begin();
    let result = pipeline.end();
    assert!(result.is_ok());
}

/// Test post-processing configuration cloning
#[wasm_bindgen_test]
fn test_post_processing_config_cloning() {
    let mut config1 = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    config1.set_parameter("threshold", 1.2);
    config1.set_parameter("blur_radius", 2.0);
    config1.disable();
    
    let config2 = config1.clone();
    
    assert_eq!(config1.effect, config2.effect);
    assert_eq!(config1.intensity, config2.intensity);
    assert_eq!(config1.enabled, config2.enabled);
    assert_eq!(config1.parameters.len(), config2.parameters.len());
    assert_eq!(config1.get_parameter("threshold"), config2.get_parameter("threshold"));
    assert_eq!(config1.get_parameter("blur_radius"), config2.get_parameter("blur_radius"));
}

/// Test bloom configuration cloning
#[wasm_bindgen_test]
fn test_bloom_config_cloning() {
    let config1 = BloomConfig {
        threshold: 1.5,
        intensity: 0.8,
        blur_passes: 7,
        blur_radius: 2.5,
    };
    
    let config2 = config1.clone();
    
    assert_eq!(config1.threshold, config2.threshold);
    assert_eq!(config1.intensity, config2.intensity);
    assert_eq!(config1.blur_passes, config2.blur_passes);
    assert_eq!(config1.blur_radius, config2.blur_radius);
}

/// Test SSAO configuration cloning
#[wasm_bindgen_test]
fn test_ssao_config_cloning() {
    let config1 = SSAOConfig {
        sample_radius: 0.8,
        sample_count: 32,
        bias: 0.05,
        intensity: 1.5,
    };
    
    let config2 = config1.clone();
    
    assert_eq!(config1.sample_radius, config2.sample_radius);
    assert_eq!(config1.sample_count, config2.sample_count);
    assert_eq!(config1.bias, config2.bias);
    assert_eq!(config1.intensity, config2.intensity);
}

/// Test tone mapping configuration cloning
#[wasm_bindgen_test]
fn test_tone_mapping_config_cloning() {
    let config1 = ToneMappingConfig {
        exposure: 2.0,
        white_point: 15.0,
        operator: ToneMappingOperator::Reinhard,
    };
    
    let config2 = config1.clone();
    
    assert_eq!(config1.exposure, config2.exposure);
    assert_eq!(config1.white_point, config2.white_point);
    assert_eq!(config1.operator, config2.operator);
}

/// Test post-processing effect parameter validation
#[wasm_bindgen_test]
fn test_post_processing_effect_parameter_validation() {
    let mut config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5);
    
    // Test setting various parameter types
    config.set_parameter("threshold", 1.2);
    config.set_parameter("intensity", 0.8);
    config.set_parameter("blur_passes", 5.0);
    config.set_parameter("blur_radius", 2.5);
    
    // Test getting parameters
    assert_eq!(config.get_parameter("threshold"), Some(1.2));
    assert_eq!(config.get_parameter("intensity"), Some(0.8));
    assert_eq!(config.get_parameter("blur_passes"), Some(5.0));
    assert_eq!(config.get_parameter("blur_radius"), Some(2.5));
    
    // Test parameter count
    assert_eq!(config.parameters.len(), 4);
}

/// Test post-processing pipeline effect ordering
#[wasm_bindgen_test]
fn test_post_processing_pipeline_effect_ordering() {
    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut pipeline = PostProcessingPipeline::new(context, 512, 512).unwrap();
    
    // Add effects in specific order
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::Bloom, 0.5));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::GaussianBlur, 0.3));
    pipeline.add_effect(PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0));
    
    // Verify order is maintained
    let effects = pipeline.get_effects();
    assert_eq!(effects.len(), 3);
    assert_eq!(effects[0].effect, PostProcessingEffect::Bloom);
    assert_eq!(effects[1].effect, PostProcessingEffect::GaussianBlur);
    assert_eq!(effects[2].effect, PostProcessingEffect::ToneMapping);
}
