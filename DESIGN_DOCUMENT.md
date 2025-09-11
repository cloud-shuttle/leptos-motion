# 🎨 Leptos Motion Design Document: Three.js Feature Integration

## 📋 **Document Overview**

This design document outlines the architecture and implementation strategy for integrating Three.js-level 3D capabilities into the Leptos Motion library while maintaining our competitive advantages of type safety, performance, and reactive integration.

---

## 🎯 **Design Principles**

### **Core Principles**
```rust
1. 🛡️ Type Safety First: All 3D operations must be type-safe
2. ⚡ Performance Optimized: Maintain 60+ FPS performance
3. 🔄 Reactive Integration: Native Leptos signal integration
4. 📦 Minimal Bundle Size: Keep bundle size under 200KB
5. 🧠 Memory Safe: Zero-cost abstractions and memory safety
6. 🎨 Developer Experience: Intuitive, ergonomic API design
7. 🔧 Extensible: Plugin architecture for future enhancements
8. 🌐 Cross-Platform: Web, mobile, and desktop compatibility
```

### **Architecture Goals**
```rust
✅ Maintain existing CSS-based 3D animations
✅ Add WebGL rendering as an optional enhancement
✅ Provide seamless migration path
✅ Keep API consistent and intuitive
✅ Ensure backward compatibility
✅ Optimize for both performance and developer experience
```

---

## 🏗️ **System Architecture**

### **High-Level Architecture**
```rust
┌─────────────────────────────────────────────────────────────┐
│                    Leptos Motion v1.0                      │
├─────────────────────────────────────────────────────────────┤
│  🎨 Animation Layer (Existing + Enhanced)                  │
│  ├── CSS 3D Transforms (Current)                           │
│  ├── WebGL 3D Rendering (New)                              │
│  └── Hybrid Mode (CSS + WebGL)                             │
├─────────────────────────────────────────────────────────────┤
│  🔧 Core Engine Layer                                      │
│  ├── Scene Graph Management                                │
│  ├── Renderer Abstraction                                  │
│  ├── Camera System                                         │
│  ├── Geometry System                                       │
│  ├── Material System                                       │
│  └── Animation System                                      │
├─────────────────────────────────────────────────────────────┤
│  🎯 Integration Layer                                      │
│  ├── Leptos Signal Integration                             │
│  ├── Component System                                      │
│  ├── Gesture Handling                                      │
│  └── Event System                                          │
├─────────────────────────────────────────────────────────────┤
│  🚀 Platform Layer                                         │
│  ├── WebGL/WebGPU Backend                                 │
│  ├── WASM Optimizations                                    │
│  ├── Memory Management                                     │
│  └── Performance Monitoring                                │
└─────────────────────────────────────────────────────────────┘
```

### **Component Architecture**
```rust
// New crate structure:
leptos-motion-core/          // Core animation engine (existing)
leptos-motion-dom/           // DOM integration (existing)
leptos-motion-webgl/         // WebGL rendering engine (new)
leptos-motion-geometry/      // 3D geometry system (new)
leptos-motion-materials/     // Material and shader system (new)
leptos-motion-lighting/      // Lighting system (new)
leptos-motion-loaders/       // 3D model loaders (new)
leptos-motion-physics/       // Physics integration (new)
leptos-motion-vr/            // VR/AR support (new)
leptos-motion-gestures/      // Gesture handling (existing)
leptos-motion-layout/        // Layout animations (existing)
leptos-motion-scroll/        // Scroll animations (existing)
leptos-motion-macros/        // Compile-time optimizations (existing)
```

---

## 🎨 **Core Systems Design**

### **1. Scene Graph System** 🌳
```rust
// Type-safe scene graph with reactive updates
#[derive(Debug, Clone)]
pub struct Scene {
    pub objects: Vec<Object3D>,
    pub cameras: Vec<Camera>,
    pub lights: Vec<Light>,
    pub background: Option<Background>,
    pub fog: Option<Fog>,
}

#[derive(Debug, Clone)]
pub struct Object3D {
    pub id: Uuid,
    pub name: String,
    pub transform: Transform3D,
    pub geometry: Option<Geometry>,
    pub material: Option<Material>,
    pub children: Vec<Object3D>,
    pub parent: Option<Uuid>,
    pub visible: bool,
    pub cast_shadow: bool,
    pub receive_shadow: bool,
}

// Reactive scene updates
impl Scene {
    pub fn add_object(&mut self, object: Object3D) -> Uuid {
        let id = object.id;
        self.objects.push(object);
        id
    }
    
    pub fn remove_object(&mut self, id: Uuid) -> Option<Object3D> {
        self.objects.iter().position(|o| o.id == id)
            .map(|i| self.objects.remove(i))
    }
    
    pub fn update_object_transform(&mut self, id: Uuid, transform: Transform3D) {
        if let Some(object) = self.objects.iter_mut().find(|o| o.id == id) {
            object.transform = transform;
        }
    }
}
```

### **2. Renderer System** 🎨
```rust
// Abstract renderer trait for multiple backends
pub trait Renderer {
    fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), RenderError>;
    fn set_size(&mut self, width: u32, height: u32);
    fn set_pixel_ratio(&mut self, ratio: f64);
    fn dispose(&mut self);
}

// WebGL renderer implementation
pub struct WebGLRenderer {
    context: WebGL2RenderingContext,
    canvas: HtmlCanvasElement,
    shadow_map: ShadowMap,
    post_processing: PostProcessingPipeline,
}

impl Renderer for WebGLRenderer {
    fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), RenderError> {
        // 1. Update shadow maps
        self.update_shadow_maps(scene)?;
        
        // 2. Render scene to frame buffer
        self.render_scene(scene, camera)?;
        
        // 3. Apply post-processing effects
        self.apply_post_processing()?;
        
        // 4. Present to screen
        self.present()?;
        
        Ok(())
    }
}

// CSS renderer for fallback
pub struct CSSRenderer {
    container: HtmlElement,
    objects: HashMap<Uuid, HtmlElement>,
}

impl Renderer for CSSRenderer {
    fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), RenderError> {
        // Render using CSS transforms
        for object in &scene.objects {
            if let Some(element) = self.objects.get(&object.id) {
                self.apply_css_transform(element, &object.transform);
            }
        }
        Ok(())
    }
}
```

### **3. Camera System** 📷
```rust
// Type-safe camera system
#[derive(Debug, Clone)]
pub struct Camera {
    pub id: Uuid,
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub up: Vector3,
    pub near: f64,
    pub far: f64,
    pub camera_type: CameraType,
}

#[derive(Debug, Clone)]
pub enum CameraType {
    Perspective {
        fov: f64,
        aspect: f64,
    },
    Orthographic {
        left: f64,
        right: f64,
        top: f64,
        bottom: f64,
    },
}

impl Camera {
    pub fn new_perspective(fov: f64, aspect: f64, near: f64, far: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Perspective Camera".to_string(),
            position: Vector3::new(0.0, 0.0, 5.0),
            rotation: Quaternion::identity(),
            up: Vector3::new(0.0, 1.0, 0.0),
            near,
            far,
            camera_type: CameraType::Perspective { fov, aspect },
        }
    }
    
    pub fn get_projection_matrix(&self) -> Matrix4 {
        match &self.camera_type {
            CameraType::Perspective { fov, aspect } => {
                Matrix4::perspective(*fov, *aspect, self.near, self.far)
            }
            CameraType::Orthographic { left, right, top, bottom } => {
                Matrix4::orthographic(*left, *right, *top, *bottom, self.near, self.far)
            }
        }
    }
    
    pub fn get_view_matrix(&self) -> Matrix4 {
        Matrix4::look_at(self.position, self.position + self.forward(), self.up)
    }
}
```

### **4. Geometry System** 🔺
```rust
// Type-safe geometry system
#[derive(Debug, Clone)]
pub struct Geometry {
    pub id: Uuid,
    pub name: String,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub uvs: Vec<Vector2>,
    pub indices: Vec<u32>,
    pub bounding_box: BoundingBox,
    pub geometry_type: GeometryType,
}

#[derive(Debug, Clone)]
pub enum GeometryType {
    Box { width: f64, height: f64, depth: f64 },
    Sphere { radius: f64, width_segments: u32, height_segments: u32 },
    Plane { width: f64, height: f64, width_segments: u32, height_segments: u32 },
    Cylinder { radius_top: f64, radius_bottom: f64, height: f64, radial_segments: u32 },
    Torus { radius: f64, tube: f64, radial_segments: u32, tubular_segments: u32 },
    Custom { vertices: Vec<Vector3>, indices: Vec<u32> },
}

impl Geometry {
    pub fn new_box(width: f64, height: f64, depth: f64) -> Self {
        let (vertices, normals, uvs, indices) = generate_box_geometry(width, height, depth);
        let bounding_box = BoundingBox::from_vertices(&vertices);
        
        Self {
            id: Uuid::new_v4(),
            name: "Box".to_string(),
            vertices,
            normals,
            uvs,
            indices,
            bounding_box,
            geometry_type: GeometryType::Box { width, height, depth },
        }
    }
    
    pub fn new_sphere(radius: f64, width_segments: u32, height_segments: u32) -> Self {
        let (vertices, normals, uvs, indices) = generate_sphere_geometry(radius, width_segments, height_segments);
        let bounding_box = BoundingBox::from_vertices(&vertices);
        
        Self {
            id: Uuid::new_v4(),
            name: "Sphere".to_string(),
            vertices,
            normals,
            uvs,
            indices,
            bounding_box,
            geometry_type: GeometryType::Sphere { radius, width_segments, height_segments },
        }
    }
}
```

### **5. Material System** 🎨
```rust
// Type-safe material system
#[derive(Debug, Clone)]
pub struct Material {
    pub id: Uuid,
    pub name: String,
    pub material_type: MaterialType,
    pub transparent: bool,
    pub opacity: f64,
    pub side: MaterialSide,
    pub depth_test: bool,
    pub depth_write: bool,
    pub blending: BlendingMode,
}

#[derive(Debug, Clone)]
pub enum MaterialType {
    Basic {
        color: Color,
        wireframe: bool,
    },
    Lambert {
        color: Color,
        emissive: Color,
        emissive_intensity: f64,
    },
    Phong {
        color: Color,
        emissive: Color,
        emissive_intensity: f64,
        specular: Color,
        shininess: f64,
    },
    Standard {
        color: Color,
        metalness: f64,
        roughness: f64,
        emissive: Color,
        emissive_intensity: f64,
        normal_map: Option<Texture>,
        ao_map: Option<Texture>,
        emissive_map: Option<Texture>,
        metalness_map: Option<Texture>,
        roughness_map: Option<Texture>,
    },
    Custom {
        vertex_shader: String,
        fragment_shader: String,
        uniforms: HashMap<String, UniformValue>,
    },
}

impl Material {
    pub fn new_basic(color: Color) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Basic Material".to_string(),
            material_type: MaterialType::Basic { color, wireframe: false },
            transparent: false,
            opacity: 1.0,
            side: MaterialSide::Front,
            depth_test: true,
            depth_write: true,
            blending: BlendingMode::Normal,
        }
    }
    
    pub fn new_standard(color: Color, metalness: f64, roughness: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Standard Material".to_string(),
            material_type: MaterialType::Standard {
                color,
                metalness,
                roughness,
                emissive: Color::black(),
                emissive_intensity: 0.0,
                normal_map: None,
                ao_map: None,
                emissive_map: None,
                metalness_map: None,
                roughness_map: None,
            },
            transparent: false,
            opacity: 1.0,
            side: MaterialSide::Front,
            depth_test: true,
            depth_write: true,
            blending: BlendingMode::Normal,
        }
    }
}
```

### **6. Lighting System** 💡
```rust
// Type-safe lighting system
#[derive(Debug, Clone)]
pub struct Light {
    pub id: Uuid,
    pub name: String,
    pub position: Vector3,
    pub rotation: Quaternion,
    pub color: Color,
    pub intensity: f64,
    pub light_type: LightType,
    pub cast_shadow: bool,
    pub shadow_map_size: u32,
}

#[derive(Debug, Clone)]
pub enum LightType {
    Ambient,
    Directional {
        direction: Vector3,
    },
    Point {
        distance: f64,
        decay: f64,
    },
    Spot {
        direction: Vector3,
        angle: f64,
        penumbra: f64,
        distance: f64,
        decay: f64,
    },
}

impl Light {
    pub fn new_ambient(color: Color, intensity: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Ambient Light".to_string(),
            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            color,
            intensity,
            light_type: LightType::Ambient,
            cast_shadow: false,
            shadow_map_size: 1024,
        }
    }
    
    pub fn new_directional(color: Color, intensity: f64, direction: Vector3) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Directional Light".to_string(),
            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            color,
            intensity,
            light_type: LightType::Directional { direction },
            cast_shadow: true,
            shadow_map_size: 2048,
        }
    }
}
```

---

## 🔄 **Reactive Integration Design**

### **Leptos Signal Integration**
```rust
// Reactive 3D scene with Leptos signals
#[component]
pub fn Reactive3DScene(
    #[prop(optional)] initial_scene: Option<Scene>,
    #[prop(optional)] camera: Option<Camera>,
    #[prop(optional)] renderer_type: Option<RendererType>,
    children: Children,
) -> impl IntoView {
    let (scene, set_scene) = create_signal(initial_scene.unwrap_or_else(Scene::new));
    let (camera, set_camera) = create_signal(camera.unwrap_or_else(|| Camera::new_perspective(75.0, 1.0, 0.1, 1000.0)));
    let (renderer, set_renderer) = create_signal(renderer_type.unwrap_or(RendererType::WebGL));
    
    // Reactive scene updates
    let update_scene = move |updater: impl FnOnce(&mut Scene)| {
        set_scene.update(|scene| updater(scene));
    };
    
    // Render loop
    let render_loop = move || {
        // This would be called by the renderer
        // scene.get() and camera.get() provide reactive updates
    };
    
    view! {
        <div class="leptos-motion-3d-scene">
            <canvas
                node_ref=create_node_ref::<HtmlCanvasElement>()
                style="width: 100%; height: 100%;"
            />
            {children()}
        </div>
    }
}

// Reactive 3D object component
#[component]
pub fn Reactive3DObject(
    #[prop(optional)] geometry: Option<Geometry>,
    #[prop(optional)] material: Option<Material>,
    #[prop(optional)] position: Option<Vector3>,
    #[prop(optional)] rotation: Option<Quaternion>,
    #[prop(optional)] scale: Option<Vector3>,
    children: Children,
) -> impl IntoView {
    let (transform, set_transform) = create_signal(Transform3D::new());
    
    // Update transform when props change
    Effect::new(move |_| {
        let mut new_transform = Transform3D::new();
        if let Some(pos) = position {
            new_transform = new_transform.translate_x(pos.x).translate_y(pos.y).translate_z(pos.z);
        }
        if let Some(rot) = rotation {
            new_transform = new_transform.rotate_x(rot.x).rotate_y(rot.y).rotate_z(rot.z);
        }
        if let Some(scl) = scale {
            new_transform = new_transform.scale_x(scl.x).scale_y(scl.y).scale_z(scl.z);
        }
        set_transform.set(new_transform);
    });
    
    view! {
        <div class="leptos-motion-3d-object">
            {children()}
        </div>
    }
}
```

---

## 🚀 **Performance Optimization Design**

### **Memory Management**
```rust
// Zero-cost abstractions for 3D operations
pub struct GeometryPool {
    geometries: HashMap<Uuid, Geometry>,
    vertex_buffers: HashMap<Uuid, WebGLBuffer>,
    index_buffers: HashMap<Uuid, WebGLBuffer>,
}

impl GeometryPool {
    pub fn get_or_create_buffer(&mut self, geometry_id: Uuid, geometry: &Geometry) -> Result<WebGLBuffer, RenderError> {
        if let Some(buffer) = self.vertex_buffers.get(&geometry_id) {
            return Ok(*buffer);
        }
        
        let buffer = self.create_vertex_buffer(geometry)?;
        self.vertex_buffers.insert(geometry_id, buffer);
        Ok(buffer)
    }
}

// Efficient rendering with instancing
pub struct InstancedRenderer {
    instance_data: Vec<InstanceData>,
    instance_buffer: WebGLBuffer,
    max_instances: usize,
}

impl InstancedRenderer {
    pub fn render_instanced(&mut self, geometry: &Geometry, material: &Material, instances: &[Transform3D]) {
        // Batch instance data
        for (i, transform) in instances.iter().enumerate() {
            self.instance_data[i] = InstanceData::from_transform(transform);
        }
        
        // Update instance buffer
        self.update_instance_buffer();
        
        // Render all instances in one draw call
        self.draw_instanced(geometry, material, instances.len());
    }
}
```

### **Rendering Pipeline Optimization**
```rust
// Efficient rendering pipeline
pub struct RenderingPipeline {
    shadow_pass: ShadowPass,
    geometry_pass: GeometryPass,
    lighting_pass: LightingPass,
    post_processing_pass: PostProcessingPass,
    present_pass: PresentPass,
}

impl RenderingPipeline {
    pub fn render(&mut self, scene: &Scene, camera: &Camera) -> Result<(), RenderError> {
        // 1. Shadow mapping pass
        self.shadow_pass.render(scene)?;
        
        // 2. Geometry pass (G-buffer)
        self.geometry_pass.render(scene, camera)?;
        
        // 3. Lighting pass
        self.lighting_pass.render(scene, camera)?;
        
        // 4. Post-processing pass
        self.post_processing_pass.render()?;
        
        // 5. Present to screen
        self.present_pass.render()?;
        
        Ok(())
    }
}
```

---

## 🔧 **API Design**

### **High-Level API**
```rust
// Simple, intuitive API for common use cases
pub fn create_3d_scene() -> SceneBuilder {
    SceneBuilder::new()
}

pub struct SceneBuilder {
    scene: Scene,
}

impl SceneBuilder {
    pub fn add_cube(mut self, position: Vector3, material: Material) -> Self {
        let geometry = Geometry::new_box(1.0, 1.0, 1.0);
        let object = Object3D::new(geometry, material)
            .with_position(position);
        self.scene.add_object(object);
        self
    }
    
    pub fn add_sphere(mut self, position: Vector3, radius: f64, material: Material) -> Self {
        let geometry = Geometry::new_sphere(radius, 32, 32);
        let object = Object3D::new(geometry, material)
            .with_position(position);
        self.scene.add_object(object);
        self
    }
    
    pub fn add_light(mut self, light: Light) -> Self {
        self.scene.add_light(light);
        self
    }
    
    pub fn build(self) -> Scene {
        self.scene
    }
}

// Usage example
let scene = create_3d_scene()
    .add_cube(Vector3::new(0.0, 0.0, 0.0), Material::new_basic(Color::red()))
    .add_sphere(Vector3::new(2.0, 0.0, 0.0), 0.5, Material::new_standard(Color::blue(), 0.0, 0.5))
    .add_light(Light::new_directional(Color::white(), 1.0, Vector3::new(1.0, 1.0, 1.0)))
    .build();
```

### **Component API**
```rust
// Leptos component integration
#[component]
pub fn Motion3DScene(
    #[prop(optional)] background_color: Option<Color>,
    #[prop(optional)] camera_position: Option<Vector3>,
    children: Children,
) -> impl IntoView {
    let (scene, set_scene) = create_signal(Scene::new());
    let (camera, set_camera) = create_signal(Camera::new_perspective(75.0, 1.0, 0.1, 1000.0));
    
    view! {
        <div class="motion-3d-scene">
            <canvas node_ref=create_node_ref::<HtmlCanvasElement>() />
            <div class="motion-3d-content">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Motion3DCube(
    #[prop(optional)] position: Option<Vector3>,
    #[prop(optional)] rotation: Option<Quaternion>,
    #[prop(optional)] scale: Option<Vector3>,
    #[prop(optional)] material: Option<Material>,
    children: Children,
) -> impl IntoView {
    let (transform, set_transform) = create_signal(Transform3D::new());
    
    view! {
        <div class="motion-3d-cube">
            {children()}
        </div>
    }
}
```

---

## 🎯 **Migration Strategy**

### **Backward Compatibility**
```rust
// Existing CSS-based animations continue to work
#[component]
pub fn MotionDiv(
    #[prop(optional)] animate: Option<AnimationTarget>,
    children: Children,
) -> impl IntoView {
    // Existing implementation unchanged
    view! {
        <div class="motion-div">
            {children()}
        </div>
    }
}

// New WebGL-based animations as enhancement
#[component]
pub fn Motion3D(
    #[prop(optional)] geometry: Option<Geometry>,
    #[prop(optional)] material: Option<Material>,
    #[prop(optional)] animate: Option<AnimationTarget>,
    children: Children,
) -> impl IntoView {
    // New WebGL implementation
    view! {
        <div class="motion-3d">
            {children()}
        </div>
    }
}
```

### **Progressive Enhancement**
```rust
// Automatic fallback from WebGL to CSS
pub enum RendererType {
    Auto,    // Try WebGL, fallback to CSS
    WebGL,   // Force WebGL
    CSS,     // Force CSS
}

impl RendererType {
    pub fn detect_capabilities() -> Self {
        if WebGL2RenderingContext::is_supported() {
            Self::WebGL
        } else {
            Self::CSS
        }
    }
}
```

---

## 📊 **Success Criteria**

### **Technical Requirements**
```rust
✅ 60+ FPS for complex 3D scenes
✅ <200KB bundle size (vs 600KB+ for Three.js)
✅ <200ms startup time
✅ 95%+ test coverage
✅ Zero memory leaks
✅ Cross-browser compatibility
✅ Type safety for all 3D operations
✅ Native Leptos signal integration
```

### **Feature Requirements**
```rust
✅ 80% feature parity with Three.js core
✅ Scene graph management
✅ WebGL rendering pipeline
✅ 3D geometry system
✅ Material and shader system
✅ Lighting system
✅ Camera system
✅ 3D model loading
✅ Animation system
✅ Post-processing effects
```

### **Developer Experience Requirements**
```rust
✅ Intuitive API design
✅ Comprehensive documentation
✅ Rich examples and tutorials
✅ Type-safe development
✅ Hot reload support
✅ Debug tools and profilers
✅ Community plugins and extensions
```

---

## 🎯 **Conclusion**

This design document provides a comprehensive roadmap for integrating Three.js-level 3D capabilities into Leptos Motion while maintaining our competitive advantages. The key is to build a type-safe, performant, and reactive 3D animation library that leverages Rust's strengths while providing the power and flexibility of modern 3D graphics.

**Next Steps:**
1. Create detailed implementation roadmap
2. Begin Phase 1 development with WebGL rendering engine
3. Establish development milestones and success metrics

---

*Last updated: December 2024*
*Leptos Motion v0.8.2*
*Design Document v1.0*

