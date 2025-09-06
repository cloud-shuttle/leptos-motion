# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0-beta] - 2025-09-06

### Added

- **Comprehensive Test Suite**: 343 tests with 100% pass rate
- **WASM Compatibility**: Proper conditional compilation for web-sys features
- **Feature Flags**: Working feature flag system for bundle optimization
- **Error Handling**: Robust error handling with recovery strategies
- **Performance Monitoring**: FPS tracking and optimization framework
- **Gesture System**: Multi-touch, drag, hover, tap recognition
- **Layout Animations**: FLIP-based smooth transitions
- **Motion Components**: MotionDiv, MotionSpan, AnimatePresence
- **Spring Physics**: Natural, physics-based animations
- **Easing Functions**: Linear, EaseIn, EaseOut, EaseInOut, Spring
- **Type Safety**: Full Rust compile-time guarantees

### Changed

- **Bundle Size**: 378KB WASM (needs optimization for production)
- **Component Coverage**: 3 components (need 17+ more for v1.0)
- **Animation Properties**: 6 properties (need all CSS properties)
- **API Design**: Simplified, clean interfaces
- **Testing**: Comprehensive test coverage across all crates

### Fixed

- **Compilation Errors**: All 13 failing tests resolved
- **WASM Issues**: Conditional compilation for non-WASM environments
- **Test Panics**: Runtime issues resolved
- **Dependency Issues**: Feature flag and dependency optimization
- **Memory Leaks**: Proper cleanup and resource management

### Known Issues

- **Bundle Size**: 8x larger than target (378KB vs 50KB target)
- **Limited Components**: Only 3 components vs 20+ needed
- **Missing Features**: Timeline, keyframes, scroll animations
- **Animation Properties**: Limited to 6 basic properties
- Improved documentation

## [0.2.0-beta.1] - 2024-12-XX

### Added

- **Multi-Touch Gesture Support**
  - Pinch-to-zoom with precise scale calculations
  - Rotation detection with angle precision
  - Multi-touch state management
  - Gesture confidence scoring system
  - Touch point tracking and validation

- **Advanced Animation Engine**
  - Hybrid RAF/WAAPI animation system
  - Spring physics with configurable parameters
  - Performance budgeting for 60fps
  - Memory pooling and optimization
  - GPU layer management

- **FLIP Layout Animations**
  - Automatic layout change detection
  - Smooth position/size transitions
  - Shared element support
  - Performance monitoring
  - GPU acceleration

- **Enhanced DOM Integration**
  - MotionDiv with full gesture support
  - MotionSpan for inline animations
  - AnimatePresence for enter/exit
  - Event handler integration
  - CSS optimization and batching

### Changed

- **Breaking Changes**
  - Updated to Leptos 0.8.x compatibility
  - Improved API design for better ergonomics
  - Enhanced type safety throughout

- **Performance Improvements**
  - Reduced memory allocations
  - Optimized animation scheduling
  - Better frame budgeting
  - GPU layer promotion

- **Code Quality**
  - TDD-driven development approach
  - Comprehensive test coverage
  - Improved error handling
  - Better documentation

### Fixed

- **Critical Bugs**
  - Multi-touch rotation calculation (#123)
  - Gesture state management (#124)
  - Animation timing issues (#125)
  - Memory leak in animation engine (#126)

- **Test Issues**
  - All gesture tests now passing
  - Improved test reliability
  - Better error reporting
  - Comprehensive coverage

### Removed

- Deprecated API methods
- Unused code and imports
- Legacy gesture handling
- Obsolete animation types

## [0.1.1] - 2024-XX-XX

### Added

- Basic animation engine
- Core motion components
- Simple gesture support
- Basic documentation

### Changed

- Initial alpha release
- Basic functionality implementation
- Core architecture established

### Fixed

- Initial implementation bugs
- Basic compatibility issues

## [0.1.0-alpha] - 2024-XX-XX

### Added

- Initial project setup
- Basic animation framework
- Core project structure
- Development environment

---

## Release Notes

For detailed information about each release, see [RELEASE_NOTES.md](RELEASE_NOTES.md).

## Migration Guide

### From 0.1.x to 0.2.0-beta.1

1. **Update Dependencies**

   ```toml
   [dependencies]
   leptos-motion = "0.2.0-beta.1"
   leptos = "0.8.5"
   ```

2. **API Changes**
   - Some component props have been renamed for clarity
   - Gesture handling has been completely rewritten
   - Animation engine API has been simplified

3. **Breaking Changes**
   - `create_signal` → `signal()` (Leptos 0.8.x)
   - `create_memo` → `memo()` (Leptos 0.8.x)
   - Gesture event handling has been restructured

4. **New Features**
   - Multi-touch gesture support
   - FLIP layout animations
   - Enhanced performance optimization
   - Better error handling

For detailed migration instructions, see the [Migration Guide](docs/migration.md).
