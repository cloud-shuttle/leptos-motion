import { test, expect } from '@playwright/test';

test.describe('Motion Component Tests', () => {
  test('should detect ReactiveMotionDiv unresponsiveness', async ({ page }) => {
    // This test specifically targets the ReactiveMotionDiv issue we discovered

    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Check if motion components are present
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();

    if (motionCount === 0) {
      console.log('No motion components found - skipping test');
      return;
    }

    console.log(`Found ${motionCount} motion components`);

    // Test responsiveness before interacting with motion components
    const beforeInteraction = await page.evaluate(() => {
      return new Promise(resolve => {
        const startTime = Date.now();
        setTimeout(() => {
          const endTime = Date.now();
          resolve(endTime - startTime);
        }, 100);
      });
    });

    console.log(`Response time before interaction: ${beforeInteraction}ms`);

    // Try to interact with motion components
    const firstMotionDiv = motionDivs.first();

    try {
      // Try to click on the motion component
      await firstMotionDiv.click({ timeout: 1000 });

      // Test responsiveness after interaction
      const afterInteraction = await page.evaluate(() => {
        return new Promise(resolve => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 100);
        });
      });

      console.log(`Response time after interaction: ${afterInteraction}ms`);

      // Response time should not increase dramatically
      expect(afterInteraction).toBeLessThan(1000);
    } catch (error) {
      console.log('Motion component interaction failed:', error);
      // This might indicate the component is unresponsive
      throw new Error('Motion component appears to be unresponsive');
    }
  });

  test('should test motion component buttons', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Look for motion-related buttons
    const motionButtons = page.locator('button').filter({ hasText: /toggle|motion|animate/i });
    const buttonCount = await motionButtons.count();

    if (buttonCount === 0) {
      console.log('No motion buttons found - skipping test');
      return;
    }

    console.log(`Found ${buttonCount} motion buttons`);

    // Test each motion button
    for (let i = 0; i < buttonCount; i++) {
      const button = motionButtons.nth(i);
      const buttonText = await button.textContent();

      console.log(`Testing motion button: ${buttonText}`);

      // Test responsiveness before clicking
      const beforeClick = await page.evaluate(() => {
        return new Promise(resolve => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 100);
        });
      });

      // Click the button
      await button.click();

      // Wait for potential animation
      await page.waitForTimeout(500);

      // Test responsiveness after clicking
      const afterClick = await page.evaluate(() => {
        return new Promise(resolve => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 100);
        });
      });

      console.log(`Response time before: ${beforeClick}ms, after: ${afterClick}ms`);

      // Response time should not increase dramatically
      expect(afterClick).toBeLessThan(1000);

      // Test if we can still interact with the page
      const canInteract = await page.evaluate(() => {
        return new Promise(resolve => {
          // Try to create a simple element
          const div = document.createElement('div');
          div.textContent = 'test';
          document.body.appendChild(div);
          document.body.removeChild(div);
          resolve(true);
        });
      });

      expect(canInteract).toBe(true);
    }
  });

  test('should detect infinite loops in motion components', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Monitor console for signs of infinite loops
    const consoleMessages: string[] = [];
    page.on('console', msg => {
      consoleMessages.push(msg.text());
    });

    // Look for motion components
    const motionDivs = page.locator('[data-motion]');
    const motionCount = await motionDivs.count();

    if (motionCount === 0) {
      console.log('No motion components found - skipping test');
      return;
    }

    // Interact with motion components
    const firstMotionDiv = motionDivs.first();
    await firstMotionDiv.click();

    // Wait and monitor console
    await page.waitForTimeout(2000);

    // Check for signs of infinite loops
    const repeatedMessages = consoleMessages.filter(
      msg =>
        msg.includes('Animation triggered') || msg.includes('Returning') || msg.includes('Effect')
    );

    // Count occurrences of each message
    const messageCounts: { [key: string]: number } = {};
    repeatedMessages.forEach(msg => {
      messageCounts[msg] = (messageCounts[msg] || 0) + 1;
    });

    // Check for excessive repetition (potential infinite loop)
    Object.entries(messageCounts).forEach(([msg, count]) => {
      if (count > 10) {
        console.log(`Potential infinite loop detected: "${msg}" repeated ${count} times`);
        expect(count).toBeLessThan(10);
      }
    });
  });

  test('should test motion component with multiple interactions', async ({ page }) => {
    await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

    // Wait for WASM to load
    await page.waitForTimeout(3000);

    // Find motion buttons
    const motionButtons = page.locator('button').filter({ hasText: /toggle|motion|animate/i });
    const buttonCount = await motionButtons.count();

    if (buttonCount === 0) {
      console.log('No motion buttons found - skipping test');
      return;
    }

    const button = motionButtons.first();

    // Perform multiple rapid interactions
    for (let i = 0; i < 5; i++) {
      console.log(`Rapid interaction ${i + 1}/5`);

      // Test responsiveness before
      const before = await page.evaluate(() => {
        return new Promise(resolve => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 50);
        });
      });

      // Click button
      await button.click();

      // Test responsiveness after
      const after = await page.evaluate(() => {
        return new Promise(resolve => {
          const startTime = Date.now();
          setTimeout(() => {
            const endTime = Date.now();
            resolve(endTime - startTime);
          }, 50);
        });
      });

      console.log(`Response time: ${before}ms -> ${after}ms`);

      // Response time should remain reasonable
      expect(after).toBeLessThan(500);

      // Small delay between interactions
      await page.waitForTimeout(100);
    }
  });
});
