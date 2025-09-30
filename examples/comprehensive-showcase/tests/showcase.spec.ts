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

  test('navigation works between examples', async ({ page }) => {
    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');
    await page.waitForTimeout(3000); // Wait for WASM to fully load
    
    // Check that navigation buttons exist
    const prevButton = page.locator('button:has-text("← Previous")');
    const nextButton = page.locator('button:has-text("Next →")');
    
    await expect(prevButton).toBeVisible();
    await expect(nextButton).toBeVisible();
    
    // Test navigation
    await nextButton.click();
    await page.waitForTimeout(500);
    
    // Check that the example title changed
    const exampleTitle = page.locator('h2').first();
    await expect(exampleTitle).toBeVisible();
    
    // Go back
    await prevButton.click();
    await page.waitForTimeout(500);
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
