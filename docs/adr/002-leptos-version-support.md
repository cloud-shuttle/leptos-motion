# ADR-002: Leptos Version Support Strategy

## Status

Accepted

## Context

The leptos-motion library is built on top of the Leptos reactive UI framework. We need to establish a clear strategy for supporting different versions of Leptos to ensure compatibility, maintainability, and provide a clear upgrade path for users.

Key considerations:

- Leptos is rapidly evolving with frequent releases
- Breaking changes in Leptos can affect our API compatibility
- Users need predictable upgrade paths
- We want to leverage the latest Leptos features and improvements
- Backward compatibility vs. forward compatibility trade-offs

## Decision

We will maintain support for the **latest stable version of Leptos** and provide a clear upgrade strategy.

### Version Support Policy

1. **Primary Support**: Always support the latest stable version of Leptos
2. **Current Target**: Leptos v0.8.8 (as of this ADR)
3. **Upgrade Strategy**: Proactive updates to new Leptos versions within 2-4 weeks of release
4. **Breaking Changes**: Handle Leptos breaking changes through our own API abstractions
5. **Documentation**: Maintain clear migration guides for Leptos version updates

### Implementation Strategy

#### Version Management

- Update `Cargo.toml` dependencies to use the latest stable Leptos version
- Use semantic versioning for our own releases to indicate Leptos compatibility
- Maintain a compatibility matrix in our documentation

#### API Abstraction

- Create abstraction layers to minimize direct exposure to Leptos internals
- Use trait-based designs to allow for future Leptos API changes
- Provide our own component APIs that can adapt to Leptos changes

#### Testing Strategy

- Test against the latest Leptos version in CI/CD
- Maintain compatibility tests that verify our APIs work with target Leptos versions
- Use feature flags for experimental Leptos features

#### Documentation

- Clear version compatibility information in README
- Migration guides for Leptos version updates
- Changelog entries that indicate Leptos version requirements

## Consequences

### Positive

- **Latest Features**: Access to the newest Leptos features and improvements
- **Performance**: Benefit from Leptos performance optimizations
- **Security**: Receive security updates and bug fixes promptly
- **Community Alignment**: Stay aligned with the Leptos ecosystem
- **Developer Experience**: Users get the best experience with the latest Leptos

### Negative

- **Breaking Changes**: Need to handle Leptos breaking changes regularly
- **Maintenance Overhead**: Regular updates require testing and validation
- **User Migration**: Users may need to update Leptos versions to use our library
- **Compatibility Issues**: Potential issues with users on older Leptos versions

### Neutral

- **Release Cadence**: Our releases may be tied to Leptos release cycles
- **Documentation Updates**: Need to maintain compatibility documentation

## Implementation Notes

### Dependency Management

```toml
# Cargo.toml
[dependencies]
leptos = { version = "0.8", features = ["csr", "hydrate"] }
leptos_meta = "0.8"
leptos_router = "0.8"
```

### Version Compatibility Matrix

| leptos-motion | Leptos | Status         |
| ------------- | ------ | -------------- |
| 0.8.3+        | 0.8.8+ | ✅ Supported   |
| 0.8.2         | 0.8.7  | ⚠️ Deprecated  |
| 0.8.1         | 0.8.6  | ❌ Unsupported |

### Migration Process

1. **Monitor**: Track Leptos releases and breaking changes
2. **Test**: Run our test suite against new Leptos versions
3. **Update**: Update dependencies and fix any breaking changes
4. **Validate**: Ensure all features work correctly
5. **Release**: Publish new version with updated Leptos support
6. **Document**: Update compatibility matrix and migration guides

### Breaking Change Handling

- Use feature flags for experimental Leptos features
- Provide migration guides for significant API changes
- Maintain backward compatibility where possible through adapter patterns
- Clear deprecation warnings with migration paths

### Testing Strategy

```rust
// Example compatibility test
#[cfg(test)]
mod leptos_compatibility {
    use leptos::*;
    use leptos_motion::*;

    #[test]
    fn test_motion_div_compatibility() {
        // Test that our components work with current Leptos version
        let _ = view! {
            <MotionDiv
                initial=vec![("opacity", 0.0)]
                animate=vec![("opacity", 1.0)]
                transition=Transition::default()
            >
                "Test content"
            </MotionDiv>
        };
    }
}
```

## References

- [Leptos GitHub Repository](https://github.com/leptos-rs/leptos)
- [Leptos Documentation](https://leptos.dev/)
- [Leptos Changelog](https://github.com/leptos-rs/leptos/blob/main/CHANGELOG.md)
