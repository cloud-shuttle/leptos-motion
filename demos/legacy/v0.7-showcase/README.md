# Leptos Motion v0.7.0 Showcase

A comprehensive demonstration of the new advanced animation features in Leptos Motion v0.7.0.

## 🌟 Features Demonstrated

### 🌊 Spring Physics Engine

- Natural, physics-based animations
- Configurable tension, friction, and mass
- Real-time parameter adjustment
- Smooth, organic motion

### 👻 AnimatePresence Component

- Enter/exit animations for conditional rendering
- Multiple presence modes (Sync, Immediate)
- Smooth transitions with custom timing
- Automatic cleanup and state management

### 🎭 Variants System

- Named animation states
- Smooth transitions between states
- Interactive state switching
- Reusable animation patterns

### ⏰ Timeline Sequences

- Multi-step animation orchestration
- Precise timing control
- Step-by-step playback
- Complex animation sequences

### ⚡ Performance Optimizations

- Memory pool demonstrations
- Caching system showcase
- Edge case handling
- Performance monitoring

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- wasm-pack
- Python 3 (for serving)

### Build and Run

1. **Build the showcase:**

   ```bash
   cd examples/v0.7-showcase
   ./build.sh
   ```

2. **Serve the showcase:**

   ```bash
   python3 -m http.server 8000
   ```

3. **Open in browser:**
   ```
   http://localhost:8000
   ```

## 🎮 Interactive Features

### Spring Physics Demo

- Click the green circle to animate
- Use buttons to adjust spring parameters
- See real-time physics simulation

### AnimatePresence Demo

- Toggle visibility with Show/Hide button
- Switch between presence modes
- Observe smooth enter/exit animations

### Variants Demo

- Hover over the purple square
- Click to see tap animation
- Use buttons to manually set states

### Timeline Demo

- Play the complete sequence
- Step through individual animations
- Reset to start over

### Performance Demo

- Animate 5 or 25 elements
- Monitor performance in real-time
- See optimization benefits

## 📊 Performance Monitoring

The showcase includes real-time performance monitoring:

- **FPS**: Frames per second
- **Frame Time**: Average frame duration
- **Memory**: JavaScript heap usage

## 🛠️ Technical Details

### Architecture

- Built with Leptos 0.8.8
- Uses leptos-motion v0.7.0
- WASM compilation for performance
- Modern CSS with backdrop filters

### Performance Features

- Object pooling for animation targets
- Intelligent caching with LRU eviction
- Edge case handling for extreme values
- Memory leak prevention

### Browser Compatibility

- Modern browsers with WASM support
- CSS Grid and Flexbox
- ES6 modules
- Performance API

## 🎨 Styling

The showcase uses modern CSS features:

- CSS Grid for responsive layout
- Backdrop filters for glassmorphism
- CSS custom properties
- Smooth transitions and animations

## 🔧 Customization

### Adding New Demos

1. Create a new component in `src/lib.rs`
2. Add it to the `App` component
3. Include it in the showcase grid

### Modifying Animations

- Adjust spring parameters
- Change transition durations
- Modify easing functions
- Add new animation properties

### Performance Tuning

- Adjust element counts
- Modify animation complexity
- Test different configurations
- Monitor performance impact

## 📚 Learning Resources

- [Leptos Motion Documentation](https://docs.rs/leptos-motion)
- [Spring Physics Guide](./docs/spring-physics.md)
- [AnimatePresence Tutorial](./docs/animate-presence.md)
- [Variants System Guide](./docs/variants.md)
- [Timeline Sequences](./docs/timeline.md)

## 🤝 Contributing

Found a bug or want to add a feature?

1. Open an issue on GitHub
2. Fork the repository
3. Create a pull request
4. Follow the contribution guidelines

## 📄 License

This showcase is part of the Leptos Motion project and is licensed under the MIT License.

---

**Leptos Motion v0.7.0** - Advanced animation capabilities for the Leptos ecosystem.
