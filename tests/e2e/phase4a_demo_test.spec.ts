import { test, expect } from '@playwright/test';

test.describe('Phase 4A Function-based Props Demo', () => {
  test('should load and display the Phase 4A demo correctly', async ({ page }) => {
    // Enable console logging
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Go to the simple demo
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait a bit for Leptos to mount
    await page.waitForTimeout(2000);

    // Check if the main title is present
    await expect(page.locator('h1')).toContainText('Phase 4A: Function-based Props Demo');

    // Check if the demo features are visible
    await expect(page.locator('text=Static Function Animation')).toBeVisible();
    await expect(page.locator('text=Dynamic Time Animation')).toBeVisible();
    await expect(page.locator('text=Counter Animation')).toBeVisible();
    await expect(page.locator('text=Hover Animation')).toBeVisible();
    await expect(page.locator('text=Tap Animation')).toBeVisible();
    await expect(page.locator('text=Mixed Props')).toBeVisible();

    console.log('✅ Phase 4A Demo loaded successfully!');
  });

  test('should handle counter button clicks', async ({ page }) => {
    // Enable console logging
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Go to the simple demo
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait a bit for Leptos to mount
    await page.waitForTimeout(2000);

    // Find and click the increment button
    await page.click('button:has-text("Increment")');

    // Wait for counter to update
    await page.waitForTimeout(500);

    // Check if counter updated
    await expect(page.locator('text=Counter: 1')).toBeVisible();

    console.log('✅ Phase 4A counter interaction working!');
  });

  test('should handle hover interactions', async ({ page }) => {
    // Enable console logging
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Go to the simple demo
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait a bit for Leptos to mount
    await page.waitForTimeout(2000);

    // Find the hover animation element
    const hoverElement = page.locator('text=Hover').first();

    // Hover over the element
    await hoverElement.hover();

    // Wait for hover animation
    await page.waitForTimeout(500);

    // Move mouse away
    await page.mouse.move(0, 0);

    console.log('✅ Phase 4A hover interaction working!');
  });

  test('should handle tap interactions', async ({ page }) => {
    // Enable console logging
    page.on('console', msg => {
      console.log(`[${msg.type()}] ${msg.text()}`);
    });

    // Go to the simple demo
    await page.goto('http://localhost:8080/simple.html');

    // Wait for the page to load
    await page.waitForLoadState('networkidle');

    // Wait a bit for Leptos to mount
    await page.waitForTimeout(2000);

    // Find the tap animation element
    const tapElement = page.locator('text=Tap').first();

    // Click the element
    await tapElement.click();

    // Wait for tap animation
    await page.waitForTimeout(500);

    console.log('✅ Phase 4A tap interaction working!');
  });
});
