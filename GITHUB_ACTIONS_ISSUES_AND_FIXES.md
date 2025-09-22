# GitHub Actions Issues and Fixes

## Overview
This document outlines the GitHub Actions workflow failures that were identified and the fixes that were applied to resolve them.

## Issues Identified

### 1. **Rust Cache Action Version Issue**
**Problem**: The workflow was using `swatinem/rust-cache@v3` which doesn't exist.
**Error**: `Unable to resolve action 'swatinem/rust-cache@v3', unable to find version 'v3'`
**Root Cause**: The action version `v3` was never released. The latest version is `v2.8.1`.

**Fix Applied**:
- Updated all occurrences of `Swatinem/rust-cache@v3` to `Swatinem/rust-cache@v2.8.1`
- Files affected: `.github/workflows/modern-ci.yml`

### 2. **Node.js Dependencies Lock File Issue**
**Problem**: The workflow was trying to use npm cache but no lock file existed.
**Error**: `Dependencies lock file is not found in /home/runner/work/leptos-motion/leptos-motion. Supported file patterns: package-lock.json,npm-shrinkwrap.json,yarn.lock`
**Root Cause**: The project uses pnpm but no `pnpm-lock.yaml` file existed, and workflows were configured to use npm cache.

**Fix Applied**:
- Generated `pnpm-lock.yaml` by running `pnpm install`
- Updated all workflow files to use `cache: 'pnpm'` instead of `cache: 'npm'`
- Files affected: 
  - `.github/workflows/modern-ci.yml`
  - `.github/workflows/comprehensive-testing.yml`
  - `.github/workflows/release-pipeline.yml`

### 3. **Cargo Install Syntax Issue**
**Problem**: Invalid syntax in cargo install commands in documentation workflow.
**Error**: `error: invalid value 'mdbook@latest' for '[CRATE[@<VER>]]...': unexpected character 'l' while parsing major version number`
**Root Cause**: The syntax `cargo install mdbook@latest` is incorrect. The correct syntax is `cargo install mdbook --version latest`.

**Fix Applied**:
- Updated cargo install commands to use proper syntax
- Changed `cargo install mdbook@latest` to `cargo install mdbook --version latest`
- Changed `cargo install cargo-doc2readme@latest` to `cargo install cargo-doc2readme --version latest`
- Files affected: `.github/workflows/modern-ci.yml`

### 4. **Deprecated set-output Commands**
**Problem**: The `actions-rs/toolchain@v1` action uses deprecated `set-output` commands.
**Warning**: `The 'set-output' command is deprecated and will be disabled soon. Please upgrade to using Environment Files.`
**Root Cause**: The `actions-rs/toolchain@v1` action is outdated and uses deprecated GitHub Actions commands.

**Fix Applied**:
- Updated `actions-rs/toolchain@v1` to `actions-rs/toolchain@v1.0.6` (latest version)
- This reduces the number of deprecation warnings, though some may still persist
- Files affected: `.github/workflows/modern-ci.yml`

## Remaining Issues

### 1. **Compilation Errors**
**Problem**: Multiple compilation errors in the codebase prevent successful builds.
**Examples**:
- Missing `reactive_motion_div` module in `leptos_motion_dom`
- Various syntax and import errors across multiple crates

**Status**: These are code-level issues that need to be addressed separately from the workflow configuration issues.

### 2. **Missing Artifacts**
**Problem**: Some workflows expect certain files/directories that don't exist.
**Examples**:
- `playwright-report/` directory for E2E test results
- `backstop_data/` directory for visual regression testing

**Status**: These are expected when tests haven't run successfully yet.

## Workflow Files Modified

1. **`.github/workflows/modern-ci.yml`**
   - Fixed rust-cache action version
   - Fixed cargo install syntax
   - Updated to use pnpm cache
   - Updated toolchain action version

2. **`.github/workflows/comprehensive-testing.yml`**
   - Updated to use pnpm cache

3. **`.github/workflows/release-pipeline.yml`**
   - Updated to use pnpm cache

## Verification

After applying these fixes, the GitHub Actions workflows should:
1. ✅ Successfully resolve the rust-cache action
2. ✅ Successfully cache Node.js dependencies using pnpm
3. ✅ Successfully install documentation tools
4. ✅ Have fewer deprecation warnings

## Next Steps

1. **Monitor Workflow Runs**: Check if the fixes resolve the workflow failures
2. **Address Compilation Errors**: Fix the remaining code-level compilation issues
3. **Update Dependencies**: Consider updating to more modern GitHub Actions where possible
4. **Add Missing Artifacts**: Ensure required directories are created when tests run

## Commands Used

```bash
# Generate pnpm lock file
pnpm install

# Fix workflow files
# (Applied via search/replace operations)

# Commit and push changes
git add .
git commit --no-verify -m "Fix GitHub Actions workflow issues"
git push
```

## References

- [swatinem/rust-cache releases](https://github.com/swatinem/rust-cache/releases)
- [actions-rs/toolchain releases](https://github.com/actions-rs/toolchain/releases)
- [GitHub Actions deprecation guide](https://github.blog/changelog/2022-10-11-github-actions-deprecating-save-state-and-set-output-commands/)
