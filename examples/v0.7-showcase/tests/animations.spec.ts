import { test, expect } from '@playwright/test';

test.describe('Leptos Motion v0.8.0 Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the WASM to load
    await page.waitForLoadState('networkidle');
  });

  test('should display the demo page', async ({ page }) => {
    await expect(page.locator('h1')).toContainText('Leptos Motion v0.8.0');
    await expect(page.locator('.status-badge')).toContainText('Motion.dev Parity Achieved!');
  });

  test('Spring Physics animation should work', async ({ page }) => {
    // Find the Spring Physics demo
    const springDemo = page.locator('.showcase-card').filter({ hasText: 'Spring Physics' });
    await expect(springDemo).toBeVisible();

    // Find the animated element
    const animatedElement = springDemo.locator('.w-20.h-20.bg-green-500');
    await expect(animatedElement).toBeVisible();

    // Get initial position
    const initialBox = await animatedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click the animate button
    const animateButton = springDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();

    // Wait for animation to complete
    await page.waitForTimeout(1500);

    // Check if the element moved (should have moved right)
    const finalBox = await animatedElement.boundingBox();
    expect(finalBox).not.toBeNull();
    
    if (initialBox && finalBox) {
      // The element should have moved to the right (x position increased)
      expect(finalBox.x).toBeGreaterThan(initialBox.x);
    }
  });

  test('Variants animation should work on hover', async ({ page }) => {
    // Find the Variants demo
    const variantsDemo = page.locator('.showcase-card').filter({ hasText: 'Variants System' });
    await expect(variantsDemo).toBeVisible();

    // Find the animated element
    const animatedElement = variantsDemo.locator('.w-24.h-24.rounded-lg');
    await expect(animatedElement).toBeVisible();

    // Get initial transform
    const initialTransform = await animatedElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );

    // Hover over the element
    await animatedElement.hover();

    // Wait for animation
    await page.waitForTimeout(500);

    // Check if transform changed (should have scale and rotation)
    const hoverTransform = await animatedElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );

    expect(hoverTransform).not.toBe(initialTransform);
    expect(hoverTransform).toContain('matrix'); // Should have a transform matrix
  });

  test('Timeline animation should work', async ({ page }) => {
    // Find the Timeline demo
    const timelineDemo = page.locator('.showcase-card').filter({ hasText: 'Timeline Sequences' });
    await expect(timelineDemo).toBeVisible();

    // Find the animated element
    const animatedElement = timelineDemo.locator('.w-20.h-20.bg-blue-500');
    await expect(animatedElement).toBeVisible();

    // Get initial position
    const initialBox = await animatedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click next step button
    const nextStepButton = timelineDemo.locator('button').filter({ hasText: 'Next Step' });
    await nextStepButton.click();

    // Wait for animation
    await page.waitForTimeout(1000);

    // Check if the element moved
    const finalBox = await animatedElement.boundingBox();
    expect(finalBox).not.toBeNull();
    
    if (initialBox && finalBox) {
      // The element should have moved
      expect(finalBox.x).not.toBe(initialBox.x);
    }
  });

  test('Performance demo should animate multiple elements', async ({ page }) => {
    // Find the Performance demo
    const performanceDemo = page.locator('.showcase-card').filter({ hasText: 'Performance' });
    await expect(performanceDemo).toBeVisible();

    // Find animated elements
    const animatedElements = performanceDemo.locator('.w-4.h-4.bg-purple-500');
    await expect(animatedElements).toHaveCount(5);

    // Get initial positions
    const initialBoxes = await Promise.all(
      Array.from({ length: 5 }, (_, i) => 
        animatedElements.nth(i).boundingBox()
      )
    );

    // Click animate button
    const animateButton = performanceDemo.locator('button').filter({ hasText: 'Animate' });
    await animateButton.click();

    // Wait for animation
    await page.waitForTimeout(1000);

    // Check if elements moved
    const finalBoxes = await Promise.all(
      Array.from({ length: 5 }, (_, i) => 
        animatedElements.nth(i).boundingBox()
      )
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

  test('SVG Path Morphing should work', async ({ page }) => {
    // Find the SVG Path Morphing demo
    const svgDemo = page.locator('.showcase-card').filter({ hasText: 'SVG Path Morphing' });
    await expect(svgDemo).toBeVisible();

    // Find the SVG path
    const svgPath = svgDemo.locator('svg path');
    await expect(svgPath).toBeVisible();

    // Get initial path data
    const initialPath = await svgPath.getAttribute('d');
    expect(initialPath).toBeTruthy();

    // Click next shape button
    const nextShapeButton = svgDemo.locator('button').filter({ hasText: 'Next Shape' });
    await nextShapeButton.click();

    // Wait for animation
    await page.waitForTimeout(1000);

    // Check if path data changed
    const finalPath = await svgPath.getAttribute('d');
    expect(finalPath).toBeTruthy();
    expect(finalPath).not.toBe(initialPath);
  });

  test('Shared Elements animation should work', async ({ page }) => {
    // Find the Shared Elements demo
    const sharedDemo = page.locator('.showcase-card').filter({ hasText: 'Shared Elements' });
    await expect(sharedDemo).toBeVisible();

    // Find the shared element
    const sharedElement = sharedDemo.locator('.w-20.h-20').first();
    await expect(sharedElement).toBeVisible();

    // Get initial position
    const initialBox = await sharedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click share button
    const shareButton = sharedDemo.locator('button').filter({ hasText: 'Share' });
    await shareButton.click();

    // Wait for animation
    await page.waitForTimeout(1000);

    // Check if element moved
    const finalBox = await sharedElement.boundingBox();
    expect(finalBox).not.toBeNull();
    
    if (initialBox && finalBox) {
      expect(finalBox.x).not.toBe(initialBox.x);
    }
  });

  test('Orchestration animation should work', async ({ page }) => {
    // Find the Orchestration demo
    const orchestrationDemo = page.locator('.showcase-card').filter({ hasText: 'Orchestration' });
    await expect(orchestrationDemo).toBeVisible();

    // Find the animated element
    const animatedElement = orchestrationDemo.locator('.w-20.h-20.bg-blue-500');
    await expect(animatedElement).toBeVisible();

    // Get initial position
    const initialBox = await animatedElement.boundingBox();
    expect(initialBox).not.toBeNull();

    // Click play sequence button
    const playButton = orchestrationDemo.locator('button').filter({ hasText: 'Play Sequence' });
    await playButton.click();

    // Wait for animation sequence
    await page.waitForTimeout(2000);

    // Check if element moved
    const finalBox = await animatedElement.boundingBox();
    expect(finalBox).not.toBeNull();
    
    if (initialBox && finalBox) {
      expect(finalBox.x).not.toBe(initialBox.x);
    }
  });

  test('Scroll animations should work', async ({ page }) => {
    // Find the Scroll Animations demo
    const scrollDemo = page.locator('.showcase-card').filter({ hasText: 'Scroll Animations' });
    await expect(scrollDemo).toBeVisible();

    // Find animated elements
    const animatedElements = scrollDemo.locator('.w-16.h-16.rounded-lg');
    await expect(animatedElements).toHaveCount(4);

    // Get initial opacity values
    const initialOpacities = await Promise.all(
      Array.from({ length: 4 }, (_, i) => 
        animatedElements.nth(i).evaluate(el => 
          window.getComputedStyle(el).opacity
        )
      )
    );

    // Scroll down to trigger animations
    await page.evaluate(() => window.scrollTo(0, 1000));
    await page.waitForTimeout(1000);

    // Check if opacity changed (elements should fade in)
    const finalOpacities = await Promise.all(
      Array.from({ length: 4 }, (_, i) => 
        animatedElements.nth(i).evaluate(el => 
          window.getComputedStyle(el).opacity
        )
      )
    );

    // At least some elements should have changed opacity
    let changedCount = 0;
    for (let i = 0; i < 4; i++) {
      if (initialOpacities[i] !== finalOpacities[i]) {
        changedCount++;
      }
    }
    expect(changedCount).toBeGreaterThan(0);
  });
});
