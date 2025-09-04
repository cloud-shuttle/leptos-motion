# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Advanced gesture combinations (swipe, long-press)
- Performance benchmarking tools
- Developer debugging utilities
- Community examples showcase

### Changed
- Performance optimizations
- Enhanced error handling
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
