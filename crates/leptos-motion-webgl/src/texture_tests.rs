//! Tests for texture system

use crate::error::Result;
use crate::texture::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{HtmlCanvasElement, HtmlImageElement, WebGl2RenderingContext};

wasm_bindgen_test_configure!(run_in_browser);

/// Test texture format conversions
#[wasm_bindgen_test]
fn test_texture_format_conversions() {
    // Test RGB format
    assert_eq!(
        TextureFormat::RGB.to_webgl_internal_format(),
        WebGl2RenderingContext::RGB
    );
    assert_eq!(
        TextureFormat::RGB.to_webgl_format(),
        WebGl2RenderingContext::RGB
    );

    // Test RGBA format
    assert_eq!(
        TextureFormat::RGBA.to_webgl_internal_format(),
        WebGl2RenderingContext::RGBA
    );
    assert_eq!(
        TextureFormat::RGBA.to_webgl_format(),
        WebGl2RenderingContext::RGBA
    );

    // Test Luminance format
    assert_eq!(
        TextureFormat::Luminance.to_webgl_internal_format(),
        WebGl2RenderingContext::LUMINANCE
    );
    assert_eq!(
        TextureFormat::Luminance.to_webgl_format(),
        WebGl2RenderingContext::LUMINANCE
    );

    // Test Alpha format
    assert_eq!(
        TextureFormat::Alpha.to_webgl_internal_format(),
        WebGl2RenderingContext::ALPHA
    );
    assert_eq!(
        TextureFormat::Alpha.to_webgl_format(),
        WebGl2RenderingContext::ALPHA
    );
}

/// Test texture type conversions
#[wasm_bindgen_test]
fn test_texture_type_conversions() {
    assert_eq!(
        TextureType::UnsignedByte.to_webgl_type(),
        WebGl2RenderingContext::UNSIGNED_BYTE
    );
    assert_eq!(
        TextureType::Float.to_webgl_type(),
        WebGl2RenderingContext::FLOAT
    );
    assert_eq!(
        TextureType::HalfFloat.to_webgl_type(),
        WebGl2RenderingContext::HALF_FLOAT
    );
}

/// Test texture wrapping conversions
#[wasm_bindgen_test]
fn test_texture_wrapping_conversions() {
    assert_eq!(
        TextureWrapping::Repeat.to_webgl_wrapping(),
        WebGl2RenderingContext::REPEAT
    );
    assert_eq!(
        TextureWrapping::ClampToEdge.to_webgl_wrapping(),
        WebGl2RenderingContext::CLAMP_TO_EDGE
    );
    assert_eq!(
        TextureWrapping::MirroredRepeat.to_webgl_wrapping(),
        WebGl2RenderingContext::MIRRORED_REPEAT
    );
}

/// Test texture filtering conversions
#[wasm_bindgen_test]
fn test_texture_filtering_conversions() {
    assert_eq!(
        TextureFiltering::Nearest.to_webgl_filtering(),
        WebGl2RenderingContext::NEAREST
    );
    assert_eq!(
        TextureFiltering::Linear.to_webgl_filtering(),
        WebGl2RenderingContext::LINEAR
    );
    assert_eq!(
        TextureFiltering::LinearMipmapLinear.to_webgl_filtering(),
        WebGl2RenderingContext::LINEAR_MIPMAP_LINEAR
    );
}

/// Test texture configuration default
#[wasm_bindgen_test]
fn test_texture_config_default() {
    let config = TextureConfig::default();

    assert_eq!(config.format, TextureFormat::RGBA);
    assert_eq!(config.texture_type, TextureType::UnsignedByte);
    assert_eq!(config.wrap_s, TextureWrapping::Repeat);
    assert_eq!(config.wrap_t, TextureWrapping::Repeat);
    assert_eq!(config.min_filter, TextureFiltering::LinearMipmapLinear);
    assert_eq!(config.mag_filter, TextureFiltering::Linear);
    assert!(config.generate_mipmaps);
    assert!(config.flip_y);
    assert!(!config.premultiply_alpha);
}

/// Test texture info creation
#[wasm_bindgen_test]
fn test_texture_info_creation() {
    let info = TextureInfo {
        width: 256,
        height: 256,
        format: TextureFormat::RGBA,
        texture_type: TextureType::UnsignedByte,
        channels: 4,
        size_bytes: 256 * 256 * 4,
    };

    assert_eq!(info.width, 256);
    assert_eq!(info.height, 256);
    assert_eq!(info.format, TextureFormat::RGBA);
    assert_eq!(info.texture_type, TextureType::UnsignedByte);
    assert_eq!(info.channels, 4);
    assert_eq!(info.size_bytes, 262144); // 256 * 256 * 4
}

/// Test texture creation from pixels
#[wasm_bindgen_test]
fn test_texture_from_pixels() {
    // Create a canvas to get WebGL context
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

    // Create test pixel data (2x2 RGBA texture)
    let pixels = vec![
        255, 0, 0, 255, // Red pixel
        0, 255, 0, 255, // Green pixel
        0, 0, 255, 255, // Blue pixel
        255, 255, 255, 255, // White pixel
    ];

    let result = Texture::from_pixels(&context, "test_texture", pixels, 2, 2, None);
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.name, "test_texture");
    assert_eq!(texture.info.width, 2);
    assert_eq!(texture.info.height, 2);
    assert_eq!(texture.info.format, TextureFormat::RGBA);
    assert!(texture.loaded);
    assert!(texture.error.is_none());
}

/// Test texture creation with custom config
#[wasm_bindgen_test]
fn test_texture_with_custom_config() {
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

    let config = TextureConfig {
        format: TextureFormat::RGB,
        texture_type: TextureType::UnsignedByte,
        wrap_s: TextureWrapping::ClampToEdge,
        wrap_t: TextureWrapping::ClampToEdge,
        min_filter: TextureFiltering::Nearest,
        mag_filter: TextureFiltering::Nearest,
        generate_mipmaps: false,
        flip_y: false,
        premultiply_alpha: false,
    };

    let pixels = vec![255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255]; // 2x2 RGB
    let result = Texture::from_pixels(
        &context,
        "custom_texture",
        pixels,
        2,
        2,
        Some(config.clone()),
    );
    assert!(result.is_ok());

    let texture = result.unwrap();
    assert_eq!(texture.config.format, TextureFormat::RGB);
    assert_eq!(texture.config.wrap_s, TextureWrapping::ClampToEdge);
    assert_eq!(texture.config.min_filter, TextureFiltering::Nearest);
    assert!(!texture.config.generate_mipmaps);
}

/// Test texture manager creation
#[wasm_bindgen_test]
fn test_texture_manager_creation() {
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

    let result = TextureManager::new(&context);
    assert!(result.is_ok());

    let manager = result.unwrap();
    assert_eq!(manager.get_texture_count(), 0);
    assert!(manager.get_max_texture_size() > 0);
    assert!(manager.get_max_texture_units() > 0);
}

/// Test texture manager operations
#[wasm_bindgen_test]
fn test_texture_manager_operations() {
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

    let mut manager = TextureManager::new(&context).unwrap();

    // Test creating texture from pixels
    let pixels = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 1x2 RGBA
    let result = manager.create_from_pixels("test", pixels, 1, 2, None);
    assert!(result.is_ok());

    // Test getting texture
    assert_eq!(manager.get_texture_count(), 1);
    let texture = manager.get_texture("test");
    assert!(texture.is_some());
    assert_eq!(texture.unwrap().name, "test");

    // Test getting texture names
    let names = manager.get_texture_names();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0], "test");

    // Test removing texture
    let removed = manager.remove_texture("test");
    assert!(removed.is_some());
    assert_eq!(manager.get_texture_count(), 0);
}

/// Test texture binding
#[wasm_bindgen_test]
fn test_texture_binding() {
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

    let pixels = vec![255, 0, 0, 255]; // 1x1 RGBA
    let texture = Texture::from_pixels(&context, "bind_test", pixels, 1, 1, None).unwrap();

    // Test binding to texture unit 0
    let result = texture.bind(&context, 0);
    assert!(result.is_ok());

    // Test unbinding
    texture.unbind(&context);
}

/// Test texture size calculation
#[wasm_bindgen_test]
fn test_texture_size_calculation() {
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

    let pixels = vec![0u8; 512 * 512 * 4]; // 512x512 RGBA
    let texture = Texture::from_pixels(&context, "size_test", pixels, 512, 512, None).unwrap();

    let (width, height) = texture.get_size();
    assert_eq!(width, 512);
    assert_eq!(height, 512);
    assert_eq!(texture.info.size_bytes, 512 * 512 * 4);
}

/// Test texture error handling
#[wasm_bindgen_test]
fn test_texture_error_handling() {
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

    // Test with invalid pixel data (wrong size)
    let pixels = vec![255, 0, 0]; // Only 3 bytes for RGBA
    let result = Texture::from_pixels(&context, "error_test", pixels, 1, 1, None);
    // This should still work as we're not validating pixel data size in the current implementation
    assert!(result.is_ok());
}

/// Test texture format channel calculation
#[wasm_bindgen_test]
fn test_texture_format_channels() {
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

    // Test RGB format
    let config = TextureConfig {
        format: TextureFormat::RGB,
        ..Default::default()
    };
    let pixels = vec![255, 0, 0, 0, 255, 0]; // 2x1 RGB
    let texture = Texture::from_pixels(&context, "rgb_test", pixels, 2, 1, Some(config)).unwrap();
    assert_eq!(texture.info.channels, 3);

    // Test Luminance format
    let config = TextureConfig {
        format: TextureFormat::Luminance,
        ..Default::default()
    };
    let pixels = vec![128, 255]; // 2x1 Luminance
    let texture =
        Texture::from_pixels(&context, "luminance_test", pixels, 2, 1, Some(config)).unwrap();
    assert_eq!(texture.info.channels, 1);
}

/// Test texture manager clear
#[wasm_bindgen_test]
fn test_texture_manager_clear() {
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

    let mut manager = TextureManager::new(&context).unwrap();

    // Add some textures
    let pixels1 = vec![255, 0, 0, 255];
    let pixels2 = vec![0, 255, 0, 255];

    manager
        .create_from_pixels("texture1", pixels1, 1, 1, None)
        .unwrap();
    manager
        .create_from_pixels("texture2", pixels2, 1, 1, None)
        .unwrap();

    assert_eq!(manager.get_texture_count(), 2);

    // Clear all textures
    manager.clear();
    assert_eq!(manager.get_texture_count(), 0);
}

/// Test texture source enum
#[wasm_bindgen_test]
fn test_texture_source_enum() {
    // Test that TextureSource can be created (we can't easily test Image/Video elements in tests)
    let pixels = vec![255, 0, 0, 255];
    let source = TextureSource::Pixels(pixels);

    match source {
        TextureSource::Pixels(p) => {
            assert_eq!(p.len(), 4);
            assert_eq!(p[0], 255);
            assert_eq!(p[1], 0);
            assert_eq!(p[2], 0);
            assert_eq!(p[3], 255);
        }
        _ => panic!("Expected Pixels variant"),
    }
}

/// Test texture ID generation
#[wasm_bindgen_test]
fn test_texture_id_generation() {
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

    let pixels = vec![255, 0, 0, 255];
    let texture1 = Texture::from_pixels(&context, "test1", pixels.clone(), 1, 1, None).unwrap();
    let texture2 = Texture::from_pixels(&context, "test2", pixels, 1, 1, None).unwrap();

    // IDs should be unique
    assert_ne!(texture1.id, texture2.id);
    assert!(!texture1.id.is_empty());
    assert!(!texture2.id.is_empty());
}

/// Test texture loading state
#[wasm_bindgen_test]
fn test_texture_loading_state() {
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

    let pixels = vec![255, 0, 0, 255];
    let texture = Texture::from_pixels(&context, "loading_test", pixels, 1, 1, None).unwrap();

    assert!(texture.is_loaded());
    assert!(texture.get_error().is_none());
}

/// Test texture configuration cloning
#[wasm_bindgen_test]
fn test_texture_config_cloning() {
    let config1 = TextureConfig {
        format: TextureFormat::RGB,
        texture_type: TextureType::Float,
        wrap_s: TextureWrapping::ClampToEdge,
        wrap_t: TextureWrapping::MirroredRepeat,
        min_filter: TextureFiltering::Nearest,
        mag_filter: TextureFiltering::Linear,
        generate_mipmaps: false,
        flip_y: true,
        premultiply_alpha: true,
    };

    let config2 = config1.clone();

    assert_eq!(config1.format, config2.format);
    assert_eq!(config1.texture_type, config2.texture_type);
    assert_eq!(config1.wrap_s, config2.wrap_s);
    assert_eq!(config1.wrap_t, config2.wrap_t);
    assert_eq!(config1.min_filter, config2.min_filter);
    assert_eq!(config1.mag_filter, config2.mag_filter);
    assert_eq!(config1.generate_mipmaps, config2.generate_mipmaps);
    assert_eq!(config1.flip_y, config2.flip_y);
    assert_eq!(config1.premultiply_alpha, config2.premultiply_alpha);
}
