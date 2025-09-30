# WebGL Renderer Design
## Hardware-Accelerated 3D Graphics

**File**: `crates/leptos-motion-webgl/src/renderer.rs`  
**Lines**: Target <300 (currently ~400)  
**Status**: STUB - Missing core implementation  

---

## 🎯 **Renderer Overview**

The WebGL Renderer provides hardware-accelerated 3D graphics capabilities for advanced animations and visual effects.

### **Core Responsibilities**
1. **WebGL Context Management**: Initialize and configure WebGL2 context
2. **Shader Program Management**: Compile and link vertex/fragment shaders
3. **Geometry Processing**: Handle vertex buffers and mesh data
4. **Render Pipeline**: Execute draw calls with proper state management
5. **Performance Optimization**: Minimize draw calls and state changes

### **Integration Points**
- **Animation Engine**: Receives animation values for shader uniforms
- **Scene Graph**: Renders hierarchical 3D scene objects
- **Texture System**: Manages texture loading and caching
- **Material System**: Handles shader materials and lighting

---

## 🏗️ **Architecture**

### **Core Components**
```rust
pub struct WebGLRenderer {
    // WebGL context
    gl: WebGl2RenderingContext,

    // Shader programs
    programs: HashMap<String, ShaderProgram>,

    // Geometry buffers
    vertex_buffers: HashMap<String, WebGlBuffer>,
    index_buffers: HashMap<String, WebGlBuffer>,

    // Render state
    current_program: Option<String>,
    current_textures: [Option<WebGlTexture>; 16],

    // Performance tracking
    stats: RenderStats,
}

pub struct ShaderProgram {
    program: WebGlProgram,
    uniforms: HashMap<String, WebGlUniformLocation>,
    attributes: HashMap<String, u32>,
}
```

### **Render Pipeline**
```rust
impl WebGLRenderer {
    pub fn new(canvas: &HtmlCanvasElement) -> Result<Self> {
        // 1. Get WebGL2 context
        let gl = canvas.get_context("webgl2")?
            .dyn_into::<WebGl2RenderingContext>()?;

        // 2. Configure context
        Self::configure_context(&gl)?;

        // 3. Initialize renderer
        Ok(Self {
            gl,
            programs: HashMap::new(),
            vertex_buffers: HashMap::new(),
            index_buffers: HashMap::new(),
            current_program: None,
            current_textures: [None; 16],
            stats: RenderStats::default(),
        })
    }

    fn configure_context(gl: &WebGl2RenderingContext) -> Result<()> {
        // Enable extensions
        gl.get_extension("EXT_color_buffer_float")?;
        gl.get_extension("OES_texture_float_linear")?;

        // Set default state
        gl.enable(WebGl2RenderingContext::DEPTH_TEST);
        gl.enable(WebGl2RenderingContext::BLEND);
        gl.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);

        Ok(())
    }
}
```

---

## 🎨 **Shader System**

### **Shader Program Management**
```rust
impl WebGLRenderer {
    pub fn create_program(&mut self, name: &str, vertex_src: &str, fragment_src: &str) -> Result<()> {
        // 1. Compile vertex shader
        let vertex_shader = self.compile_shader(WebGl2RenderingContext::VERTEX_SHADER, vertex_src)?;

        // 2. Compile fragment shader
        let fragment_shader = self.compile_shader(WebGl2RenderingContext::FRAGMENT_SHADER, fragment_src)?;

        // 3. Link program
        let program = self.link_program(vertex_shader, fragment_shader)?;

        // 4. Extract uniforms and attributes
        let uniforms = self.extract_uniforms(&program)?;
        let attributes = self.extract_attributes(&program)?;

        // 5. Store program
        self.programs.insert(name.to_string(), ShaderProgram {
            program,
            uniforms,
            attributes,
        });

        Ok(())
    }

    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader> {
        let shader = self.gl.create_shader(shader_type).ok_or("Failed to create shader")?;

        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        if !self.gl.get_shader_compile_status(&shader) {
            let error = self.gl.get_shader_info_log(&shader).unwrap_or_default();
            return Err(format!("Shader compilation failed: {}", error));
        }

        Ok(shader)
    }
}
```

### **Built-in Shaders**
```rust
pub const BASIC_VERTEX_SHADER: &str = r#"
    #version 300 es
    in vec3 a_position;
    in vec2 a_texcoord;
    in vec3 a_normal;

    uniform mat4 u_model_view_projection;

    out vec2 v_texcoord;
    out vec3 v_normal;

    void main() {
        gl_Position = u_model_view_projection * vec4(a_position, 1.0);
        v_texcoord = a_texcoord;
        v_normal = a_normal;
    }
"#;

pub const BASIC_FRAGMENT_SHADER: &str = r#"
    #version 300 es
    precision mediump float;

    in vec2 v_texcoord;
    in vec3 v_normal;

    uniform sampler2D u_texture;
    uniform vec3 u_light_direction;
    uniform float u_time;

    out vec4 frag_color;

    void main() {
        vec4 tex_color = texture(u_texture, v_texcoord);
        float lighting = dot(normalize(v_normal), normalize(u_light_direction)) * 0.5 + 0.5;
        frag_color = tex_color * vec4(vec3(lighting), 1.0);
    }
"#;
```

---

## 🔄 **Render Loop**

### **Frame Rendering**
```rust
impl WebGLRenderer {
    pub fn render_frame(&mut self, scene: &Scene) -> Result<()> {
        let start_time = instant::now();

        // 1. Clear frame
        self.clear_frame()?;

        // 2. Update camera matrices
        self.update_camera(scene.camera)?;

        // 3. Render all objects
        for object in &scene.objects {
            self.render_object(object)?;
        }

        // 4. Update performance stats
        let frame_time = instant::now() - start_time;
        self.stats.record_frame(frame_time);

        Ok(())
    }

    fn clear_frame(&self) -> Result<()> {
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
        Ok(())
    }

    fn render_object(&mut self, object: &SceneObject) -> Result<()> {
        // 1. Set shader program
        self.use_program(&object.material.shader_name)?;

        // 2. Set uniforms
        self.set_uniforms(&object.material.uniforms)?;

        // 3. Bind textures
        self.bind_textures(&object.material.textures)?;

        // 4. Set vertex attributes
        self.bind_geometry(&object.geometry)?;

        // 5. Set model matrix
        self.set_model_matrix(&object.transform)?;

        // 6. Draw
        self.draw_geometry(&object.geometry)?;

        Ok(())
    }
}
```

---

## 📊 **Geometry Management**

### **Buffer Creation**
```rust
pub struct Geometry {
    pub vertices: Vec<f32>,
    pub indices: Vec<u16>,
    pub texcoords: Option<Vec<f32>>,
    pub normals: Option<Vec<f32>>,
    pub vertex_buffer: Option<WebGlBuffer>,
    pub index_buffer: Option<WebGlBuffer>,
}

impl WebGLRenderer {
    pub fn create_geometry(&mut self, name: &str, geometry: &mut Geometry) -> Result<()> {
        // Create vertex buffer
        let vertex_buffer = self.gl.create_buffer().ok_or("Failed to create vertex buffer")?;
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

        // Upload vertex data
        unsafe {
            let vert_array = js_sys::Float32Array::view(&geometry.vertices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Create index buffer
        let index_buffer = self.gl.create_buffer().ok_or("Failed to create index buffer")?;
        self.gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        // Upload index data
        unsafe {
            let index_array = js_sys::Uint16Array::view(&geometry.indices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &index_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Store buffers
        geometry.vertex_buffer = Some(vertex_buffer);
        geometry.index_buffer = Some(index_buffer);

        self.vertex_buffers.insert(name.to_string(), vertex_buffer);
        self.index_buffers.insert(name.to_string(), index_buffer);

        Ok(())
    }
}
```

---

## 🎭 **Material System**

### **Material Definition**
```rust
#[derive(Clone, Debug)]
pub struct Material {
    pub shader_name: String,
    pub uniforms: HashMap<String, UniformValue>,
    pub textures: Vec<TextureBinding>,
    pub blend_mode: BlendMode,
    pub depth_test: bool,
}

#[derive(Clone, Debug)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Mat4([f32; 16]),
    Texture(u32),
}

pub struct TextureBinding {
    pub name: String,
    pub texture: WebGlTexture,
    pub unit: u32,
}
```

### **Uniform Setting**
```rust
impl WebGLRenderer {
    pub fn set_uniforms(&self, uniforms: &HashMap<String, UniformValue>) -> Result<()> {
        for (name, value) in uniforms {
            if let Some(location) = self.get_current_uniform_location(name) {
                match value {
                    UniformValue::Float(v) => self.gl.uniform1f(Some(location), *v),
                    UniformValue::Vec2(v) => self.gl.uniform2f(Some(location), v[0], v[1]),
                    UniformValue::Vec3(v) => self.gl.uniform3f(Some(location), v[0], v[1], v[2]),
                    UniformValue::Vec4(v) => self.gl.uniform4f(Some(location), v[0], v[1], v[2], v[3]),
                    UniformValue::Mat4(v) => {
                        let mat = js_sys::Float32Array::view(v);
                        self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, &mat);
                    }
                    UniformValue::Texture(unit) => self.gl.uniform1i(Some(location), *unit as i32),
                }
            }
        }
        Ok(())
    }
}
```

---

## 🖼️ **Texture System**

### **Texture Loading**
```rust
impl WebGLRenderer {
    pub async fn load_texture(&self, url: &str) -> Result<WebGlTexture> {
        // 1. Create texture
        let texture = self.gl.create_texture().ok_or("Failed to create texture")?;

        // 2. Bind texture
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(&texture));

        // 3. Set parameters
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::LINEAR as i32);
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::LINEAR as i32);

        // 4. Load image
        let image = self.load_image(url).await?;

        // 5. Upload to GPU
        self.gl.tex_image_2d_with_u32_and_u32_and_image(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            &image,
        )?;

        Ok(texture)
    }

    async fn load_image(&self, url: &str) -> Result<HtmlImageElement> {
        let image = HtmlImageElement::new()?;
        image.set_src(url);

        // Wait for load
        let (tx, rx) = oneshot::channel();
        let onload = Closure::wrap(Box::new(move || {
            let _ = tx.send(());
        }) as Box<dyn FnMut()>);

        image.set_onload(Some(onload.as_ref().unchecked_ref()));
        let _ = rx.await;

        Ok(image)
    }
}
```

---

## 📊 **Performance Monitoring**

### **Render Statistics**
```rust
#[derive(Clone, Debug, Default)]
pub struct RenderStats {
    pub frame_count: u64,
    pub draw_calls: u64,
    pub triangles_rendered: u64,
    pub texture_uploads: u64,
    pub shader_switches: u64,
    pub average_frame_time: f64,
}

impl RenderStats {
    pub fn record_frame(&mut self, frame_time: f64) {
        self.frame_count += 1;
        self.average_frame_time = (self.average_frame_time + frame_time) / 2.0;
    }

    pub fn record_draw_call(&mut self, triangle_count: usize) {
        self.draw_calls += 1;
        self.triangles_rendered += triangle_count as u64;
    }

    pub fn report(&self) {
        log::info!("WebGL Render Stats:");
        log::info!("  Frames: {}", self.frame_count);
        log::info!("  Draw Calls: {}", self.draw_calls);
        log::info!("  Triangles: {}", self.triangles_rendered);
        log::info!("  Avg Frame Time: {:.2}ms", self.average_frame_time);
        log::info!("  Triangles/Frame: {:.0}", self.triangles_rendered as f64 / self.frame_count as f64);
    }
}
```

---

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[wasm_bindgen_test]
fn test_renderer_creation() {
    // Create canvas
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap()
        .dyn_into::<HtmlCanvasElement>().unwrap();

    // Create renderer
    let renderer = WebGLRenderer::new(&canvas).unwrap();

    // Verify WebGL2 context
    assert!(renderer.gl.get_parameter(WebGl2RenderingContext::VERSION)
        .unwrap().as_string().unwrap().contains("WebGL 2.0"));
}

#[wasm_bindgen_test]
fn test_shader_compilation() {
    let mut renderer = create_test_renderer();

    // Create basic shader program
    renderer.create_program(
        "basic",
        BASIC_VERTEX_SHADER,
        BASIC_FRAGMENT_SHADER
    ).unwrap();

    // Verify program exists
    assert!(renderer.programs.contains_key("basic"));
}
```

### **Integration Tests**
```rust
#[wasm_bindgen_test]
async fn test_texture_loading() {
    let renderer = create_test_renderer();

    // Create test texture (1x1 pixel)
    let texture = renderer.load_texture("data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChAI9jzyr5AAAAABJRU5ErkJggg==").await.unwrap();

    // Verify texture created
    assert!(texture.is_some());
}
```

---

## 🎯 **Implementation Plan**

### **Phase 1: Core Renderer (Week 1-2)**
- [ ] WebGL2 context initialization
- [ ] Shader compilation and linking
- [ ] Basic render loop
- [ ] Frame clearing and setup

### **Phase 2: Geometry System (Week 3-4)**
- [ ] Vertex and index buffer creation
- [ ] Geometry loading and storage
- [ ] Vertex attribute binding
- [ ] Draw call execution

### **Phase 3: Material System (Week 5-6)**
- [ ] Uniform value setting
- [ ] Texture loading and binding
- [ ] Material definition structures
- [ ] Shader program switching

### **Phase 4: Scene Integration (Week 7-8)**
- [ ] Scene graph rendering
- [ ] Camera matrix handling
- [ ] Object transform application
- [ ] Performance optimization

**Target Completion**: 8 weeks for functional WebGL renderer with basic 3D rendering capabilities.
