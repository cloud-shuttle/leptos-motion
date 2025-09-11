//! Comprehensive tests for the physics system

use super::*;
use crate::physics::{BoundingBox as PhysicsBoundingBox, Collision, ContactPoint};

/// Test physics world configuration
#[test]
fn test_physics_world_config() {
    let config = PhysicsWorldConfig::default();
    assert_eq!(config.gravity, [0.0, -9.81, 0.0]);
    assert_eq!(config.time_step, 1.0 / 60.0);
    assert_eq!(config.max_iterations, 10);
    assert_eq!(config.collision_tolerance, 0.001);
    assert!(config.continuous_collision_detection);
}

/// Test rigid body types
#[test]
fn test_rigid_body_types() {
    assert_eq!(RigidBodyType::Static, RigidBodyType::Static);
    assert_eq!(RigidBodyType::Dynamic, RigidBodyType::Dynamic);
    assert_eq!(RigidBodyType::Kinematic, RigidBodyType::Kinematic);

    assert_ne!(RigidBodyType::Static, RigidBodyType::Dynamic);
    assert_ne!(RigidBodyType::Dynamic, RigidBodyType::Kinematic);
    assert_ne!(RigidBodyType::Static, RigidBodyType::Kinematic);
}

/// Test collision shapes
#[test]
fn test_collision_shapes() {
    // Test box collision shape
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 2.0, 3.0],
    };
    assert_eq!(box_shape.get_volume(), 48.0); // 8 * 1 * 2 * 3

    // Test sphere collision shape
    let sphere_shape = CollisionShape::Sphere { radius: 2.0 };
    let expected_volume = (4.0 / 3.0) * std::f32::consts::PI * 8.0; // 4/3 * π * r³
    assert!((sphere_shape.get_volume() - expected_volume).abs() < 0.001);

    // Test plane collision shape
    let plane_shape = CollisionShape::Plane {
        normal: [0.0, 1.0, 0.0],
        distance: 0.0,
    };
    assert_eq!(plane_shape.get_volume(), 0.0);

    // Test capsule collision shape
    let capsule_shape = CollisionShape::Capsule {
        radius: 1.0,
        height: 2.0,
    };
    let sphere_volume = (4.0 / 3.0) * std::f32::consts::PI;
    let cylinder_volume = std::f32::consts::PI * 2.0;
    let expected_volume = sphere_volume + cylinder_volume;
    assert!((capsule_shape.get_volume() - expected_volume).abs() < 0.001);

    // Test cylinder collision shape
    let cylinder_shape = CollisionShape::Cylinder {
        half_extents: [1.0, 2.0, 1.0],
    };
    let expected_volume = std::f32::consts::PI * 4.0; // π * r² * h
    assert!((cylinder_shape.get_volume() - expected_volume).abs() < 0.001);
}

/// Test bounding box
#[test]
fn test_bounding_box() {
    let mut bbox = PhysicsBoundingBox::new();

    // Test initial state
    assert_eq!(bbox.min, [f32::INFINITY, f32::INFINITY, f32::INFINITY]);
    assert_eq!(bbox.max, [-f32::INFINITY, -f32::INFINITY, -f32::INFINITY]);

    // Test expansion
    bbox.expand([1.0, 2.0, 3.0]);
    assert_eq!(bbox.min, [1.0, 2.0, 3.0]);
    assert_eq!(bbox.max, [1.0, 2.0, 3.0]);

    bbox.expand([-1.0, -2.0, -3.0]);
    assert_eq!(bbox.min, [-1.0, -2.0, -3.0]);
    assert_eq!(bbox.max, [1.0, 2.0, 3.0]);

    // Test center calculation
    let center = bbox.get_center();
    assert_eq!(center, [0.0, 0.0, 0.0]);

    // Test size calculation
    let size = bbox.get_size();
    assert_eq!(size, [2.0, 4.0, 6.0]);

    // Test intersection
    let bbox2 = PhysicsBoundingBox::from_min_max([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
    assert!(bbox.intersects(&bbox2));

    let bbox3 = PhysicsBoundingBox::from_min_max([5.0, 5.0, 5.0], [7.0, 7.0, 7.0]);
    assert!(!bbox.intersects(&bbox3));
}

/// Test rigid body creation
#[test]
fn test_rigid_body_creation() {
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };

    // Test static body
    let static_body = RigidBody::new("static_box", RigidBodyType::Static, box_shape.clone());
    assert_eq!(static_body.name, "static_box");
    assert_eq!(static_body.body_type, RigidBodyType::Static);
    assert_eq!(static_body.mass, 0.0);
    assert_eq!(static_body.inverse_mass, 0.0);
    assert!(!static_body.id.is_empty());

    // Test dynamic body
    let dynamic_body = RigidBody::new("dynamic_box", RigidBodyType::Dynamic, box_shape.clone());
    assert_eq!(dynamic_body.name, "dynamic_box");
    assert_eq!(dynamic_body.body_type, RigidBodyType::Dynamic);
    assert_eq!(dynamic_body.mass, 1.0);
    assert_eq!(dynamic_body.inverse_mass, 1.0);

    // Test kinematic body
    let kinematic_body = RigidBody::new("kinematic_box", RigidBodyType::Kinematic, box_shape);
    assert_eq!(kinematic_body.name, "kinematic_box");
    assert_eq!(kinematic_body.body_type, RigidBodyType::Kinematic);
    assert_eq!(kinematic_body.mass, 0.0);
    assert_eq!(kinematic_body.inverse_mass, 0.0);
}

/// Test rigid body forces
#[test]
fn test_rigid_body_forces() {
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let mut body = RigidBody::new("test_body", RigidBodyType::Dynamic, box_shape);

    // Test applying force
    body.apply_force([10.0, 0.0, 0.0]);
    assert_eq!(body.forces, [10.0, 0.0, 0.0]);

    // Test applying force at point
    body.apply_force_at_point([0.0, 10.0, 0.0], [1.0, 0.0, 0.0]);
    assert_eq!(body.forces, [10.0, 10.0, 0.0]);
    assert_eq!(body.torques, [0.0, 0.0, 10.0]); // Cross product of [1,0,0] and [0,10,0]

    // Test applying torque
    body.apply_torque([0.0, 0.0, 5.0]);
    assert_eq!(body.torques, [0.0, 0.0, 15.0]);
}

/// Test rigid body position and rotation
#[test]
fn test_rigid_body_position_rotation() {
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let mut body = RigidBody::new("test_body", RigidBodyType::Dynamic, box_shape);

    // Test setting position
    body.set_position([5.0, 10.0, 15.0]);
    assert_eq!(body.position, [5.0, 10.0, 15.0]);

    // Test setting rotation
    body.set_rotation([0.0, 0.0, 0.0, 1.0]); // Identity quaternion
    assert_eq!(body.rotation, [0.0, 0.0, 0.0, 1.0]);

    // Test bounding box update
    assert_eq!(body.bounding_box.min, [4.0, 9.0, 14.0]); // position - half_extents
    assert_eq!(body.bounding_box.max, [6.0, 11.0, 16.0]); // position + half_extents
}

/// Test rigid body sleep state
#[test]
fn test_rigid_body_sleep_state() {
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let mut body = RigidBody::new("test_body", RigidBodyType::Dynamic, box_shape);

    // Test initial sleep state
    assert!(!body.is_sleeping);
    assert_eq!(body.sleep_time, 0.0);

    // Test sleep threshold
    body.linear_velocity = [0.05, 0.05, 0.05]; // Below threshold
    body.angular_velocity = [0.05, 0.05, 0.05]; // Below threshold

    // Update sleep state
    body.update_sleep_state(0.5);
    assert_eq!(body.sleep_time, 0.5);

    body.update_sleep_state(0.6); // Total 1.1 seconds
    assert!(body.is_sleeping);

    // Test wake up
    body.linear_velocity = [1.0, 0.0, 0.0]; // Above threshold
    body.update_sleep_state(0.1);
    assert!(!body.is_sleeping);
    assert_eq!(body.sleep_time, 0.0);
}

/// Test physics world creation
#[test]
fn test_physics_world_creation() {
    let config = PhysicsWorldConfig::default();
    let world = PhysicsWorld::new(config);

    assert_eq!(world.get_body_count(), 0);
    assert_eq!(world.get_collision_count(), 0);
    assert_eq!(world.accumulated_time, 0.0);
}

/// Test adding and removing bodies
#[test]
fn test_physics_world_bodies() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Create a body
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let body = RigidBody::new("test_body", RigidBodyType::Dynamic, box_shape);
    let body_id = body.id.clone();

    // Add body
    world.add_body(body).unwrap();
    assert_eq!(world.get_body_count(), 1);

    // Get body
    let retrieved_body = world.get_body(&body_id).unwrap();
    assert_eq!(retrieved_body.name, "test_body");

    // Get mutable body
    let mut_body = world.get_body_mut(&body_id).unwrap();
    mut_body.set_position([1.0, 2.0, 3.0]);
    assert_eq!(mut_body.position, [1.0, 2.0, 3.0]);

    // Remove body
    let removed_body = world.remove_body(&body_id).unwrap();
    assert_eq!(removed_body.name, "test_body");
    assert_eq!(world.get_body_count(), 0);

    // Try to remove non-existent body
    assert!(world.remove_body("nonexistent").is_none());
}

/// Test physics world step
#[test]
fn test_physics_world_step() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Create a falling body
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let mut body = RigidBody::new("falling_body", RigidBodyType::Dynamic, box_shape);
    body.set_position([0.0, 10.0, 0.0]); // Start at height 10

    world.add_body(body).unwrap();

    // Step simulation
    world.step(1.0 / 60.0).unwrap(); // One frame at 60 FPS

    // Check that body has moved due to gravity
    let body = world.get_body("falling_body").unwrap();
    assert!(body.position[1] < 10.0); // Should have fallen
    assert!(body.linear_velocity[1] < 0.0); // Should have negative Y velocity
}

/// Test collision detection
#[test]
fn test_collision_detection() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Create two spheres that will collide
    let sphere_shape = CollisionShape::Sphere { radius: 1.0 };
    let mut body1 = RigidBody::new("sphere1", RigidBodyType::Dynamic, sphere_shape.clone());
    body1.set_position([0.0, 0.0, 0.0]);

    let mut body2 = RigidBody::new("sphere2", RigidBodyType::Dynamic, sphere_shape);
    body2.set_position([1.5, 0.0, 0.0]); // Close enough to collide (distance < 2.0)

    world.add_body(body1).unwrap();
    world.add_body(body2).unwrap();

    // Step simulation to detect collision
    world.step(1.0 / 60.0).unwrap();

    // Check that collision was detected
    assert!(world.get_collision_count() > 0);

    let collisions = world.get_collisions();
    assert_eq!(collisions.len(), 1);

    let collision = &collisions[0];
    assert!(collision.body_a == "sphere1" || collision.body_a == "sphere2");
    assert!(collision.body_b == "sphere1" || collision.body_b == "sphere2");
    assert_ne!(collision.body_a, collision.body_b);

    // Check contact point
    assert_eq!(collision.contacts.len(), 1);
    let contact = &collision.contacts[0];
    assert!(contact.penetration > 0.0);
}

/// Test collision resolution
#[test]
fn test_collision_resolution() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Create two spheres moving towards each other
    let sphere_shape = CollisionShape::Sphere { radius: 1.0 };
    let mut body1 = RigidBody::new("sphere1", RigidBodyType::Dynamic, sphere_shape.clone());
    body1.set_position([0.0, 0.0, 0.0]);
    body1.linear_velocity = [1.0, 0.0, 0.0]; // Moving right

    let mut body2 = RigidBody::new("sphere2", RigidBodyType::Dynamic, sphere_shape);
    body2.set_position([3.0, 0.0, 0.0]);
    body2.linear_velocity = [-1.0, 0.0, 0.0]; // Moving left

    world.add_body(body1).unwrap();
    world.add_body(body2).unwrap();

    // Step simulation multiple times
    for _ in 0..10 {
        world.step(1.0 / 60.0).unwrap();
    }

    // Check that bodies have separated due to collision resolution
    let body1 = world.get_body("sphere1").unwrap();
    let body2 = world.get_body("sphere2").unwrap();

    // Bodies should have bounced off each other
    assert!(body1.linear_velocity[0] < 0.0); // Should be moving left now
    assert!(body2.linear_velocity[0] > 0.0); // Should be moving right now
}

/// Test static body behavior
#[test]
fn test_static_body_behavior() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Create a static ground and a falling dynamic body
    let ground_shape = CollisionShape::Box {
        half_extents: [10.0, 0.5, 10.0],
    };
    let mut ground = RigidBody::new("ground", RigidBodyType::Static, ground_shape);
    ground.set_position([0.0, -1.0, 0.0]);

    let box_shape = CollisionShape::Box {
        half_extents: [0.5, 0.5, 0.5],
    };
    let mut falling_box = RigidBody::new("falling_box", RigidBodyType::Dynamic, box_shape);
    falling_box.set_position([0.0, 5.0, 0.0]);

    world.add_body(ground).unwrap();
    world.add_body(falling_box).unwrap();

    // Step simulation
    for _ in 0..60 {
        // 1 second at 60 FPS
        world.step(1.0 / 60.0).unwrap();
    }

    // Check that ground didn't move
    let ground = world.get_body("ground").unwrap();
    assert_eq!(ground.position, [0.0, -1.0, 0.0]);
    assert_eq!(ground.linear_velocity, [0.0, 0.0, 0.0]);

    // Check that falling box hit the ground
    let falling_box = world.get_body("falling_box").unwrap();
    assert!(falling_box.position[1] < 1.0); // Should be near or below ground level
}

/// Test world clearing
#[test]
fn test_physics_world_clear() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config);

    // Add some bodies
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let body1 = RigidBody::new("body1", RigidBodyType::Dynamic, box_shape.clone());
    let body2 = RigidBody::new("body2", RigidBodyType::Dynamic, box_shape);

    world.add_body(body1).unwrap();
    world.add_body(body2).unwrap();

    assert_eq!(world.get_body_count(), 2);

    // Clear world
    world.clear();

    assert_eq!(world.get_body_count(), 0);
    assert_eq!(world.get_collision_count(), 0);
}

/// Test collision shape bounding boxes
#[test]
fn test_collision_shape_bounding_boxes() {
    // Test box bounding box
    let box_shape = CollisionShape::Box {
        half_extents: [2.0, 3.0, 4.0],
    };
    let box_bbox = box_shape.get_bounding_box();
    assert_eq!(box_bbox.min, [-2.0, -3.0, -4.0]);
    assert_eq!(box_bbox.max, [2.0, 3.0, 4.0]);

    // Test sphere bounding box
    let sphere_shape = CollisionShape::Sphere { radius: 5.0 };
    let sphere_bbox = sphere_shape.get_bounding_box();
    assert_eq!(sphere_bbox.min, [-5.0, -5.0, -5.0]);
    assert_eq!(sphere_bbox.max, [5.0, 5.0, 5.0]);

    // Test capsule bounding box
    let capsule_shape = CollisionShape::Capsule {
        radius: 2.0,
        height: 6.0,
    };
    let capsule_bbox = capsule_shape.get_bounding_box();
    assert_eq!(capsule_bbox.min, [-2.0, -3.0, -2.0]);
    assert_eq!(capsule_bbox.max, [2.0, 3.0, 2.0]);
}

/// Test inertia tensor calculations
#[test]
fn test_inertia_tensor_calculations() {
    let box_shape = CollisionShape::Box {
        half_extents: [1.0, 1.0, 1.0],
    };
    let body = RigidBody::new("test_body", RigidBodyType::Dynamic, box_shape);

    // Check that inertia tensor is diagonal for a box
    assert!(body.inertia[1] == 0.0 && body.inertia[2] == 0.0);
    assert!(body.inertia[3] == 0.0 && body.inertia[5] == 0.0);
    assert!(body.inertia[6] == 0.0 && body.inertia[7] == 0.0);

    // Check that diagonal elements are positive
    assert!(body.inertia[0] > 0.0);
    assert!(body.inertia[4] > 0.0);
    assert!(body.inertia[8] > 0.0);

    // Check that inverse inertia tensor is also diagonal
    assert!(body.inverse_inertia[1] == 0.0 && body.inverse_inertia[2] == 0.0);
    assert!(body.inverse_inertia[3] == 0.0 && body.inverse_inertia[5] == 0.0);
    assert!(body.inverse_inertia[6] == 0.0 && body.inverse_inertia[7] == 0.0);
}

/// Test contact point structure
#[test]
fn test_contact_point() {
    let contact = ContactPoint {
        position: [1.0, 2.0, 3.0],
        normal: [0.0, 1.0, 0.0],
        penetration: 0.5,
        point_a: [1.0, 2.0, 3.0],
        point_b: [1.0, 2.0, 3.0],
    };

    assert_eq!(contact.position, [1.0, 2.0, 3.0]);
    assert_eq!(contact.normal, [0.0, 1.0, 0.0]);
    assert_eq!(contact.penetration, 0.5);
    assert_eq!(contact.point_a, [1.0, 2.0, 3.0]);
    assert_eq!(contact.point_b, [1.0, 2.0, 3.0]);
}

/// Test collision structure
#[test]
fn test_collision() {
    let contact = ContactPoint {
        position: [1.0, 2.0, 3.0],
        normal: [0.0, 1.0, 0.0],
        penetration: 0.5,
        point_a: [1.0, 2.0, 3.0],
        point_b: [1.0, 2.0, 3.0],
    };

    let collision = Collision {
        body_a: "body1".to_string(),
        body_b: "body2".to_string(),
        contacts: vec![contact],
        friction: 0.5,
        restitution: 0.3,
    };

    assert_eq!(collision.body_a, "body1");
    assert_eq!(collision.body_b, "body2");
    assert_eq!(collision.contacts.len(), 1);
    assert_eq!(collision.friction, 0.5);
    assert_eq!(collision.restitution, 0.3);
}
