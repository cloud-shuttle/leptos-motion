# Release Process

This document outlines the process for releasing new versions of Leptos Motion.

## Release Types

### Major Release (1.0.0, 2.0.0, etc.)

- Breaking changes to the API
- Major new features
- Significant architectural changes

### Minor Release (0.1.0, 0.2.0, etc.)

- New features (backward compatible)
- Performance improvements
- New components or APIs

### Patch Release (0.1.1, 0.1.2, etc.)

- Bug fixes
- Documentation updates
- Minor improvements

## Pre-Release Checklist

### 1. Code Quality

- [ ] All tests passing
- [ ] No clippy warnings
- [ ] Code formatted with `cargo fmt`
- [ ] Documentation updated
- [ ] Examples working

### 2. Testing

- [ ] Unit tests passing
- [ ] Integration tests passing
- [ ] E2E tests passing
- [ ] Performance benchmarks stable
- [ ] All examples compile and run

### 3. Documentation

- [ ] README updated
- [ ] API documentation complete
- [ ] Migration guide updated (if needed)
- [ ] Changelog updated
- [ ] Release notes prepared

### 4. Version Management

- [ ] Version bumped in Cargo.toml
- [ ] Version consistency checked
- [ ] Dependencies updated
- [ ] Lock file committed

## Release Process

### Step 1: Prepare Release Branch

```bash
# Create release branch
git checkout -b release/v0.3.0

# Update version in Cargo.toml
# Update CHANGELOG.md
# Update documentation
```

### Step 2: Run Quality Checks

```bash
# Run all quality checks
./scripts/quality/test-quality.sh

# Run all tests
./scripts/testing/test-all.sh

# Run benchmarks
./scripts/testing/benchmark.sh
```

### Step 3: Update Documentation

```bash
# Update version references
find docs/ -name "*.md" -exec sed -i 's/0\.2\.0/0.3.0/g' {} \;

# Update examples
find examples/ -name "*.toml" -exec sed -i 's/0\.2\.0/0.3.0/g' {} \;
```

### Step 4: Create Release Notes

Create a comprehensive release notes document:

````markdown
# Leptos Motion v0.3.0 Release Notes

## 🎉 What's New

### Major Features

- New MotionDivFlexible component
- Enhanced gesture support
- Improved performance

### Bug Fixes

- Fixed compilation issues in examples
- Resolved type compatibility problems
- Improved error handling

### Breaking Changes

- None in this release

## 📦 Installation

```toml
[dependencies]
leptos-motion = "0.3.0"
```
````

## 🔄 Migration

No migration required for this release.

## 📚 Documentation

- [Getting Started](https://docs.rs/leptos-motion/0.3.0/leptos_motion/)
- [API Reference](https://docs.rs/leptos-motion/0.3.0/leptos_motion/)
- [Examples](https://github.com/cloud-shuttle/leptos-motion/tree/main/examples)

## 🐛 Bug Reports

Found a bug? Please report it on [GitHub Issues](https://github.com/cloud-shuttle/leptos-motion/issues).

## 🙏 Contributors

Thanks to all contributors who made this release possible!

````

### Step 5: Tag Release

```bash
# Create and push tag
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0
````

### Step 6: Publish to Crates.io

```bash
# Publish to crates.io
cargo publish

# Verify publication
cargo search leptos-motion
```

### Step 7: Create GitHub Release

1. Go to GitHub releases page
2. Click "Create a new release"
3. Select the tag created in step 5
4. Add release title and description
5. Attach any additional files
6. Publish the release

### Step 8: Update Documentation

```bash
# Update docs.rs (automatic)
# Update website if applicable
# Update any external references
```

## Post-Release Tasks

### 1. Announce Release

- [ ] Post on social media
- [ ] Update community forums
- [ ] Notify contributors
- [ ] Update project status

### 2. Monitor

- [ ] Watch for issues
- [ ] Monitor download statistics
- [ ] Check for breaking changes
- [ ] Respond to feedback

### 3. Plan Next Release

- [ ] Review roadmap
- [ ] Prioritize features
- [ ] Set next release date
- [ ] Update project status

## Release Automation

### GitHub Actions

The project uses GitHub Actions for automated testing and release:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo fmt --all -- --check

  publish:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo publish --token ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

### Release Scripts

Create release automation scripts:

```bash
#!/bin/bash
# scripts/release/prepare-release.sh

set -e

VERSION=$1
if [ -z "$VERSION" ]; then
    echo "Usage: $0 <version>"
    exit 1
fi

echo "Preparing release $VERSION..."

# Update version
sed -i "s/version = \".*\"/version = \"$VERSION\"/" Cargo.toml

# Update changelog
echo "## [$VERSION] - $(date +%Y-%m-%d)" >> CHANGELOG.md

# Run tests
cargo test --workspace

# Format code
cargo fmt --all

# Check quality
cargo clippy --workspace -- -D warnings

echo "Release $VERSION prepared successfully!"
```

## Emergency Releases

For critical bug fixes:

1. **Assess Impact** - Determine if immediate release is needed
2. **Create Hotfix Branch** - Branch from latest stable
3. **Apply Fix** - Make minimal changes
4. **Test Thoroughly** - Ensure fix doesn't break anything
5. **Release Quickly** - Follow standard process but faster
6. **Communicate** - Notify users of the emergency release

## Release Communication

### Release Announcements

Template for release announcements:

````markdown
🚀 **Leptos Motion v0.3.0 is now available!**

## What's New

- [List key features]
- [List improvements]
- [List bug fixes]

## Installation

```toml
[dependencies]
leptos-motion = "0.3.0"
```
````

## Full Release Notes

[Link to release notes]

## Migration Guide

[Link to migration guide if needed]

Thanks to all contributors! 🙏

```

### Social Media

- **Twitter**: Announce with hashtags #Rust #Leptos #Animation
- **Reddit**: Post in r/rust and r/leptos
- **Discord**: Announce in relevant channels
- **Blog**: Write detailed blog post if major release

## Release Metrics

Track these metrics for each release:

- **Download Count**: Monitor crates.io downloads
- **GitHub Stars**: Track repository growth
- **Issue Reports**: Monitor for new issues
- **Community Feedback**: Track social media engagement
- **Performance**: Monitor benchmark results

## Rollback Plan

If a release has critical issues:

1. **Identify Problem** - Determine scope and impact
2. **Communicate** - Notify users immediately
3. **Create Hotfix** - Develop fix as quickly as possible
4. **Test Thoroughly** - Ensure fix resolves issue
5. **Release Patch** - Follow emergency release process
6. **Monitor** - Watch for additional issues

## Release Schedule

### Regular Releases
- **Major**: Every 6-12 months
- **Minor**: Every 1-3 months
- **Patch**: As needed for bug fixes

### Release Calendar
- **Q1 2024**: v0.3.0 (Current)
- **Q2 2024**: v0.4.0 (Planned)
- **Q3 2024**: v0.5.0 (Planned)
- **Q4 2024**: v1.0.0 (Target)

## Best Practices

1. **Test Early and Often** - Don't wait until release day
2. **Document Everything** - Keep detailed release notes
3. **Communicate Clearly** - Keep users informed
4. **Monitor Feedback** - Respond to issues quickly
5. **Plan Ahead** - Have next release planned
6. **Automate When Possible** - Use scripts and CI/CD
7. **Be Transparent** - Share release process and decisions

---

**Last Updated**: Version 0.3.0 - December 2024

For questions about the release process, please open a GitHub issue or contact the maintainers.
```
