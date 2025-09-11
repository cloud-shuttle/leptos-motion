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

pub mod camera;
pub mod error;
pub mod geometry;
pub mod lighting;
pub mod material;
pub mod model_loader;
pub mod physics;
pub mod post_processing;
pub mod renderer;
pub mod scene;
pub mod shader;
pub mod shadow_mapping;
pub mod texture;
pub mod utils;

#[cfg(test)]
mod integration_tests;
#[cfg(test)]
mod lighting_tests;
#[cfg(test)]
mod model_loader_tests;
#[cfg(test)]
mod phase3_integration_tests;
#[cfg(test)]
mod physics_tests;
#[cfg(test)]
mod post_processing_tests;
#[cfg(test)]
mod shadow_mapping_tests;
#[cfg(test)]
mod texture_tests;

// Re-export main types for convenience
pub use camera::{Camera, OrthographicCamera, PerspectiveCamera};
pub use error::{Result, WebGLError};
pub use geometry::Geometry;
pub use lighting::{AmbientLight, Color, DirectionalLight, LightManager, PointLight, SpotLight};
pub use material::{BasicMaterial, LambertMaterial, Material, PhongMaterial, StandardMaterial};
pub use model_loader::{BoundingBox, Mesh, Model, ModelFormat, ModelLoadOptions, ModelLoader};
pub use physics::{
    BoundingBox as PhysicsBoundingBox, CollisionShape, PhysicsWorld, PhysicsWorldConfig, RigidBody,
    RigidBodyType,
};
pub use post_processing::{PostProcessingConfig, PostProcessingEffect};
pub use renderer::WebGLRenderer;
pub use scene::{Object3D, Scene};
pub use shadow_mapping::{
    DirectionalShadowMap, PointShadowMap, SceneBounds, ShadowMapConfig, ShadowMapFiltering,
    ShadowMapResolution, ShadowMappingManager,
};
pub use texture::{Texture, TextureConfig, TextureFormat, TextureManager, TextureType};

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
        assert_eq!(
            basic_material.base.material_type,
            material::MaterialType::Basic
        );
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
