import { test, expect } from '@playwright/test';

test.describe('Leptos Motion E2E Tests', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the test app
    await page.goto('/test-app.html');
    
    // Wait for the page to load
    await page.waitForSelector('h1', { timeout: 10000 });
  });



  test('should display the main test page', async ({ page }) => {
    // Check that the main content is visible
    await expect(page.locator('h1')).toContainText('Leptos Motion Test App');
    
    // Check that test sections are present
    await expect(page.locator('.test-section')).toHaveCount(6);
  });

  test('should animate elements on hover', async ({ page }) => {
    // Find a hoverable element
    const hoverElement = page.locator('[data-testid="hover-animation"]').first();
    
    // Get initial transform
    const initialTransform = await hoverElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );
    
    // Hover over the element
    await hoverElement.hover();
    
    // Wait for animation to complete
    await page.waitForTimeout(500);
    
    // Check that transform has changed
    const finalTransform = await hoverElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );
    
    expect(finalTransform).not.toBe(initialTransform);
  });

  test('should animate elements on click', async ({ page }) => {
    // Find a clickable element
    const clickElement = page.locator('[data-testid="click-animation"]').first();
    
    // Get initial scale
    const initialScale = await clickElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );
    
    // Click the element
    await clickElement.click();
    
    // Wait for animation to complete
    await page.waitForTimeout(300);
    
    // Check that scale has changed
    const finalScale = await clickElement.evaluate(el => 
      window.getComputedStyle(el).transform
    );
    
    expect(finalScale).not.toBe(initialScale);
  });

  test('should support drag and drop', async ({ page }) => {
    // Find a draggable element
    const draggable = page.locator('[data-testid="draggable"]').first();
    
    // Get initial position
    const initialRect = await draggable.boundingBox();
    
    // Perform drag operation
    await draggable.dragTo(page.locator('[data-testid="drop-zone"]'));
    
    // Wait for animation to complete
    await page.waitForTimeout(500);
    
    // Check that position has changed
    const finalRect = await draggable.boundingBox();
    
    expect(finalRect?.x).not.toBe(initialRect?.x);
    expect(finalRect?.y).not.toBe(initialRect?.y);
  });

  test('should handle layout animations', async ({ page }) => {
    // Find a layout animation trigger
    const trigger = page.locator('[data-testid="layout-trigger"]').first();
    const layoutBox = page.locator('[data-testid="layout-box"]').first();
    
    // Get initial layout measurements
    const initialLayout = await layoutBox.evaluate(el => ({
      width: el.offsetWidth,
      height: el.offsetHeight
    }));
    
    // Trigger layout change
    await trigger.click();
    
    // Wait for layout animation
    await page.waitForTimeout(800);
    
    // Check that layout has changed
    const finalLayout = await layoutBox.evaluate(el => ({
      width: el.offsetWidth,
      height: el.offsetHeight
    }));
    
    // The layout should have changed (expanded)
    expect(finalLayout.width).toBeGreaterThan(initialLayout.width);
    expect(finalLayout.height).toBeGreaterThan(initialLayout.height);
  });

  test('should respect reduced motion preference', async ({ page }) => {
    // Set reduced motion preference
    await page.addInitScript(() => {
      Object.defineProperty(window, 'matchMedia', {
        writable: true,
        value: jest.fn().mockImplementation(query => ({
          matches: query === '(prefers-reduced-motion: reduce)',
          media: query,
          onchange: null,
          addListener: jest.fn(),
          removeListener: jest.fn(),
          addEventListener: jest.fn(),
          removeEventListener: jest.fn(),
          dispatchEvent: jest.fn(),
        })),
      });
    });
    
    // Reload page with reduced motion preference
    await page.reload();
    
    // Find an animated element
    const animatedElement = page.locator('[data-testid="animated"]').first();
    
    // Get initial state
    const initialOpacity = await animatedElement.evaluate(el => 
      window.getComputedStyle(el).opacity
    );
    
    // Trigger animation
    await animatedElement.click();
    
    // Wait for what should be a very short animation
    await page.waitForTimeout(100);
    
    // Check that animation was minimal or disabled
    const finalOpacity = await animatedElement.evaluate(el => 
      window.getComputedStyle(el).opacity
    );
    
    // With reduced motion, the change should be minimal
    const opacityChange = Math.abs(parseFloat(finalOpacity) - parseFloat(initialOpacity));
    expect(opacityChange).toBeLessThan(0.1);
  });

  test('should handle performance test buttons', async ({ page }) => {
    // Find performance test buttons
    const performanceButtons = page.locator('[data-testid="performance-test"]');
    const count = await performanceButtons.count();
    
    // Verify we have performance test buttons
    expect(count).toBeGreaterThan(0);
    
    // Click a few buttons to ensure they're interactive
    for (let i = 0; i < Math.min(count, 3); i++) {
      const button = performanceButtons.nth(i);
      await expect(button).toBeVisible();
      await button.click();
      
      // Small delay to ensure click is registered
      await page.waitForTimeout(100);
    }
    
    // Verify the page is still responsive
    await expect(page.locator('h1')).toContainText('Leptos Motion Test App');
  });
});
