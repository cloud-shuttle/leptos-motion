//! Integration tests for Phase 2 features (texture system, lighting, model loading)

use crate::texture::*;
use crate::lighting::*;
use crate::model_loader::*;
use crate::geometry::*;
use crate::material::*;
use wasm_bindgen_test::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, HtmlCanvasElement};

wasm_bindgen_test_configure!(run_in_browser);

/// Test complete Phase 2 integration: texture + lighting + model loading
#[wasm_bindgen_test]
fn test_phase2_integration() {
    // Create WebGL context
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

    // 1. Test texture system
    let mut texture_manager = TextureManager::new(&context).unwrap();
    
    // Create a simple 2x2 RGBA texture
    let pixels = vec![
        255, 0, 0, 255,   // Red pixel
        0, 255, 0, 255,   // Green pixel
        0, 0, 255, 255,   // Blue pixel
        255, 255, 255, 255 // White pixel
    ];
    
    texture_manager.create_from_pixels("test_texture", pixels, 2, 2, None).unwrap();
    assert_eq!(texture_manager.get_texture_count(), 1);
    
    let texture = texture_manager.get_texture("test_texture").unwrap();
    assert!(texture.is_loaded());
    assert_eq!(texture.get_size(), (2, 2));

    // 2. Test lighting system
    let mut light_manager = LightManager::new();
    
    // Add ambient light
    light_manager.add_ambient_light("ambient", Color::white(), 0.3).unwrap();
    
    // Add directional light (sun)
    light_manager.add_directional_light("sun", Color::white(), 1.0, [0.0, -1.0, 0.0]).unwrap();
    
    // Add point light
    light_manager.add_point_light("bulb", Color::white(), 1.0, [0.0, 5.0, 0.0]).unwrap();
    
    // Add spot light
    light_manager.add_spot_light("flashlight", Color::white(), 1.0, [0.0, 0.0, 0.0], [0.0, -1.0, 0.0], 0.5, 1.0).unwrap();
    
    assert_eq!(light_manager.get_total_light_count(), 4);
    assert_eq!(light_manager.get_enabled_light_count(), 4);

    // 3. Test model loading system
    let model_loader = ModelLoader::new();
    
    // Create a simple OBJ model
    let obj_data = r#"
# Simple triangle with texture coordinates
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0
vt 0.0 0.0
vt 1.0 0.0
vt 0.5 1.0
vn 0.0 0.0 1.0
f 1/1/1 2/2/1 3/3/1
"#;
    
    let options = ModelLoadOptions {
        generate_normals: true,
        generate_tangents: false,
        flip_y: false,
        scale: 1.0,
        center: false,
        max_vertices: None,
        max_indices: None,
    };
    
    let mut model = model_loader.load_from_data(obj_data, ModelFormat::OBJ, options).unwrap();
    assert_eq!(model.get_mesh_count(), 1);
    assert_eq!(model.get_total_vertex_count(), 3);
    assert_eq!(model.get_total_index_count(), 3);
    
    // Get mesh data first
    let mesh_vertex_count = model.get_mesh_by_index(0).unwrap().geometry.vertices.len();
    let mesh_index_count = model.get_mesh_by_index(0).unwrap().geometry.get_indices_len();
    let bbox = model.get_mesh_by_index(0).unwrap().bounding_box;
    
    assert_eq!(mesh_vertex_count, 3);
    assert_eq!(mesh_index_count, 3);

    // 4. Test material system integration
    let material = Material::new("test_material", MaterialType::Basic);
    model.add_material("test_material".to_string(), material);
    assert_eq!(model.get_material_count(), 1);

    // 5. Test geometry integration
    let mut geometry = Geometry::new("test_geometry");
    geometry.vertices = vec![
        Vertex::new(),
        Vertex::new(),
        Vertex::new(),
    ];
    geometry.set_indices(vec![0, 1, 2]);
    
    // Verify geometry properties
    assert_eq!(geometry.vertices.len(), 3);
    assert_eq!(geometry.get_indices_len(), 3);

    // 6. Test bounding box calculations
    assert!(bbox.volume() > 0.0);
    assert!(bbox.diagonal_length() > 0.0);

    // 7. Test light calculations
    let point_light = light_manager.get_point_light("bulb").unwrap();
    let attenuation = point_light.calculate_attenuation(10.0);
    assert!(attenuation > 0.0 && attenuation <= 1.0);
    
    let spot_light = light_manager.get_spot_light("flashlight").unwrap();
    let spot_factor = spot_light.calculate_spot_factor([0.0, 1.0, 0.0]);
    assert_eq!(spot_factor, 1.0);

    // 8. Test texture binding
    let texture = texture_manager.get_texture("test_texture").unwrap();
    texture.bind(&context, 0).unwrap();
    texture.unbind(&context);

    // 9. Test color operations
    let color = Color::from_rgb(255, 128, 64);
    let rgb_array = color.as_rgb_array();
    assert_eq!(rgb_array[0], 1.0);
    assert_eq!(rgb_array[1], 128.0 / 255.0);
    assert_eq!(rgb_array[2], 64.0 / 255.0);

    // 10. Test model statistics
    let stats = model.get_stats();
    assert_eq!(stats.mesh_count, 1);
    assert_eq!(stats.material_count, 1);
    assert_eq!(stats.total_vertices, 3);
    assert_eq!(stats.total_indices, 3);
    assert!(stats.bounding_box_volume > 0.0);
    assert!(stats.bounding_box_diagonal > 0.0);
}

/// Test texture and lighting integration
#[wasm_bindgen_test]
fn test_texture_lighting_integration() {
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

    // Create texture manager
    let mut texture_manager = TextureManager::new(&context).unwrap();
    
    // Create different texture formats
    let rgb_pixels = vec![255, 0, 0, 0, 255, 0, 0, 0, 255]; // 1x1 RGB
    let rgba_pixels = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 1x1 RGBA
    
    let rgb_config = TextureConfig {
        format: TextureFormat::RGB,
        ..Default::default()
    };
    
    texture_manager.create_from_pixels("rgb_texture", rgb_pixels, 1, 1, Some(rgb_config)).unwrap();
    texture_manager.create_from_pixels("rgba_texture", rgba_pixels, 1, 1, None).unwrap();
    
    assert_eq!(texture_manager.get_texture_count(), 2);

    // Create lighting system
    let mut light_manager = LightManager::new();
    
    // Test different light types with different colors
    light_manager.add_ambient_light("warm_ambient", Color::from_rgb(255, 200, 150), 0.2).unwrap();
    light_manager.add_directional_light("cool_sun", Color::from_rgb(150, 200, 255), 0.8, [0.0, -1.0, 0.0]).unwrap();
    light_manager.add_point_light("warm_bulb", Color::from_rgb(255, 200, 100), 1.0, [2.0, 3.0, 1.0]).unwrap();
    
    // Test light intensity modifications
    if let Some(light) = light_manager.get_ambient_light_mut("warm_ambient") {
        light.light.set_intensity(0.5);
        assert_eq!(light.light.intensity, 0.5);
    }
    
    // Test light position modifications
    if let Some(light) = light_manager.get_point_light_mut("warm_bulb") {
        light.set_position([1.0, 2.0, 3.0]);
        assert_eq!(light.get_position(), [1.0, 2.0, 3.0]);
    }
    
    // Test light range modifications
    if let Some(light) = light_manager.get_point_light_mut("warm_bulb") {
        light.set_range(50.0);
        assert_eq!(light.get_range(), 50.0);
    }
    
    assert_eq!(light_manager.get_total_light_count(), 3);
}

/// Test model loading with different options
#[wasm_bindgen_test]
fn test_model_loading_options_integration() {
    let model_loader = ModelLoader::new();
    
    // Test with scaling
    let obj_data_scaled = r#"
v 2.0 0.0 0.0
v 4.0 0.0 0.0
v 3.0 2.0 0.0
f 1 2 3
"#;
    
    let scaled_options = ModelLoadOptions {
        scale: 0.5,
        ..Default::default()
    };
    
    let scaled_model = model_loader.load_from_data(obj_data_scaled, ModelFormat::OBJ, scaled_options).unwrap();
    let scaled_mesh = scaled_model.get_mesh_by_index(0).unwrap();
    
    // Verify scaling was applied
    for vertex in &scaled_mesh.geometry.vertices {
        let pos = vertex.get_position();
        assert!(pos[0] <= 2.0); // Should be scaled down
        assert!(pos[1] <= 1.0);
    }
    
    // Test with centering
    let obj_data_centered = r#"
v 10.0 10.0 0.0
v 12.0 10.0 0.0
v 11.0 12.0 0.0
f 1 2 3
"#;
    
    let centered_options = ModelLoadOptions {
        center: true,
        ..Default::default()
    };
    
    let centered_model = model_loader.load_from_data(obj_data_centered, ModelFormat::OBJ, centered_options).unwrap();
    let centered_mesh = centered_model.get_mesh_by_index(0).unwrap();
    
    // Verify centering was applied
    let bbox = &centered_mesh.bounding_box;
    let center_x = (bbox.min[0] + bbox.max[0]) * 0.5;
    let center_y = (bbox.min[1] + bbox.max[1]) * 0.5;
    assert!((center_x.abs() - 0.0).abs() < 0.001);
    assert!((center_y.abs() - 0.0).abs() < 0.001);
    
    // Test with Y flipping
    let obj_data_flipped = r#"
v 0.0 2.0 0.0
v 1.0 2.0 0.0
v 0.5 3.0 0.0
f 1 2 3
"#;
    
    let flipped_options = ModelLoadOptions {
        flip_y: true,
        ..Default::default()
    };
    
    let flipped_model = model_loader.load_from_data(obj_data_flipped, ModelFormat::OBJ, flipped_options).unwrap();
    let flipped_mesh = flipped_model.get_mesh_by_index(0).unwrap();
    
    // Verify Y flipping was applied
    for vertex in &flipped_mesh.geometry.vertices {
        let pos = vertex.get_position();
        assert!(pos[1] <= 0.0); // Should be negative after flipping
    }
}

/// Test error handling across all systems
#[wasm_bindgen_test]
fn test_error_handling_integration() {
    let model_loader = ModelLoader::new();
    
    // Test unsupported format
    let result = model_loader.load_from_data("", ModelFormat::GLTF, ModelLoadOptions::default());
    assert!(result.is_err());
    
    // Test invalid OBJ data
    let invalid_obj = "invalid obj data";
    let result = model_loader.load_from_data(invalid_obj, ModelFormat::OBJ, ModelLoadOptions::default());
    // This should still work as we handle empty/invalid data gracefully
    assert!(result.is_ok());
    
    let model = result.unwrap();
    assert!(model.is_empty());
}

/// Test performance characteristics
#[wasm_bindgen_test]
fn test_performance_integration() {
    let model_loader = ModelLoader::new();
    
    // Create a more complex model
    let mut complex_obj = String::new();
    complex_obj.push_str("# Complex model with many vertices\n");
    
    // Generate 100 vertices
    for i in 0..100 {
        complex_obj.push_str(&format!("v {} {} 0.0\n", i as f32 * 0.1, (i % 10) as f32 * 0.1));
    }
    
    // Generate faces
    for i in 0..98 {
        complex_obj.push_str(&format!("f {} {} {}\n", i + 1, i + 2, i + 3));
    }
    
    let start_time = web_sys::js_sys::Date::now();
    let model = model_loader.load_from_data(&complex_obj, ModelFormat::OBJ, ModelLoadOptions::default()).unwrap();
    let end_time = web_sys::js_sys::Date::now();
    
    let processing_time = end_time - start_time;
    
    // Verify model was loaded correctly
    assert_eq!(model.get_mesh_count(), 1);
    assert_eq!(model.get_total_vertex_count(), 100);
    assert_eq!(model.get_total_index_count(), 98 * 3);
    
    // Verify reasonable processing time (should be very fast for this size)
    assert!(processing_time < 1000.0); // Less than 1 second
    
    // Test model statistics
    let stats = model.get_stats();
    assert_eq!(stats.mesh_count, 1);
    assert_eq!(stats.total_vertices, 100);
    assert_eq!(stats.total_indices, 98 * 3);
    assert!(stats.bounding_box_volume > 0.0);
}

/// Test memory management
#[wasm_bindgen_test]
fn test_memory_management_integration() {
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

    // Test texture manager memory management
    let mut texture_manager = TextureManager::new(&context).unwrap();
    
    // Create multiple textures
    for i in 0..5 {
        let pixels = vec![255, 0, 0, 255];
        texture_manager.create_from_pixels(&format!("texture_{}", i), pixels, 1, 1, None).unwrap();
    }
    
    assert_eq!(texture_manager.get_texture_count(), 5);
    
    // Remove some textures
    texture_manager.remove_texture("texture_0");
    texture_manager.remove_texture("texture_1");
    
    assert_eq!(texture_manager.get_texture_count(), 3);
    
    // Clear all textures
    texture_manager.clear();
    assert_eq!(texture_manager.get_texture_count(), 0);
    
    // Test light manager memory management
    let mut light_manager = LightManager::new();
    
    // Add multiple lights
    for i in 0..10 {
        light_manager.add_ambient_light(&format!("ambient_{}", i), Color::white(), 0.1).unwrap();
    }
    
    assert_eq!(light_manager.get_total_light_count(), 10);
    
    // Remove some lights
    light_manager.remove_ambient_light("ambient_0");
    light_manager.remove_ambient_light("ambient_1");
    
    assert_eq!(light_manager.get_total_light_count(), 8);
    
    // Clear all lights
    light_manager.clear();
    assert_eq!(light_manager.get_total_light_count(), 0);
}
