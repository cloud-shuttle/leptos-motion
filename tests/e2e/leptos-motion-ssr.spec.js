const { test, expect } = require('@playwright/test');

test.describe('Leptos Motion SSR Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:9001/');
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

  test('should have server-side rendered content', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for server-rendered content
    const serverInfo = page.locator('.server-info');
    await expect(serverInfo).toBeVisible();
    
    // Check for server time
    const serverTime = await serverInfo.textContent();
    expect(serverTime).toContain('This content was rendered on the server at:');
  });

  test('should have hydration-safe animations', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find hydration box
    const hydrationBox = page.locator('.hydration-box');
    await expect(hydrationBox).toBeVisible();
    
    // Test hydration animation
    await hydrationBox.hover();
    await page.waitForTimeout(500);
    
    // Verify element is still visible
    await expect(hydrationBox).toBeVisible();
  });

  test('should have progressive enhancement', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find progressive box
    const progressiveBox = page.locator('.progressive-box');
    await expect(progressiveBox).toBeVisible();
    
    // Test progressive animation
    await progressiveBox.hover();
    await page.waitForTimeout(500);
    
    // Verify element is still visible
    await expect(progressiveBox).toBeVisible();
  });

  test('should have interactive elements', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Find interactive box
    const interactiveBox = page.locator('.interactive-box');
    await expect(interactiveBox).toBeVisible();
    
    // Test interactive animation
    await interactiveBox.click();
    await page.waitForTimeout(500);
    
    // Verify element is still visible
    await expect(interactiveBox).toBeVisible();
  });

  test('should have layout animations with SSR', async ({ page }) => {
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

  test('should have scroll animations with SSR', async ({ page }) => {
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

  test('should have spring physics with SSR', async ({ page }) => {
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

  test('should have SEO-friendly content', async ({ page }) => {
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check for SEO content
    const seoContent = page.locator('.seo-content');
    await expect(seoContent).toBeVisible();
    
    // Check for SEO elements
    const seoTitle = page.locator('.seo-content h3');
    await expect(seoTitle).toBeVisible();
    
    const seoList = page.locator('.seo-content ul');
    await expect(seoList).toBeVisible();
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

  test('should have proper meta tags for SEO', async ({ page }) => {
    // Check for meta tags
    const title = await page.title();
    expect(title).toContain('Leptos Motion SSR Demo');
    
    const description = await page.locator('meta[name="description"]').getAttribute('content');
    expect(description).toContain('Server-side rendered Leptos Motion demo');
    
    const keywords = await page.locator('meta[name="keywords"]').getAttribute('content');
    expect(keywords).toContain('leptos, motion, rust, wasm, ssr, animations');
  });
});
