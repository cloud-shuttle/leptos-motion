//! Tests for model loading system

use crate::model_loader::*;
use crate::error::Result;
use wasm_bindgen_test::*;
use std::str::FromStr;

wasm_bindgen_test_configure!(run_in_browser);

/// Test model format parsing
#[wasm_bindgen_test]
fn test_model_format_parsing() {
    assert_eq!(ModelFormat::from_str("obj").unwrap(), ModelFormat::OBJ);
    assert_eq!(ModelFormat::from_str("OBJ").unwrap(), ModelFormat::OBJ);
    assert_eq!(ModelFormat::from_str("gltf").unwrap(), ModelFormat::GLTF);
    assert_eq!(ModelFormat::from_str("glb").unwrap(), ModelFormat::GLB);
    assert_eq!(ModelFormat::from_str("fbx").unwrap(), ModelFormat::FBX);
    assert_eq!(ModelFormat::from_str("dae").unwrap(), ModelFormat::DAE);
    
    // Test unsupported format
    assert!(ModelFormat::from_str("unsupported").is_err());
}

/// Test model load options default
#[wasm_bindgen_test]
fn test_model_load_options_default() {
    let options = ModelLoadOptions::default();
    
    assert!(options.generate_normals);
    assert!(!options.generate_tangents);
    assert!(!options.flip_y);
    assert_eq!(options.scale, 1.0);
    assert!(!options.center);
    assert!(options.max_vertices.is_none());
    assert!(options.max_indices.is_none());
}

/// Test bounding box creation
#[wasm_bindgen_test]
fn test_bounding_box_creation() {
    let bbox = BoundingBox::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]);
    
    assert_eq!(bbox.min, [-1.0, -1.0, -1.0]);
    assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
    assert_eq!(bbox.center, [0.0, 0.0, 0.0]);
    assert_eq!(bbox.size, [2.0, 2.0, 2.0]);
}

/// Test bounding box operations
#[wasm_bindgen_test]
fn test_bounding_box_operations() {
    let mut bbox = BoundingBox::empty();
    
    // Test expanding with points
    bbox.expand([1.0, 2.0, 3.0]);
    bbox.expand([-1.0, -2.0, -3.0]);
    
    assert_eq!(bbox.min, [-1.0, -2.0, -3.0]);
    assert_eq!(bbox.max, [1.0, 2.0, 3.0]);
    assert_eq!(bbox.center, [0.0, 0.0, 0.0]);
    assert_eq!(bbox.size, [2.0, 4.0, 6.0]);
    
    // Test contains
    assert!(bbox.contains([0.0, 0.0, 0.0]));
    assert!(bbox.contains([1.0, 2.0, 3.0]));
    assert!(bbox.contains([-1.0, -2.0, -3.0]));
    assert!(!bbox.contains([2.0, 0.0, 0.0]));
    
    // Test intersects
    let other_bbox = BoundingBox::new([0.0, 0.0, 0.0], [2.0, 2.0, 2.0]);
    assert!(bbox.intersects(&other_bbox));
    
    let non_intersecting_bbox = BoundingBox::new([5.0, 5.0, 5.0], [6.0, 6.0, 6.0]);
    assert!(!bbox.intersects(&non_intersecting_bbox));
    
    // Test volume
    assert_eq!(bbox.volume(), 2.0 * 4.0 * 6.0);
    
    // Test diagonal length
    let diagonal = bbox.diagonal_length();
    let expected = (4.0_f32 + 16.0 + 36.0).sqrt();
    assert!((diagonal - expected).abs() < 0.001);
}

/// Test bounding box expansion
#[wasm_bindgen_test]
fn test_bounding_box_expansion() {
    let mut bbox1 = BoundingBox::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let bbox2 = BoundingBox::new([-1.0, -1.0, -1.0], [2.0, 2.0, 2.0]);
    
    bbox1.expand_bbox(&bbox2);
    
    assert_eq!(bbox1.min, [-1.0, -1.0, -1.0]);
    assert_eq!(bbox1.max, [2.0, 2.0, 2.0]);
}

/// Test model creation
#[wasm_bindgen_test]
fn test_model_creation() {
    let options = ModelLoadOptions::default();
    let model = Model::new("test_model".to_string(), ModelFormat::OBJ, options.clone());
    
    assert_eq!(model.name, "test_model");
    assert_eq!(model.format, ModelFormat::OBJ);
    assert_eq!(model.load_options.generate_normals, options.generate_normals);
    assert!(model.is_empty());
    assert_eq!(model.get_mesh_count(), 0);
    assert_eq!(model.get_material_count(), 0);
    assert_eq!(model.get_total_vertex_count(), 0);
    assert_eq!(model.get_total_index_count(), 0);
}

/// Test model statistics
#[wasm_bindgen_test]
fn test_model_statistics() {
    let options = ModelLoadOptions::default();
    let mut model = Model::new("test_model".to_string(), ModelFormat::OBJ, options);
    
    // Create a simple mesh
    let mut geometry = crate::geometry::Geometry::new("test");
    geometry.vertices = vec![
        crate::geometry::Vertex::new(),
        crate::geometry::Vertex::new(),
        crate::geometry::Vertex::new(),
    ];
    geometry.set_indices(vec![0, 1, 2]);
    
    let mesh = Mesh {
        name: "test_mesh".to_string(),
        geometry,
        material: None,
        bounding_box: BoundingBox::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]),
    };
    
    model.add_mesh(mesh);
    
    let stats = model.get_stats();
    assert_eq!(stats.mesh_count, 1);
    assert_eq!(stats.material_count, 0);
    assert_eq!(stats.total_vertices, 3);
    assert_eq!(stats.total_indices, 3);
    assert_eq!(stats.bounding_box_volume, 8.0);
}

/// Test OBJ loader with simple triangle
#[wasm_bindgen_test]
fn test_obj_loader_simple_triangle() {
    let obj_data = r#"
# Simple triangle
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0
f 1 2 3
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    assert_eq!(model.get_mesh_count(), 1);
    assert_eq!(model.get_total_vertex_count(), 3);
    assert_eq!(model.get_total_index_count(), 3);
    
    let mesh = model.get_mesh_by_index(0).unwrap();
    assert_eq!(mesh.geometry.vertices.len(), 3);
    assert_eq!(mesh.geometry.get_indices_len(), 3);
    
    // Check that normals were generated
    for vertex in &mesh.geometry.vertices {
        let normal = vertex.get_normal();
        let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
        assert!((length - 1.0).abs() < 0.001); // Should be normalized
    }
}

/// Test OBJ loader with quad (two triangles)
#[wasm_bindgen_test]
fn test_obj_loader_quad() {
    let obj_data = r#"
# Simple quad
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
f 1 2 3
f 1 3 4
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    assert_eq!(model.get_mesh_count(), 1);
    assert_eq!(model.get_total_vertex_count(), 4);
    assert_eq!(model.get_total_index_count(), 6);
}

/// Test OBJ loader with texture coordinates
#[wasm_bindgen_test]
fn test_obj_loader_with_texture_coords() {
    let obj_data = r#"
# Triangle with texture coordinates
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0
vt 0.0 0.0
vt 1.0 0.0
vt 0.5 1.0
f 1/1 2/2 3/3
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    let mesh = model.get_mesh_by_index(0).unwrap();
    
    // Check that texture coordinates were loaded
    for vertex in &mesh.geometry.vertices {
        let tex_coord = vertex.get_tex_coord();
        assert!(tex_coord[0] >= 0.0 && tex_coord[0] <= 1.0);
        assert!(tex_coord[1] >= 0.0 && tex_coord[1] <= 1.0);
    }
}

/// Test OBJ loader with normals
#[wasm_bindgen_test]
fn test_obj_loader_with_normals() {
    let obj_data = r#"
# Triangle with normals
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0
vn 0.0 0.0 1.0
f 1//1 2//1 3//1
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    let mesh = model.get_mesh_by_index(0).unwrap();
    
    // Check that normals were loaded
    for vertex in &mesh.geometry.vertices {
        let normal = vertex.get_normal();
        assert_eq!(normal, [0.0, 0.0, 1.0]);
    }
}

/// Test OBJ loader with scaling
#[wasm_bindgen_test]
fn test_obj_loader_with_scaling() {
    let obj_data = r#"
# Simple triangle
v 1.0 0.0 0.0
v 2.0 0.0 0.0
v 1.5 2.0 0.0
f 1 2 3
"#;
    
    let mut options = ModelLoadOptions::default();
    options.scale = 0.5;
    
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    let mesh = model.get_mesh_by_index(0).unwrap();
    
    // Check that vertices were scaled
    for vertex in &mesh.geometry.vertices {
        let pos = vertex.get_position();
        assert!(pos[0] <= 1.0); // Should be scaled down
        assert!(pos[1] <= 1.0);
    }
}

/// Test OBJ loader with Y flipping
#[wasm_bindgen_test]
fn test_obj_loader_with_y_flipping() {
    let obj_data = r#"
# Simple triangle
v 0.0 1.0 0.0
v 1.0 1.0 0.0
v 0.5 2.0 0.0
f 1 2 3
"#;
    
    let mut options = ModelLoadOptions::default();
    options.flip_y = true;
    
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    let mesh = model.get_mesh_by_index(0).unwrap();
    
    // Check that Y coordinates were flipped
    for vertex in &mesh.geometry.vertices {
        let pos = vertex.get_position();
        assert!(pos[1] <= 0.0); // Should be negative after flipping
    }
}

/// Test OBJ loader with centering
#[wasm_bindgen_test]
fn test_obj_loader_with_centering() {
    let obj_data = r#"
# Triangle offset from origin
v 10.0 10.0 0.0
v 11.0 10.0 0.0
v 10.5 11.0 0.0
f 1 2 3
"#;
    
    let mut options = ModelLoadOptions::default();
    options.center = true;
    
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    let mesh = model.get_mesh_by_index(0).unwrap();
    
    // Check that the model is centered around origin
    let bbox = &mesh.bounding_box;
    let center_x = (bbox.min[0] + bbox.max[0]) * 0.5;
    let center_y = (bbox.min[1] + bbox.max[1]) * 0.5;
    let center_z = (bbox.min[2] + bbox.max[2]) * 0.5;
    
    assert!((center_x.abs() - 0.0).abs() < 0.001);
    assert!((center_y.abs() - 0.0).abs() < 0.001);
    assert!((center_z.abs() - 0.0).abs() < 0.001);
}

/// Test OBJ loader with complex face
#[wasm_bindgen_test]
fn test_obj_loader_complex_face() {
    let obj_data = r#"
# Quad with all vertex data
v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 1.0 1.0 0.0
v 0.0 1.0 0.0
vt 0.0 0.0
vt 1.0 0.0
vt 1.0 1.0
vt 0.0 1.0
vn 0.0 0.0 1.0
f 1/1/1 2/2/1 3/3/1 4/4/1
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    // Should create 2 triangles from the quad
    assert_eq!(model.get_total_index_count(), 6);
}

/// Test OBJ loader with empty data
#[wasm_bindgen_test]
fn test_obj_loader_empty_data() {
    let obj_data = "";
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    assert!(model.is_empty());
}

/// Test OBJ loader with comments and empty lines
#[wasm_bindgen_test]
fn test_obj_loader_with_comments() {
    let obj_data = r#"
# This is a comment
# Another comment

v 0.0 0.0 0.0
v 1.0 0.0 0.0
v 0.5 1.0 0.0

f 1 2 3
"#;
    
    let options = ModelLoadOptions::default();
    let result = ObjLoader::load(obj_data, options);
    assert!(result.is_ok());
    
    let model = result.unwrap();
    assert_eq!(model.get_total_vertex_count(), 3);
    assert_eq!(model.get_total_index_count(), 3);
}

/// Test model loader manager
#[wasm_bindgen_test]
fn test_model_loader_manager() {
    let loader = ModelLoader::new();
    
    assert!(loader.is_format_supported(ModelFormat::OBJ));
    assert!(!loader.is_format_supported(ModelFormat::GLTF));
    
    let supported_formats = loader.get_supported_formats();
    assert_eq!(supported_formats.len(), 1);
    assert_eq!(supported_formats[0], ModelFormat::OBJ);
}

/// Test model loader with unsupported format
#[wasm_bindgen_test]
fn test_model_loader_unsupported_format() {
    let loader = ModelLoader::new();
    let options = ModelLoadOptions::default();
    
    let result = loader.load_from_data("", ModelFormat::GLTF, options);
    assert!(result.is_err());
}

/// Test model mesh operations
#[wasm_bindgen_test]
fn test_model_mesh_operations() {
    let options = ModelLoadOptions::default();
    let mut model = Model::new("test".to_string(), ModelFormat::OBJ, options);
    
    // Create a mesh
    let mut geometry = crate::geometry::Geometry::new("test");
    geometry.vertices = vec![crate::geometry::Vertex::new()];
    geometry.set_indices(vec![0]);
    
    let mesh = Mesh {
        name: "test_mesh".to_string(),
        geometry,
        material: None,
        bounding_box: BoundingBox::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]),
    };
    
    model.add_mesh(mesh);
    
    // Test getting mesh by name
    assert!(model.get_mesh("test_mesh").is_some());
    assert!(model.get_mesh("nonexistent").is_none());
    
    // Test getting mesh by index
    assert!(model.get_mesh_by_index(0).is_some());
    assert!(model.get_mesh_by_index(1).is_none());
}

/// Test model material operations
#[wasm_bindgen_test]
fn test_model_material_operations() {
    let options = ModelLoadOptions::default();
    let mut model = Model::new("test".to_string(), ModelFormat::OBJ, options);
    
    // Create a material
    let material = crate::material::Material::new("test_material", crate::material::MaterialType::Basic);
    
    model.add_material("test_material".to_string(), material);
    
    // Test getting material
    assert!(model.get_material("test_material").is_some());
    assert!(model.get_material("nonexistent").is_none());
    
    assert_eq!(model.get_material_count(), 1);
}
