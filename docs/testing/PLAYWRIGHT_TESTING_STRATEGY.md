# Playwright Testing Strategy for Leptos Motion

## Overview

This document outlines how to use Playwright tests to catch responsiveness issues in Leptos Motion applications, specifically the issues we discovered with the `ReactiveMotionDiv` component.

## Key Findings

### ✅ What Playwright Tests Can Detect

1. **Page Unresponsiveness**: Tests timeout when pages become unresponsive
2. **DOM Interaction Failures**: Tests fail when buttons/elements can't be clicked
3. **JavaScript Execution Issues**: Tests fail when `page.evaluate()` can't execute
4. **Memory Leaks**: Tests can monitor memory usage over time
5. **Infinite Loops**: Tests can detect excessive console message repetition

### ❌ What Playwright Tests Cannot Detect

1. **Visual Animation Issues**: Tests can't see if animations are actually working
2. **Performance Degradation**: Tests can't detect subtle performance issues
3. **CSS/Style Issues**: Tests can't verify visual styling is correct

## Test Categories

### 1. Responsiveness Tests

```typescript
test('page should remain responsive after WASM loads', async ({ page }) => {
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);
  
  // Test response time
  const responseTime = await page.evaluate(() => {
    return new Promise((resolve) => {
      const startTime = Date.now();
      setTimeout(() => {
        const endTime = Date.now();
        resolve(endTime - startTime);
      }, 100);
    });
  });
  
  expect(responseTime).toBeLessThan(1000);
});
```

### 2. Motion Component Tests

```typescript
test('should detect motion component issues', async ({ page }) => {
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);
  
  // Check for motion components
  const motionDivs = page.locator('[data-motion]');
  const motionCount = await motionDivs.count();
  
  if (motionCount > 0) {
    // Test interaction with motion components
    const firstMotionDiv = motionDivs.first();
    await firstMotionDiv.click();
    
    // Test if page is still responsive
    const isResponsive = await page.evaluate(() => {
      return new Promise((resolve) => {
        setTimeout(() => resolve(true), 100);
      });
    });
    
    expect(isResponsive).toBe(true);
  }
});
```

### 3. Button Interaction Tests

```typescript
test('should test button interactions', async ({ page }) => {
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);
  
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  
  for (let i = 0; i < buttonCount; i++) {
    const button = buttons.nth(i);
    await button.click();
    
    // Test responsiveness after each click
    const isResponsive = await page.evaluate(() => {
      return new Promise((resolve) => {
        setTimeout(() => resolve(true), 100);
      });
    });
    
    expect(isResponsive).toBe(true);
  }
});
```

### 4. Memory Leak Tests

```typescript
test('should detect memory leaks', async ({ page }) => {
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);
  
  // Get initial memory usage
  const initialMemory = await page.evaluate(() => {
    return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
  });
  
  // Perform interactions
  const buttons = page.locator('button');
  for (let i = 0; i < 5; i++) {
    await buttons.nth(i % await buttons.count()).click();
    await page.waitForTimeout(200);
  }
  
  // Get final memory usage
  const finalMemory = await page.evaluate(() => {
    return (performance as any).memory ? (performance as any).memory.usedJSHeapSize : 0;
  });
  
  const memoryIncrease = finalMemory - initialMemory;
  const memoryIncreasePercent = (memoryIncrease / initialMemory) * 100;
  
  expect(memoryIncreasePercent).toBeLessThan(50);
});
```

### 5. Infinite Loop Detection

```typescript
test('should detect infinite loops', async ({ page }) => {
  const consoleMessages: string[] = [];
  
  page.on('console', msg => {
    consoleMessages.push(msg.text());
  });
  
  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);
  
  // Interact with motion components
  const motionButtons = page.locator('button').filter({ hasText: /toggle|motion/i });
  if (await motionButtons.count() > 0) {
    await motionButtons.first().click();
    await page.waitForTimeout(2000);
  }
  
  // Check for excessive repetition
  const messageCounts: { [key: string]: number } = {};
  consoleMessages.forEach(msg => {
    messageCounts[msg] = (messageCounts[msg] || 0) + 1;
  });
  
  Object.entries(messageCounts).forEach(([msg, count]) => {
    if (count > 10) {
      expect(count).toBeLessThan(10);
    }
  });
});
```

## Test Results Interpretation

### ✅ Passing Tests
- **Page loads successfully**
- **Buttons are clickable**
- **Response times are reasonable**
- **No excessive console messages**
- **Memory usage is stable**

### ❌ Failing Tests
- **Tests timeout** → Page is unresponsive
- **Button clicks fail** → DOM interactions blocked
- **Response times too high** → Performance issues
- **Excessive console messages** → Infinite loops
- **Memory usage increasing** → Memory leaks

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Playwright Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions/setup-node@v3
      with:
        node-version: 18
    - name: Install dependencies
      run: npm install
    - name: Install Playwright
      run: npx playwright install
    - name: Build WASM
      run: wasm-pack build --target web --out-dir pkg --dev
    - name: Start server
      run: python3 -m http.server 8080 &
    - name: Run Playwright tests
      run: npx playwright test
    - name: Upload test results
      uses: actions/upload-artifact@v3
      if: failure()
      with:
        name: playwright-report
        path: playwright-report/
```

## Best Practices

### 1. Test Early and Often
- Run tests on every commit
- Test both simple and complex components
- Test with and without motion components

### 2. Use Appropriate Timeouts
```typescript
// Short timeout for responsiveness tests
await page.waitForTimeout(100);

// Longer timeout for WASM loading
await page.waitForTimeout(3000);
```

### 3. Test Different Scenarios
- Simple components only
- Motion components only
- Mixed components
- Multiple interactions

### 4. Monitor Console Output
```typescript
const errors: string[] = [];
page.on('console', msg => {
  if (msg.type() === 'error') {
    errors.push(msg.text());
  }
});
```

### 5. Test Memory Usage
```typescript
const memory = await page.evaluate(() => {
  return (performance as any).memory?.usedJSHeapSize || 0;
});
```

## Conclusion

Playwright tests are excellent for detecting responsiveness issues in Leptos Motion applications. They can catch:

- **Unresponsive pages** (timeouts)
- **DOM interaction failures** (click failures)
- **Performance issues** (slow response times)
- **Memory leaks** (increasing memory usage)
- **Infinite loops** (excessive console messages)

However, they cannot detect visual animation issues, so they should be combined with manual testing for a complete validation strategy.

---

*This testing strategy was developed based on the debugging experience with the ReactiveMotionDiv responsiveness issue.*
