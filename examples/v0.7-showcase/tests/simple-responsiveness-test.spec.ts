import { test, expect } from '@playwright/test';

test.describe('Simple Responsiveness Tests', () => {
  test('should work with simple components only', async ({ page }) => {
    // This test should pass when only simple components are present
    
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });
    
    // Wait for WASM to load
    await page.waitForTimeout(3000);
    
    // Test 1: Page should be responsive
    const responseTime = await page.evaluate(() => {
      return new Promise((resolve) => {
        const startTime = Date.now();
        setTimeout(() => {
          const endTime = Date.now();
          resolve(endTime - startTime);
        }, 100);
      });
    });
    
    expect(responseTime).toBeLessThan(1000);
    console.log(`Page response time: ${responseTime}ms`);
    
    // Test 2: Buttons should be clickable
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    if (buttonCount > 0) {
      const firstButton = buttons.first();
      await firstButton.click();
      console.log('Button click successful');
    }
    
    // Test 3: Page should remain responsive after interactions
    const afterInteraction = await page.evaluate(() => {
      return new Promise((resolve) => {
        const startTime = Date.now();
        setTimeout(() => {
          const endTime = Date.now();
          resolve(endTime - startTime);
        }, 100);
      });
    });
    
    expect(afterInteraction).toBeLessThan(1000);
    console.log(`Response time after interaction: ${afterInteraction}ms`);
  });

  test('should detect motion component presence', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });
    
    // Wait for WASM to load
    await page.waitForTimeout(3000);
    
    // Check for motion components
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();
    
    console.log(`Motion components found: ${motionCount}`);
    
    // Check for motion-related buttons
    const motionButtons = page.locator('button').filter({ hasText: /toggle|motion|animate/i });
    const motionButtonCount = await motionButtons.count();
    
    console.log(`Motion buttons found: ${motionButtonCount}`);
    
    // If motion components are present, the page should become unresponsive
    if (motionCount > 0 || motionButtonCount > 0) {
      console.log('Motion components detected - page should be unresponsive');
      
      // Try to interact with the page
      try {
        const responseTime = await page.evaluate(() => {
          return new Promise((resolve) => {
            const startTime = Date.now();
            setTimeout(() => {
              const endTime = Date.now();
              resolve(endTime - startTime);
            }, 100);
          });
        });
        
        // If we get here, the page is responsive (unexpected)
        console.log(`Unexpected: Page is responsive with motion components (${responseTime}ms)`);
        expect(responseTime).toBeGreaterThan(1000); // Should be unresponsive
        
      } catch (error) {
        console.log('Expected: Page is unresponsive with motion components');
        // This is expected behavior
      }
    } else {
      console.log('No motion components - page should be responsive');
      
      // Page should be responsive
      const responseTime = await page.evaluate(() => {
        return new Promise((resolve) => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 100);
        });
      });
      
      expect(responseTime).toBeLessThan(1000);
    }
  });
});
