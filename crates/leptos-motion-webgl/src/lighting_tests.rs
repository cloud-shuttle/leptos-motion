//! Tests for lighting system

use crate::error::Result;
use crate::lighting::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Test color creation and methods
#[wasm_bindgen_test]
fn test_color_creation() {
    // Test basic color creation
    let color = Color::new(1.0, 0.5, 0.0, 1.0);
    assert_eq!(color.r, 1.0);
    assert_eq!(color.g, 0.5);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 1.0);

    // Test from RGB
    let color = Color::from_rgb(255, 128, 0);
    assert_eq!(color.r, 1.0);
    assert_eq!(color.g, 128.0 / 255.0);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 1.0);

    // Test from RGBA
    let color = Color::from_rgba(255, 128, 0, 200);
    assert_eq!(color.r, 1.0);
    assert_eq!(color.g, 128.0 / 255.0);
    assert_eq!(color.b, 0.0);
    assert_eq!(color.a, 200.0 / 255.0);

    // Test predefined colors
    assert_eq!(Color::white(), Color::new(1.0, 1.0, 1.0, 1.0));
    assert_eq!(Color::black(), Color::new(0.0, 0.0, 0.0, 1.0));
    assert_eq!(Color::red(), Color::new(1.0, 0.0, 0.0, 1.0));
    assert_eq!(Color::green(), Color::new(0.0, 1.0, 0.0, 1.0));
    assert_eq!(Color::blue(), Color::new(0.0, 0.0, 1.0, 1.0));
}

/// Test color array conversion
#[wasm_bindgen_test]
fn test_color_array_conversion() {
    let color = Color::new(1.0, 0.5, 0.25, 0.8);

    let array = color.as_array();
    assert_eq!(array, [1.0, 0.5, 0.25, 0.8]);

    let rgb_array = color.as_rgb_array();
    assert_eq!(rgb_array, [1.0, 0.5, 0.25]);
}

/// Test ambient light creation
#[wasm_bindgen_test]
fn test_ambient_light_creation() {
    let light = AmbientLight::new("ambient1", Color::white(), 0.5);

    assert_eq!(light.light.name, "ambient1");
    assert_eq!(light.light.light_type, LightType::Ambient);
    assert_eq!(light.light.color, Color::white());
    assert_eq!(light.light.intensity, 0.5);
    assert!(light.light.enabled);
}

/// Test directional light creation
#[wasm_bindgen_test]
fn test_directional_light_creation() {
    let direction = [0.0, -1.0, 0.0];
    let light = DirectionalLight::new("sun", Color::white(), 1.0, direction);

    assert_eq!(light.light.name, "sun");
    assert_eq!(light.light.light_type, LightType::Directional);
    assert_eq!(light.light.color, Color::white());
    assert_eq!(light.light.intensity, 1.0);
    assert!(light.light.enabled);

    // Direction should be normalized
    let expected_direction = [0.0, -1.0, 0.0];
    assert_eq!(light.direction, expected_direction);
}

/// Test directional light direction operations
#[wasm_bindgen_test]
fn test_directional_light_direction() {
    let mut light = DirectionalLight::new("test", Color::white(), 1.0, [3.0, 4.0, 0.0]);

    // Test getting direction
    let direction = light.get_direction();
    let expected = [0.6, 0.8, 0.0]; // Normalized [3, 4, 0]
    assert!((direction[0] - expected[0]).abs() < 0.001);
    assert!((direction[1] - expected[1]).abs() < 0.001);
    assert!((direction[2] - expected[2]).abs() < 0.001);

    // Test setting direction
    light.set_direction([1.0, 1.0, 1.0]);
    let new_direction = light.get_direction();
    let expected_normalized = [0.577, 0.577, 0.577]; // Normalized [1, 1, 1]
    assert!((new_direction[0] - expected_normalized[0]).abs() < 0.001);
    assert!((new_direction[1] - expected_normalized[1]).abs() < 0.001);
    assert!((new_direction[2] - expected_normalized[2]).abs() < 0.001);
}

/// Test directional light shadow operations
#[wasm_bindgen_test]
fn test_directional_light_shadow() {
    let mut light = DirectionalLight::new("test", Color::white(), 1.0, [0.0, -1.0, 0.0]);

    assert!(!light.is_casting_shadow());

    light.enable_shadow();
    assert!(light.is_casting_shadow());

    light.disable_shadow();
    assert!(!light.is_casting_shadow());
}

/// Test point light creation
#[wasm_bindgen_test]
fn test_point_light_creation() {
    let position = [0.0, 5.0, 0.0];
    let light = PointLight::new("bulb", Color::white(), 1.0, position);

    assert_eq!(light.light.name, "bulb");
    assert_eq!(light.light.light_type, LightType::Point);
    assert_eq!(light.light.color, Color::white());
    assert_eq!(light.light.intensity, 1.0);
    assert!(light.light.enabled);

    assert_eq!(light.position, position);
    assert_eq!(light.range, 100.0);
    assert_eq!(light.constant_attenuation, 1.0);
    assert_eq!(light.linear_attenuation, 0.09);
    assert_eq!(light.quadratic_attenuation, 0.032);
}

/// Test point light position operations
#[wasm_bindgen_test]
fn test_point_light_position() {
    let mut light = PointLight::new("test", Color::white(), 1.0, [0.0, 0.0, 0.0]);

    // Test getting position
    assert_eq!(light.get_position(), [0.0, 0.0, 0.0]);

    // Test setting position
    light.set_position([1.0, 2.0, 3.0]);
    assert_eq!(light.get_position(), [1.0, 2.0, 3.0]);
}

/// Test point light range operations
#[wasm_bindgen_test]
fn test_point_light_range() {
    let mut light = PointLight::new("test", Color::white(), 1.0, [0.0, 0.0, 0.0]);

    // Test getting range
    assert_eq!(light.get_range(), 100.0);

    // Test setting range
    light.set_range(50.0);
    assert_eq!(light.get_range(), 50.0);

    // Test negative range (should be clamped to 0)
    light.set_range(-10.0);
    assert_eq!(light.get_range(), 0.0);
}

/// Test point light attenuation
#[wasm_bindgen_test]
fn test_point_light_attenuation() {
    let mut light = PointLight::new("test", Color::white(), 1.0, [0.0, 0.0, 0.0]);

    // Test setting attenuation
    light.set_attenuation(1.0, 0.1, 0.01);
    assert_eq!(light.constant_attenuation, 1.0);
    assert_eq!(light.linear_attenuation, 0.1);
    assert_eq!(light.quadratic_attenuation, 0.01);

    // Test negative attenuation (should be clamped to 0)
    light.set_attenuation(-1.0, -0.1, -0.01);
    assert_eq!(light.constant_attenuation, 0.0);
    assert_eq!(light.linear_attenuation, 0.0);
    assert_eq!(light.quadratic_attenuation, 0.0);
}

/// Test point light attenuation calculation
#[wasm_bindgen_test]
fn test_point_light_attenuation_calculation() {
    let mut light = PointLight::new("test", Color::white(), 1.0, [0.0, 0.0, 0.0]);
    light.set_attenuation(1.0, 0.1, 0.01);
    light.set_range(100.0);

    // Test attenuation at distance 0
    let attenuation = light.calculate_attenuation(0.0);
    assert_eq!(attenuation, 1.0);

    // Test attenuation at distance 10
    let attenuation = light.calculate_attenuation(10.0);
    let expected = 1.0 / (1.0 + 0.1 * 10.0 + 0.01 * 100.0);
    assert!((attenuation - expected).abs() < 0.001);

    // Test attenuation beyond range
    let attenuation = light.calculate_attenuation(150.0);
    assert_eq!(attenuation, 0.0);
}

/// Test point light shadow operations
#[wasm_bindgen_test]
fn test_point_light_shadow() {
    let mut light = PointLight::new("test", Color::white(), 1.0, [0.0, 0.0, 0.0]);

    assert!(!light.is_casting_shadow());

    light.enable_shadow();
    assert!(light.is_casting_shadow());

    light.disable_shadow();
    assert!(!light.is_casting_shadow());
}

/// Test spot light creation
#[wasm_bindgen_test]
fn test_spot_light_creation() {
    let position = [0.0, 5.0, 0.0];
    let direction = [0.0, -1.0, 0.0];
    let inner_angle = 0.5;
    let outer_angle = 1.0;

    let light = SpotLight::new(
        "flashlight",
        Color::white(),
        1.0,
        position,
        direction,
        inner_angle,
        outer_angle,
    );

    assert_eq!(light.light.name, "flashlight");
    assert_eq!(light.light.light_type, LightType::Spot);
    assert_eq!(light.light.color, Color::white());
    assert_eq!(light.light.intensity, 1.0);
    assert!(light.light.enabled);

    assert_eq!(light.position, position);
    assert_eq!(light.direction, direction);
    assert_eq!(light.inner_cone_angle, inner_angle);
    assert_eq!(light.outer_cone_angle, outer_angle);
    assert_eq!(light.range, 100.0);
}

/// Test spot light position operations
#[wasm_bindgen_test]
fn test_spot_light_position() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    // Test getting position
    assert_eq!(light.get_position(), [0.0, 0.0, 0.0]);

    // Test setting position
    light.set_position([1.0, 2.0, 3.0]);
    assert_eq!(light.get_position(), [1.0, 2.0, 3.0]);
}

/// Test spot light direction operations
#[wasm_bindgen_test]
fn test_spot_light_direction() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [3.0, 4.0, 0.0],
        0.5,
        1.0,
    );

    // Test getting direction (should be normalized)
    let direction = light.get_direction();
    let expected = [0.6, 0.8, 0.0]; // Normalized [3, 4, 0]
    assert!((direction[0] - expected[0]).abs() < 0.001);
    assert!((direction[1] - expected[1]).abs() < 0.001);
    assert!((direction[2] - expected[2]).abs() < 0.001);

    // Test setting direction
    light.set_direction([1.0, 1.0, 1.0]);
    let new_direction = light.get_direction();
    let expected_normalized = [0.577, 0.577, 0.577]; // Normalized [1, 1, 1]
    assert!((new_direction[0] - expected_normalized[0]).abs() < 0.001);
    assert!((new_direction[1] - expected_normalized[1]).abs() < 0.001);
    assert!((new_direction[2] - expected_normalized[2]).abs() < 0.001);
}

/// Test spot light cone angles
#[wasm_bindgen_test]
fn test_spot_light_cone_angles() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    // Test getting cone angles
    let (inner, outer) = light.get_cone_angles();
    assert_eq!(inner, 0.5);
    assert_eq!(outer, 1.0);

    // Test setting cone angles
    light.set_cone_angles(0.3, 0.8);
    let (inner, outer) = light.get_cone_angles();
    assert_eq!(inner, 0.3);
    assert_eq!(outer, 0.8);

    // Test clamping (outer should be at least inner)
    light.set_cone_angles(0.8, 0.3);
    let (inner, outer) = light.get_cone_angles();
    assert_eq!(inner, 0.8);
    assert_eq!(outer, 0.8); // Clamped to inner
}

/// Test spot light range operations
#[wasm_bindgen_test]
fn test_spot_light_range() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    // Test getting range
    assert_eq!(light.get_range(), 100.0);

    // Test setting range
    light.set_range(50.0);
    assert_eq!(light.get_range(), 50.0);

    // Test negative range (should be clamped to 0)
    light.set_range(-10.0);
    assert_eq!(light.get_range(), 0.0);
}

/// Test spot light attenuation
#[wasm_bindgen_test]
fn test_spot_light_attenuation() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    // Test setting attenuation
    light.set_attenuation(1.0, 0.1, 0.01);
    assert_eq!(light.constant_attenuation, 1.0);
    assert_eq!(light.linear_attenuation, 0.1);
    assert_eq!(light.quadratic_attenuation, 0.01);

    // Test negative attenuation (should be clamped to 0)
    light.set_attenuation(-1.0, -0.1, -0.01);
    assert_eq!(light.constant_attenuation, 0.0);
    assert_eq!(light.linear_attenuation, 0.0);
    assert_eq!(light.quadratic_attenuation, 0.0);
}

/// Test spot light spot factor calculation
#[wasm_bindgen_test]
fn test_spot_light_spot_factor() {
    let light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    // Test spot factor in inner cone (should be 1.0)
    let light_direction = [0.0, 1.0, 0.0]; // Opposite to light direction
    let spot_factor = light.calculate_spot_factor(light_direction);
    assert_eq!(spot_factor, 1.0);

    // Test spot factor outside outer cone (should be 0.0)
    let light_direction = [1.0, 0.0, 0.0]; // Perpendicular to light direction
    let spot_factor = light.calculate_spot_factor(light_direction);
    assert_eq!(spot_factor, 0.0);
}

/// Test spot light attenuation calculation
#[wasm_bindgen_test]
fn test_spot_light_attenuation_calculation() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );
    light.set_attenuation(1.0, 0.1, 0.01);
    light.set_range(100.0);

    // Test attenuation at distance 0
    let attenuation = light.calculate_attenuation(0.0);
    assert_eq!(attenuation, 1.0);

    // Test attenuation at distance 10
    let attenuation = light.calculate_attenuation(10.0);
    let expected = 1.0 / (1.0 + 0.1 * 10.0 + 0.01 * 100.0);
    assert!((attenuation - expected).abs() < 0.001);

    // Test attenuation beyond range
    let attenuation = light.calculate_attenuation(150.0);
    assert_eq!(attenuation, 0.0);
}

/// Test spot light shadow operations
#[wasm_bindgen_test]
fn test_spot_light_shadow() {
    let mut light = SpotLight::new(
        "test",
        Color::white(),
        1.0,
        [0.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        0.5,
        1.0,
    );

    assert!(!light.is_casting_shadow());

    light.enable_shadow();
    assert!(light.is_casting_shadow());

    light.disable_shadow();
    assert!(!light.is_casting_shadow());
}

/// Test light manager creation
#[wasm_bindgen_test]
fn test_light_manager_creation() {
    let manager = LightManager::new();

    assert_eq!(manager.get_total_light_count(), 0);
    assert_eq!(manager.get_enabled_light_count(), 0);

    let (max_ambient, max_directional, max_point, max_spot) = manager.get_max_lights();
    assert_eq!(max_ambient, 8);
    assert_eq!(max_directional, 4);
    assert_eq!(max_point, 16);
    assert_eq!(max_spot, 8);
}

/// Test light manager operations
#[wasm_bindgen_test]
fn test_light_manager_operations() {
    let mut manager = LightManager::new();

    // Test adding lights
    manager
        .add_ambient_light("ambient1", Color::white(), 0.3)
        .unwrap();
    manager
        .add_directional_light("sun", Color::white(), 1.0, [0.0, -1.0, 0.0])
        .unwrap();
    manager
        .add_point_light("bulb", Color::white(), 1.0, [0.0, 5.0, 0.0])
        .unwrap();
    manager
        .add_spot_light(
            "flashlight",
            Color::white(),
            1.0,
            [0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
            0.5,
            1.0,
        )
        .unwrap();

    assert_eq!(manager.get_total_light_count(), 4);
    assert_eq!(manager.get_enabled_light_count(), 4);

    // Test getting lights
    assert!(manager.get_ambient_light("ambient1").is_some());
    assert!(manager.get_directional_light("sun").is_some());
    assert!(manager.get_point_light("bulb").is_some());
    assert!(manager.get_spot_light("flashlight").is_some());

    // Test getting non-existent light
    assert!(manager.get_ambient_light("nonexistent").is_none());

    // Test removing lights
    let removed = manager.remove_ambient_light("ambient1");
    assert!(removed.is_some());
    assert_eq!(manager.get_total_light_count(), 3);

    // Test clearing all lights
    manager.clear();
    assert_eq!(manager.get_total_light_count(), 0);
}

/// Test light manager maximum limits
#[wasm_bindgen_test]
fn test_light_manager_limits() {
    let mut manager = LightManager::new();

    // Test maximum ambient lights
    for i in 0..8 {
        let result = manager.add_ambient_light(&format!("ambient{}", i), Color::white(), 0.1);
        assert!(result.is_ok());
    }

    // Adding one more should fail
    let result = manager.add_ambient_light("ambient9", Color::white(), 0.1);
    assert!(result.is_err());

    // Test setting custom limits
    manager.set_max_lights(4, 2, 8, 4);
    let (max_ambient, max_directional, max_point, max_spot) = manager.get_max_lights();
    assert_eq!(max_ambient, 4);
    assert_eq!(max_directional, 2);
    assert_eq!(max_point, 8);
    assert_eq!(max_spot, 4);
}

/// Test light enabling/disabling
#[wasm_bindgen_test]
fn test_light_enabling_disabling() {
    let mut manager = LightManager::new();

    manager
        .add_ambient_light("ambient1", Color::white(), 0.3)
        .unwrap();
    manager
        .add_directional_light("sun", Color::white(), 1.0, [0.0, -1.0, 0.0])
        .unwrap();

    assert_eq!(manager.get_enabled_light_count(), 2);

    // Disable a light
    if let Some(light) = manager.get_ambient_light_mut("ambient1") {
        light.light.disable();
    }

    assert_eq!(manager.get_enabled_light_count(), 1);

    // Re-enable the light
    if let Some(light) = manager.get_ambient_light_mut("ambient1") {
        light.light.enable();
    }

    assert_eq!(manager.get_enabled_light_count(), 2);
}

/// Test light intensity modification
#[wasm_bindgen_test]
fn test_light_intensity_modification() {
    let mut manager = LightManager::new();

    manager
        .add_point_light("bulb", Color::white(), 1.0, [0.0, 0.0, 0.0])
        .unwrap();

    // Test getting and modifying intensity
    if let Some(light) = manager.get_point_light_mut("bulb") {
        assert_eq!(light.light.intensity, 1.0);

        light.light.set_intensity(2.0);
        assert_eq!(light.light.intensity, 2.0);

        // Test negative intensity (should be clamped to 0)
        light.light.set_intensity(-1.0);
        assert_eq!(light.light.intensity, 0.0);
    }
}

/// Test light color modification
#[wasm_bindgen_test]
fn test_light_color_modification() {
    let mut manager = LightManager::new();

    manager
        .add_spot_light(
            "flashlight",
            Color::white(),
            1.0,
            [0.0, 0.0, 0.0],
            [0.0, -1.0, 0.0],
            0.5,
            1.0,
        )
        .unwrap();

    // Test getting and modifying color
    if let Some(light) = manager.get_spot_light_mut("flashlight") {
        assert_eq!(light.light.color, Color::white());

        light.light.set_color(Color::red());
        assert_eq!(light.light.color, Color::red());
    }
}
