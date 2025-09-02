# Leptos Motion - Development Makefile
# Usage: make <target>

.PHONY: help install build test clean dev format lint check-all release install-browsers

# Default target
help:
	@echo "🚀 Leptos Motion Development Commands"
	@echo ""
	@echo "📦 Setup & Installation:"
	@echo "  install          Install all dependencies (Rust + Node.js)"
	@echo "  install-browsers Install Playwright browsers"
	@echo ""
	@echo "🔨 Build & Development:"
	@echo "  build            Build all Rust crates and examples"
	@echo "  dev              Start development server"
	@echo "  watch            Watch for changes and rebuild"
	@echo ""
	@echo "🧪 Testing:"
	@echo "  test             Run all tests (Rust + E2E)"
	@echo "  test-rust        Run Rust tests only"
	@echo "  test-e2e         Run Playwright E2E tests"
	@echo "  test-e2e-ui      Run E2E tests with UI"
	@echo "  test-e2e-debug   Run E2E tests in debug mode"
	@echo ""
	@echo "🔍 Quality & Linting:"
	@echo "  format           Format Rust code"
	@echo "  lint             Run clippy and other linters"
	@echo "  check-all        Run all quality checks"
	@echo ""
	@echo "🧹 Maintenance:"
	@echo "  clean            Clean all build artifacts"
	@echo "  clean-deps       Clean dependencies"
	@echo ""
	@echo "📚 Documentation:"
	@echo "  docs             Build documentation"
	@echo "  docs-serve       Serve documentation locally"
	@echo ""
	@echo "🚀 Release:"
	@echo "  release          Prepare release build"
	@echo "  publish          Publish to crates.io"

# Installation
install: install-rust install-node

install-rust:
	@echo "🔧 Installing Rust dependencies..."
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	cargo install cargo-watch
	cargo install cargo-edit
	cargo install cargo-audit
	cargo install cargo-tarpaulin

install-node:
	@echo "📦 Installing Node.js dependencies..."
	pnpm install

install-browsers:
	@echo "🌐 Installing Playwright browsers..."
	pnpm install:browsers

# Build targets
build: build-rust build-examples

build-rust:
	@echo "🔨 Building Rust crates..."
	cargo build --release

build-examples:
	@echo "🎨 Building examples..."
	cd examples/showcase && trunk build

# Development
dev:
	@echo "🚀 Starting development server..."
	cd examples/showcase && trunk serve --open

watch:
	@echo "👀 Watching for changes..."
	cargo watch -x check -x test -x run

# Testing
test: test-rust test-e2e

test-rust:
	@echo "🧪 Running Rust tests..."
	cargo test --workspace

test-e2e:
	@echo "🌐 Running E2E tests..."
	pnpm test:e2e

test-e2e-ui:
	@echo "🖥️  Running E2E tests with UI..."
	pnpm test:e2e:ui

test-e2e-debug:
	@echo "🐛 Running E2E tests in debug mode..."
	pnpm test:e2e:debug

# Quality checks
format:
	@echo "✨ Formatting Rust code..."
	cargo fmt

lint:
	@echo "🔍 Running linters..."
	cargo clippy --workspace -- -D warnings
	cargo audit

check-all: format lint test
	@echo "✅ All quality checks passed!"

# Documentation
docs:
	@echo "📚 Building documentation..."
	cargo doc --workspace --no-deps

docs-serve:
	@echo "🌐 Serving documentation..."
	cargo doc --workspace --no-deps --open

# Cleaning
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf target/
	rm -rf dist/
	rm -rf examples/*/dist/
	rm -rf examples/*/target/

clean-deps:
	@echo "🧹 Cleaning dependencies..."
	rm -rf node_modules/
	rm -rf .pnpm-store/
	rm -rf target/

# Release
release: clean build check-all
	@echo "🚀 Preparing release build..."
	cargo build --release
	@echo "✅ Release build ready!"

publish:
	@echo "📦 Publishing to crates.io..."
	cargo publish --workspace

# Nix development environment
nix-shell:
	@echo "🐧 Entering Nix development environment..."
	nix develop

nix-build:
	@echo "🔨 Building with Nix..."
	nix build

nix-check:
	@echo "✅ Running Nix checks..."
	nix flake check

# Performance testing
bench:
	@echo "⚡ Running benchmarks..."
	cargo bench

profile:
	@echo "📊 Running performance profiling..."
	cargo build --release
	cd examples/showcase && trunk build --release

# Docker (optional)
docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t leptos-motion .

docker-run:
	@echo "🐳 Running Docker container..."
	docker run -p 3000:3000 leptos-motion

# CI/CD helpers
ci-setup:
	@echo "🔧 Setting up CI environment..."
	rustup target add wasm32-unknown-unknown
	cargo install trunk
	pnpm install
	pnpm install:browsers

ci-test:
	@echo "🧪 Running CI tests..."
	cargo test --workspace
	cargo clippy --workspace -- -D warnings
	cargo fmt --check
	pnpm test:e2e

# Development utilities
update-deps:
	@echo "🔄 Updating dependencies..."
	cargo update
	pnpm update

check-updates:
	@echo "🔍 Checking for updates..."
	cargo outdated
	pnpm outdated

# Quick development commands
quick-test:
	@echo "⚡ Quick test run..."
	cargo test --lib

quick-build:
	@echo "⚡ Quick build..."
	cargo check

quick-dev:
	@echo "⚡ Quick dev server..."
	cd examples/showcase && trunk serve

# Helpers for common tasks
fix:
	@echo "🔧 Fixing common issues..."
	cargo fix --allow-dirty
	cargo fmt

setup-dev:
	@echo "🚀 Setting up development environment..."
	make install
	make install-browsers
	make build
	@echo "✅ Development environment ready!"

# Show project status
status:
	@echo "📊 Project Status:"
	@echo "  Rust version: $(shell rustc --version)"
	@echo "  Cargo version: $(shell cargo --version)"
	@echo "  Node version: $(shell node --version)"
	@echo "  pnpm version: $(shell pnpm --version)"
	@echo "  Trunk version: $(shell trunk --version)"
	@echo "  Git status: $(shell git status --porcelain | wc -l) changes"
	@echo "  Build status: $(shell if [ -d "target/release" ]; then echo "✅ Built"; else echo "❌ Not built"; fi)"
