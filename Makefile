# Leptos Motion Makefile
# Provides convenient commands for development, testing, and contract validation

.PHONY: help build test contract-tests clean format lint check-all

# Default target
help:
	@echo "Leptos Motion - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  build          - Build all crates"
	@echo "  test           - Run all tests"
	@echo "  contract-tests - Run contract tests"
	@echo "  clean          - Clean build artifacts"
	@echo ""
	@echo "Code Quality:"
	@echo "  format         - Format all code"
	@echo "  lint           - Run clippy lints"
	@echo "  check-all      - Run all checks (format, lint, test, contract-tests)"
	@echo ""
	@echo "Contract Testing:"
	@echo "  contract-api   - Run API contract tests"
	@echo "  contract-perf  - Run performance contract tests"
	@echo "  contract-mem   - Run memory contract tests"
	@echo "  contract-error - Run error handling contract tests"
	@echo "  contract-cross - Run cross-crate contract tests"
	@echo ""
	@echo "Demos:"
	@echo "  demo-basic     - Run basic reactive demo"
	@echo "  demo-showcase  - Run comprehensive showcase demo"
	@echo "  demo-webgl     - Run WebGL demo"
	@echo ""

# Build all crates
build:
	cargo build --workspace

# Run all tests
test:
	cargo test --workspace

# Run contract tests
contract-tests:
	cargo test --package leptos-motion-contracts --lib contract_tests

# Clean build artifacts
clean:
	cargo clean

# Format all code
format:
	cargo fmt --all

# Run clippy lints
lint:
	cargo clippy --workspace -- -D warnings

# Run all checks
check-all: format lint test contract-tests
	@echo "✅ All checks passed!"

# Contract test targets
contract-api:
	cargo test --package leptos-motion-contracts --lib api_contracts

contract-perf:
	cargo test --package leptos-motion-contracts --lib performance_contracts

contract-mem:
	cargo test --package leptos-motion-contracts --lib memory_contracts

contract-error:
	cargo test --package leptos-motion-contracts --lib error_contracts

contract-cross:
	cargo test --package leptos-motion-contracts --lib cross_crate_contracts

# Demo targets
demo-basic:
	cd demos/basic/reactive-demo && trunk serve --open

demo-showcase:
	cd demos/showcase/comprehensive-demo && trunk serve --open

demo-webgl:
	cd demos/advanced/webgl-demo && trunk serve --open

# CI/CD targets
ci-test: format lint test contract-tests
	@echo "✅ CI tests passed!"

ci-contract: contract-tests
	@echo "✅ Contract tests passed!"

# Development targets
dev-setup:
	@echo "Setting up development environment..."
	rustup component add rustfmt clippy
	@echo "✅ Development environment ready!"

# Contract test report generation
contract-report:
	cargo test --package leptos-motion-contracts --lib contract_tests -- --nocapture > contract_test_report.txt
	@echo "Contract test report generated: contract_test_report.txt"

# Performance benchmarks
bench:
	cargo bench --workspace

# Documentation
docs:
	cargo doc --workspace --open

# Release preparation
release-check: check-all bench
	@echo "✅ Ready for release!"

# Contract test validation
validate-contracts:
	@echo "Validating all contracts..."
	@make contract-api
	@make contract-perf
	@make contract-mem
	@make contract-error
	@make contract-cross
	@echo "✅ All contracts validated!"

# Quick development cycle
dev: format lint test
	@echo "✅ Development cycle complete!"

# Full validation (for CI)
validate-all: format lint test contract-tests bench
	@echo "✅ Full validation complete!"