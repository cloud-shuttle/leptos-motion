# WebGL System Design Document

**Version**: 1.0  
**Status**: Design Phase  
**Target**: Production-ready 3D rendering system  
**Timeline**: 6 weeks implementation  

---

## 🎯 **Design Goals**

### **Primary Objectives**
- **Type-safe 3D operations** with Rust's compile-time guarantees
- **60fps performance** on modern browsers
- **Reactive integration** with Leptos signals
- **Memory-safe** WebGL operations
- **Cross-browser compatibility** (Chrome, Firefox, Safari, Edge)

### **Secondary Objectives**
- **Competitive with Three.js** in performance and features
- **Modular architecture** for easy extension
- **Comprehensive error handling** with detailed diagnostics
- **Developer-friendly** API with clear documentation

---

## 🏗️ **System Architecture**

### **High-Level Architecture**
```
┌─────────────────────────────────────────────────────────────┐
│                    WebGL Rendering System                   │
├─────────────────────────────────────────────────────────────┤
│  🎨 Rendering Layer                                        │
│  ├── Scene Graph Management                                │
│  ├── Camera System (Perspective/Orthographic)              │
│  ├── Material System (PBR, Custom Shaders)                 │
│  └── Post-Processing Pipeline                              │
├─────────────────────────────────────────────────────────────┤
│  💡 Lighting System                                        │
│  ├── Ambient Lighting                                      │
│  ├── Directional Lighting (Sun)                            │
│  ├── Point Lighting                                        │
│  ├── Spot Lighting                                         │
│  └── Shadow Mapping                                        │
├─────────────────────────────────────────────────────────────┤
│  🔧 Core Systems                                           │
│  ├── Geometry Management (Buffers, VAOs)                   │
│  ├── Texture Management (Loading, Caching)                 │
│  ├── Shader System (Compilation, Linking)                  │
│  └── Physics Integration (Collision, Rigid Bodies)         │
├─────────────────────────────────────────────────────────────┤
│  🎮 Advanced Features                                      │
│  ├── Model Loading (OBJ, GLTF)                             │
│  ├── Animation System (Keyframes, Morphing)                │
│  ├── Particle Systems                                      │
│  └── GPU Instancing                                        │
└─────────────────────────────────────────────────────────────┘
```

### **Component Architecture**
```rust
// Core rendering components
pub struct WebGLRenderer {
    context: WebGl2RenderingContext,
    scene: Scene,
    camera: Box<dyn Camera>,
    shader_manager: ShaderManager,
    texture_manager: TextureManager,
    geometry_manager: GeometryManager,
}

pub struct Scene {
    objects: Vec<Object3D>,
    lights: Vec<Box<dyn Light>>,
    background: Option<Background>,
    fog: Option<Fog>,
}

pub struct Object3D {
    id: Uuid,
    name: String,
    transform: Transform3D,
    geometry: Option<Geometry>,
    material: Option<Material>,
    children: Vec<Object3D>,
    parent: Option<Uuid>,
    visible: bool,
}
```

---

## 🎨 **Rendering Pipeline**

### **1. Scene Graph System**
```rust
impl Scene {
    pub fn add_object(&mut self, object: Object3D) -> Uuid {
        let id = object.id;
        self.objects.push(object);
        id
    }
    
    pub fn get_object(&self, id: Uuid) -> Option<&Object3D> {
        self.objects.iter().find(|obj| obj.id == id)
    }
    
    pub fn update_object(&mut self, id: Uuid, transform: Transform3D) -> Result<()> {
        if let Some(obj) = self.objects.iter_mut().find(|obj| obj.id == id) {
            obj.transform = transform;
            Ok(())
        } else {
            Err(WebGLError::SceneError("Object not found".to_string()))
        }
    }
}
```

### **2. Camera System**
```rust
pub trait Camera {
    fn get_view_matrix(&self) -> Matrix4<f32>;
    fn get_projection_matrix(&self) -> Matrix4<f32>;
    fn update(&mut self, delta_time: f32);
}

pub struct PerspectiveCamera {
    fov: f32,
    aspect: f32,
    near: f32,
    far: f32,
    position: Vector3<f32>,
    target: Vector3<f32>,
    up: Vector3<f32>,
}

impl Camera for PerspectiveCamera {
    fn get_view_matrix(&self) -> Matrix4<f32> {
        look_at(self.position, self.target, self.up)
    }
    
    fn get_projection_matrix(&self) -> Matrix4<f32> {
        perspective(self.fov, self.aspect, self.near, self.far)
    }
}
```

### **3. Material System**
```rust
pub struct Material {
    pub name: String,
    pub shader: ShaderProgram,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: HashMap<String, Texture>,
    pub properties: MaterialProperties,
}

pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4(Matrix4<f32>),
    Texture(Texture),
}

impl Material {
    pub fn bind(&self, context: &WebGl2RenderingContext) -> Result<()> {
        self.shader.use_program(context)?;
        
        for (name, value) in &self.uniforms {
            self.shader.set_uniform(context, name, value)?;
        }
        
        for (name, texture) in &self.textures {
            texture.bind(context, name)?;
        }
        
        Ok(())
    }
}
```

---

## 💡 **Lighting System Design**

### **Light Types**
```rust
pub trait Light {
    fn get_type(&self) -> LightType;
    fn get_color(&self) -> [f32; 3];
    fn get_intensity(&self) -> f32;
    fn get_position(&self) -> [f32; 3];
    fn get_direction(&self) -> [f32; 3];
    fn get_shadow_map(&self) -> Option<&ShadowMap>;
}

pub struct AmbientLight {
    pub name: String,
    pub color: [f32; 3],
    pub intensity: f32,
}

pub struct DirectionalLight {
    pub name: String,
    pub color: [f32; 3],
    pub intensity: f32,
    pub direction: [f32; 3],
    pub shadow_map: Option<ShadowMap>,
}

pub struct PointLight {
    pub name: String,
    pub color: [f32; 3],
    pub intensity: f32,
    pub position: [f32; 3],
    pub range: f32,
    pub attenuation: [f32; 3], // constant, linear, quadratic
}
```

### **Lighting Calculations**
```rust
impl LightingManager {
    pub fn calculate_lighting(&self, position: [f32; 3], normal: [f32; 3], 
                            view_dir: [f32; 3]) -> [f32; 3] {
        let mut color = [0.0, 0.0, 0.0];
        
        // Ambient lighting
        for light in &self.ambient_lights {
            color[0] += light.color[0] * light.intensity;
            color[1] += light.color[1] * light.intensity;
            color[2] += light.color[2] * light.intensity;
        }
        
        // Directional lighting
        for light in &self.directional_lights {
            let light_dir = normalize(light.direction);
            let diff = max(dot(normal, light_dir), 0.0);
            
            // Diffuse
            color[0] += light.color[0] * light.intensity * diff;
            color[1] += light.color[1] * light.intensity * diff;
            color[2] += light.color[2] * light.intensity * diff;
            
            // Specular
            let reflect_dir = reflect(-light_dir, normal);
            let spec = pow(max(dot(view_dir, reflect_dir), 0.0), 32.0);
            color[0] += light.color[0] * light.intensity * spec;
            color[1] += light.color[1] * light.intensity * spec;
            color[2] += light.color[2] * light.intensity * spec;
        }
        
        color
    }
}
```

---

## 🔧 **Physics Integration**

### **Collision Detection**
```rust
pub struct PhysicsWorld {
    pub bodies: Vec<RigidBody>,
    pub gravity: [f32; 3],
    pub time_step: f32,
}

impl PhysicsWorld {
    pub fn add_body(&mut self, body: RigidBody) -> u32 {
        let id = self.bodies.len() as u32;
        self.bodies.push(body);
        id
    }
    
    pub fn step(&mut self, delta_time: f32) {
        // Apply gravity
        for body in &mut self.bodies {
            if body.is_dynamic() {
                body.velocity[1] += self.gravity[1] * delta_time;
            }
        }
        
        // Update positions
        for body in &mut self.bodies {
            if body.is_dynamic() {
                body.position[0] += body.velocity[0] * delta_time;
                body.position[1] += body.velocity[1] * delta_time;
                body.position[2] += body.velocity[2] * delta_time;
            }
        }
        
        // Check collisions
        self.check_collisions();
    }
    
    fn check_collisions(&mut self) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                if let Some(contact) = self.check_collision(&self.bodies[i], &self.bodies[j]) {
                    self.resolve_collision(&mut self.bodies[i], &mut self.bodies[j], &contact);
                }
            }
        }
    }
}
```

---

## 🎮 **Advanced Features**

### **Model Loading**
```rust
pub struct ModelLoader {
    pub supported_formats: Vec<ModelFormat>,
}

impl ModelLoader {
    pub fn load_model(&self, data: &[u8], format: ModelFormat) -> Result<Model> {
        match format {
            ModelFormat::OBJ => self.load_obj(data),
            ModelFormat::GLTF => self.load_gltf(data),
            _ => Err(WebGLError::ModelError("Unsupported format".to_string())),
        }
    }
    
    fn load_obj(&self, data: &[u8]) -> Result<Model> {
        let obj_data = String::from_utf8(data.to_vec())?;
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::new();
        
        // Parse OBJ file
        for line in obj_data.lines() {
            if line.starts_with("v ") {
                // Parse vertex
                let coords: Vec<f32> = line[2..].split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect();
                vertices.extend_from_slice(&coords);
            } else if line.starts_with("f ") {
                // Parse face
                let face_indices: Vec<usize> = line[2..].split_whitespace()
                    .map(|s| s.split('/').next().unwrap().parse::<usize>().unwrap() - 1)
                    .collect();
                indices.extend_from_slice(&face_indices);
            }
        }
        
        Ok(Model {
            vertices,
            normals,
            tex_coords,
            indices,
        })
    }
}
```

---

## 📊 **Performance Considerations**

### **Optimization Strategies**
- **Frustum culling** - Only render objects in view
- **Occlusion culling** - Skip hidden objects
- **LOD system** - Use lower detail for distant objects
- **GPU instancing** - Render multiple objects with single draw call
- **Texture atlasing** - Combine textures to reduce state changes
- **Shader caching** - Cache compiled shaders
- **Buffer pooling** - Reuse vertex buffers

### **Memory Management**
- **Automatic cleanup** - Drop unused resources
- **Reference counting** - Share resources between objects
- **Garbage collection** - Periodic cleanup of unused assets
- **Memory monitoring** - Track memory usage and leaks

---

## 🧪 **Testing Strategy**

### **Unit Tests**
- **Shader compilation** tests
- **Buffer management** tests
- **Texture loading** tests
- **Lighting calculations** tests
- **Physics integration** tests

### **Integration Tests**
- **Rendering pipeline** tests
- **Scene graph** tests
- **Camera system** tests
- **Material system** tests
- **Post-processing** tests

### **Performance Tests**
- **Frame rate** benchmarks
- **Memory usage** tests
- **Rendering performance** tests
- **Physics performance** tests

---

**This design document provides the foundation for a production-ready WebGL rendering system that integrates seamlessly with the Leptos Motion library.**
