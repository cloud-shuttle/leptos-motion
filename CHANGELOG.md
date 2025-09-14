# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.9.1] - 2024-12-19

### Added
- **Performance Benchmarking Suite**: Comprehensive performance testing tools
  - `performance-demo/` - WASM-powered performance benchmark
  - `performance-benchmark.html` - JavaScript-based performance testing
  - Real-time FPS monitoring and frame time tracking
  - Stress testing capabilities for concurrent animations
  - Memory usage monitoring and optimization tools

- **Comprehensive Documentation**: Complete API reference and usage guides
  - `docs/API_REFERENCE.md` - Complete API documentation with examples
  - `docs/USAGE_GUIDE.md` - Comprehensive usage guide with patterns
  - `README.md` - Updated project overview with quick start guide
  - Migration guide from v0.6 to v0.9
  - Performance optimization best practices

- **Working Examples**: Fully functional demonstration applications
  - `simple-working-demo/` - Basic reactive animations showcase
  - `phase2-reactive-demo/` - Advanced reactive features demonstration
  - All examples now properly compile and run in browsers

### Fixed
- **Critical Animation Engine Issues**: Resolved core animation problems
  - Fixed `start_animation_loop` method visibility (now public)
  - Resolved `AnimationValue` display formatting for CSS properties
  - Fixed transform property handling with proper string formatting
  - Corrected reactive signal disposal issues in component lifecycle

- **WASM Compilation Issues**: Resolved WebAssembly build problems
  - Fixed workspace configuration errors in example crates
  - Corrected dependency path references
  - Resolved `wasm-bindgen` initialization issues
  - Fixed reactive signal disposal panics

- **API Compatibility**: Updated to match current Leptos API
  - Migrated from deprecated `create_rw_signal` to `RwSignal::new()`
  - Updated `Transition` struct field from `easing` to `ease`
  - Fixed `repeat` field to use `RepeatConfig` enum
  - Corrected `animate` prop type expectations

- **DOM Integration**: Improved DOM interaction and mounting
  - Fixed `mount_to` vs `mount_to_body` usage patterns
  - Resolved `HtmlElement` casting issues with `JsCast`
  - Improved component lifecycle management
  - Fixed global state management with `OnceLock`

### Changed
- **Enhanced Animation Engine**: Improved core animation functionality
  - Better handling of complex transform properties
  - Improved CSS property value formatting
  - Enhanced reactive signal integration
  - Optimized DOM update batching

- **Improved Error Handling**: Better error messages and debugging
  - More descriptive compilation error messages
  - Enhanced runtime error handling
  - Better debugging information for animation issues
  - Improved panic handling in WASM context

### Performance
- **Optimized Rendering**: Improved animation performance
  - Better frame rate consistency
  - Reduced DOM update overhead
  - Optimized reactive signal updates
  - Enhanced memory management

- **WASM Optimization**: Better WebAssembly performance
  - Reduced bundle size
  - Improved compilation speed
  - Better runtime performance
  - Enhanced memory efficiency

### Documentation
- **Complete API Reference**: Comprehensive documentation
  - All public APIs documented with examples
  - Usage patterns and best practices
  - Performance optimization guidelines
  - Troubleshooting guide

- **Working Examples**: Functional demonstration code
  - All examples compile and run successfully
  - Clear demonstration of library capabilities
  - Performance benchmarking tools
  - Real-world usage patterns

## [0.9.0] - 2024-12-18

### Added
- Initial release of Leptos Motion
- Core animation engine with WASM support
- ReactiveMotionDivV2 component
- DragMotionDiv component
- Basic animation types and transitions
- Easing functions and repeat configurations
- Drag constraints and momentum physics

### Features
- WASM-powered animations
- Reactive API integration with Leptos
- Type-safe animation configuration
- Hardware-accelerated CSS transforms
- Drag and drop functionality
- Staggered animations support

---

## Migration Guide

### From v0.6 to v0.9.1

1. **Update Component Names**
   ```rust
   // Old
   use leptos_motion_dom::MotionDiv;
   
   // New
   use leptos_motion_dom::reactive_motion_div_v2::ReactiveMotionDivV2;
   ```

2. **Update Signal Usage**
   ```rust
   // Old
   let (value, set_value) = create_rw_signal(0.0);
   
   // New
   let (value, set_value) = RwSignal::new(0.0);
   ```

3. **Update Transition Configuration**
   ```rust
   // Old
   let transition = Transition {
       easing: Some(Easing::EaseInOut),
       repeat: None,
       // ...
   };
   
   // New
   let transition = Transition {
       ease: Easing::EaseInOut,
       repeat: RepeatConfig::Never,
       // ...
   };
   ```

4. **Update Prop Types**
   ```rust
   // Old
   <MotionDiv animate=animate_signal>
   
   // New
   <ReactiveMotionDivV2 animate=animate_signal.read_only()>
   ```

### Breaking Changes

- `MotionDiv` → `ReactiveMotionDivV2`
- `create_rw_signal` → `RwSignal::new()`
- `easing` → `ease` in Transition struct
- `repeat: Option<u32>` → `repeat: RepeatConfig`
- `animate` prop now expects `ReadSignal` instead of `&RwSignal`

### Performance Improvements

- Better animation engine performance
- Optimized DOM updates
- Reduced memory usage
- Improved WASM compilation
- Enhanced reactive signal handling

---

## Contributing

Contributions are welcome! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## Support

- GitHub Issues: [Report bugs or request features](https://github.com/cloud-shuttle/leptos-motion/issues)
- Documentation: [API Reference](docs/API_REFERENCE.md) and [Usage Guide](docs/USAGE_GUIDE.md)
- Examples: [Working examples](examples/) and [demos](demos/)

## License

This project is licensed under the MIT OR Apache-2.0 License - see the [LICENSE](LICENSE) file for details.
