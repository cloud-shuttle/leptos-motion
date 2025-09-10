import { test, expect } from '@playwright/test';

test.describe('Minimal Demo Test - Isolate the Issue', () => {
  test('Step 1: HTML should load without WASM', async ({ page }) => {
    // Test: Basic HTML should load and be responsive
    await page.goto('/');

    // Wait for DOM content to load (not network idle)
    await page.waitForLoadState('domcontentloaded', { timeout: 5000 });

    // Should be able to access basic DOM
    const body = await page.$('body');
    expect(body).toBeTruthy();

    const bodyText = await page.textContent('body');
    expect(bodyText).toBeTruthy();
    expect(bodyText!.length).toBeGreaterThan(0);
  });

  test('Step 2: WASM script should be present but not executed', async ({ page }) => {
    // Test: WASM script tag should exist but not cause hanging
    await page.goto('/');

    // Wait for DOM content only
    await page.waitForLoadState('domcontentloaded', { timeout: 5000 });

    // Check that WASM script exists
    const scriptTags = await page.$$('script[type="module"]');
    expect(scriptTags.length).toBeGreaterThan(0);

    // Check script content
    const scriptContent = await page.content();
    expect(scriptContent).toContain('comprehensive-demo-cbb9f9e91b5563c5.js');

    // Page should still be responsive
    const body = await page.$('body');
    expect(body).toBeTruthy();
  });

  test('Step 3: WASM execution should not hang the browser', async ({ page }) => {
    // Test: WASM execution should complete without hanging
    const startTime = Date.now();

    await page.goto('/');

    // Wait for network idle (WASM files loaded)
    await page.waitForLoadState('networkidle', { timeout: 10000 });

    const loadTime = Date.now() - startTime;
    console.log(`WASM load time: ${loadTime}ms`);

    // Should still be responsive after WASM loads
    const body = await page.$('body');
    expect(body).toBeTruthy();

    // Should be able to get body text
    const bodyText = await page.textContent('body');
    expect(bodyText).toBeTruthy();
  });

  test('Step 4: WASM bindings should be available', async ({ page }) => {
    // Test: WASM should execute and expose bindings
    await page.goto('/');

    // Wait for WASM to load
    await page.waitForLoadState('networkidle', { timeout: 10000 });

    // Wait a bit for WASM to initialize
    await page.waitForTimeout(2000);

    // Check if WASM bindings are available
    const hasBindings = await page.evaluate(() => {
      return typeof window.wasmBindings !== 'undefined';
    });

    expect(hasBindings).toBe(true);
  });

  test('Step 5: Leptos should mount without hanging', async ({ page }) => {
    // Test: Leptos mounting should complete
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(`[${msg.type()}] ${msg.text()}`);
    });

    await page.goto('/');

    // Wait for WASM to load
    await page.waitForLoadState('networkidle', { timeout: 10000 });

    // Wait for Leptos to mount
    await page.waitForTimeout(5000);

    // Check console messages
    console.log('Console messages:', consoleMessages);

    // Should see mounting messages
    const mountingMessages = consoleMessages.filter(
      msg => msg.includes('Leptos Motion') || msg.includes('mount')
    );
    expect(mountingMessages.length).toBeGreaterThan(0);

    // Should not see error messages
    const errorMessages = consoleMessages.filter(
      msg => msg.includes('[error]') || msg.includes('Error') || msg.includes('panic')
    );
    expect(errorMessages).toHaveLength(0);
  });
});
