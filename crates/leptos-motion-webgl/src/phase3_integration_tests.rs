//! End-to-end integration tests for Phase 3 features (post-processing, shadows, physics)

use super::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use crate::shadow_mapping::LightCollection;
use crate::post_processing::{PostProcessingConfig, PostProcessingPipeline};

/// Test post-processing effects integration
#[test]
fn test_post_processing_integration() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // Create post-processing manager
    let mut post_processing = PostProcessingPipeline::new(context, 800, 600).unwrap();
    
    // Add bloom effect
    let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.0);
    post_processing.add_effect(bloom_config);
    
    // Add SSAO effect
    let ssao_config = PostProcessingConfig::new(PostProcessingEffect::SSAO, 1.0);
    post_processing.add_effect(ssao_config);
    
    // Add tone mapping effect
    let tone_mapping_config = PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0);
    post_processing.add_effect(tone_mapping_config);
    
    // Verify effects were added
    assert_eq!(post_processing.get_effect_count(), 3);
    
    // Test getting effects
    let effects = post_processing.get_effects();
    assert_eq!(effects.len(), 3);
    assert_eq!(effects[0].effect, PostProcessingEffect::Bloom);
    assert_eq!(effects[1].effect, PostProcessingEffect::SSAO);
    assert_eq!(effects[2].effect, PostProcessingEffect::ToneMapping);
    
    // Test removing effects
    post_processing.remove_effect(1); // Remove SSAO (index 1)
    assert_eq!(post_processing.get_effect_count(), 2);
    
    // Test clearing effects
    post_processing.clear_effects();
    assert_eq!(post_processing.get_effect_count(), 0);
}

/// Test shadow mapping integration
#[test]
fn test_shadow_mapping_integration() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // Create shadow mapping manager
    let mut shadow_manager = ShadowMappingManager::new(context);
    
    // Add directional shadow map
    let directional_config = ShadowMapConfig {
        resolution: ShadowMapResolution::High,
        near_plane: 0.1,
        far_plane: 100.0,
        bias: 0.005,
        normal_offset: 0.0,
        filtering: ShadowMapFiltering::PCF,
    };
    shadow_manager.add_directional_shadow_map("sun", directional_config).unwrap();
    
    // Add point shadow map
    let point_config = ShadowMapConfig {
        resolution: ShadowMapResolution::Medium,
        near_plane: 0.1,
        far_plane: 50.0,
        bias: 0.01,
        normal_offset: 0.0,
        filtering: ShadowMapFiltering::VSM,
    };
    shadow_manager.add_point_shadow_map("lamp", point_config).unwrap();
    
    // Verify shadow maps were added
    assert_eq!(shadow_manager.get_total_shadow_map_count(), 2);
    assert!(shadow_manager.get_directional_shadow_map("sun").is_some());
    assert!(shadow_manager.get_point_shadow_map("lamp").is_some());
    
    // Create light collection
    let mut light_collection = LightCollection::new();
    light_collection.add_directional_light("sun".to_string(), DirectionalLight {
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
    });
    light_collection.add_point_light("lamp".to_string(), PointLight {
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
    });
    
    // Create scene bounds
    let scene_bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);
    
    // Update shadow maps
    shadow_manager.update_shadow_maps(&light_collection, &scene_bounds);
    
    // Verify matrices were updated
    let sun_shadow_map = shadow_manager.get_directional_shadow_map("sun").unwrap();
    let identity = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    assert_ne!(sun_shadow_map.projection_matrix, identity);
    assert_ne!(sun_shadow_map.view_matrix, identity);
    assert_ne!(sun_shadow_map.view_projection_matrix, identity);
    
    let lamp_shadow_map = shadow_manager.get_point_shadow_map("lamp").unwrap();
    assert_ne!(lamp_shadow_map.projection_matrix, identity);
    for i in 0..6 {
        assert_ne!(lamp_shadow_map.view_matrices[i], identity);
        assert_ne!(lamp_shadow_map.view_projection_matrices[i], identity);
    }
}

/// Test physics system integration
#[test]
fn test_physics_system_integration() {
    // Create physics world
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);
    
    // Create ground plane
    let ground_shape = CollisionShape::Plane {
        normal: [0.0, 1.0, 0.0],
        distance: 0.0,
    };
    let ground = RigidBody::new("ground", RigidBodyType::Static, ground_shape);
    world.add_body(ground).unwrap();
    
    // Create falling boxes
    let box_shape = CollisionShape::Box {
        half_extents: [0.5, 0.5, 0.5],
    };
    let mut box1 = RigidBody::new("box1", RigidBodyType::Dynamic, box_shape.clone());
    box1.set_position([0.0, 10.0, 0.0]);
    world.add_body(box1).unwrap();
    
    let mut box2 = RigidBody::new("box2", RigidBodyType::Dynamic, box_shape);
    box2.set_position([2.0, 10.0, 0.0]);
    world.add_body(box2).unwrap();
    
    // Create bouncing spheres
    let sphere_shape = CollisionShape::Sphere { radius: 0.5 };
    let mut sphere1 = RigidBody::new("sphere1", RigidBodyType::Dynamic, sphere_shape.clone());
    sphere1.set_position([-2.0, 10.0, 0.0]);
    sphere1.restitution = 0.8; // Bouncy
    world.add_body(sphere1).unwrap();
    
    let mut sphere2 = RigidBody::new("sphere2", RigidBodyType::Dynamic, sphere_shape);
    sphere2.set_position([-2.0, 12.0, 0.0]);
    sphere2.restitution = 0.8; // Bouncy
    world.add_body(sphere2).unwrap();
    
    // Verify initial state
    assert_eq!(world.get_body_count(), 4);
    assert_eq!(world.get_collision_count(), 0);
    
    // Simulate physics for 1 second
    for _ in 0..60 {
        world.step(1.0 / 60.0).unwrap();
    }
    
    // Check that bodies have moved due to gravity
    let box1 = world.get_body("box1").unwrap();
    let box2 = world.get_body("box2").unwrap();
    let sphere1 = world.get_body("sphere1").unwrap();
    let sphere2 = world.get_body("sphere2").unwrap();
    
    // Boxes should have fallen
    assert!(box1.position[1] < 10.0);
    assert!(box2.position[1] < 10.0);
    
    // Spheres should have fallen and possibly collided
    assert!(sphere1.position[1] < 10.0);
    assert!(sphere2.position[1] < 12.0);
    
    // Check for collisions
    assert!(world.get_collision_count() > 0);
}

/// Test complete Phase 3 integration
#[test]
fn test_phase3_complete_integration() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // 1. Create post-processing manager
    let mut post_processing = PostProcessingPipeline::new(context.clone(), 800, 600).unwrap();
    
    // Add effects
        let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.0);
        post_processing.add_effect(bloom_config);
    
        let ssao_config = PostProcessingConfig::new(PostProcessingEffect::SSAO, 1.0);
        post_processing.add_effect(ssao_config);
    
        let tone_mapping_config = PostProcessingConfig::new(PostProcessingEffect::ToneMapping, 1.0);
        post_processing.add_effect(tone_mapping_config);
    
    // 2. Create shadow mapping manager
    let mut shadow_manager = ShadowMappingManager::new(context.clone());
    
    let shadow_config = ShadowMapConfig {
        resolution: ShadowMapResolution::High,
        near_plane: 0.1,
        far_plane: 100.0,
        bias: 0.005,
        normal_offset: 0.0,
        filtering: ShadowMapFiltering::PCF,
    };
    shadow_manager.add_directional_shadow_map("sun", shadow_config).unwrap();
    
    // 3. Create physics world
    let physics_config = PhysicsWorldConfig::default();
    let mut physics_world = PhysicsWorld::new(physics_config);
    
    // Add physics bodies
    let ground_shape = CollisionShape::Plane {
        normal: [0.0, 1.0, 0.0],
        distance: 0.0,
    };
    let ground = RigidBody::new("ground", RigidBodyType::Static, ground_shape);
    physics_world.add_body(ground).unwrap();
    
    let box_shape = CollisionShape::Box {
        half_extents: [0.5, 0.5, 0.5],
    };
    let mut box_body = RigidBody::new("box", RigidBodyType::Dynamic, box_shape);
    box_body.set_position([0.0, 10.0, 0.0]);
    physics_world.add_body(box_body).unwrap();
    
    // 4. Create light collection for shadows
    let mut light_collection = LightCollection::new();
    light_collection.add_directional_light("sun".to_string(), DirectionalLight {
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
    });
    
    // 5. Create scene bounds
    let scene_bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);
    
    // 6. Simulate one frame
    // Update physics
    physics_world.step(1.0 / 60.0).unwrap();
    
    // Update shadow maps
    shadow_manager.update_shadow_maps(&light_collection, &scene_bounds);
    
    // 7. Verify all systems are working
    assert_eq!(post_processing.get_effect_count(), 2);
    assert_eq!(shadow_manager.get_total_shadow_map_count(), 1);
    assert_eq!(physics_world.get_body_count(), 2);
    
    // Check that physics body moved
    let box_body = physics_world.get_body("box").unwrap();
    assert!(box_body.position[1] < 10.0); // Should have fallen
    
    // Check that shadow map was updated
    let sun_shadow_map = shadow_manager.get_directional_shadow_map("sun").unwrap();
    let identity = [1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
    assert_ne!(sun_shadow_map.projection_matrix, identity);
}

/// Test performance with multiple systems
#[test]
fn test_phase3_performance() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // Create all systems
    let mut post_processing = PostProcessingPipeline::new(context.clone(), 800, 600).unwrap();
    let mut shadow_manager = ShadowMappingManager::new(context.clone());
    let mut physics_world = PhysicsWorld::new(PhysicsWorldConfig::default());
    
    // Add multiple effects
    for i in 0..5 {
        let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.0);
        post_processing.add_effect(bloom_config);
    }
    
    // Add multiple shadow maps
    for i in 0..3 {
        let shadow_config = ShadowMapConfig {
            resolution: ShadowMapResolution::Medium,
            near_plane: 0.1,
            far_plane: 100.0,
            bias: 0.005,
            normal_offset: 0.0,
            filtering: ShadowMapFiltering::PCF,
        };
        shadow_manager.add_directional_shadow_map(&format!("light_{}", i), shadow_config).unwrap();
    }
    
    // Add multiple physics bodies
    for i in 0..10 {
        let box_shape = CollisionShape::Box {
            half_extents: [0.5, 0.5, 0.5],
        };
        let mut box_body = RigidBody::new(&format!("box_{}", i), RigidBodyType::Dynamic, box_shape);
        box_body.set_position([i as f32 * 2.0, 10.0, 0.0]);
        physics_world.add_body(box_body).unwrap();
    }
    
    // Simulate multiple frames
    let start_time = std::time::Instant::now();
    
    for _ in 0..60 { // 1 second at 60 FPS
        physics_world.step(1.0 / 60.0).unwrap();
    }
    
    let elapsed = start_time.elapsed();
    
    // Verify performance (should complete in reasonable time)
    assert!(elapsed.as_millis() < 1000); // Less than 1 second
    
    // Verify all systems are still working
    assert_eq!(post_processing.get_effect_count(), 5);
    assert_eq!(shadow_manager.get_total_shadow_map_count(), 3);
    assert_eq!(physics_world.get_body_count(), 10);
}

/// Test error handling across systems
#[test]
fn test_phase3_error_handling() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // Test post-processing errors
    let mut post_processing = PostProcessingPipeline::new(context.clone(), 800, 600).unwrap();
    
    // Try to add effect with duplicate name
    let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.0);
    post_processing.add_effect(bloom_config.clone());
    
    // Add another effect (no error expected)
    post_processing.add_effect(bloom_config);
    
    // Try to remove non-existent effect (by index)
    let result = post_processing.remove_effect(999);
    assert!(result.is_none());
    
    // Test shadow mapping errors
    let mut shadow_manager = ShadowMappingManager::new(context.clone());
    
    // Try to add beyond limit
    shadow_manager.set_max_shadow_maps(1, 1);
    
    let shadow_config = ShadowMapConfig::default();
    shadow_manager.add_directional_shadow_map("light1", shadow_config.clone()).unwrap();
    
    let result = shadow_manager.add_directional_shadow_map("light2", shadow_config);
    assert!(result.is_err());
    
    // Test physics errors
    let mut physics_world = PhysicsWorld::new(PhysicsWorldConfig::default());
    
    let box_shape = CollisionShape::Box {
        half_extents: [0.5, 0.5, 0.5],
    };
    let body = RigidBody::new("test", RigidBodyType::Dynamic, box_shape.clone());
    let body_id = body.id.clone();
    
    physics_world.add_body(body).unwrap();
    
    // Try to add body with duplicate ID
    let body2 = RigidBody::new("test2", RigidBodyType::Dynamic, box_shape);
    let result = physics_world.add_body(body2);
    assert!(result.is_err());
    
    // Try to get non-existent body
    assert!(physics_world.get_body("nonexistent").is_none());
}

/// Test system interaction and data flow
#[test]
fn test_phase3_system_interaction() {
    let canvas = create_test_canvas();
    let context = canvas.get_context("webgl2").unwrap().unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();
    
    // Create systems
    let mut post_processing = PostProcessingPipeline::new(context.clone(), 800, 600).unwrap();
    let mut shadow_manager = ShadowMappingManager::new(context.clone());
    let mut physics_world = PhysicsWorld::new(PhysicsWorldConfig::default());
    
    // Add effects
        let bloom_config = PostProcessingConfig::new(PostProcessingEffect::Bloom, 1.0);
        post_processing.add_effect(bloom_config);
    
    // Add shadow map
    let shadow_config = ShadowMapConfig::default();
    shadow_manager.add_directional_shadow_map("sun", shadow_config).unwrap();
    
    // Add physics bodies
    let ground_shape = CollisionShape::Plane {
        normal: [0.0, 1.0, 0.0],
        distance: 0.0,
    };
    let ground = RigidBody::new("ground", RigidBodyType::Static, ground_shape);
    physics_world.add_body(ground).unwrap();
    
    let box_shape = CollisionShape::Box {
        half_extents: [0.5, 0.5, 0.5],
    };
    let mut box_body = RigidBody::new("box", RigidBodyType::Dynamic, box_shape);
    box_body.set_position([0.0, 10.0, 0.0]);
    physics_world.add_body(box_body).unwrap();
    
    // Create light collection
    let mut light_collection = LightCollection::new();
    light_collection.add_directional_light("sun".to_string(), DirectionalLight {
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
    });
    
    // Create scene bounds
    let scene_bounds = SceneBounds::new([-10.0, -5.0, -10.0], [10.0, 5.0, 10.0]);
    
    // Simulate multiple frames
    for frame in 0..10 {
        // Update physics
        physics_world.step(1.0 / 60.0).unwrap();
        
        // Update shadow maps based on physics positions
        shadow_manager.update_shadow_maps(&light_collection, &scene_bounds);
        
        // Verify systems are working together
        assert_eq!(post_processing.get_effect_count(), 1);
        assert_eq!(shadow_manager.get_total_shadow_map_count(), 1);
        assert_eq!(physics_world.get_body_count(), 2);
        
        // Check that physics body is moving
        let box_body = physics_world.get_body("box").unwrap();
        let expected_y = 10.0 - (frame as f32 + 1.0) * 0.5; // Approximate fall distance
        assert!(box_body.position[1] < expected_y + 1.0); // Allow some tolerance
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
