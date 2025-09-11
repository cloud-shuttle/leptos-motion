//! # Leptos Motion WebGL
//!
//! A high-performance WebGL rendering engine for Leptos Motion, providing
//! full 3D graphics capabilities with type safety and reactive integration.
//!
//! ## Features
//!
//! - **WebGL2 Rendering**: Modern WebGL2 context with full feature support
//! - **Type Safety**: Rust's type system ensures compile-time safety for 3D operations
//! - **Reactive Integration**: Native Leptos signal integration for dynamic scenes
//! - **Performance**: Optimized rendering pipeline with minimal overhead
//! - **Scene Graph**: Hierarchical object management with efficient traversal
//! - **Camera System**: Multiple camera types with controls and animations
//! - **Material System**: Flexible material system with lighting support
//! - **Geometry Management**: Efficient vertex buffer management and instancing
//!
//! ## Quick Start
//!
//! ```rust
//! use leptos_motion_webgl::{WebGLRenderer, Scene, PerspectiveCamera};
//! use leptos::prelude::*;
//!
//! #[component]
//! fn WebGLDemo() -> impl IntoView {
//!     let canvas_ref = NodeRef::<leptos::html::Canvas>::new();
//!     
//!     create_effect(move |_| {
//!         if let Some(canvas) = canvas_ref.get() {
//!             let mut renderer = WebGLRenderer::new(canvas).unwrap();
//!             let scene = Scene::new();
//!             let mut camera = PerspectiveCamera::new(75.0, 1.0, 0.1, 1000.0);
//!             
//!             renderer.render(&scene, &mut camera);
//!         }
//!     });
//!     
//!     view! {
//!         <canvas node_ref=canvas_ref width="800" height="600"></canvas>
//!     }
//! }
//! ```

pub mod renderer;
pub mod scene;
pub mod camera;
pub mod geometry;
pub mod material;
pub mod shader;
pub mod utils;
pub mod error;
pub mod texture;
pub mod lighting;
pub mod model_loader;
pub mod post_processing;
pub mod shadow_mapping;
pub mod physics;

#[cfg(test)]
mod texture_tests;
#[cfg(test)]
mod lighting_tests;
#[cfg(test)]
mod model_loader_tests;
#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod post_processing_tests;
#[cfg(test)]
mod shadow_mapping_tests;
#[cfg(test)]
mod physics_tests;
#[cfg(test)]
mod phase3_integration_tests;

// Re-export main types for convenience
pub use renderer::WebGLRenderer;
pub use scene::{Scene, Object3D};
pub use camera::{Camera, PerspectiveCamera, OrthographicCamera};
pub use geometry::Geometry;
pub use material::{Material, BasicMaterial, LambertMaterial, PhongMaterial, StandardMaterial};
pub use error::{WebGLError, Result};
pub use texture::{Texture, TextureManager, TextureConfig, TextureFormat, TextureType};
pub use lighting::{LightManager, AmbientLight, DirectionalLight, PointLight, SpotLight, Color};
pub use model_loader::{ModelLoader, Model, Mesh, BoundingBox, ModelFormat, ModelLoadOptions};
pub use post_processing::{PostProcessingConfig, PostProcessingEffect};
pub use shadow_mapping::{ShadowMappingManager, DirectionalShadowMap, PointShadowMap, ShadowMapConfig, ShadowMapResolution, ShadowMapFiltering, SceneBounds};
pub use physics::{PhysicsWorld, RigidBody, CollisionShape, PhysicsWorldConfig, RigidBodyType, BoundingBox as PhysicsBoundingBox};

/// WebGL version information
pub const WEBGL_VERSION: &str = "2.0";

/// Maximum number of lights supported
pub const MAX_LIGHTS: usize = 8;

/// Maximum number of textures per material
pub const MAX_TEXTURES: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_creation() {
        let box_geometry = Geometry::create_box(1.0, 1.0, 1.0);
        assert_eq!(box_geometry.name, "BoxGeometry");
        assert!(box_geometry.vertex_count > 0);
        assert!(box_geometry.index_count > 0);
    }

    #[test]
    fn test_sphere_geometry_creation() {
        let sphere_geometry = Geometry::create_sphere(1.0, 8, 6);
        assert_eq!(sphere_geometry.name, "SphereGeometry");
        assert!(sphere_geometry.vertex_count > 0);
        assert!(sphere_geometry.index_count > 0);
    }

    #[test]
    fn test_plane_geometry_creation() {
        let plane_geometry = Geometry::create_plane(1.0, 1.0, 1, 1);
        assert_eq!(plane_geometry.name, "PlaneGeometry");
        assert!(plane_geometry.vertex_count > 0);
        assert!(plane_geometry.index_count > 0);
    }

    #[test]
    fn test_material_creation() {
        let basic_material = BasicMaterial::new("TestMaterial");
        assert_eq!(basic_material.base.name, "TestMaterial");
        assert_eq!(basic_material.base.material_type, material::MaterialType::Basic);
    }

    #[test]
    fn test_camera_creation() {
        let camera = PerspectiveCamera::new(75.0, 1.0, 0.1, 1000.0);
        assert_eq!(camera.base.name, "PerspectiveCamera");
        assert_eq!(camera.fov, 75.0);
        assert_eq!(camera.aspect, 1.0);
    }

    #[test]
    fn test_scene_creation() {
        let scene = Scene::new();
        assert_eq!(scene.name, "Scene");
        assert_eq!(scene.root.name, "root");
    }

    #[test]
    fn test_object3d_creation() {
        let object = Object3D::new("TestObject");
        assert_eq!(object.name, "TestObject");
        assert_eq!(object.position, [0.0, 0.0, 0.0]);
        assert_eq!(object.scale, [1.0, 1.0, 1.0]);
    }
}