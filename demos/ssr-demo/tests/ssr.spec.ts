import { test, expect } from '@playwright/test';

test.describe('Leptos Motion - SSR Demo', () => {
  let baseUrl: string;

  test.beforeAll(async () => {
    baseUrl = 'http://127.0.0.1:3000';
    console.log(`✅ Using SSR demo server at ${baseUrl}`);
  });

  test('SSR demo page loads successfully', async ({ page }) => {
    await page.goto(baseUrl);

    // Check page title
    await expect(page).toHaveTitle('Leptos Motion - SSR Demo');

    // Check main heading
    const heading = page.locator('h1').filter({ hasText: 'Leptos Motion' });
    await expect(heading).toBeVisible();

    // Check SSR status message
    const ssrStatus = page.locator('text=Server-Side Rendered ✓');
    await expect(ssrStatus).toBeVisible();

    // Check demo active message
    const demoActive = page.locator('text=SSR Demo Active');
    await expect(demoActive).toBeVisible();

    console.log('✅ SSR page loads successfully');
  });

  test('health endpoint returns correct status', async ({ request }) => {
    const response = await request.get(`${baseUrl}/health`);
    expect(response.status()).toBe(200);

    const health = await response.json();
    expect(health.status).toBe('healthy');
    expect(health.ssr).toBe('enabled');
    expect(health.motion_div).toBe('ready');

    console.log('✅ Health endpoint returns correct status');
  });

  test('page content is server-rendered (no WASM loading)', async ({ page }) => {
    // Listen for console messages
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
    });

    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');

    // Wait a bit to ensure any client-side scripts would have run
    await page.waitForTimeout(2000);

    // Check that content is immediately visible (server-rendered)
    const heading = page.locator('h1').filter({ hasText: 'Leptos Motion' });
    await expect(heading).toBeVisible();

    const ssrStatus = page.locator('text=Server-Side Rendered ✓');
    await expect(ssrStatus).toBeVisible();

    // Check for features list
    const featuresList = page.locator('ul li').filter({ hasText: 'MotionDiv components with SSR support' });
    await expect(featuresList).toBeVisible();

    // Check that there are no WASM-related console errors
    const errorMessages = consoleMessages.filter(msg =>
      msg.includes('Error') || msg.includes('Failed') || msg.includes('wasm')
    );
    expect(errorMessages.length).toBe(0);

    console.log('✅ Page content is server-rendered correctly');
  });

  test('page has correct styling', async ({ page }) => {
    await page.goto(baseUrl);

    // Check background gradient
    const body = page.locator('body');
    const background = await body.evaluate(el => getComputedStyle(el).backgroundImage);
    expect(background).toContain('linear-gradient');

    // Check container styling
    const container = page.locator('.container');
    await expect(container).toBeVisible();

    // Check heading styling
    const heading = page.locator('h1');
    const fontSize = await heading.evaluate(el => getComputedStyle(el).fontSize);
    expect(fontSize).toBe('48px');

    console.log('✅ Page has correct styling');
  });

  test('page is performant (fast loading)', async ({ page }) => {
    const startTime = Date.now();

    await page.goto(baseUrl);
    await page.waitForLoadState('networkidle');

    const loadTime = Date.now() - startTime;
    console.log(`SSR page load time: ${loadTime}ms`);

    // SSR should load reasonably quickly (under 1 second for a simple page)
    expect(loadTime).toBeLessThan(1000);

    console.log('✅ Page loads performantly');
  });

  test('server handles multiple requests', async ({ request }) => {
    // Test multiple requests to ensure server stability
    const promises = [];
    for (let i = 0; i < 5; i++) {
      promises.push(request.get(baseUrl));
    }

    const responses = await Promise.all(promises);
    responses.forEach(response => {
      expect(response.status()).toBe(200);
    });

    console.log('✅ Server handles multiple requests correctly');
  });

  test('content is accessible', async ({ page }) => {
    await page.goto(baseUrl);

    // Check semantic HTML
    const heading = page.locator('h1');
    await expect(heading).toBeVisible();

    const headings = page.locator('h2, h3');
    await expect(headings).toHaveCount(2);

    // Check list content
    const listItems = page.locator('ul li');
    await expect(listItems).toHaveCount(5); // Should have 5 feature items

    console.log('✅ Content is accessible with proper semantic structure');
  });

  test('responsive design elements present', async ({ page }) => {
    await page.goto(baseUrl);

    // Check viewport meta tag
    const viewport = page.locator('meta[name="viewport"]');
    await expect(viewport).toHaveAttribute('content', 'width=device-width, initial-scale=1.0');

    // Check container has max-width for responsiveness
    const container = page.locator('.container');
    const maxWidth = await container.evaluate(el => getComputedStyle(el).maxWidth);
    expect(maxWidth).toBe('800px');

    console.log('✅ Responsive design elements are present');
  });
});
