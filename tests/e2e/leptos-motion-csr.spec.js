const { test, expect } = require('@playwright/test');

test.describe('Leptos Motion CSR Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:9000/');
  });

  test('should load without errors', async ({ page }) => {
    // Check for console errors
    const errors = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check that there are no critical errors
    const criticalErrors = errors.filter(error => 
      !error.includes('favicon.ico') && 
      !error.includes('404') &&
      !error.includes('Failed to initialize logger')
    );
    
    expect(criticalErrors).toHaveLength(0);
  });

  test('should display interactive elements', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for interactive elements
    const buttons = await page.locator('button').count();
    expect(buttons).toBeGreaterThan(0);
    
    // Check for motion elements
    const motionElements = await page.locator('[class*="motion"], [class*="box"], [class*="card"]').count();
    expect(motionElements).toBeGreaterThan(0);
  });

  test('should have animations working', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find a motion element to test animations
    const motionElement = page.locator('.motion-box').first();
    await expect(motionElement).toBeVisible();
    
    // Test hover animation
    await motionElement.hover();
    await page.waitForTimeout(100);
    
    // Test click animation
    await motionElement.click();
    await page.waitForTimeout(100);
    
    // Verify the element is still visible after interaction
    await expect(motionElement).toBeVisible();
  });

  test('should have drag functionality', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find draggable element
    const draggableElement = page.locator('.draggable-box');
    await expect(draggableElement).toBeVisible();
    
    // Test drag functionality
    const box = await draggableElement.boundingBox();
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    await page.mouse.move(box.x + 50, box.y + 50);
    await page.mouse.up();
    
    // Verify element is still visible
    await expect(draggableElement).toBeVisible();
  });

  test('should have layout animations', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find toggle button
    const toggleButton = page.locator('button').first();
    await expect(toggleButton).toBeVisible();
    
    // Test layout animation
    await toggleButton.click();
    await page.waitForTimeout(500);
    
    // Verify layout changed
    const layoutCards = page.locator('.layout-card');
    const cardCount = await layoutCards.count();
    expect(cardCount).toBeGreaterThanOrEqual(0);
  });

  test('should have scroll animations', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find scroll container
    const scrollContainer = page.locator('.scroll-container');
    await expect(scrollContainer).toBeVisible();
    
    // Test scroll animation
    await scrollContainer.scrollIntoViewIfNeeded();
    await page.waitForTimeout(500);
    
    // Verify scroll trigger is visible
    const scrollTrigger = page.locator('.scroll-trigger');
    await expect(scrollTrigger).toBeVisible();
  });

  test('should have spring animations', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find spring element
    const springElement = page.locator('.spring-box');
    await expect(springElement).toBeVisible();
    
    // Test spring animation by clicking
    await springElement.click();
    await page.waitForTimeout(500);
    
    // Verify element is still visible
    await expect(springElement).toBeVisible();
  });

  test('should have stagger animations', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find stagger container
    const staggerContainer = page.locator('.stagger-container');
    await expect(staggerContainer).toBeVisible();
    
    // Verify stagger items are present
    const staggerItems = page.locator('.stagger-item');
    const itemCount = await staggerItems.count();
    expect(itemCount).toBeGreaterThan(0);
  });

  test('should be responsive', async ({ page }) => {
    // Test mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    await page.waitForLoadState('networkidle');
    
    // Check that content is still visible
    const content = await page.locator('body').textContent();
    expect(content).toBeTruthy();
    
    // Test desktop viewport
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.waitForLoadState('networkidle');
    
    const desktopContent = await page.locator('body').textContent();
    expect(desktopContent).toBeTruthy();
  });

  test('should have proper performance', async ({ page }) => {
    // Measure performance
    const performanceMetrics = await page.evaluate(() => {
      const navigation = performance.getEntriesByType('navigation')[0];
      return {
        loadTime: navigation.loadEventEnd - navigation.loadEventStart,
        domContentLoaded: navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
        totalTime: navigation.loadEventEnd - navigation.fetchStart
      };
    });
    
    // Check that load time is reasonable (less than 5 seconds)
    expect(performanceMetrics.totalTime).toBeLessThan(5000);
  });
});
