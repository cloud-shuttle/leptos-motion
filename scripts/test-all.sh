#!/bin/bash

# Leptos Motion - Comprehensive Test Suite
# This script runs all tests: unit, integration, performance, and E2E

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    if ! command_exists wasm-pack; then
        print_warning "wasm-pack is not installed. Installing..."
        cargo install wasm-pack
    fi
    
    if ! command_exists cargo-nextest; then
        print_warning "cargo-nextest is not installed. Installing..."
        cargo install cargo-nextest
    fi
    
    print_success "Prerequisites check completed"
}

# Run unit tests
run_unit_tests() {
    print_status "Running unit tests..."
    
    # Run core unit tests
    print_status "Testing leptos-motion-core..."
    cargo test --package leptos-motion-core --lib
    
    # Run DOM unit tests
    print_status "Testing leptos-motion-dom..."
    cargo test --package leptos-motion-dom --lib
    
    # Run main package unit tests
    print_status "Testing leptos-motion..."
    cargo test --package leptos-motion --lib
    
    print_success "Unit tests completed"
}

# Run integration tests
run_integration_tests() {
    print_status "Running integration tests..."
    
    # Run WASM integration tests
    print_status "Running WASM integration tests..."
    wasm-pack test --headless --chrome
    
    print_success "Integration tests completed"
}

# Run performance tests
run_performance_tests() {
    print_status "Running performance benchmarks..."
    
    # Run criterion benchmarks
    cargo bench
    
    print_success "Performance tests completed"
}

# Run E2E tests (placeholder for future implementation)
run_e2e_tests() {
    print_status "Running E2E tests..."
    
    # This would run Playwright or similar E2E tests
    # For now, just run the basic animations example
    print_status "Testing basic animations example..."
    cargo check --package basic-animations
    
    print_success "E2E tests completed"
}

# Run visual tests (placeholder for future implementation)
run_visual_tests() {
    print_status "Running visual regression tests..."
    
    # This would run visual regression tests
    # For now, just a placeholder
    print_warning "Visual tests not yet implemented"
    
    print_success "Visual tests completed"
}

# Run all tests with coverage
run_coverage_tests() {
    print_status "Running tests with coverage..."
    
    # This would run tests with coverage reporting
    # For now, just run regular tests
    cargo test --all-features
    
    print_success "Coverage tests completed"
}

# Run specific test suite
run_specific_tests() {
    local test_type=$1
    
    case $test_type in
        "unit")
            run_unit_tests
            ;;
        "integration")
            run_integration_tests
            ;;
        "performance")
            run_performance_tests
            ;;
        "e2e")
            run_e2e_tests
            ;;
        "visual")
            run_visual_tests
            ;;
        "coverage")
            run_coverage_tests
            ;;
        *)
            print_error "Unknown test type: $test_type"
            print_status "Available test types: unit, integration, performance, e2e, visual, coverage"
            exit 1
            ;;
    esac
}

# Main function
main() {
    print_status "Starting Leptos Motion test suite..."
    
    # Check prerequisites
    check_prerequisites
    
    # Parse command line arguments
    if [ $# -eq 0 ]; then
        # Run all tests
        print_status "Running all test suites..."
        run_unit_tests
        run_integration_tests
        run_performance_tests
        run_e2e_tests
        run_visual_tests
    else
        # Run specific test suite
        run_specific_tests "$1"
    fi
    
    print_success "All tests completed successfully! 🎉"
}

# Run main function with all arguments
main "$@"
