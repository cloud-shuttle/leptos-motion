import { test, expect } from '@playwright/test';

test.describe('Interactive Elements Debug - TDD Approach', () => {
  test.beforeEach(async ({ page }) => {
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', error => console.log('PAGE ERROR:', error.message));
    
    await page.goto('/');
    await page.waitForTimeout(3000); // Wait longer for full render
  });

  test('Step 1: Check what buttons actually exist', async ({ page }) => {
    // Let's see what buttons are actually rendered
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    
    console.log(`Found ${buttonCount} buttons`);
    
    // Log each button's text
    for (let i = 0; i < buttonCount; i++) {
      const button = buttons.nth(i);
      const text = await button.textContent();
      const isVisible = await button.isVisible();
      console.log(`Button ${i}: "${text}" (visible: ${isVisible})`);
    }
    
    await page.screenshot({ path: 'debug-step1-buttons.png' });
    
    // We expect at least some buttons
    expect(buttonCount).toBeGreaterThan(0);
  });

  test('Step 2: Check if buttons have click handlers', async ({ page }) => {
    // Check if buttons have any event listeners
    const hasClickHandlers = await page.evaluate(() => {
      const buttons = document.querySelectorAll('button');
      let hasHandlers = false;
      
      buttons.forEach(button => {
        // Check if button has any onclick or event listeners
        if (button.onclick || button.getAttribute('on:click')) {
          hasHandlers = true;
        }
      });
      
      return hasHandlers;
    });
    
    console.log('Buttons have click handlers:', hasClickHandlers);
    
    await page.screenshot({ path: 'debug-step2-click-handlers.png' });
    
    // This might fail - let's see what we find
    expect(hasClickHandlers).toBe(true);
  });

  test('Step 3: Check if MotionDiv components are properly set up', async ({ page }) => {
    // Check if our MotionDiv components have the expected attributes
    const motionDivs = page.locator('[class*="animated-box"], [class*="content-box"], [class*="gesture-box"]');
    const motionDivCount = await motionDivs.count();
    
    console.log(`Found ${motionDivCount} MotionDiv components`);
    
    // Check if they have motion-related attributes
    const hasMotionAttributes = await page.evaluate(() => {
      const motionDivs = document.querySelectorAll('[class*="animated-box"], [class*="content-box"], [class*="gesture-box"]');
      let hasAttributes = false;
      
      motionDivs.forEach(div => {
        if (div.getAttribute('data-flip-initial') || 
            div.style.transform || 
            div.style.opacity !== '') {
          hasAttributes = true;
        }
      });
      
      return hasAttributes;
    });
    
    console.log('MotionDiv components have motion attributes:', hasMotionAttributes);
    
    await page.screenshot({ path: 'debug-step3-motion-attributes.png' });
    
    expect(motionDivCount).toBeGreaterThan(0);
  });

  test('Step 4: Check if state signals are working', async ({ page }) => {
    // Check if Leptos signals are properly initialized
    const hasSignals = await page.evaluate(() => {
      // Look for any reactive state in the DOM
      const body = document.body;
      const hasCounter = body.textContent?.includes('Count: 0');
      const hasToggle = body.textContent?.includes('Hide') || body.textContent?.includes('Show');
      
      return { hasCounter, hasToggle };
    });
    
    console.log('State signals status:', hasSignals);
    
    await page.screenshot({ path: 'debug-step4-state-signals.png' });
    
    // We expect at least some state to be present
    expect(hasSignals.hasCounter || hasSignals.hasToggle).toBe(true);
  });

  test('Step 5: Check browser console for errors', async ({ page }) => {
    // Wait a bit and check for any JavaScript errors
    await page.waitForTimeout(2000);
    
    // This test is more about observation
    console.log('Checking for console errors...');
    
    await page.screenshot({ path: 'debug-step5-console-check.png' });
    
    // This should always pass - it's just for debugging
    expect(true).toBe(true);
  });
});
