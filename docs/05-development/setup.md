# 🚀 Leptos Motion - Development Setup

This guide will help you set up the Leptos Motion development environment using **pnpm**, **Nix**, **Rust**, and **Make**.

## 🎯 **Prerequisites**

### **Required Software**

- **Nix** (with flakes enabled) - [Install Nix](https://nixos.org/download.html)
- **Git** - [Install Git](https://git-scm.com/downloads)
- **Make** - Usually pre-installed on Unix systems

### **Optional Software**

- **VS Code** with Rust extension
- **Direnv** - Automatic environment activation

## 🚀 **Quick Start (Recommended)**

### **1. Clone and Enter the Project**

```bash
git clone https://github.com/cloud-shuttle/leptos-motion.git
cd leptos-motion
```

### **2. Enter Nix Development Environment**

```bash
nix develop
```

This will automatically:

- Install Rust toolchain with WASM support
- Install Node.js 20 and pnpm
- Install all development tools (trunk, cargo-watch, etc.)
- Set up environment variables

### **3. Install Dependencies**

```bash
# Install Node.js dependencies
pnpm install

# Install Playwright browsers
pnpm install:browsers
```

### **4. Start Development**

```bash
# Build everything
make build

# Start development server
make dev

# Run tests
make test
```

## 🔧 **Alternative Setup (Manual)**

If you prefer not to use Nix, you can set up the environment manually:

### **1. Install Rust**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WASM target
rustup target add wasm32-unknown-unknown
```

### **2. Install Rust Tools**

```bash
cargo install trunk
cargo install cargo-watch
cargo install cargo-edit
cargo install cargo-audit
cargo install cargo-tarpaulin
```

### **3. Install Node.js and pnpm**

```bash
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 20
nvm use 20

# Install pnpm
npm install -g pnpm
```

### **4. Install Dependencies**

```bash
pnpm install
pnpm install:browsers
```

## 🛠️ **Available Commands**

### **Make Commands**

```bash
make help              # Show all available commands
make install           # Install all dependencies
make build             # Build all crates and examples
make dev               # Start development server
make test              # Run all tests
make clean             # Clean build artifacts
make format            # Format Rust code
make lint              # Run linters
make check-all         # Run all quality checks
```

### **Nix Commands**

```bash
nix develop            # Enter development environment
nix build              # Build with Nix
nix flake check        # Check flake configuration
```

### **pnpm Commands**

```bash
pnpm test:e2e          # Run E2E tests
pnpm test:e2e:ui       # Run E2E tests with UI
pnpm test:e2e:debug    # Run E2E tests in debug mode
pnpm install:browsers  # Install Playwright browsers
```

## 🧪 **Testing**

### **Rust Tests**

```bash
# Run all Rust tests
cargo test --workspace

# Run specific crate tests
cargo test -p leptos-motion-core

# Run tests with output
cargo test -- --nocapture
```

### **E2E Tests (Playwright)**

```bash
# Run all E2E tests
pnpm test:e2e

# Run with UI
pnpm test:e2e:ui

# Run in debug mode
pnpm test:e2e:debug

# Run specific test file
pnpm test:e2e tests/e2e/playwright_tests.spec.ts
```

### **Performance Tests**

```bash
# Run benchmarks
cargo bench

# Run performance tests
cargo test --test performance
```

## 🔨 **Building**

### **Development Build**

```bash
# Quick check
cargo check

# Development build
cargo build

# Watch for changes
cargo watch -x check -x test
```

### **Release Build**

```bash
# Release build
cargo build --release

# Build examples
cd examples/showcase && trunk build --release
```

### **Documentation**

```bash
# Build docs
cargo doc --workspace --no-deps

# Serve docs locally
cargo doc --workspace --no-deps --open
```

## 🌐 **Development Server**

### **Start Showcase Example**

```bash
# Development mode
cd examples/showcase
trunk serve

# With auto-reload
trunk serve --watch
```

### **Build for Production**

```bash
cd examples/showcase
trunk build --release
```

## 🐛 **Debugging**

### **Rust Debugging**

```bash
# Enable backtraces
export RUST_BACKTRACE=1

# Enable logging
export RUST_LOG=debug

# Run with logging
RUST_LOG=debug cargo test
```

### **WASM Debugging**

```bash
# Enable WASM logging
export RUST_LOG=leptos_motion=debug

# Use browser dev tools
# Check Console tab for Rust logs
```

### **Playwright Debugging**

```bash
# Run with headed mode
pnpm test:e2e:headed

# Run with debug mode
pnpm test:e2e:debug

# Use Playwright Inspector
PWDEBUG=1 pnpm test:e2e
```

## 🔍 **Code Quality**

### **Formatting**

```bash
# Format Rust code
cargo fmt

# Check formatting
cargo fmt --check
```

### **Linting**

```bash
# Run clippy
cargo clippy --workspace -- -D warnings

# Run audit
cargo audit

# Run all quality checks
make check-all
```

## 📦 **Package Management**

### **Rust Dependencies**

```bash
# Add dependency
cargo add package-name

# Add dev dependency
cargo add --dev package-name

# Update dependencies
cargo update

# Check for outdated packages
cargo outdated
```

### **Node.js Dependencies**

```bash
# Add dependency
pnpm add package-name

# Add dev dependency
pnpm add -D package-name

# Update dependencies
pnpm update

# Check for outdated packages
pnpm outdated
```

## 🚀 **CI/CD**

### **Local CI Checks**

```bash
# Run all CI checks locally
make ci-test

# Setup CI environment
make ci-setup
```

### **GitHub Actions**

The project includes GitHub Actions workflows for:

- Rust testing and quality checks
- E2E testing with Playwright
- Documentation building
- Release automation

## 🐧 **Nix Development Environment**

### **What's Included**

- **Rust**: Latest stable with WASM target
- **Node.js**: Version 20
- **pnpm**: Latest version
- **Development Tools**: trunk, cargo-watch, etc.
- **System Tools**: git, ripgrep, fd, bat, exa
- **Build Tools**: make, gcc, pkg-config

### **Environment Variables**

```bash
RUST_BACKTRACE=1
RUST_LOG=info
CARGO_INCREMENTAL=1
RUSTFLAGS="-C target-cpu=native"
```

### **Customizing the Environment**

Edit `flake.nix` to:

- Add more packages
- Change Rust version
- Modify environment variables
- Add custom build steps

## 🔧 **Troubleshooting**

### **Common Issues**

#### **Nix Issues**

```bash
# Enable flakes
mkdir -p ~/.config/nix
echo "experimental-features = nix-command flakes" >> ~/.config/nix/nix.conf

# Clear Nix cache
nix store gc
```

#### **Rust Issues**

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build
```

#### **WASM Issues**

```bash
# Reinstall WASM target
rustup target remove wasm32-unknown-unknown
rustup target add wasm32-unknown-unknown

# Reinstall trunk
cargo install --force trunk
```

#### **Node.js Issues**

```bash
# Clear pnpm cache
pnpm store prune

# Reinstall dependencies
rm -rf node_modules pnpm-lock.yaml
pnpm install
```

### **Getting Help**

- Check the [Rust Book](https://doc.rust-lang.org/book/)
- Check the [Leptos Book](https://leptos.dev/)
- Check the [Trunk Documentation](https://trunkrs.dev/)
- Check the [Playwright Documentation](https://playwright.dev/)

## 🎉 **You're Ready!**

Your development environment is now set up! Start exploring:

1. **Read the code**: Start with `src/lib.rs` and `examples/showcase/`
2. **Run examples**: Use `make dev` to see the showcase
3. **Write tests**: Add tests in the `tests/` directory
4. **Build features**: Work on new animation capabilities

Happy coding! 🦀✨
