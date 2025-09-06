# Pre-commit Hooks Setup

This document describes the pre-commit hooks configuration for the leptos-motion project.

## Overview

Pre-commit hooks automatically run code quality checks and formatting before each commit, ensuring consistent code quality across the project.

## Installation

### Automatic Installation

Run the following command to install pre-commit hooks:

```bash
make pre-commit-install
```

Or manually:

```bash
pre-commit install
pre-commit install --hook-type commit-msg
```

### Manual Installation

If you prefer to install manually:

```bash
pip install pre-commit
pre-commit install
pre-commit install --hook-type commit-msg
```

## Configuration

The pre-commit configuration is defined in `.pre-commit-config.yaml` and includes:

### General File Checks

- Trailing whitespace removal
- End-of-file fixing
- YAML, JSON, and TOML validation
- Merge conflict detection
- Large file detection

### Rust-Specific Hooks

- Code formatting with `cargo fmt`
- Linting with `cargo clippy`
- Compilation checks with `cargo check`

### Security Checks

- Dependency auditing with `cargo audit`

### Documentation

- Markdown linting with markdownlint
- Link validation

### TypeScript/JavaScript

- ESLint for code quality
- Prettier for formatting

### Custom Hooks

- Version consistency across workspace
- Example build validation
- Documentation build checks
- TODO/FIXME detection in production code
- Large binary file detection
- Cargo.toml validation

## Usage

### Running All Hooks

```bash
make pre-commit
```

Or manually:

```bash
pre-commit run --all-files
```

### Running Specific Hooks

```bash
pre-commit run <hook-name> --all-files
```

### Updating Hooks

```bash
make pre-commit-update
```

Or manually:

```bash
pre-commit autoupdate
```

## Available Hooks

| Hook Name                   | Description                             |
| --------------------------- | --------------------------------------- |
| `trailing-whitespace`       | Remove trailing whitespace              |
| `end-of-file-fixer`         | Ensure files end with newline           |
| `check-yaml`                | Validate YAML files                     |
| `check-json`                | Validate JSON files                     |
| `check-toml`                | Validate TOML files                     |
| `fmt`                       | Format Rust code                        |
| `clippy`                    | Lint Rust code                          |
| `cargo-check`               | Check Rust compilation                  |
| `cargo-audit`               | Security audit                          |
| `markdownlint`              | Lint markdown files                     |
| `eslint`                    | Lint TypeScript/JavaScript              |
| `prettier`                  | Format code files                       |
| `check-version-consistency` | Check workspace version consistency     |
| `check-examples-build`      | Validate example builds                 |
| `check-docs-build`          | Validate documentation builds           |
| `check-todos`               | Check for TODO/FIXME in production code |
| `check-large-files`         | Check for large binary files            |
| `validate-cargo-toml`       | Validate Cargo.toml files               |

## Configuration Files

- `.pre-commit-config.yaml` - Main pre-commit configuration
- `.markdownlint.yaml` - Markdown linting rules
- `.eslintrc.js` - ESLint configuration
- `.prettierrc` - Prettier formatting rules

## Troubleshooting

### Common Issues

1. **Hook fails with "command not found"**
   - Ensure the required tools are installed (cargo, node, python)
   - Run `pre-commit install` to ensure hooks are properly installed

2. **Formatting changes are made automatically**
   - This is expected behavior - the hooks fix formatting issues
   - Review the changes and commit them

3. **Hooks are too slow**
   - Some hooks can be slow on large codebases
   - Consider running specific hooks instead of all hooks
   - Use `pre-commit run --files <file>` to run on specific files

4. **Version consistency check fails**
   - Ensure all crates use workspace version references
   - Check that workspace version is consistent across all Cargo.toml files

### Skipping Hooks

To skip hooks for a specific commit:

```bash
git commit --no-verify -m "commit message"
```

**Note**: Only skip hooks when absolutely necessary, as they help maintain code quality.

## Customization

### Adding New Hooks

Edit `.pre-commit-config.yaml` to add new hooks:

```yaml
- repo: local
  hooks:
    - id: my-custom-hook
      name: My Custom Hook
      entry: ./scripts/my-custom-hook.sh
      language: system
      files: ^.*\.rs$
```

### Modifying Existing Hooks

Edit the configuration in `.pre-commit-config.yaml` and run:

```bash
pre-commit autoupdate
```

## Integration with CI/CD

The pre-commit hooks can be integrated into CI/CD pipelines:

```yaml
- name: Run pre-commit hooks
  run: pre-commit run --all-files
```

## Best Practices

1. **Always run hooks before committing**
2. **Don't skip hooks unless absolutely necessary**
3. **Keep hooks up to date**
4. **Add new hooks for project-specific requirements**
5. **Document any custom hooks**

## Support

For issues with pre-commit hooks:

1. Check the pre-commit documentation: https://pre-commit.com/
2. Review the project's `.pre-commit-config.yaml`
3. Check the pre-commit log: `~/.cache/pre-commit/pre-commit.log`
4. Open an issue in the project repository
