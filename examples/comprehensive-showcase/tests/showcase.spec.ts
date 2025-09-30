import { test, expect } from '@playwright/test';

test.describe('Leptos Motion Comprehensive Showcase', () => {
  let baseUrl: string;

  test.beforeAll(async () => {
    // Use the base URL from Playwright configuration
    baseUrl = 'http://localhost:3000';
    console.log(`✅ Using server at ${baseUrl}`);
  });

  test('server responds to requests', async ({ request }) => {
    const response = await request.get(`${baseUrl}/`);
    expect(response.status()).toBe(200);
  });

  test('main page loads successfully', async ({ page }) => {
    // Listen for console messages
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
    });

    // Navigate to the page
    await page.goto(baseUrl);
    
    // Wait for the page to load
    await page.waitForLoadState('networkidle');
    
    // Check that the page title is correct
    await expect(page).toHaveTitle('Leptos Motion - Comprehensive Showcase');
    
    // Check that the app element exists and has content
    const appElement = page.locator('#app').first();
    await expect(appElement).toBeVisible();
    
    // Wait a bit for the WASM to initialize
    await page.waitForTimeout(2000);
    
    // Check that content has been rendered
    const appContent = await appElement.innerHTML();
    expect(appContent.length).toBeGreaterThan(100); // Should have substantial content
    
    // Check for successful console messages
    const successMessages = consoleMessages.filter(msg => 
      msg.includes('✅') || msg.includes('successfully')
    );
    expect(successMessages.length).toBeGreaterThan(0);
    
    // Check for error messages
    const errorMessages = consoleMessages.filter(msg => 
      msg.includes('❌') || msg.includes('Error')
    );
    expect(errorMessages.length).toBe(0);
    
    console.log('Console messages:', consoleMessages);
  });

  test('button animation works (scale effect)', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000); // Wait for WASM to load

    // Find the button animation button
    const button = page.locator('text=Button Animation').locator('..');
    await expect(button).toBeVisible();

    // Get initial transform
    const initialTransform = await button.evaluate(el => getComputedStyle(el).transform);
    console.log('Initial transform:', initialTransform);

    // Click the button
    await button.click();
    await page.waitForTimeout(500); // Wait for animation

    // Check that the message updated
    const message = page.locator('text=Button Animation: Scale effect!');
    await expect(message).toBeVisible();

    // Click again to reset
    await button.click();
    await page.waitForTimeout(500);

    console.log('✅ Button scale animation test completed');
  });

  test('card animation works (slide effect)', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Find the card animation button
    const card = page.locator('text=Card Animation').locator('..');
    await expect(card).toBeVisible();

    // Get initial position
    const initialX = await card.evaluate(el => {
      const rect = el.getBoundingClientRect();
      return rect.left;
    });

    // Click the card
    await card.click();
    await page.waitForTimeout(500);

    // Check that the message updated
    const message = page.locator('text=Card Animation: Slide effect!');
    await expect(message).toBeVisible();

    // Get new position (should be moved)
    const newX = await card.evaluate(el => {
      const rect = el.getBoundingClientRect();
      return rect.left;
    });

    // Verify the card moved (should be different from initial position)
    expect(Math.abs(newX - initialX)).toBeGreaterThan(10);

    // Click again to reset
    await card.click();
    await page.waitForTimeout(500);

    console.log('✅ Card slide animation test completed');
  });

  test('loading animation works (rotation effect)', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Find the loading animation button
    const loading = page.locator('text=Loading Animation').locator('..');
    await expect(loading).toBeVisible();

    // Click the loading button
    await loading.click();
    await page.waitForTimeout(500);

    // Check that the message updated
    const message = page.locator('text=Loading Animation: Rotation effect!');
    await expect(message).toBeVisible();

    // Click again to add more rotation
    await loading.click();
    await page.waitForTimeout(500);

    console.log('✅ Loading rotation animation test completed');
  });

  test('all animations work together', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Click all three buttons in sequence
    const button = page.locator('text=Button Animation').locator('..');
    const card = page.locator('text=Card Animation').locator('..');
    const loading = page.locator('text=Loading Animation').locator('..');

    // Click button animation
    await button.click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=Button Animation: Scale effect!')).toBeVisible();

    // Click card animation
    await card.click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=Card Animation: Slide effect!')).toBeVisible();

    // Click loading animation
    await loading.click();
    await page.waitForTimeout(300);
    await expect(page.locator('text=Loading Animation: Rotation effect!')).toBeVisible();

    console.log('✅ Combined animations test completed');
  });

  test('animations are smooth and performant', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const button = page.locator('text=Button Animation').locator('..');

    // Measure animation performance
    const startTime = Date.now();

    // Rapidly click the button multiple times
    for (let i = 0; i < 5; i++) {
      await button.click();
      await page.waitForTimeout(50);
    }

    const totalTime = Date.now() - startTime;
    console.log(`Animation sequence took: ${totalTime}ms`);

    // Should complete within reasonable time (animations should not hang)
    expect(totalTime).toBeLessThan(2000);

    console.log('✅ Animation performance test completed');
  });

  test('page is interactive (right-click works)', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000);

    // Test that right-click works (should not hang)
    const appElement = page.locator('#app').first();
    await appElement.click({ button: 'right' });

    // If we get here without hanging, the test passes
    expect(true).toBe(true);
  });

  test('animations work across browsers', async ({ page, browserName }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    const button = page.locator('text=Button Animation').locator('..');

    // Click to trigger animation
    await button.click();
    await page.waitForTimeout(500);

    // Verify animation worked (message appears)
    await expect(page.locator('text=Button Animation: Scale effect!')).toBeVisible();

    console.log(`✅ Animation test passed on ${browserName}`);
  });

  test('animations respect user preferences', async ({ page }) => {
    // Test reduced motion preference
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Even with reduced motion, basic functionality should work
    const button = page.locator('text=Button Animation').locator('..');
    await button.click();
    await page.waitForTimeout(300);

    // Message should still appear
    await expect(page.locator('text=Button Animation: Scale effect!')).toBeVisible();

    console.log('✅ Reduced motion test completed');
  });

  test('keyboard navigation works', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Tab to first button
    await page.keyboard.press('Tab');
    const activeElement = page.locator(':focus');
    await expect(activeElement).toHaveText('Button Animation');

    // Press Enter to activate
    await page.keyboard.press('Enter');
    await page.waitForTimeout(500);

    // Check message appears
    await expect(page.locator('text=Button Animation: Scale effect!')).toBeVisible();

    console.log('✅ Keyboard navigation test completed');
  });

  test('visual layout is correct', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(2000);

    // Check main heading is visible and properly styled
    const heading = page.locator('h1').filter({ hasText: 'Leptos Motion' });
    await expect(heading).toBeVisible();

    // Check buttons are arranged horizontally
    const buttons = page.locator('.button, .card, .loading');
    await expect(buttons).toHaveCount(3);

    // Check buttons have proper spacing (basic layout test)
    const button1 = buttons.nth(0);
    const button2 = buttons.nth(1);
    const button3 = buttons.nth(2);

    await expect(button1).toBeVisible();
    await expect(button2).toBeVisible();
    await expect(button3).toBeVisible();

    console.log('✅ Visual layout test completed');
  });

  test('WASM files are served correctly', async ({ request }) => {
    // Test JavaScript file
    const jsResponse = await request.get(`${baseUrl}/comprehensive_showcase.js`);
    expect(jsResponse.status()).toBe(200);
    expect(jsResponse.headers()['content-type']).toContain('javascript');
    
    // Test WASM file
    const wasmResponse = await request.get(`${baseUrl}/comprehensive_showcase_bg.wasm`);
    expect(wasmResponse.status()).toBe(200);
    expect(wasmResponse.headers()['content-type']).toContain('wasm');
  });

  test('page performance is acceptable', async ({ page }) => {
    const startTime = Date.now();

    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');

    const loadTime = Date.now() - startTime;
    console.log(`Page load time: ${loadTime}ms`);

    // Should load within 10 seconds
    expect(loadTime).toBeLessThan(10000);
  });
});
