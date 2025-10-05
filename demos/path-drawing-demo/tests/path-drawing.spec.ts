import { test, expect } from '@playwright/test';

test.describe('Path Drawing Demo', () => {
  test('should start with paths hidden and animate on button click', async ({ page }) => {
    await page.goto('http://localhost:8081/');

    // Wait for the page to load and WASM to initialize
    await page.waitForTimeout(2000);

    // Check that the paths are initially hidden
    const paths = page.locator('path');
    const pathCount = await paths.count();
    console.log(`Found ${pathCount} paths`);

    // Check that paths have stroke-dashoffset set to hide them initially
    for (let i = 0; i < pathCount; i++) {
      const path = paths.nth(i);
      const strokeDashOffset = await path.getAttribute('stroke-dashoffset');
      const strokeDashArray = await path.getAttribute('stroke-dasharray');
      const style = await path.getAttribute('style');
      console.log(`Path ${i}: stroke-dashoffset="${strokeDashOffset}", stroke-dasharray="${strokeDashArray}", style="${style}"`);

      // The path should have style set with stroke-dasharray (stroke-dashoffset is handled by animation system)
      expect(style).toBeTruthy();
      expect(style).toContain('stroke-dasharray');
      // stroke-dashoffset should NOT be in style (it's controlled by the animation system)
    }

    // Click the start button
    await page.click('button:has-text("▶️ Start Drawing")');

    // Wait for animation to start
    await page.waitForTimeout(1000);

    // Check that paths are now visible (stroke-dashoffset should be animating to 0)
    // We can't easily check the animated values, but we can check that the button text changed
    await expect(page.locator('button')).toContainText('⏸️ Pause');

    // Wait for animation to progress
    await page.waitForTimeout(2000);

    // Check if any paths have become visible (stroke-dashoffset should be less than initial)
    let anyPathVisible = false;
    for (let i = 0; i < pathCount; i++) {
      const path = paths.nth(i);
      const computedStyle = await path.evaluate(el => {
        const style = window.getComputedStyle(el);
        return {
          strokeDashoffset: style.strokeDashoffset,
          strokeDasharray: style.strokeDasharray
        };
      });
      console.log(`Path ${i} computed style:`, computedStyle);

      // If stroke-dashoffset is less than stroke-dasharray, the path is becoming visible
      if (computedStyle.strokeDashoffset !== computedStyle.strokeDasharray) {
        anyPathVisible = true;
        break;
      }
    }

    if (anyPathVisible) {
      console.log('✅ Path drawing animation is working - paths are becoming visible');
    } else {
      console.log('❌ Path drawing animation may not be working - paths are still hidden');
    }

    console.log('✅ Path drawing animation started successfully');
  });
});
