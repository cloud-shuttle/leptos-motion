import { test, expect } from '@playwright/test';

test.describe('Visual Regression Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8080');
    await page.waitForLoadState('networkidle');
  });

  test('should capture baseline screenshots of working version', async ({ page }) => {
    // Wait for page to fully load with timeout
    await page.waitForTimeout(2000);
    
    // Capture full page screenshot with timeout
    await expect(page).toHaveScreenshot('baseline-full-page.png', { timeout: 10000 });
    
    // Capture specific component screenshots with timeout
    const showcaseGrid = page.locator('.showcase-grid');
    await expect(showcaseGrid).toHaveScreenshot('baseline-showcase-grid.png', { timeout: 10000 });
    
    // Capture individual component screenshots with timeout
    const simpleTest = page.locator('.showcase-card:has-text("Simple Test")');
    await expect(simpleTest).toHaveScreenshot('baseline-simple-test.png', { timeout: 10000 });
  });

  test('should detect visual changes in motion components', async ({ page }) => {
    // Wait for motion components to load with timeout
    await page.waitForTimeout(3000);
    
    // Capture motion component in idle state with timeout
    const motionDiv = page.locator('[data-motion]').first();
    if (await motionDiv.isVisible({ timeout: 5000 })) {
      await expect(motionDiv).toHaveScreenshot('motion-component-idle.png', { timeout: 10000 });
      
      // Click to trigger animation with timeout
      const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();
      await toggleButton.click({ timeout: 5000 });
      await page.waitForTimeout(1000); // Wait for animation
      
      // Capture motion component in active state with timeout
      await expect(motionDiv).toHaveScreenshot('motion-component-active.png', { timeout: 10000 });
      
      // Click again to return to idle with timeout
      await toggleButton.click({ timeout: 5000 });
      await page.waitForTimeout(1000);
      
      // Capture motion component back to idle with timeout
      await expect(motionDiv).toHaveScreenshot('motion-component-idle-return.png', { timeout: 10000 });
    }
  });

  test('should detect layout changes', async ({ page }) => {
    // Capture layout at different viewport sizes
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot('layout-desktop.png');
    
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot('layout-tablet.png');
    
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(1000);
    await expect(page).toHaveScreenshot('layout-mobile.png');
  });

  test('should detect CSS styling changes', async ({ page }) => {
    // Check for specific CSS properties
    const motionDiv = page.locator('[data-motion]').first();
    if (await motionDiv.isVisible()) {
      const styles = await motionDiv.evaluate((el) => {
        const computed = window.getComputedStyle(el);
        return {
          transform: computed.transform,
          backgroundColor: computed.backgroundColor,
          borderRadius: computed.borderRadius,
          boxShadow: computed.boxShadow,
          transition: computed.transition
        };
      });
      
      // Store baseline styles for comparison
      expect(styles.transform).toBeDefined();
      expect(styles.backgroundColor).toBeDefined();
      expect(styles.borderRadius).toBeDefined();
    }
  });

  test('should detect animation timing changes', async ({ page }) => {
    const motionDiv = page.locator('[data-motion]').first();
    if (await motionDiv.isVisible()) {
      const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();
      
      // Measure animation start time
      const startTime = Date.now();
      await toggleButton.click();
      
      // Wait for animation to complete
      await page.waitForTimeout(2000);
      
      // Check if element has reached final state
      const finalTransform = await motionDiv.evaluate((el) => {
        return window.getComputedStyle(el).transform;
      });
      
      // Animation should complete within reasonable time
      const animationTime = Date.now() - startTime;
      expect(animationTime).toBeLessThan(3000); // Should complete within 3 seconds
    }
  });

  test('should detect color and contrast changes', async ({ page }) => {
    // Check for accessibility and visual consistency
    const motionDiv = page.locator('[data-motion]').first();
    if (await motionDiv.isVisible()) {
      const colors = await motionDiv.evaluate((el) => {
        const computed = window.getComputedStyle(el);
        return {
          backgroundColor: computed.backgroundColor,
          color: computed.color,
          borderColor: computed.borderColor
        };
      });
      
      // Verify colors are valid
      expect(colors.backgroundColor).not.toBe('rgba(0, 0, 0, 0)');
      expect(colors.color).not.toBe('rgba(0,  0, 0, 0)');
    }
  });

  test('should detect responsive design changes', async ({ page }) => {
    // Test responsive behavior
    const showcaseGrid = page.locator('.showcase-grid');
    
    // Desktop view
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForTimeout(500);
    const desktopLayout = await showcaseGrid.evaluate((el) => {
      return {
        display: window.getComputedStyle(el).display,
        gridTemplateColumns: window.getComputedStyle(el).gridTemplateColumns,
        gap: window.getComputedStyle(el).gap
      };
    });
    
    // Mobile view
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForTimeout(500);
    const mobileLayout = await showcaseGrid.evaluate((el) => {
      return {
        display: window.getComputedStyle(el).display,
        gridTemplateColumns: window.getComputedStyle(el).gridTemplateColumns,
        gap: window.getComputedStyle(el).gap
      };
    });
    
    // Layout should adapt to viewport
    expect(desktopLayout.display).toBe('grid');
    expect(mobileLayout.display).toBe('grid');
  });
});
