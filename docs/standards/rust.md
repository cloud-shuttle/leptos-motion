# Rust Coding Standards

## Code Quality Requirements
- **Formatting**: All code must be formatted with `rustfmt`
- **Linting**: All code must pass `clippy` with strict settings
- **Documentation**: All public APIs must be documented
- **Testing**: 100% test coverage for all public functions

## Documentation Standards
```rust
/// Processes animation data with the given configuration.
///
/// # Arguments
/// * `data` - The animation data to process
/// * `config` - Configuration for the processing
///
/// # Returns
/// * `Ok(ProcessedData)` - Successfully processed data
/// * `Err(ProcessingError)` - Error during processing
///
/// # Examples
/// ```
/// let data = AnimationData::new();
/// let config = ProcessingConfig::default();
/// let result = process_animation_data(data, config)?;
/// ```
pub fn process_animation_data(
    data: AnimationData,
    config: ProcessingConfig,
) -> Result<ProcessedData, ProcessingError> {
    // Implementation
}
```
