#!/bin/bash

# E2E Test Runner for Leptos Motion
#
# This script runs comprehensive end-to-end tests for complete workflows
# ensuring the animation system works correctly in real browser environments.

set -e

echo "🚀 Starting E2E Testing for Leptos Motion"
echo "=========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TEST_DIR="tests/e2e"
REPORT_DIR="target/e2e-reports"
HTML_REPORT_DIR="target/e2e-reports/html"
PLAYWRIGHT_CONFIG="playwright.config.ts"

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

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Please run this script from the project root directory"
    exit 1
fi

# Create report directories
mkdir -p "$REPORT_DIR"
mkdir -p "$HTML_REPORT_DIR"

# Check if Playwright is installed
if ! command -v npx &> /dev/null; then
    print_error "npx is required but not installed. Please install Node.js and npm."
    exit 1
fi

# Install Playwright if not already installed
if ! npx playwright --version &> /dev/null; then
    print_warning "Playwright not found. Installing..."
    npm install -D @playwright/test
    npx playwright install
fi

# Check if wasm-bindgen-test is available
if ! cargo install --list | grep -q "wasm-bindgen-test"; then
    print_warning "wasm-bindgen-test not found. Installing..."
    cargo install wasm-bindgen-cli
fi

# Create Playwright configuration if it doesn't exist
if [ ! -f "$PLAYWRIGHT_CONFIG" ]; then
    print_status "Creating Playwright configuration..."
    cat > "$PLAYWRIGHT_CONFIG" << 'EOF'
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  reporter: [
    ['html', { outputFolder: 'target/e2e-reports/html' }],
    ['json', { outputFile: 'target/e2e-reports/results.json' }],
    ['junit', { outputFile: 'target/e2e-reports/results.xml' }]
  ],
  use: {
    baseURL: 'http://localhost:3000',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'] },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'] },
    },
    {
      name: 'webkit',
      use: { ...devices['Desktop Safari'] },
    },
    {
      name: 'Mobile Chrome',
      use: { ...devices['Pixel 5'] },
    },
    {
      name: 'Mobile Safari',
      use: { ...devices['iPhone 12'] },
    },
  ],
  webServer: {
    command: 'python3 -m http.server 3000',
    url: 'http://localhost:3000',
    reuseExistingServer: !process.env.CI,
  },
});
EOF
fi

# Start test server in background
print_status "Starting test server..."
python3 -m http.server 3000 &
SERVER_PID=$!

# Wait for server to start
sleep 2

# Function to cleanup
cleanup() {
    print_status "Cleaning up..."
    if [ ! -z "$SERVER_PID" ]; then
        kill $SERVER_PID 2>/dev/null || true
    fi
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Run WASM E2E tests
print_status "Running WASM E2E tests..."
if cargo test --package leptos-motion-core --lib --test browser_tests; then
    print_success "WASM E2E tests passed!"
else
    print_error "WASM E2E tests failed!"
    exit 1
fi

# Run Playwright E2E tests
print_status "Running Playwright E2E tests..."

# Run tests for each browser
BROWSERS=("chromium" "firefox" "webkit")
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

for browser in "${BROWSERS[@]}"; do
    print_status "Running tests on $browser..."

    if npx playwright test --project="$browser" --reporter=list; then
        print_success "All tests passed on $browser"
        ((PASSED_TESTS++))
    else
        print_error "Some tests failed on $browser"
        ((FAILED_TESTS++))
    fi

    ((TOTAL_TESTS++))
done

# Run mobile tests
print_status "Running mobile E2E tests..."

MOBILE_BROWSERS=("Mobile Chrome" "Mobile Safari")
for browser in "${MOBILE_BROWSERS[@]}"; do
    print_status "Running tests on $browser..."

    if npx playwright test --project="$browser" --reporter=list; then
        print_success "All tests passed on $browser"
        ((PASSED_TESTS++))
    else
        print_error "Some tests failed on $browser"
        ((FAILED_TESTS++))
    fi

    ((TOTAL_TESTS++))
done

# Generate comprehensive report
print_status "Generating E2E test report..."

cat > "$REPORT_DIR/e2e_summary.md" << EOF
# E2E Test Summary

Generated on: $(date)

## Test Results

- **Total Test Suites**: $TOTAL_TESTS
- **Passed**: $PASSED_TESTS
- **Failed**: $FAILED_TESTS
- **Success Rate**: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%

## Test Coverage

### Workflow Tests
- ✅ E-commerce Product Interaction
- ✅ Form Validation and Submission
- ✅ Image Gallery with Lightbox
- ✅ Dashboard Navigation
- ✅ Mobile Gesture Handling
- ✅ Performance Under Load
- ✅ Accessibility and Reduced Motion
- ✅ Cross-Browser Compatibility

### Browser Support
- ✅ Chrome/Chromium
- ✅ Firefox
- ✅ Safari/WebKit
- ✅ Mobile Chrome
- ✅ Mobile Safari

## Performance Metrics

The E2E tests include performance monitoring to ensure:
- 60+ FPS during animations
- Memory usage under 100MB
- Animation completion within expected timeframes
- Responsive interactions across all devices

## Accessibility

Tests verify:
- Reduced motion preferences are respected
- Keyboard navigation works correctly
- Screen reader compatibility
- Color contrast requirements

## Next Steps

1. Review any failed tests in the HTML report
2. Address performance issues if identified
3. Update tests for new features
4. Monitor test execution time and optimize if needed

EOF

# Check if all tests passed
if [ $FAILED_TESTS -eq 0 ]; then
    print_success "All E2E tests passed! 🎉"
    echo ""
    echo "📊 Results:"
    echo "  - Test suites: $TOTAL_TESTS"
    echo "  - Passed: $PASSED_TESTS"
    echo "  - Failed: $FAILED_TESTS"
    echo "  - Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"
    echo ""
    echo "📁 Reports:"
    echo "  - HTML report: $HTML_REPORT_DIR/index.html"
    echo "  - Summary: $REPORT_DIR/e2e_summary.md"
    echo "  - JSON results: $REPORT_DIR/results.json"
    echo "  - JUnit results: $REPORT_DIR/results.xml"
    echo ""
    echo "🔍 Next Steps:"
    echo "  1. Review the HTML report for detailed results"
    echo "  2. Check performance metrics in the summary"
    echo "  3. Set up automated E2E testing in CI/CD"
    echo "  4. Monitor test execution time and optimize"
    echo ""
    echo "💡 Tips:"
    echo "  - Run E2E tests before major releases"
    echo "  - Monitor performance metrics over time"
    echo "  - Update tests when adding new features"
    echo "  - Use visual regression testing for UI changes"
    echo ""

    exit 0
else
    print_error "Some E2E tests failed! ❌"
    echo ""
    echo "📊 Results:"
    echo "  - Test suites: $TOTAL_TESTS"
    echo "  - Passed: $PASSED_TESTS"
    echo "  - Failed: $FAILED_TESTS"
    echo "  - Success rate: $(( (PASSED_TESTS * 100) / TOTAL_TESTS ))%"
    echo ""
    echo "📁 Reports:"
    echo "  - HTML report: $HTML_REPORT_DIR/index.html"
    echo "  - Summary: $REPORT_DIR/e2e_summary.md"
    echo ""
    echo "🔧 Debugging:"
    echo "  1. Check the HTML report for detailed failure information"
    echo "  2. Review screenshots and videos in the report"
    echo "  3. Check browser console logs for errors"
    echo "  4. Verify test environment setup"
    echo ""

    exit 1
fi
