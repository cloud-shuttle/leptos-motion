import { test, expect } from '@playwright/test';

test.describe('Leptos DOM Reconciliation - TDD Approach', () => {
  test.beforeEach(async ({ page }) => {
    page.on('console', msg => console.log('PAGE LOG:', msg.text()));
    page.on('pageerror', error => console.log('PAGE ERROR:', error.message));
    
    await page.goto('/');
    await page.waitForTimeout(3000);
  });

  test('Step 1: Check if Leptos is actively reconciling the DOM', async ({ page }) => {
    // Check if Leptos is actively watching and reconciling DOM changes
    const leptosReconciliationActive = await page.evaluate(() => {
      try {
        // Try to detect if Leptos is actively reconciling
        const body = document.body;
        const hasLeptosAttributes = body.hasAttribute('data-leptos') || 
                                  body.hasAttribute('data-leptos-version');
        
        // Check for any hidden Leptos reconciliation data
        const hiddenElements = document.querySelectorAll('[style*="display: none"], [hidden]');
        const hasLeptosReconciliationData = Array.from(hiddenElements).some(el => 
          el.textContent?.includes('leptos') || 
          el.getAttribute('data-leptos') ||
          el.className?.includes('leptos')
        );
        
        // Check if there are any MutationObserver-like mechanisms
        const hasMutationObservers = typeof (window as any).__LEPTOS_MUTATION_OBSERVER !== 'undefined';
        
        return { 
          hasLeptosAttributes, 
          hasLeptosReconciliationData, 
          hasMutationObservers,
          hiddenCount: hiddenElements.length
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Leptos reconciliation check:', leptosReconciliationActive);
    
    await page.screenshot({ path: 'test-step1-leptos-reconciliation.png' });
    
    expect(leptosReconciliationActive).toBeDefined();
  });

  test('Step 2: Try to disable Leptos reconciliation temporarily', async ({ page }) => {
    // Try to temporarily disable any Leptos reconciliation mechanisms
    const reconciliationDisabled = await page.evaluate(() => {
      try {
        // Look for any global Leptos reconciliation functions
        const hasLeptosReconcile = typeof (window as any).leptos_reconcile !== 'undefined';
        const hasLeptosUpdate = typeof (window as any).leptos_update !== 'undefined';
        
        // Try to override or disable them
        if (hasLeptosReconcile) {
          (window as any).leptos_reconcile = () => console.log('Leptos reconciliation disabled');
        }
        if (hasLeptosUpdate) {
          (window as any).leptos_update = () => console.log('Leptos update disabled');
        }
        
        // Check if we can find any other Leptos global functions
        const leptosGlobals = Object.keys(window).filter(key => 
          key.toLowerCase().includes('leptos')
        );
        
        return { 
          hasLeptosReconcile, 
          hasLeptosUpdate, 
          leptosGlobals,
          disabled: hasLeptosReconcile || hasLeptosUpdate
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Leptos reconciliation disable attempt:', reconciliationDisabled);
    
    await page.screenshot({ path: 'test-step2-disable-reconciliation.png' });
    
    expect(reconciliationDisabled).toBeDefined();
  });

  test('Step 3: Test DOM manipulation with reconciliation disabled', async ({ page }) => {
    // Try to manipulate DOM after attempting to disable reconciliation
    const domManipulationWorks = await page.evaluate(() => {
      try {
        // First try to disable any Leptos mechanisms
        if (typeof (window as any).leptos_reconcile !== 'undefined') {
          (window as any).leptos_reconcile = () => console.log('Reconciliation disabled');
        }
        
        // Now try to manually change button text
        const buttons = document.querySelectorAll('button');
        let successCount = 0;
        
        buttons.forEach((button, index) => {
          const originalText = button.textContent;
          
          // Try to change the text
          button.textContent = `Modified ${index}: ${originalText}`;
          
          // Check if it actually changed
          if (button.textContent !== originalText) {
            successCount++;
            console.log(`Button ${index} text changed successfully`);
          } else {
            console.log(`Button ${index} text change failed`);
          }
        });
        
        return { successCount, totalButtons: buttons.length };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('DOM manipulation with reconciliation disabled:', domManipulationWorks);
    
    await page.screenshot({ path: 'test-step3-dom-manipulation-test.png' });
    
    expect(domManipulationWorks.successCount).toBeGreaterThan(0);
  });

  test('Step 4: Check if Leptos is using a different reconciliation strategy', async ({ page }) => {
    // Look for alternative Leptos reconciliation strategies
    const alternativeStrategies = await page.evaluate(() => {
      try {
        // Check for different types of Leptos mechanisms
        const hasWebComponents = customElements.get('leptos-app') !== undefined;
        const hasShadowDOM = document.querySelector('*').shadowRoot !== null;
        const hasCustomElements = document.querySelectorAll('*').length > 0 && 
                                 Array.from(document.querySelectorAll('*')).some(el => 
                                   el.tagName.toLowerCase().includes('leptos')
                                 );
        
        // Check for any other reactive frameworks
        const hasReact = typeof (window as any).React !== 'undefined';
        const hasVue = typeof (window as any).Vue !== 'undefined';
        const hasSvelte = typeof (window as any).Svelte !== 'undefined';
        
        // Look for any hidden reconciliation data
        const allElements = document.querySelectorAll('*');
        const hasReconciliationData = Array.from(allElements).some(el => 
          el.getAttribute('data-reconcile') ||
          el.getAttribute('data-reactive') ||
          el.className?.includes('reconcile') ||
          el.className?.includes('reactive')
        );
        
        return { 
          hasWebComponents, 
          hasShadowDOM, 
          hasCustomElements,
          hasReact,
          hasVue,
          hasSvelte,
          hasReconciliationData,
          totalElements: allElements.length
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Alternative reconciliation strategies:', alternativeStrategies);
    
    await page.screenshot({ path: 'test-step4-alternative-strategies.png' });
    
    expect(alternativeStrategies).toBeDefined();
  });

  test('Step 5: Try to understand the exact reconciliation mechanism', async ({ page }) => {
    // Try to understand exactly how Leptos is reconciling
    const reconciliationMechanism = await page.evaluate(() => {
      try {
        // Monitor DOM changes to see what's happening
        let changeCount = 0;
        const observer = new MutationObserver((mutations) => {
          mutations.forEach((mutation) => {
            if (mutation.type === 'childList' || mutation.type === 'characterData') {
              changeCount++;
              console.log(`DOM change detected: ${mutation.type}`, mutation);
            }
          });
        });
        
        // Start observing
        observer.observe(document.body, { 
          childList: true, 
          subtree: true, 
          characterData: true 
        });
        
        // Try to change button text
        const button = document.querySelector('button');
        if (button) {
          const originalText = button.textContent;
          button.textContent = 'TESTING RECONCILIATION';
          
          // Wait a bit to see if it gets reverted
          setTimeout(() => {
            const currentText = button.textContent;
            console.log(`Button text after change: "${currentText}", Original: "${originalText}"`);
            console.log(`Total DOM changes detected: ${changeCount}`);
          }, 100);
        }
        
        return { 
          observerStarted: true, 
          changeCount: 0,
          message: 'Monitoring DOM changes for reconciliation'
        };
      } catch (e) {
        return { error: e.message };
      }
    });
    
    console.log('Reconciliation mechanism investigation:', reconciliationMechanism);
    
    // Wait a bit to see the results
    await page.waitForTimeout(2000);
    
    await page.screenshot({ path: 'test-step5-reconciliation-mechanism.png' });
    
    expect(reconciliationMechanism).toBeDefined();
  });
});
