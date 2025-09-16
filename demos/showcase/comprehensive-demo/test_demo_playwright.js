const { chromium } = require('playwright');

async function testDemoWithPlaywright() {
  console.log('🎭 Starting Playwright test for Leptos Motion Demo...');
  
  const browser = await chromium.launch({ 
    headless: false, // Show browser for debugging
    slowMo: 1000 // Slow down actions for visibility
  });
  
  try {
    const page = await browser.newPage();
    
    // Set up console logging
    page.on('console', msg => {
      console.log(`📝 Console ${msg.type()}: ${msg.text()}`);
    });
    
    // Set up error handling
    page.on('pageerror', error => {
      console.log(`❌ Page error: ${error.message}`);
    });
    
    console.log('🌐 Navigating to demo...');
    await page.goto('http://localhost:8080/dist/index.html');
    
    console.log('⏳ Waiting for page to load...');
    await page.waitForLoadState('networkidle');
    
    // Check page title
    const title = await page.title();
    console.log(`📄 Page title: "${title}"`);
    
    // Check if main heading is present
    const heading = page.locator('h1').first();
    await heading.waitFor({ timeout: 5000 });
    const headingText = await heading.textContent();
    console.log(`📝 Main heading: "${headingText}"`);
    
    // Check if there's a button
    const buttons = page.locator('button');
    const buttonCount = await buttons.count();
    console.log(`🔘 Found ${buttonCount} button(s)`);
    
    if (buttonCount > 0) {
      const firstButton = buttons.first();
      const buttonText = await firstButton.textContent();
      console.log(`🔘 First button text: "${buttonText}"`);
      
      // Check if button is visible and clickable
      const isVisible = await firstButton.isVisible();
      const isEnabled = await firstButton.isEnabled();
      console.log(`🔘 Button visible: ${isVisible}, enabled: ${isEnabled}`);
      
      if (isVisible && isEnabled) {
        console.log('🖱️ Clicking button to test animation...');
        await firstButton.click();
        
        // Wait a bit for animation to start
        await page.waitForTimeout(1000);
        
        // Check if any elements have transform styles
        const elementsWithTransform = page.locator('[style*="transform"]');
        const transformCount = await elementsWithTransform.count();
        console.log(`🎨 Found ${transformCount} element(s) with transform styles`);
        
        if (transformCount > 0) {
          const firstTransform = elementsWithTransform.first();
          const transformValue = await firstTransform.evaluate(el => el.style.transform);
          console.log(`🎨 Transform value: "${transformValue}"`);
        }
      }
    }
    
    // Check for any animated elements
    const animatedElements = page.locator('[style*="transition"], [style*="animation"]');
    const animatedCount = await animatedElements.count();
    console.log(`🎬 Found ${animatedCount} element(s) with animation styles`);
    
    // Check for WASM-related elements or scripts
    const scripts = page.locator('script[src*="comprehensive-demo"]');
    const scriptCount = await scripts.count();
    console.log(`📜 Found ${scriptCount} WASM script(s)`);
    
    // Take a screenshot
    await page.screenshot({ path: 'demo_playwright_screenshot.png' });
    console.log('📸 Screenshot saved as demo_playwright_screenshot.png');
    
    // Check for any error messages
    const errorElements = page.locator('.error, [class*="error"]');
    const errorCount = await errorElements.count();
    if (errorCount > 0) {
      console.log(`❌ Found ${errorCount} error element(s)`);
      for (let i = 0; i < errorCount; i++) {
        const errorText = await errorElements.nth(i).textContent();
        console.log(`❌ Error ${i + 1}: "${errorText}"`);
      }
    }
    
    console.log('✅ Playwright test completed successfully!');
    
  } catch (error) {
    console.log(`❌ Playwright test failed: ${error.message}`);
  } finally {
    await browser.close();
  }
}

// Run the test
testDemoWithPlaywright().catch(console.error);
