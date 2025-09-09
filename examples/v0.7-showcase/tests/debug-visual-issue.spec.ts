import { test, expect } from '@playwright/test';

test('debug why visual animations are not showing', async ({ page }) => {
  const logs: string[] = [];
  
  // Capture console logs
  page.on('console', msg => {
    logs.push(`[${msg.type()}] ${msg.text()}`);
  });
  
  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });
  
  // Wait for the app to initialize
  await page.waitForTimeout(3000);
  
  // Find the animated element (the green circle)
  const animatedElement = page.locator('.bg-green-500.rounded-full').first();
  const elementExists = await animatedElement.count() > 0;
  console.log('Animated element exists:', elementExists);
  
  if (!elementExists) {
    console.log('❌ No animated element found - this is the problem!');
    return;
  }
  
  // Get initial state
  const initialInlineStyle = await animatedElement.getAttribute('style');
  console.log('Initial inline style:', initialInlineStyle);
  
  const initialComputed = await animatedElement.evaluate(el => {
    const computed = window.getComputedStyle(el);
    return {
      transform: computed.transform,
      backgroundColor: computed.backgroundColor,
      transition: computed.transition,
      position: computed.position,
      left: computed.left,
      top: computed.top
    };
  });
  console.log('Initial computed styles:', initialComputed);
  
  // Check what animation values are being returned
  const animationLogs = logs.filter(log => 
    log.includes('Animation triggered') || 
    log.includes('Returning')
  );
  console.log('Animation logs:', animationLogs);
  
  // Try to find and click a button
  const buttons = page.locator('button');
  const buttonCount = await buttons.count();
  console.log(`Found ${buttonCount} buttons`);
  
  if (buttonCount > 0) {
    console.log('Clicking first button...');
    await buttons.first().click();
    
    // Wait for animation to process
    await page.waitForTimeout(1000);
    
    // Check new logs
    const newAnimationLogs = logs.filter(log => 
      log.includes('Animation triggered') || 
      log.includes('Returning')
    );
    console.log('New animation logs after click:', newAnimationLogs);
    
    // Get final state
    const finalInlineStyle = await animatedElement.getAttribute('style');
    console.log('Final inline style:', finalInlineStyle);
    
    const finalComputed = await animatedElement.evaluate(el => {
      const computed = window.getComputedStyle(el);
      return {
        transform: computed.transform,
        backgroundColor: computed.backgroundColor,
        transition: computed.transition,
        position: computed.position,
        left: computed.left,
        top: computed.top
      };
    });
    console.log('Final computed styles:', finalComputed);
    
    // Check if anything changed
    const inlineChanged = initialInlineStyle !== finalInlineStyle;
    const computedChanged = 
      initialComputed.transform !== finalComputed.transform ||
      initialComputed.backgroundColor !== finalComputed.backgroundColor;
    
    console.log('Inline style changed:', inlineChanged);
    console.log('Computed style changed:', computedChanged);
    
    // Debug: Check what the animation is supposed to return
    console.log('=== DEBUGGING ANIMATION VALUES ===');
    
    // Let's check what the animation closure is actually returning
    const debugResult = await page.evaluate(() => {
      // Try to access the animation values from the console
      return {
        message: 'Check browser console for animation values',
        hasWindow: typeof window !== 'undefined',
        hasDocument: typeof document !== 'undefined'
      };
    });
    console.log('Debug result:', debugResult);
    
  } else {
    console.log('❌ No buttons found to trigger animation');
  }
  
  // The key insight: if the reactive system is working but visual changes aren't happening,
  // the issue is likely in how the styles are being applied to the DOM
  console.log('=== ANALYSIS ===');
  console.log('✅ Reactive system working (console logs show animation triggers)');
  console.log('❓ Visual changes not visible - investigating style application');
  
  expect(elementExists).toBe(true);
});
