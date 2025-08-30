# Changelog

All notable changes to Leptos Motion will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Advanced gesture recognition system
- Scroll-triggered animations
- Additional easing functions
- Performance monitoring tools
- More animation presets

### Changed
- Improved performance for concurrent animations
- Enhanced memory management
- Better error handling

### Fixed
- Memory leaks in motion value subscriptions
- Gesture conflict resolution
- Layout animation edge cases

## [0.1.0-alpha] - 2024-01-XX

### Added
- **Core Animation Engine**
  - High-performance animation engine with spring physics
  - Support for multiple easing functions (linear, ease-in, ease-out, ease-in-out, back, spring)
  - Interpolation for numbers, pixels, degrees, colors, and transforms
  - Animation lifecycle management with start, update, and complete callbacks

- **Motion Components**
  - `MotionDiv` component with full animation support
  - `MotionSpan` component for inline animations
  - `AnimatePresence` for exit animations
  - Support for initial, animate, exit, and transition props

- **Gesture Interactions**
  - Hover animations with `while_hover`
  - Tap animations with `while_tap`
  - Focus animations with `while_focus`
  - Drag interactions with constraints and axis control

- **Advanced Animation Patterns**
  - Variants for reusable animation states
  - Layout animations with automatic transitions
  - Staggered animations with configurable delays
  - Keyframe animations with complex sequences

- **Motion Values**
  - Reactive `MotionValue<T>` for tracking animation state
  - Velocity tracking and subscription system
  - Specialized `MotionNumber` and `MotionTransform` types
  - `MotionValues` collection for managing multiple values

- **Performance Features**
  - GPU-accelerated transform animations
  - RequestAnimationFrame optimization
  - Memory-efficient subscription system
  - Bundle size optimization with tree shaking

- **Developer Experience**
  - Type-safe `motion_target!` macro for creating animation targets
  - Comprehensive error handling with `AnimationError`
  - Debug logging and performance monitoring
  - Full Rust documentation with examples

- **Examples and Documentation**
  - Comprehensive API reference
  - Getting started guide with tutorials
  - Performance optimization guide
  - Interactive showcase with all features
  - E-commerce product gallery example
  - Dashboard application example

### Technical Details
- **Bundle Size**: Core library <30KB, full library <50KB
- **Performance**: 60fps for 100+ simultaneous animations
- **Memory Usage**: <10MB for typical applications
- **Browser Support**: All modern browsers with Web Animations API fallback
- **Leptos Compatibility**: Version 0.7+

### Breaking Changes
This is the initial release, so there are no breaking changes from previous versions.

### Known Issues
- Some advanced gesture features are still in development
- Layout animations may have edge cases with complex layouts
- Performance optimization is ongoing

## [0.0.1] - 2024-01-XX

### Added
- Initial project setup
- Basic animation engine architecture
- Core type definitions
- Project documentation structure

---

## Version History

### Alpha Releases
- **0.1.0-alpha**: Initial alpha release with core functionality
- **0.0.1**: Project initialization and setup

### Planned Releases
- **0.1.1**: Bug fixes and minor improvements
- **0.2.0**: Advanced features and performance improvements
- **1.0.0**: Production-ready release with full feature set

## Contributing

To contribute to this changelog, please follow the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format and add your changes under the appropriate section.

### Changelog Categories

- **Added**: New features
- **Changed**: Changes in existing functionality
- **Deprecated**: Features that will be removed in a future release
- **Removed**: Features that have been removed
- **Fixed**: Bug fixes
- **Security**: Security-related changes

### Example Entry

```markdown
## [1.2.3] - 2024-01-15

### Added
- New `MotionButton` component for button animations
- Support for custom easing functions

### Changed
- Improved performance of spring animations by 15%
- Updated minimum Leptos version to 0.7.1

### Fixed
- Memory leak in motion value subscriptions
- Incorrect interpolation for color values

### Security
- Updated dependencies to fix CVE-2024-XXXX
```

---

**Note**: This changelog is maintained by the Leptos Motion team. For questions or suggestions, please open an issue on GitHub.
