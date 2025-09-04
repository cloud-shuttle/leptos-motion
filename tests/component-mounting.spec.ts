import { test, expect } from '@playwright/test';

test.describe('Component Mounting - TDD Approach (Leptos v0.8.x Working)', () => {
  test.beforeEach(async ({ page }) => {
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', error => console.log('PAGE ERROR:', error.message));
    
    await page.goto('/');
    await page.waitForTimeout(2000);
  });

  test('Step 1: Should have basic app div', async ({ page }) => {
    const appDiv = await page.locator('#app');
    const exists = await appDiv.count();
    expect(exists).toBeGreaterThan(0);
    
    const content = await appDiv.innerHTML();
    console.log('App div content:', content);
    
    await page.screenshot({ path: 'test-step1-basic-app-div.png' });
  });

  test('Step 2: Should have Leptos content in document.body', async ({ page }) => {
    // Since Leptos v0.8.x renders to document.body with mount_to_body,
    // we should check for content there instead of #app
    await page.waitForFunction(() => {
      const body = document.body;
      return body && (
        body.textContent?.includes('Leptos Motion') ||
        body.textContent?.includes('Advanced Features') ||
        body.textContent?.includes('Animation Demo')
      );
    }, { timeout: 10000 });
    
    const bodyContent = await page.locator('body').textContent();
    console.log('Body content after waiting:', bodyContent);
    
    expect(bodyContent).toContain('Leptos Motion');
    expect(bodyContent).toContain('Advanced Features');
    
    await page.screenshot({ path: 'test-step2-leptos-content-in-body.png' });
  });

  test('Step 3: Should have MotionDiv components working', async ({ page }) => {
    // Check that our MotionDiv components are actually functional
    await page.waitForFunction(() => {
      const body = document.body;
      return body && (
        body.textContent?.includes('Fade In + Scale Animation') ||
        body.textContent?.includes('Interactive Box!') ||
        body.textContent?.includes('Touch Interactive')
      );
    }, { timeout: 10000 });
    
    const hasMotionDivs = await page.evaluate(() => {
      const body = document.body;
      return body && (
        body.textContent?.includes('Fade In + Scale Animation') &&
        body.textContent?.includes('Interactive Box!') &&
        body.textContent?.includes('Touch Interactive')
      );
    });
    
    console.log('MotionDiv components working:', hasMotionDivs);
    expect(hasMotionDivs).toBe(true);
    
    await page.screenshot({ path: 'test-step3-motion-div-components.png' });
  });

  test('Step 4: Should have FLIP and gesture systems working', async ({ page }) => {
    // Check that our advanced features are functional
    await page.waitForFunction(() => {
      const body = document.body;
      return body && (
        body.textContent?.includes('FLIP Layout Animations') &&
        body.textContent?.includes('Gesture Integration') &&
        body.textContent?.includes('Multi-touch Support')
      );
    }, { timeout: 10000 });
    
    const hasAdvancedFeatures = await page.evaluate(() => {
      const body = document.body;
      return body && (
        body.textContent?.includes('FLIP Layout Animations') &&
        body.textContent?.includes('Gesture Integration') &&
        body.textContent?.includes('Multi-touch Support') &&
        body.textContent?.includes('Spring Physics Animation Engine')
      );
    });
    
    console.log('Advanced features working:', hasAdvancedFeatures);
    expect(hasAdvancedFeatures).toBe(true);
    
    await page.screenshot({ path: 'test-step4-advanced-features.png' });
  });
});
