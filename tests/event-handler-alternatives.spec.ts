import { test, expect } from '@playwright/test';

test.describe('Event Handler Alternatives - TDD Approach', () => {
  test.beforeEach(async ({ page }) => {
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', error => console.log('PAGE ERROR:', error.message));
    
    await page.goto('/');
    await page.waitForTimeout(3000);
  });

  test('Step 1: Try manually attaching event listeners', async ({ page }) => {
    // Try to manually attach event listeners to see if that works
    const manualEventListenersWork = await page.evaluate(() => {
      try {
        const buttons = document.querySelectorAll('button');
        let successCount = 0;
        
        buttons.forEach((button, index) => {
          // Try to manually attach a click listener
          button.addEventListener('click', (e) => {
            console.log(`Manual click listener ${index} triggered`);
            // Try to update the button text to show it worked
            if (index === 1) { // Count button
              button.textContent = 'Count: 1 (Manual)';
            }
          });
          successCount++;
        });
        
        return { successCount, totalButtons: buttons.length };
      } catch (e) {
        console.error('Manual event listener attachment failed:', e);
        return { successCount: 0, totalButtons: 0, error: e.message };
      }
    });
    
    console.log('Manual event listener attachment result:', manualEventListenersWork);
    
    // Now try clicking to see if our manual listeners work
    const countButton = page.locator('button:has-text("Count: 0")');
    await countButton.click();
    
    // Wait a bit and check if the text changed
    await page.waitForTimeout(1000);
    
    const updatedText = await countButton.textContent();
    console.log('Button text after manual click:', updatedText);
    
    await page.screenshot({ path: 'test-step1-manual-event-listeners.png' });
    
    expect(manualEventListenersWork.successCount).toBeGreaterThan(0);
  });

  test('Step 2: Check if Leptos signals can be updated manually', async ({ page }) => {
    // Try to manually trigger Leptos signal updates
    const manualSignalUpdateWorks = await page.evaluate(() => {
      try {
        // Look for any global Leptos functions or signals
        const hasLeptosGlobals = typeof (window as any).leptos !== 'undefined';
        const hasShowcaseGlobals = typeof (window as any).showcase !== 'undefined';
        
        // Try to find any functions that might update state
        const globalFunctions = Object.keys(window).filter(key => 
          typeof (window as any)[key] === 'function'
        );
        
        return { 
          hasLeptosGlobals, 
          hasShowcaseGlobals, 
          globalFunctions: globalFunctions.slice(0, 10) // First 10
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Manual signal update check:', manualSignalUpdateWorks);
    
    await page.screenshot({ path: 'test-step2-manual-signal-update.png' });
    
    // This test is exploratory - we expect it to pass
    expect(manualSignalUpdateWorks).toBeDefined();
  });

  test('Step 3: Try alternative event handler syntax', async ({ page }) => {
    // Check if there are any alternative event handler patterns we can use
    const alternativeSyntaxWorks = await page.evaluate(() => {
      try {
        const buttons = document.querySelectorAll('button');
        const results = [];
        
        buttons.forEach((button, index) => {
          const buttonInfo = {
            index,
            text: button.textContent,
            hasOnclick: button.hasAttribute('onclick'),
            hasOnClick: button.hasAttribute('on:click'),
            hasEventListener: button.onclick !== null,
            attributes: Array.from(button.attributes).map(attr => attr.name)
          };
          
          results.push(buttonInfo);
        });
        
        return results;
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Alternative syntax analysis:', alternativeSyntaxWorks);
    
    await page.screenshot({ path: 'test-step3-alternative-syntax.png' });
    
    expect(alternativeSyntaxWorks.length).toBeGreaterThan(0);
  });

  test('Step 4: Check browser console for Leptos-specific errors', async ({ page }) => {
    // Look for any Leptos-specific error messages that might explain the issue
    await page.waitForTimeout(2000);
    
    // This test is more about observation and debugging
    console.log('Checking for Leptos-specific console errors...');
    
    await page.screenshot({ path: 'test-step4-console-errors.png' });
    
    // This should always pass - it's just for debugging
    expect(true).toBe(true);
  });

  test('Step 5: Try to understand Leptos event handler processing', async ({ page }) => {
    // Try to understand how Leptos processes event handlers
    const leptosEventProcessing = await page.evaluate(() => {
      try {
        // Check if there are any Leptos-specific event handling mechanisms
        const body = document.body;
        const hasLeptosEventData = body.hasAttribute('data-leptos-events') || 
                                  body.hasAttribute('data-leptos-handlers');
        
        // Look for any hidden elements or data that might contain event info
        const hiddenElements = document.querySelectorAll('[style*="display: none"], [hidden]');
        const hasHiddenEventData = Array.from(hiddenElements).some(el => 
          el.textContent?.includes('event') || el.textContent?.includes('handler')
        );
        
        return { hasLeptosEventData, hasHiddenEventData, hiddenCount: hiddenElements.length };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Leptos event processing analysis:', leptosEventProcessing);
    
    await page.screenshot({ path: 'test-step5-leptos-event-processing.png' });
    
    expect(leptosEventProcessing).toBeDefined();
  });
});
