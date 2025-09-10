import { test, expect } from '@playwright/test';

test.describe('Isolate the Hang - Find the Exact Problem', () => {
  test('Step 1: Leptos mounts but DOM becomes unresponsive', async ({ page }) => {
    // Test: Find exactly where the hang occurs after Leptos mounting
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
    });

    await page.goto('/');

    // Wait for WASM to load
    await page.waitForLoadState('networkidle', { timeout: 10000 });

    // Wait for Leptos to mount (we know this works)
    await page.waitForTimeout(3000);

    // Check console messages
    const mountingMessages = consoleMessages.filter(
      msg => msg.includes('mounted successfully') || msg.includes('completed successfully')
    );
    expect(mountingMessages.length).toBeGreaterThan(0);

    // Now try to access DOM - this is where it hangs
    console.log('About to try DOM access...');
    const body = await page.$('body');
    console.log('DOM access successful!');
    expect(body).toBeTruthy();
  });

  test("Step 2: Check if it's a specific DOM operation", async ({ page }) => {
    // Test: Try different DOM operations to isolate the issue
    await page.goto('/');
    await page.waitForLoadState('networkidle', { timeout: 10000 });
    await page.waitForTimeout(3000);

    // Try different DOM operations
    console.log('Trying page.content()...');
    const content = await page.content();
    console.log('page.content() successful!');
    expect(content.length).toBeGreaterThan(0);

    console.log('Trying page.textContent()...');
    const text = await page.textContent('body');
    console.log('page.textContent() successful!');
    expect(text).toBeTruthy();
  });

  test("Step 3: Check if it's event handling", async ({ page }) => {
    // Test: See if event handling is causing the hang
    await page.goto('/');
    await page.waitForLoadState('networkidle', { timeout: 10000 });
    await page.waitForTimeout(3000);

    // Try to find interactive elements
    console.log('Looking for interactive elements...');
    const buttons = await page.$$('button');
    console.log(`Found ${buttons.length} buttons`);

    const clickableElements = await page.$$('[onclick], [role="button"], button, a');
    console.log(`Found ${clickableElements.length} clickable elements`);

    // Try to hover over an element
    if (clickableElements.length > 0) {
      console.log('Trying to hover...');
      await clickableElements[0].hover();
      console.log('Hover successful!');
    }
  });

  test("Step 4: Check if it's CSS/styling", async ({ page }) => {
    // Test: See if CSS operations are causing the hang
    await page.goto('/');
    await page.waitForLoadState('networkidle', { timeout: 10000 });
    await page.waitForTimeout(3000);

    // Try to get computed styles
    console.log('Trying to get computed styles...');
    const body = await page.$('body');
    if (body) {
      const styles = await body.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return {
          fontFamily: computed.fontFamily,
          backgroundColor: computed.backgroundColor,
        };
      });
      console.log('Computed styles successful!', styles);
      expect(styles.fontFamily).toBeTruthy();
    }
  });

  test("Step 5: Check if it's animation-related", async ({ page }) => {
    // Test: See if animations are causing the hang
    await page.goto('/');
    await page.waitForLoadState('networkidle', { timeout: 10000 });
    await page.waitForTimeout(3000);

    // Look for animated elements
    console.log('Looking for animated elements...');
    const animatedElements = await page.$$(
      '.animated-box, [class*="motion"], [style*="transform"]'
    );
    console.log(`Found ${animatedElements.length} animated elements`);

    // Try to interact with animated elements
    if (animatedElements.length > 0) {
      console.log('Trying to interact with animated element...');
      await animatedElements[0].hover();
      await page.waitForTimeout(1000);
      console.log('Animation interaction successful!');
    }
  });
});
