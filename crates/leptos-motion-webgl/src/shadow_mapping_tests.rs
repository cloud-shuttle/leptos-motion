//! Comprehensive tests for the shadow mapping system

use super::*;
use crate::shadow_mapping::LightCollection;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

/// Test shadow map resolution
#[test]
fn test_shadow_map_resolution() {
    assert_eq!(ShadowMapResolution::Low.get_size(), 512);
    assert_eq!(ShadowMapResolution::Medium.get_size(), 1024);
    assert_eq!(ShadowMapResolution::High.get_size(), 2048);
    assert_eq!(ShadowMapResolution::Ultra.get_size(), 4096);
}

/// Test shadow map configuration
#[test]
fn test_shadow_map_config() {
    let config = ShadowMapConfig::default();
    assert_eq!(config.resolution, ShadowMapResolution::High);
    assert_eq!(config.near_plane, 0.1);
    assert_eq!(config.far_plane, 100.0);
    assert_eq!(config.bias, 0.005);
    assert_eq!(config.normal_offset, 0.0);
    assert_eq!(config.filtering, ShadowMapFiltering::PCF);
}

/// Test scene bounds
#[test]
fn test_scene_bounds() {
    let mut bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);

    // Test initial bounds
    assert_eq!(bounds.min, [-10.0, -5.0, -10.0]);
    assert_eq!(bounds.max, [10.0, 5.0, 10.0]);

    // Test center calculation
    let center = bounds.get_center();
    assert_eq!(center, [0.0, 0.0, 0.0]);

    // Test size calculation
    let size = bounds.get_size();
    assert_eq!(size, [20.0, 10.0, 20.0]);

    // Test expansion
    bounds.expand([15.0, 8.0, -15.0]);
    assert_eq!(bounds.min, [-10.0, -5.0, -15.0]);
    assert_eq!(bounds.max, [15.0, 8.0, 10.0]);
}

/// Test directional shadow map creation
#[test]
fn test_directional_shadow_map_creation() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let shadow_map = DirectionalShadowMap::new(&context, config).unwrap();

    assert_eq!(shadow_map.resolution, 2048);
    assert_eq!(shadow_map.config.resolution, ShadowMapResolution::High);
    assert_eq!(shadow_map.config.near_plane, 0.1);
    assert_eq!(shadow_map.config.far_plane, 100.0);
}

/// Test directional shadow map matrix updates
#[test]
fn test_directional_shadow_map_matrices() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let mut shadow_map = DirectionalShadowMap::new(&context, config).unwrap();

    // Create a directional light
    let light = DirectionalLight {
        light: crate::lighting::Light {
            id: "test".to_string(),
            name: "test".to_string(),
            light_type: crate::lighting::LightType::Directional,
            color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
            intensity: 1.0,
            enabled: true,
        },
        direction: [0.0, -1.0, 0.0],
        cast_shadow: true,
    };

    // Create scene bounds
    let scene_bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);

    // Update matrices
    shadow_map.update_matrices(&light, &scene_bounds);

    // Check that matrices are not identity matrices
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    assert_ne!(shadow_map.projection_matrix, identity);
    assert_ne!(shadow_map.view_matrix, identity);
    assert_ne!(shadow_map.view_projection_matrix, identity);
}

/// Test point shadow map creation
#[test]
fn test_point_shadow_map_creation() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let shadow_map = PointShadowMap::new(&context, config).unwrap();

    assert_eq!(shadow_map.resolution, 2048);
    assert_eq!(shadow_map.config.resolution, ShadowMapResolution::High);
    assert_eq!(shadow_map.config.near_plane, 0.1);
    assert_eq!(shadow_map.config.far_plane, 100.0);
}

/// Test point shadow map matrix updates
#[test]
fn test_point_shadow_map_matrices() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let mut shadow_map = PointShadowMap::new(&context, config).unwrap();

    // Create a point light
    let light = PointLight {
        light: crate::lighting::Light {
            id: "test".to_string(),
            name: "test".to_string(),
            light_type: crate::lighting::LightType::Point,
            color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
            intensity: 1.0,
            enabled: true,
        },
        position: [0.0, 5.0, 0.0],
        range: 100.0,
        constant_attenuation: 1.0,
        linear_attenuation: 0.0,
        quadratic_attenuation: 1.0,
        cast_shadow: true,
    };

    // Update matrices
    shadow_map.update_matrices(&light);

    // Check that matrices are not identity matrices
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    assert_ne!(shadow_map.projection_matrix, identity);

    // Check that all view matrices are different
    for i in 0..6 {
        assert_ne!(shadow_map.view_matrices[i], identity);
        assert_ne!(shadow_map.view_projection_matrices[i], identity);
    }
}

/// Test shadow mapping manager creation
#[test]
fn test_shadow_mapping_manager_creation() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let manager = ShadowMappingManager::new(context);

    assert_eq!(manager.get_total_shadow_map_count(), 0);
    assert_eq!(manager.get_max_shadow_maps(), (4, 2));
}

/// Test adding directional shadow maps
#[test]
fn test_add_directional_shadow_maps() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    // Add first shadow map
    let config = ShadowMapConfig::default();
    manager
        .add_directional_shadow_map("sun", config.clone())
        .unwrap();
    assert_eq!(manager.get_total_shadow_map_count(), 1);

    // Add second shadow map
    manager
        .add_directional_shadow_map("moon", config.clone())
        .unwrap();
    assert_eq!(manager.get_total_shadow_map_count(), 2);

    // Test getting shadow map
    let shadow_map = manager.get_directional_shadow_map("sun").unwrap();
    assert_eq!(shadow_map.resolution, 2048);

    // Test getting mutable shadow map
    let shadow_map = manager.get_directional_shadow_map_mut("sun").unwrap();
    assert_eq!(shadow_map.resolution, 2048);
}

/// Test adding point shadow maps
#[test]
fn test_add_point_shadow_maps() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    // Add first shadow map
    let config = ShadowMapConfig::default();
    manager
        .add_point_shadow_map("lamp", config.clone())
        .unwrap();
    assert_eq!(manager.get_total_shadow_map_count(), 1);

    // Add second shadow map
    manager
        .add_point_shadow_map("torch", config.clone())
        .unwrap();
    assert_eq!(manager.get_total_shadow_map_count(), 2);

    // Test getting shadow map
    let shadow_map = manager.get_point_shadow_map("lamp").unwrap();
    assert_eq!(shadow_map.resolution, 2048);

    // Test getting mutable shadow map
    let shadow_map = manager.get_point_shadow_map_mut("lamp").unwrap();
    assert_eq!(shadow_map.resolution, 2048);
}

/// Test shadow map limits
#[test]
fn test_shadow_map_limits() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    // Set lower limits
    manager.set_max_shadow_maps(2, 1);
    assert_eq!(manager.get_max_shadow_maps(), (2, 1));

    let config = ShadowMapConfig::default();

    // Add up to limit
    manager
        .add_directional_shadow_map("sun", config.clone())
        .unwrap();
    manager
        .add_directional_shadow_map("moon", config.clone())
        .unwrap();

    // Try to add beyond limit
    let result = manager.add_directional_shadow_map("star", config.clone());
    assert!(result.is_err());

    // Add point shadow map
    manager
        .add_point_shadow_map("lamp", config.clone())
        .unwrap();

    // Try to add beyond point limit
    let result = manager.add_point_shadow_map("torch", config.clone());
    assert!(result.is_err());
}

/// Test removing shadow maps
#[test]
fn test_remove_shadow_maps() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    let config = ShadowMapConfig::default();
    manager
        .add_directional_shadow_map("sun", config.clone())
        .unwrap();
    manager
        .add_point_shadow_map("lamp", config.clone())
        .unwrap();

    assert_eq!(manager.get_total_shadow_map_count(), 2);

    // Remove directional shadow map
    let removed = manager.remove_directional_shadow_map("sun");
    assert!(removed.is_some());
    assert_eq!(manager.get_total_shadow_map_count(), 1);

    // Remove point shadow map
    let removed = manager.remove_point_shadow_map("lamp");
    assert!(removed.is_some());
    assert_eq!(manager.get_total_shadow_map_count(), 0);

    // Try to remove non-existent shadow map
    let removed = manager.remove_directional_shadow_map("nonexistent");
    assert!(removed.is_none());
}

/// Test clearing shadow maps
#[test]
fn test_clear_shadow_maps() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    let config = ShadowMapConfig::default();
    manager
        .add_directional_shadow_map("sun", config.clone())
        .unwrap();
    manager
        .add_directional_shadow_map("moon", config.clone())
        .unwrap();
    manager
        .add_point_shadow_map("lamp", config.clone())
        .unwrap();

    assert_eq!(manager.get_total_shadow_map_count(), 3);

    manager.clear();
    assert_eq!(manager.get_total_shadow_map_count(), 0);
}

/// Test light collection
#[test]
fn test_light_collection() {
    let mut collection = LightCollection::new();

    // Add directional light
    let directional_light = DirectionalLight {
        light: crate::lighting::Light {
            id: "test".to_string(),
            name: "test".to_string(),
            light_type: crate::lighting::LightType::Directional,
            color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
            intensity: 1.0,
            enabled: true,
        },
        direction: [0.0, -1.0, 0.0],
        cast_shadow: true,
    };
    collection.add_directional_light("sun".to_string(), directional_light);

    // Add point light
    let point_light = PointLight {
        light: crate::lighting::Light {
            id: "test".to_string(),
            name: "test".to_string(),
            light_type: crate::lighting::LightType::Point,
            color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
            intensity: 1.0,
            enabled: true,
        },
        position: [0.0, 5.0, 0.0],
        range: 100.0,
        constant_attenuation: 1.0,
        linear_attenuation: 0.0,
        quadratic_attenuation: 1.0,
        cast_shadow: true,
    };
    collection.add_point_light("lamp".to_string(), point_light);

    // Test getting lights
    let sun = collection.get_directional_light("sun").unwrap();
    assert_eq!(sun.direction, [0.0, -1.0, 0.0]);

    let lamp = collection.get_point_light("lamp").unwrap();
    assert_eq!(lamp.position, [0.0, 5.0, 0.0]);

    // Test getting non-existent light
    assert!(collection.get_directional_light("nonexistent").is_none());
    assert!(collection.get_point_light("nonexistent").is_none());
}

/// Test shadow map update integration
#[test]
fn test_shadow_map_update_integration() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let mut manager = ShadowMappingManager::new(context);

    // Add shadow maps
    let config = ShadowMapConfig::default();
    manager
        .add_directional_shadow_map("sun", config.clone())
        .unwrap();
    manager
        .add_point_shadow_map("lamp", config.clone())
        .unwrap();

    // Create light collection
    let mut light_collection = LightCollection::new();
    light_collection.add_directional_light(
        "sun".to_string(),
        DirectionalLight {
            light: crate::lighting::Light {
                id: "test".to_string(),
                name: "test".to_string(),
                light_type: crate::lighting::LightType::Directional,
                color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
                intensity: 1.0,
                enabled: true,
            },
            direction: [0.0, -1.0, 0.0],
            cast_shadow: true,
        },
    );
    light_collection.add_point_light(
        "lamp".to_string(),
        PointLight {
            light: crate::lighting::Light {
                id: "test".to_string(),
                name: "test".to_string(),
                light_type: crate::lighting::LightType::Point,
                color: crate::lighting::Color::new(1.0, 1.0, 1.0, 1.0),
                intensity: 1.0,
                enabled: true,
            },
            position: [0.0, 5.0, 0.0],
            range: 100.0,
            constant_attenuation: 1.0,
            linear_attenuation: 0.0,
            quadratic_attenuation: 1.0,
            cast_shadow: true,
        },
    );

    // Create scene bounds
    let scene_bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);

    // Update shadow maps
    manager.update_shadow_maps(&light_collection, &scene_bounds);

    // Verify that matrices were updated
    let sun_shadow_map = manager.get_directional_shadow_map("sun").unwrap();
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    ];
    assert_ne!(sun_shadow_map.projection_matrix, identity);
    assert_ne!(sun_shadow_map.view_matrix, identity);
    assert_ne!(sun_shadow_map.view_projection_matrix, identity);

    let lamp_shadow_map = manager.get_point_shadow_map("lamp").unwrap();
    assert_ne!(lamp_shadow_map.projection_matrix, identity);
    for i in 0..6 {
        assert_ne!(lamp_shadow_map.view_matrices[i], identity);
        assert_ne!(lamp_shadow_map.view_projection_matrices[i], identity);
    }
}

/// Test shadow map filtering types
#[test]
fn test_shadow_map_filtering() {
    assert_eq!(ShadowMapFiltering::None, ShadowMapFiltering::None);
    assert_eq!(ShadowMapFiltering::PCF, ShadowMapFiltering::PCF);
    assert_eq!(ShadowMapFiltering::VSM, ShadowMapFiltering::VSM);
    assert_eq!(ShadowMapFiltering::ESM, ShadowMapFiltering::ESM);

    assert_ne!(ShadowMapFiltering::None, ShadowMapFiltering::PCF);
    assert_ne!(ShadowMapFiltering::PCF, ShadowMapFiltering::VSM);
    assert_ne!(ShadowMapFiltering::VSM, ShadowMapFiltering::ESM);
}

/// Test shadow map configuration with different settings
#[test]
fn test_shadow_map_config_variations() {
    let config = ShadowMapConfig {
        resolution: ShadowMapResolution::Low,
        near_plane: 0.01,
        far_plane: 1000.0,
        bias: 0.001,
        normal_offset: 0.1,
        filtering: ShadowMapFiltering::VSM,
    };

    assert_eq!(config.resolution, ShadowMapResolution::Low);
    assert_eq!(config.near_plane, 0.01);
    assert_eq!(config.far_plane, 1000.0);
    assert_eq!(config.bias, 0.001);
    assert_eq!(config.normal_offset, 0.1);
    assert_eq!(config.filtering, ShadowMapFiltering::VSM);
}

/// Test shadow map binding and unbinding
#[test]
fn test_shadow_map_binding() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let shadow_map = DirectionalShadowMap::new(&context, config.clone()).unwrap();

    // Test binding
    shadow_map.bind(&context);

    // Test unbinding
    shadow_map.unbind(&context);

    // Test point shadow map binding
    let point_shadow_map = PointShadowMap::new(&context, config.clone()).unwrap();

    // Test binding each face
    for face in 0..6 {
        point_shadow_map.bind_face(&context, face);
        point_shadow_map.unbind(&context);
    }
}

/// Test shadow map texture access
#[test]
fn test_shadow_map_texture_access() {
    let canvas = create_test_canvas();
    let context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()
        .unwrap();

    let config = ShadowMapConfig::default();
    let directional_shadow_map = DirectionalShadowMap::new(&context, config.clone()).unwrap();
    let point_shadow_map = PointShadowMap::new(&context, config).unwrap();

    // Test getting shadow map textures
    let _directional_texture = directional_shadow_map.get_shadow_map();
    let _point_texture = point_shadow_map.get_shadow_map();

    // Test getting view-projection matrices
    let _directional_matrix = directional_shadow_map.get_view_projection_matrix();
    for face in 0..6 {
        let _point_matrix = point_shadow_map.get_view_projection_matrix(face);
    }
}

/// Helper function to create a test canvas
fn create_test_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap()
}
