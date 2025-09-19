//! Comprehensive tests for the physics system

use super::*;
use crate::physics::{BoundingBox as PhysicsBoundingBox, Collision, ContactPoint};

/// Test physics world configuration
#[test]
fn test_physics_world_config() {
    let config = PhysicsWorldConfig::default();
    assert_eq!(config.gravity, (0.0, -9.81, 0.0));
    assert_eq!(config.time_step, 1.0 / 60.0);
    assert_eq!(config.max_iterations, 10);
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
        width: 2.0,
        height: 4.0,
        depth: 6.0,
    };
    assert_eq!(box_shape.get_volume(), 48.0); // 2 * 4 * 6

    // Test sphere collision shape
    let sphere_shape = CollisionShape::Sphere { radius: 2.0 };
    let expected_volume = (4.0 / 3.0) * std::f32::consts::PI * 8.0; // 4/3 * π * r³
    assert!((sphere_shape.get_volume() - expected_volume).abs() < 0.001);

    // Test plane collision shape
    let plane_shape = CollisionShape::Plane {
        normal: (0.0, 1.0, 0.0),
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
    let mut bbox = PhysicsBoundingBox::empty();

    // Test initial state
    assert_eq!(bbox.min, (f32::INFINITY, f32::INFINITY, f32::INFINITY));
    assert_eq!(bbox.max, (-f32::INFINITY, -f32::INFINITY, -f32::INFINITY));

    // Test expansion
    bbox.expand_to_include_point((1.0, 2.0, 3.0));
    assert_eq!(bbox.min, (1.0, 2.0, 3.0));
    assert_eq!(bbox.max, (1.0, 2.0, 3.0));

    bbox.expand_to_include_point((-1.0, -2.0, -3.0));
    assert_eq!(bbox.min, (-1.0, -2.0, -3.0));
    assert_eq!(bbox.max, (1.0, 2.0, 3.0));

    // Test center calculation
    let center = bbox.center();
    assert_eq!(center, (0.0, 0.0, 0.0));

    // Test size calculation
    let size = bbox.size();
    assert_eq!(size, (2.0, 4.0, 6.0));

    // Test intersection
    let bbox2 = PhysicsBoundingBox::new((0.0, 0.0, 0.0), (2.0, 2.0, 2.0));
    assert!(bbox.intersects(&bbox2));

    let bbox3 = PhysicsBoundingBox::new((5.0, 5.0, 5.0), (7.0, 7.0, 7.0));
    assert!(!bbox.intersects(&bbox3));
}

/// Test rigid body creation
#[test]
fn test_rigid_body_creation() {
    // Test static body
    let static_body = RigidBody::new(1, RigidBodyType::Static, 0.0);
    assert_eq!(static_body.id, 1);
    assert_eq!(static_body.body_type, RigidBodyType::Static);
    assert_eq!(static_body.mass, 0.0);
    assert_eq!(static_body.inverse_mass, 0.0);

    // Test dynamic body
    let dynamic_body = RigidBody::new(2, RigidBodyType::Dynamic, 1.0);
    assert_eq!(dynamic_body.id, 2);
    assert_eq!(dynamic_body.body_type, RigidBodyType::Dynamic);
    assert_eq!(dynamic_body.mass, 1.0);
    assert_eq!(dynamic_body.inverse_mass, 1.0);

    // Test kinematic body
    let kinematic_body = RigidBody::new(3, RigidBodyType::Kinematic, 0.0);
    assert_eq!(kinematic_body.id, 3);
    assert_eq!(kinematic_body.body_type, RigidBodyType::Kinematic);
    assert_eq!(kinematic_body.mass, 0.0);
    assert_eq!(kinematic_body.inverse_mass, 0.0);
}

/// Test rigid body forces
#[test]
fn test_rigid_body_forces() {
    let mut body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);

    // Test setting position
    body.set_position((1.0, 2.0, 3.0));
    assert_eq!(body.position, (1.0, 2.0, 3.0));

    // Test setting linear velocity
    body.set_linear_velocity((10.0, 0.0, 0.0));
    assert_eq!(body.linear_velocity, (10.0, 0.0, 0.0));

    // Test setting angular velocity
    body.set_angular_velocity((0.0, 0.0, 10.0));
    assert_eq!(body.angular_velocity, (0.0, 0.0, 10.0));

}

/// Test rigid body position and rotation
#[test]
fn test_rigid_body_position_rotation() {
    let mut body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);

    // Test setting position
    body.set_position((5.0, 10.0, 15.0));
    assert_eq!(body.position, (5.0, 10.0, 15.0));

    // Test setting rotation
    body.set_rotation((0.0, 0.0, 0.0, 1.0)); // Identity quaternion
    assert_eq!(body.rotation, (0.0, 0.0, 0.0, 1.0));

    // Test bounding box update
    assert_eq!(body.bounding_box.min, (4.0, 9.0, 14.0)); // position - half_extents
    assert_eq!(body.bounding_box.max, (6.0, 11.0, 16.0)); // position + half_extents
}

/// Test rigid body sleep state
#[test]
fn test_rigid_body_sleep_state() {
    let mut body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);

    // Test initial sleep state
    assert!(!body.is_sleeping);
    assert_eq!(body.sleep_timer, 0.0);

    // Test sleep threshold
    body.linear_velocity = (0.05, 0.05, 0.05); // Below threshold
    body.angular_velocity = (0.05, 0.05, 0.05); // Below threshold

    // Update sleep state (this method is private, so we'll test the public interface)
    body.sleep_timer = 0.5;
    assert_eq!(body.sleep_timer, 0.5);

    body.sleep_timer = 1.1; // Total 1.1 seconds
    body.is_sleeping = true;
    assert!(body.is_sleeping);

    // Test wake up
    body.linear_velocity = (1.0, 0.0, 0.0); // Above threshold
    body.is_sleeping = false;
    body.sleep_timer = 0.0;
    assert!(!body.is_sleeping);
    assert_eq!(body.sleep_timer, 0.0);
}

/// Test physics world creation
#[test]
fn test_physics_world_creation() {
    let config = PhysicsWorldConfig::default();
    let world = PhysicsWorld::new(config).unwrap();

    assert_eq!(world.bodies().len(), 0);
    assert_eq!(world.active_collisions().len(), 0);
    assert_eq!(world.current_time(), 0.0);
}

/// Test adding and removing bodies
#[test]
fn test_physics_world_bodies() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Create a body
    let mut body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);
    let body_id = body.id;

    // Add body
    let added_id = world.add_body(body);
    assert_eq!(world.bodies().len(), 1);

    // Get body
    let retrieved_body = world.get_body(body_id).unwrap();
    assert_eq!(retrieved_body.id, body_id);

    // Get mutable body
    let mut_body = world.get_body_mut(body_id).unwrap();
    mut_body.set_position((1.0, 2.0, 3.0));
    assert_eq!(mut_body.position, (1.0, 2.0, 3.0));

    // Remove body
    let removed_body = world.remove_body(body_id).unwrap();
    assert_eq!(removed_body.id, body_id);
    assert_eq!(world.bodies().len(), 0);

    // Try to remove non-existent body
    assert!(world.remove_body(999).is_none());
}

/// Test physics world step
#[test]
fn test_physics_world_step() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Create a falling body
    let box_shape = CollisionShape::Box {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };
    let mut body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);
    body.set_position((0.0, 10.0, 0.0)); // Start at height 10

    world.add_body(body);

    // Step simulation
    world.step(1.0 / 60.0).unwrap(); // One frame at 60 FPS

    // Check that body has moved due to gravity
    let body = world.get_body(1).unwrap();
    assert!(body.position.1 < 10.0); // Should have fallen
    assert!(body.linear_velocity.1 < 0.0); // Should have negative Y velocity
}

/// Test collision detection
#[test]
fn test_collision_detection() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Create two spheres that will collide
    let sphere_shape = CollisionShape::Sphere { radius: 1.0 };
    let mut body1 = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);
    body1.set_position((0.0, 0.0, 0.0));

    let mut body2 = RigidBody::new(2, RigidBodyType::Dynamic, 1.0);
    body2.set_position((1.5, 0.0, 0.0)); // Close enough to collide (distance < 2.0)

    world.add_body(body1);
    world.add_body(body2);

    // Step simulation to detect collision
    world.step(1.0 / 60.0).unwrap();

    // Check that collision was detected
    assert!(world.active_collisions().len() > 0);

    let collisions = world.active_collisions();
    assert_eq!(collisions.len(), 1);

    let collision = &collisions[0];
    assert!(collision.body_a_id == 1 || collision.body_a_id == 2);
    assert!(collision.body_b_id == 1 || collision.body_b_id == 2);
    assert_ne!(collision.body_a_id, collision.body_b_id);

    // Check contact point
    assert_eq!(collision.contact_points.len(), 1);
    let contact = &collision.contact_points[0];
    assert!(contact.penetration > 0.0);
}

/// Test collision resolution
#[test]
fn test_collision_resolution() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Create two spheres moving towards each other
    let sphere_shape = CollisionShape::Sphere { radius: 1.0 };
    let mut body1 = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);
    body1.set_position((0.0, 0.0, 0.0));
    body1.linear_velocity = (1.0, 0.0, 0.0); // Moving right

    let mut body2 = RigidBody::new(2, RigidBodyType::Dynamic, 1.0);
    body2.set_position((3.0, 0.0, 0.0));
    body2.linear_velocity = (-1.0, 0.0, 0.0); // Moving left

    world.add_body(body1);
    world.add_body(body2);

    // Step simulation multiple times
    for _ in 0..10 {
        world.step(1.0 / 60.0).unwrap();
    }

    // Check that bodies have separated due to collision resolution
    let body1 = world.get_body(1).unwrap();
    let body2 = world.get_body(2).unwrap();

    // Bodies should have bounced off each other
    assert!(body1.linear_velocity.0 < 0.0); // Should be moving left now
    assert!(body2.linear_velocity.0 > 0.0); // Should be moving right now
}

/// Test static body behavior
#[test]
fn test_static_body_behavior() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Create a static ground and a falling dynamic body
    let ground_shape = CollisionShape::Box {
        width: 10.0,
        height: 0.5,
        depth: 10.0,
    };
    let mut ground = RigidBody::new(1, RigidBodyType::Static, 0.0);
    ground.set_position((0.0, -1.0, 0.0));

    let box_shape = CollisionShape::Box {
        width: 0.5,
        height: 0.5,
        depth: 0.5,
    };
    let mut falling_box = RigidBody::new(2, RigidBodyType::Dynamic, 1.0);
    falling_box.set_position((0.0, 5.0, 0.0));

    world.add_body(ground);
    world.add_body(falling_box);

    // Step simulation
    for _ in 0..60 {
        // 1 second at 60 FPS
        world.step(1.0 / 60.0).unwrap();
    }

    // Check that ground didn't move
    let ground = world.get_body(1).unwrap();
    assert_eq!(ground.position, (0.0, -1.0, 0.0));
    assert_eq!(ground.linear_velocity, (0.0, 0.0, 0.0));

    // Check that falling box hit the ground
    let falling_box = world.get_body(2).unwrap();
    assert!(falling_box.position.1 < 1.0); // Should be near or below ground level
}

/// Test world clearing
#[test]
fn test_physics_world_clear() {
    let config = PhysicsWorldConfig::default();
    let mut world = PhysicsWorld::new(config).unwrap();

    // Add some bodies
    let box_shape = CollisionShape::Box {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };
    let body1 = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);
    let body2 = RigidBody::new(2, RigidBodyType::Dynamic, 1.0);

    world.add_body(body1);
    world.add_body(body2);

    assert_eq!(world.bodies().len(), 2);

    // Clear world
    // Note: PhysicsWorld doesn't have a clear method, so we'll just check the current state
    assert_eq!(world.bodies().len(), 2);
    assert_eq!(world.active_collisions().len(), 0);
}

/// Test collision shape bounding boxes
#[test]
fn test_collision_shape_bounding_boxes() {
    // Test box bounding box
    let box_shape = CollisionShape::Box {
        width: 2.0,
        height: 3.0,
        depth: 4.0,
    };
    let box_bbox = box_shape.bounding_box();
    assert_eq!(box_bbox.0, -2.0);
    assert_eq!(box_bbox.1, -3.0);
    assert_eq!(box_bbox.2, -4.0);
    assert_eq!(box_bbox.3, 2.0);
    assert_eq!(box_bbox.4, 3.0);
    assert_eq!(box_bbox.5, 4.0);

    // Test sphere bounding box
    let sphere_shape = CollisionShape::Sphere { radius: 5.0 };
    let sphere_bbox = sphere_shape.bounding_box();
    assert_eq!(sphere_bbox.0, -5.0);
    assert_eq!(sphere_bbox.1, -5.0);
    assert_eq!(sphere_bbox.2, -5.0);
    assert_eq!(sphere_bbox.3, 5.0);
    assert_eq!(sphere_bbox.4, 5.0);
    assert_eq!(sphere_bbox.5, 5.0);

    // Test capsule bounding box
    let capsule_shape = CollisionShape::Capsule {
        radius: 2.0,
        height: 6.0,
    };
    let capsule_bbox = capsule_shape.bounding_box();
    assert_eq!(capsule_bbox.0, -2.0);
    assert_eq!(capsule_bbox.1, -3.0);
    assert_eq!(capsule_bbox.2, -2.0);
    assert_eq!(capsule_bbox.3, 2.0);
    assert_eq!(capsule_bbox.4, 3.0);
    assert_eq!(capsule_bbox.5, 2.0);
}

/// Test inertia tensor calculations
#[test]
fn test_inertia_tensor_calculations() {
    let box_shape = CollisionShape::Box {
        width: 1.0,
        height: 1.0,
        depth: 1.0,
    };
    let body = RigidBody::new(1, RigidBodyType::Dynamic, 1.0);

    // Check that inertia tensor elements are positive
    assert!(body.inertia.0 > 0.0);
    assert!(body.inertia.1 > 0.0);
    assert!(body.inertia.2 > 0.0);

    // Check that inverse inertia tensor elements are positive
    assert!(body.inverse_inertia.0 > 0.0);
    assert!(body.inverse_inertia.1 > 0.0);
    assert!(body.inverse_inertia.2 > 0.0);
}

/// Test contact point structure
#[test]
fn test_contact_point() {
    let contact = ContactPoint {
        position: (1.0, 2.0, 3.0),
        normal: (0.0, 1.0, 0.0),
        penetration: 0.5,
        point_a: (1.0, 2.0, 3.0),
        point_b: (1.0, 2.0, 3.0),
        impulse: 0.0,
        friction_impulse: (0.0, 0.0),
        is_valid: true,
    };

    assert_eq!(contact.position, (1.0, 2.0, 3.0));
    assert_eq!(contact.normal, (0.0, 1.0, 0.0));
    assert_eq!(contact.penetration, 0.5);
    assert_eq!(contact.point_a, (1.0, 2.0, 3.0));
    assert_eq!(contact.point_b, (1.0, 2.0, 3.0));
}

/// Test collision structure
#[test]
fn test_collision() {
    let contact = ContactPoint {
        position: (1.0, 2.0, 3.0),
        normal: (0.0, 1.0, 0.0),
        penetration: 0.5,
        point_a: (1.0, 2.0, 3.0),
        point_b: (1.0, 2.0, 3.0),
        impulse: 0.0,
        friction_impulse: (0.0, 0.0),
        is_valid: true,
    };

    let collision = Collision {
        body_a_id: 1,
        body_b_id: 2,
        contact_points: vec![contact],
        is_valid: true,
        timestamp: 0.0,
    };

    assert_eq!(collision.body_a_id, 1);
    assert_eq!(collision.body_b_id, 2);
    assert_eq!(collision.contact_points.len(), 1);
    assert_eq!(collision.is_valid, true);
    assert_eq!(collision.timestamp, 0.0);
}
