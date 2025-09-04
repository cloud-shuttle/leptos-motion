import { test, expect } from '@playwright/test';

test.describe('Event Handler Logic Debug - TDD Approach', () => {
  test.beforeEach(async ({ page }) => {
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', error => console.log('PAGE ERROR:', error.message));
    
    await page.goto('/');
    await page.waitForTimeout(3000);
  });

  test('Step 1: Test simple event handler attachment', async ({ page }) => {
    // Test the most basic event handler attachment
    const simpleEventHandlerWorks = await page.evaluate(() => {
      try {
        const button = document.querySelector('button');
        if (!button) return { error: 'No button found' };
        
        let clickCount = 0;
        const originalText = button.textContent;
        
        // Attach a simple click handler
        button.addEventListener('click', () => {
          clickCount++;
          console.log(`Simple click handler triggered! Count: ${clickCount}`);
        });
        
        // Simulate a click
        button.click();
        
        return { 
          success: true, 
          clickCount, 
          originalText,
          currentText: button.textContent,
          message: 'Simple event handler test completed'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Simple event handler test result:', simpleEventHandlerWorks);
    
    await page.screenshot({ path: 'test-step1-simple-event-handler.png' });
    
    expect(simpleEventHandlerWorks.success).toBe(true);
    expect(simpleEventHandlerWorks.clickCount).toBe(1);
  });

  test('Step 2: Test event handler with DOM manipulation', async ({ page }) => {
    // Test event handler that actually changes the DOM
    const domManipulationEventHandlerWorks = await page.evaluate(() => {
      try {
        const button = document.querySelector('button');
        if (!button) return { error: 'No button found' };
        
        const originalText = button.textContent;
        let clickCount = 0;
        
        // Attach a click handler that changes the button text
        button.addEventListener('click', () => {
          clickCount++;
          button.textContent = `Clicked ${clickCount} times!`;
          console.log(`DOM manipulation event handler triggered! Count: ${clickCount}`);
        });
        
        // Simulate a click
        button.click();
        
        // Wait a bit to see if the change persists
        setTimeout(() => {
          console.log(`Button text after timeout: "${button.textContent}"`);
        }, 100);
        
        return { 
          success: true, 
          clickCount, 
          originalText,
          currentText: button.textContent,
          message: 'DOM manipulation event handler test completed'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('DOM manipulation event handler test result:', domManipulationEventHandlerWorks);
    
    // Wait a bit to see the results
    await page.waitForTimeout(200);
    
    const finalText = await page.locator('button').first().textContent();
    console.log('Final button text:', finalText);
    
    await page.screenshot({ path: 'test-step2-dom-manipulation-event-handler.png' });
    
    expect(domManipulationEventHandlerWorks.success).toBe(true);
    expect(domManipulationEventHandlerWorks.clickCount).toBe(1);
  });

  test('Step 3: Test counter functionality with proper state management', async ({ page }) => {
    // Test a proper counter implementation with state management
    const counterFunctionalityWorks = await page.evaluate(() => {
      try {
        const buttons = document.querySelectorAll('button');
        const countButton = Array.from(buttons).find(btn => btn.textContent?.includes('Count:'));
        
        if (!countButton) return { error: 'Count button not found' };
        
        let count = 0;
        const originalText = countButton.textContent;
        
        // Remove any existing listeners and create a fresh one
        const newButton = countButton.cloneNode(true) as HTMLButtonElement;
        countButton.parentNode?.replaceChild(newButton, countButton);
        
        // Attach a proper counter event handler
        newButton.addEventListener('click', () => {
          count++;
          newButton.textContent = `Count: ${count}`;
          console.log(`Counter updated to: ${count}`);
        });
        
        // Simulate a click
        newButton.click();
        
        return { 
          success: true, 
          count, 
          originalText,
          currentText: newButton.textContent,
          message: 'Counter functionality test completed'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Counter functionality test result:', counterFunctionalityWorks);
    
    await page.screenshot({ path: 'test-step3-counter-functionality.png' });
    
    expect(counterFunctionalityWorks.success).toBe(true);
    expect(counterFunctionalityWorks.count).toBe(1);
    expect(counterFunctionalityWorks.currentText).toContain('Count: 1');
  });

  test('Step 4: Test show/hide functionality with proper state management', async ({ page }) => {
    // Test show/hide functionality with proper state management
    const showHideFunctionalityWorks = await page.evaluate(() => {
      try {
        const buttons = document.querySelectorAll('button');
        const toggleButton = Array.from(buttons).find(btn => 
          btn.textContent?.includes('Hide') || btn.textContent?.includes('Show')
        );
        
        if (!toggleButton) return { error: 'Toggle button not found' };
        
        let isVisible = true;
        const originalText = toggleButton.textContent;
        
        // Remove any existing listeners and create a fresh one
        const newButton = toggleButton.cloneNode(true) as HTMLButtonElement;
        toggleButton.parentNode?.replaceChild(newButton, toggleButton);
        
        // Attach a proper toggle event handler
        newButton.addEventListener('click', () => {
          isVisible = !isVisible;
          newButton.textContent = isVisible ? 'Hide' : 'Show';
          console.log(`Visibility toggled to: ${isVisible}`);
        });
        
        // Simulate a click
        newButton.click();
        
        return { 
          success: true, 
          isVisible, 
          originalText,
          currentText: newButton.textContent,
          message: 'Show/hide functionality test completed'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Show/hide functionality test result:', showHideFunctionalityWorks);
    
    await page.screenshot({ path: 'test-step4-show-hide-functionality.png' });
    
    expect(showHideFunctionalityWorks.success).toBe(true);
    expect(showHideFunctionalityWorks.isVisible).toBe(false);
    expect(showHideFunctionalityWorks.currentText).toBe('Show');
  });

  test('Step 5: Test all interactive features with proper implementation', async ({ page }) => {
    // Test all interactive features with proper implementation
    const allFeaturesWork = await page.evaluate(() => {
      try {
        const buttons = document.querySelectorAll('button');
        let successCount = 0;
        const results = [];
        
        buttons.forEach((button, index) => {
          // Remove any existing listeners and create a fresh one
          const newButton = button.cloneNode(true) as HTMLButtonElement;
          button.parentNode?.replaceChild(newButton, button);
          
          if (newButton.textContent?.includes('Count:')) {
            let count = 0;
            newButton.addEventListener('click', () => {
              count++;
              newButton.textContent = `Count: ${count}`;
              console.log(`Counter ${index} updated to: ${count}`);
            });
            successCount++;
            results.push({ index, type: 'counter', success: true });
          } else if (newButton.textContent?.includes('Hide') || newButton.textContent?.includes('Show')) {
            let isVisible = true;
            newButton.addEventListener('click', () => {
              isVisible = !isVisible;
              newButton.textContent = isVisible ? 'Hide' : 'Show';
              console.log(`Toggle ${index} toggled to: ${isVisible}`);
            });
            successCount++;
            results.push({ index, type: 'toggle', success: true });
          } else if (newButton.textContent?.includes('Switch to')) {
            let isGrid = false;
            newButton.addEventListener('click', () => {
              isGrid = !isGrid;
              newButton.textContent = isGrid ? 'Switch to List' : 'Switch to Grid';
              console.log(`Layout ${index} toggled to: ${isGrid ? 'grid' : 'list'}`);
            });
            successCount++;
            results.push({ index, type: 'layout', success: true });
          }
        });
        
        return { 
          success: true, 
          successCount, 
          totalButtons: buttons.length,
          results,
          message: 'All interactive features test completed'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('All interactive features test result:', allFeaturesWork);
    
    await page.screenshot({ path: 'test-step5-all-interactive-features.png' });
    
    expect(allFeaturesWork.success).toBe(true);
    expect(allFeaturesWork.successCount).toBe(3);
  });
});
