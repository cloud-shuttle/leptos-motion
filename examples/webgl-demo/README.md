# WebGL Advanced Features Demo

This demo showcases the advanced WebGL capabilities implemented in **Phase 3** of the `leptos-motion-webgl` library, including post-processing effects, shadow mapping, and physics simulation.

## 🎯 Features Demonstrated

### 🎨 Post-Processing Effects
- **Bloom Effect**: Glowing highlights and light bleeding for enhanced visual appeal
- **SSAO (Screen Space Ambient Occlusion)**: Depth-based shadows for improved realism
- **Tone Mapping**: HDR to LDR conversion for proper color grading

### 🌑 Shadow Mapping
- **Directional Light Shadows**: Sun/moon shadows with orthographic projection
- **Point Light Shadows**: Omnidirectional shadows using cube mapping
- **Configurable Quality**: Adjustable resolution and filtering options

### ⚡ Physics Simulation
- **Rigid Body Dynamics**: Realistic object movement and interactions
- **Collision Detection**: Support for boxes, spheres, planes, and capsules
- **Collision Response**: Impulse-based resolution with penetration correction

### 🎮 Interactive Controls
- **Real-time Adjustment**: Live parameter tweaking with immediate visual feedback
- **Performance Monitoring**: FPS and memory usage tracking
- **Visual Feedback**: Immediate effect changes as you adjust parameters

## 🚀 Getting Started

### Prerequisites
- Rust (latest stable version)
- `wasm-pack` for building WebAssembly
- A modern web browser with WebGL 2.0 support

### Installation

1. **Install wasm-pack** (if not already installed):
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Build the demo**:
   ```bash
   ./build.sh
   ```

3. **Start a local server**:
   ```bash
   # Using Python
   python -m http.server 8000
   
   # Using Node.js (if you have serve installed)
   npx serve .
   
   # Using any other HTTP server
   ```

4. **Open your browser** and navigate to `http://localhost:8000`

## 🎮 Controls

### Post-Processing Effects
- **Bloom Intensity** (0.0 - 3.0): Controls the strength of the bloom effect
- **SSAO Intensity** (0.0 - 2.0): Adjusts the ambient occlusion strength
- **Tone Mapping Exposure** (0.1 - 5.0): Controls HDR exposure levels

### Shadow Mapping
- **Shadow Bias** (0.001 - 0.02): Prevents shadow acne artifacts
- **Light Intensity** (0.1 - 3.0): Controls overall lighting strength

### Animation
- **Animation Speed** (0.0 - 3.0): Controls the speed of animations and physics

## 🏗️ Technical Architecture

### Core Systems
- **PostProcessingPipeline**: Manages effect chains and framebuffer operations
- **ShadowMappingManager**: Handles shadow map generation and binding
- **PhysicsWorld**: Simulates rigid body dynamics and collision detection
- **WebGLRenderer**: Core rendering engine with WebGL 2.0 support

### Rendering Pipeline
1. **Scene Setup**: Objects, lights, and cameras are configured
2. **Shadow Pass**: Shadow maps are rendered for each light source
3. **Main Render**: Scene is rendered to framebuffer with shadows
4. **Post-Processing**: Effects are applied in sequence
5. **Final Output**: Result is displayed on screen

## 🔧 Development

### Project Structure
```
examples/webgl-demo/
├── src/
│   └── main.rs          # Main demo application
├── Cargo.toml           # Dependencies and configuration
├── index.html           # HTML template with styling
├── build.sh             # Build script
└── README.md            # This file
```

### Key Components
- **DemoState**: Central state management for all WebGL systems
- **DemoControls**: Interactive UI controls for parameter adjustment
- **FeatureShowcase**: Information display about available features
- **Render Loop**: 60 FPS rendering with delta time updates

## 🎨 Visual Features

### Post-Processing Pipeline
The demo implements a complete post-processing pipeline with:
- Multi-pass rendering for complex effects
- Configurable effect chains
- Real-time parameter adjustment
- Performance-optimized shaders

### Shadow System
Advanced shadow mapping with:
- Multiple shadow map types (directional, point)
- Configurable resolution and quality
- Bias and filtering controls
- Real-time shadow updates

### Physics Integration
Real-time physics simulation featuring:
- Multiple collision shapes
- Rigid body dynamics
- Collision detection and response
- Configurable gravity and time steps

## 🚀 Performance

The demo is optimized for 60+ FPS with:
- Efficient WebGL state management
- Optimized shader compilation
- Smart culling and LOD systems
- Memory-efficient data structures

## 🐛 Troubleshooting

### Common Issues

1. **WebGL Context Creation Failed**
   - Ensure your browser supports WebGL 2.0
   - Try disabling hardware acceleration and re-enabling it
   - Check browser console for specific error messages

2. **Build Failures**
   - Ensure `wasm-pack` is properly installed
   - Check that all dependencies are available
   - Verify Rust toolchain is up to date

3. **Performance Issues**
   - Lower shadow map resolution
   - Reduce post-processing effect intensity
   - Close other browser tabs to free memory

### Browser Compatibility
- **Chrome/Chromium**: Full support
- **Firefox**: Full support
- **Safari**: Full support (macOS 10.15+)
- **Edge**: Full support

## 📚 Learn More

- [WebGL 2.0 Specification](https://www.khronos.org/registry/webgl/specs/latest/2.0/)
- [Leptos Framework Documentation](https://leptos.dev/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)

## 🤝 Contributing

This demo is part of the `leptos-motion` project. Contributions are welcome! Please see the main project repository for contribution guidelines.

## 📄 License

This demo is part of the `leptos-motion` project and follows the same license terms.
