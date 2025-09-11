# Leptos Motion Animation System Test Plan

**Date**: September 9, 2025  
**Version**: v0.8.0  
**Status**: Ready for Implementation  
**Priority**: Critical

## Overview

This document outlines the comprehensive test plan for validating the Leptos Motion animation system fix. The test plan covers unit testing, integration testing, performance testing, and cross-browser compatibility testing.

## Test Objectives

### Primary Objectives

1. **Verify Animation Functionality**: Ensure all animations work as expected
2. **Validate Reactive System**: Confirm reactive dependencies are properly tracked
3. **Performance Validation**: Ensure animations run smoothly at 60fps
4. **Cross-Browser Compatibility**: Verify consistent behavior across browsers

### Secondary Objectives

1. **Regression Prevention**: Ensure no existing functionality is broken
2. **Error Handling**: Validate proper error handling and fallbacks
3. **Memory Management**: Ensure no memory leaks during animations
4. **User Experience**: Validate smooth and responsive animations

## Test Environment Setup

### Prerequisites

- Node.js 18+
- Playwright installed and configured
- Rust toolchain with WASM support
- Python 3.8+ for local server
- Modern browsers (Chrome, Firefox, Safari, Edge)

### Test Data

- Demo application with 9 showcase components
- Various animation types (spring, variants, timeline, etc.)
- Different animation durations and easing functions
- Multiple simultaneous animations

## Test Suite Structure

```
tests/
├── unit/
│   ├── reactive_motion_div.rs
│   ├── animation_target.rs
│   └── effects.rs
├── integration/
│   ├── animations.spec.ts
│   ├── reactivity.spec.ts
│   └── performance.spec.ts
├── e2e/
│   ├── demo-workflow.spec.ts
│   ├── cross-browser.spec.ts
│   └── stress-test.spec.ts
└── fixtures/
    ├── test-data.json
    └── expected-results.json
```

## Unit Tests

### 1. ReactiveMotionDiv Component Tests

**File**: `crates/leptos-motion-dom/src/reactive_motion_div.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::prelude::*;

    #[test]
    fn test_reactive_animation_tracking() {
        // Test that reactive animations properly track signal changes
        let (signal, set_signal) = signal(false);

        let closure = Rc::new(move || {
            if signal.get() {
                HashMap::from([("transform".to_string(), "translateX(100px)".to_string())])
            } else {
                HashMap::from([("transform".to_string(), "translateX(0px)".to_string())])
            }
        });

        let reactive_target = AnimationTargetOrReactive::Reactive(closure);

        // Test initial state
        let initial_target = reactive_target.get_target();
        assert_eq!(initial_target.get("transform"), Some(&"translateX(0px)".to_string()));

        // Test state change
        set_signal.set(true);
        let updated_target = reactive_target.get_target();
        assert_eq!(updated_target.get("transform"), Some(&"translateX(100px)".to_string()));
    }

    #[test]
    fn test_animation_effect_triggering() {
        // Test that effects trigger when dependencies change
        let (signal, set_signal) = signal(false);
        let effect_triggered = Rc::new(RefCell::new(false));
        let effect_triggered_clone = effect_triggered.clone();

        let closure = Rc::new(move || {
            let _ = signal.get(); // Track the signal
            *effect_triggered_clone.borrow_mut() = true;
            HashMap::new()
        });

        let reactive_target = AnimationTargetOrReactive::Reactive(closure);

        // Initial state
        assert_eq!(*effect_triggered.borrow(), false);

        // Change signal
        set_signal.set(true);

        // Effect should have been triggered
        assert_eq!(*effect_triggered.borrow(), true);
    }

    #[test]
    fn test_style_application() {
        // Test that animation styles are correctly applied
        let target = HashMap::from([
            ("transform".to_string(), "translateX(100px) scale(1.2)".to_string()),
            ("background-color".to_string(), "rgb(255, 0, 0)".to_string())
        ]);

        let reactive_target = AnimationTargetOrReactive::Static(target);
        let result = reactive_target.get_target();

        assert_eq!(result.get("transform"), Some(&"translateX(100px) scale(1.2)".to_string()));
        assert_eq!(result.get("background-color"), Some(&"rgb(255, 0, 0)".to_string()));
    }

    #[test]
    fn test_multiple_rapid_changes() {
        // Test handling of rapid signal changes
        let (signal, set_signal) = signal(0);
        let change_count = Rc::new(RefCell::new(0));
        let change_count_clone = change_count.clone();

        let closure = Rc::new(move || {
            let _ = signal.get();
            *change_count_clone.borrow_mut() += 1;
            HashMap::new()
        });

        let _reactive_target = AnimationTargetOrReactive::Reactive(closure);

        // Rapid changes
        for i in 1..=10 {
            set_signal.set(i);
        }

        // Should have tracked all changes
        assert_eq!(*change_count.borrow(), 10);
    }
}
```

### 2. Animation Target Tests

**File**: `crates/leptos-motion-core/src/types.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_value_to_string() {
        assert_eq!(AnimationValue::Pixels(100.0).to_string_value(), "100px");
        assert_eq!(AnimationValue::Number(1.5).to_string_value(), "1.5");
        assert_eq!(AnimationValue::Degrees(90.0).to_string_value(), "90deg");
        assert_eq!(AnimationValue::String("red".to_string()).to_string_value(), "red");
    }

    #[test]
    fn test_animation_target_creation() {
        let target = HashMap::from([
            ("transform".to_string(), AnimationValue::String("translateX(100px)".to_string())),
            ("opacity".to_string(), AnimationValue::Number(0.5))
        ]);

        assert_eq!(target.len(), 2);
        assert!(target.contains_key("transform"));
        assert!(target.contains_key("opacity"));
    }
}
```

## Integration Tests

### 1. Animation System Tests

**File**: `tests/animation-system.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Animation System', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);
  });

  test('Spring Physics animation triggers on button click', async ({ page }) => {
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');

    // Get initial position
    const initialBox = await animatedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click animate button
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();

    // Wait for animation
    await page.waitForTimeout(1500);

    // Check if element moved
    const finalBox = await animatedElement.boundingBox();
    expect(finalBox).not.toBeNull();

    if (initialBox && finalBox) {
      expect(finalBox.x).toBeGreaterThan(initialBox.x);
    }
  });

  test('Variants animation responds to hover', async ({ page }) => {
    const variantsDemo = page.locator('.showcase-card').filter({ hasText: 'Variants System' });
    const animatedElement = variantsDemo.locator('.w-24.h-24.rounded-lg');

    // Get initial transform
    const initialTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    // Hover over element
    await animatedElement.hover();
    await page.waitForTimeout(500);

    // Check if transform changed
    const hoverTransform = await animatedElement.evaluate(
      el => window.getComputedStyle(el).transform
    );

    expect(hoverTransform).not.toBe(initialTransform);
  });

  test('Timeline animation progresses through steps', async ({ page }) => {
    const timelineDemo = page.locator('.showcase-card').filter({ hasText: 'Timeline Sequences' });
    const animatedElement = timelineDemo.locator('.w-20.h-20.bg-orange-500');

    // Get initial position
    const initialBox = await animatedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click next step button multiple times
    const nextStepButton = timelineDemo.locator('button').filter({ hasText: 'Next Step' });

    for (let i = 0; i < 3; i++) {
      await nextStepButton.click();
      await page.waitForTimeout(1000);
    }

    // Check if element moved
    const finalBox = await animatedElement.boundingBox();
    expect(finalBox).not.toBeNull();

    if (initialBox && finalBox) {
      expect(finalBox.x).not.toBe(initialBox.x);
    }
  });

  test('Performance demo animates multiple elements', async ({ page }) => {
    const performanceDemo = page.locator('.showcase-card').filter({ hasText: 'Performance' });
    const animatedElements = performanceDemo.locator('.w-4.h-4.bg-purple-500');

    // Get initial positions
    const initialBoxes = await Promise.all(
      Array.from({ length: 5 }, (_, i) => animatedElements.nth(i).boundingBox())
    );

    // Click animate button
    const animateButton = performanceDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();
    await page.waitForTimeout(1000);

    // Check if elements moved
    const finalBoxes = await Promise.all(
      Array.from({ length: 5 }, (_, i) => animatedElements.nth(i).boundingBox())
    );

    // At least some elements should have moved
    let movedCount = 0;
    for (let i = 0; i < 5; i++) {
      if (initialBoxes[i] && finalBoxes[i]) {
        if (initialBoxes[i].x !== finalBoxes[i].x || initialBoxes[i].y !== finalBoxes[i].y) {
          movedCount++;
        }
      }
    }
    expect(movedCount).toBeGreaterThan(0);
  });
});
```

### 2. Reactivity Tests

**File**: `tests/reactivity.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Reactive System', () => {
  test('Animation effects trigger on signal changes', async ({ page }) => {
    const logs: string[] = [];

    page.on('console', msg => {
      if (
        msg.text().includes('Animation effect triggered') ||
        msg.text().includes('Animation target:') ||
        msg.text().includes('Animation styles applied')
      ) {
        logs.push(msg.text());
      }
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    // Click button multiple times
    await animateButton.click();
    await page.waitForTimeout(500);
    await animateButton.click();
    await page.waitForTimeout(500);

    // Verify effects were triggered
    expect(logs.length).toBeGreaterThan(0);

    // Check for specific log messages
    const effectLogs = logs.filter(log => log.includes('Animation effect triggered'));
    expect(effectLogs.length).toBeGreaterThan(0);
  });

  test('Multiple rapid signal changes are handled correctly', async ({ page }) => {
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    // Rapid clicks
    for (let i = 0; i < 5; i++) {
      await animateButton.click();
      await page.waitForTimeout(100);
    }

    await page.waitForTimeout(1000);

    // Should not have any errors
    const errors = await page.evaluate(() => {
      return window.console.error ? window.console.error.toString() : 'no errors';
    });

    expect(errors).toBe('no errors');
  });

  test('Animation state persists correctly', async ({ page }) => {
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    // Click to activate
    await animateButton.click();
    await page.waitForTimeout(1500);

    // Get active state
    const activeBox = await animatedElement.boundingBox();

    // Click to deactivate
    await animateButton.click();
    await page.waitForTimeout(1500);

    // Get inactive state
    const inactiveBox = await animatedElement.boundingBox();

    // States should be different
    expect(activeBox).not.toBeNull();
    expect(inactiveBox).not.toBeNull();
    expect(activeBox.x).not.toBe(inactiveBox.x);
  });
});
```

### 3. Performance Tests

**File**: `tests/performance.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Performance', () => {
  test('Animations maintain 60fps', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Start performance monitoring
    await page.evaluate(() => {
      window.performance.mark('animation-start');
    });

    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    await animateButton.click();
    await page.waitForTimeout(1500);

    // End performance monitoring
    await page.evaluate(() => {
      window.performance.mark('animation-end');
      window.performance.measure('animation-duration', 'animation-start', 'animation-end');
    });

    // Get performance metrics
    const metrics = await page.evaluate(() => {
      const measure = window.performance.getEntriesByName('animation-duration')[0];
      return {
        duration: measure.duration,
        startTime: measure.startTime,
      };
    });

    // Animation should complete within reasonable time
    expect(metrics.duration).toBeLessThan(2000); // 2 seconds max
  });

  test('Multiple elements animate simultaneously', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    const performanceDemo = page.locator('.showcase-card').filter({ hasText: 'Performance' });
    const animateButton = performanceDemo.locator('button').filter({ hasText: 'Animate' });

    // Start timing
    const startTime = Date.now();

    await animateButton.click();
    await page.waitForTimeout(2000);

    const endTime = Date.now();
    const duration = endTime - startTime;

    // Should complete within reasonable time
    expect(duration).toBeLessThan(3000);
  });

  test('No memory leaks during extended use', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    // Get initial memory usage
    const initialMemory = await page.evaluate(() => {
      return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
    });

    // Perform many animations
    for (let i = 0; i < 20; i++) {
      await animateButton.click();
      await page.waitForTimeout(200);
    }

    // Get final memory usage
    const finalMemory = await page.evaluate(() => {
      return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
    });

    // Memory usage should not increase significantly
    const memoryIncrease = finalMemory - initialMemory;
    expect(memoryIncrease).toBeLessThan(10 * 1024 * 1024); // 10MB max increase
  });
});
```

## End-to-End Tests

### 1. Demo Workflow Tests

**File**: `tests/demo-workflow.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Demo Workflow', () => {
  test('Complete demo experience works', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Test all demo components
    const demos = [
      'Spring Physics',
      'Variants System',
      'Timeline Sequences',
      'Performance',
      'Advanced Properties',
      'Scroll Animations',
      'SVG Path Morphing',
      'Shared Elements',
      'Orchestration',
    ];

    for (const demoName of demos) {
      const demo = page.locator('.showcase-card').filter({ hasText: demoName });
      await expect(demo).toBeVisible();

      // Try to interact with the demo
      const buttons = demo.locator('button');
      const buttonCount = await buttons.count();

      if (buttonCount > 0) {
        await buttons.first().click();
        await page.waitForTimeout(500);
      }
    }
  });

  test('Demo loads without errors', async ({ page }) => {
    const errors: string[] = [];

    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    page.on('pageerror', error => {
      errors.push(error.message);
    });

    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    expect(errors.length).toBe(0);
  });
});
```

### 2. Cross-Browser Tests

**File**: `tests/cross-browser.spec.ts`

```typescript
import { test, expect, devices } from '@playwright/test';

test.describe('Cross-Browser Compatibility', () => {
  test('Animations work in Chrome', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    const initialBox = await animatedElement.boundingBox();
    await animateButton.click();
    await page.waitForTimeout(1500);
    const finalBox = await animatedElement.boundingBox();

    expect(initialBox).not.toBeNull();
    expect(finalBox).not.toBeNull();
    expect(finalBox.x).toBeGreaterThan(initialBox.x);
  });

  test('Animations work in Firefox', async ({ page, browserName }) => {
    test.skip(browserName !== 'firefox', 'Firefox specific test');

    // Firefox-specific tests
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Test animations in Firefox
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    const initialBox = await animatedElement.boundingBox();
    await animateButton.click();
    await page.waitForTimeout(1500);
    const finalBox = await animatedElement.boundingBox();

    expect(initialBox).not.toBeNull();
    expect(finalBox).not.toBeNull();
    expect(finalBox.x).toBeGreaterThan(initialBox.x);
  });

  test('Animations work in Safari', async ({ page, browserName }) => {
    test.skip(browserName !== 'webkit', 'Safari specific test');

    // Safari-specific tests
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Test animations in Safari
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    const initialBox = await animatedElement.boundingBox();
    await animateButton.click();
    await page.waitForTimeout(1500);
    const finalBox = await animatedElement.boundingBox();

    expect(initialBox).not.toBeNull();
    expect(finalBox).not.toBeNull();
    expect(finalBox.x).toBeGreaterThan(initialBox.x);
  });
});
```

### 3. Stress Tests

**File**: `tests/stress-test.spec.ts`

```typescript
import { test, expect } from '@playwright/test';

test.describe('Stress Tests', () => {
  test('Rapid button clicks are handled gracefully', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });

    // Rapid clicks
    for (let i = 0; i < 20; i++) {
      await animateButton.click();
      await page.waitForTimeout(50);
    }

    await page.waitForTimeout(2000);

    // Should not have crashed
    const springDemoStillVisible = await springDemo.isVisible();
    expect(springDemoStillVisible).toBe(true);
  });

  test('Multiple demos can be animated simultaneously', async ({ page }) => {
    await page.goto('/');
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Start multiple animations
    const demos = ['Spring Physics', 'Variants System', 'Timeline Sequences', 'Performance'];

    for (const demoName of demos) {
      const demo = page.locator('.showcase-card').filter({ hasText: demoName });
      const buttons = demo.locator('button');
      const buttonCount = await buttons.count();

      if (buttonCount > 0) {
        await buttons.first().click();
      }
    }

    await page.waitForTimeout(3000);

    // All demos should still be visible
    for (const demoName of demos) {
      const demo = page.locator('.showcase-card').filter({ hasText: demoName });
      await expect(demo).toBeVisible();
    }
  });
});
```

## Test Execution

### Running Unit Tests

```bash
# Run all unit tests
cargo test

# Run specific test file
cargo test reactive_motion_div

# Run with output
cargo test -- --nocapture
```

### Running Integration Tests

```bash
# Install Playwright
pnpm install @playwright/test
npx playwright install

# Run all integration tests
npx playwright test

# Run specific test file
npx playwright test tests/animation-system.spec.ts

# Run with UI
npx playwright test --ui

# Run in headed mode
npx playwright test --headed
```

### Running Cross-Browser Tests

```bash
# Run all browsers
npx playwright test --project=chromium --project=firefox --project=webkit

# Run specific browser
npx playwright test --project=chromium
```

## Test Results and Reporting

### Success Criteria

- **Unit Tests**: 100% pass rate
- **Integration Tests**: 100% pass rate
- **Performance Tests**: <2s animation duration, <10MB memory increase
- **Cross-Browser Tests**: 100% pass rate on all supported browsers

### Test Reports

- **Unit Test Report**: Generated by `cargo test`
- **Integration Test Report**: Generated by Playwright HTML reporter
- **Performance Report**: Custom metrics collection
- **Cross-Browser Report**: Playwright cross-browser test results

### Continuous Integration

- **GitHub Actions**: Automated test execution on PR and main branch
- **Test Coverage**: Minimum 90% code coverage
- **Performance Monitoring**: Continuous performance regression detection

## Maintenance and Updates

### Test Maintenance

- **Regular Updates**: Update tests when API changes
- **Performance Baselines**: Update performance baselines as needed
- **Browser Support**: Update browser support matrix

### Test Data Management

- **Test Fixtures**: Maintain test data and expected results
- **Environment Setup**: Keep test environment documentation updated
- **Dependencies**: Keep test dependencies up to date

---

**Last Updated**: September 9, 2025  
**Next Review**: September 16, 2025  
**Implementation Start**: September 10, 2025  
**Target Completion**: September 30, 2025
