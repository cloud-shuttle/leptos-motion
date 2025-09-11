# ADR-001: Package Manager - pnpm

## Status
Accepted

## Context
The leptos-motion project requires a reliable, fast, and efficient package manager for managing JavaScript/TypeScript dependencies. We need to choose between npm, yarn, and pnpm for our development workflow, CI/CD pipelines, and end-user experience.

Key considerations:
- Performance and speed of dependency installation
- Disk space efficiency
- Monorepo support
- Lock file reliability
- Developer experience
- CI/CD integration

## Decision
We will use **pnpm** as our primary package manager for all JavaScript/TypeScript dependencies.

### Rationale

1. **Performance**: pnpm is significantly faster than npm and yarn, especially for monorepos
2. **Disk Efficiency**: Uses hard links and symlinks to avoid duplicating packages across projects
3. **Strict Dependency Resolution**: Prevents phantom dependencies and ensures reproducible builds
4. **Monorepo Support**: Excellent support for workspace management
5. **Lock File Reliability**: More reliable than npm's package-lock.json
6. **Modern Tooling**: Built with modern Node.js practices and actively maintained

### Implementation

- All `package.json` files specify `"packageManager": "pnpm@8.15.0"`
- Remove all `package-lock.json` files from the repository
- Use `pnpm-lock.yaml` as the single source of truth for dependencies
- Update all documentation to reference pnpm commands instead of npm
- Configure CI/CD pipelines to use pnpm
- Use pnpm workspaces for monorepo management

## Consequences

### Positive
- **Faster CI/CD**: Reduced build times due to faster dependency installation
- **Reduced Disk Usage**: Significant savings in disk space, especially in CI environments
- **Better Dependency Management**: Stricter dependency resolution prevents common issues
- **Improved Developer Experience**: Faster `pnpm install` commands
- **Monorepo Efficiency**: Better workspace management and dependency sharing

### Negative
- **Learning Curve**: Team members need to learn pnpm-specific commands
- **Tool Compatibility**: Some tools may not be fully compatible with pnpm's symlink structure
- **Migration Effort**: Need to migrate existing npm-based workflows

### Neutral
- **Lock File Format**: Different lock file format (pnpm-lock.yaml vs package-lock.json)
- **Command Differences**: Slight differences in command syntax compared to npm

## Implementation Notes

### Migration Steps
1. Remove all `package-lock.json` files
2. Update `package.json` files to specify pnpm as package manager
3. Run `pnpm install` to generate `pnpm-lock.yaml`
4. Update documentation and scripts to use pnpm commands
5. Configure CI/CD to use pnpm

### Commands Reference
```bash
# Install dependencies
pnpm install

# Add dependency
pnpm add <package>

# Add dev dependency
pnpm add -D <package>

# Run scripts
pnpm run <script>

# Workspace commands
pnpm -r <command>  # Run command in all workspaces
```

### CI/CD Configuration
```yaml
# GitHub Actions example
- name: Setup pnpm
  uses: pnpm/action-setup@v2
  with:
    version: 8.15.0

- name: Get pnpm store directory
  shell: bash
  run: |
    echo "STORE_PATH=$(pnpm store path --silent)" >> $GITHUB_ENV

- name: Setup pnpm cache
  uses: actions/cache@v3
  with:
    path: ${{ env.STORE_PATH }}
    key: ${{ runner.os }}-pnpm-store-${{ hashFiles('**/pnpm-lock.yaml') }}
    restore-keys: |
      ${{ runner.os }}-pnpm-store-
```

## References
- [pnpm official documentation](https://pnpm.io/)
- [pnpm vs npm vs yarn comparison](https://pnpm.io/benchmarks)
- [pnpm workspace documentation](https://pnpm.io/workspaces)
