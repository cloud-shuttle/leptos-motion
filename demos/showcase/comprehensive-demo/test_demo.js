const { test, expect } = require('@playwright/test');

test('Leptos Motion Demo Test', async ({ page }) => {
  // Navigate to the demo
  await page.goto('http://localhost:8080/dist/index.html');
  
  // Wait for the page to load
  await page.waitForLoadState('networkidle');
  
  // Check if the page title contains expected content
  const title = await page.title();
  console.log('Page title:', title);
  
  // Check if the main heading is present
  const heading = await page.locator('h1').first();
  await expect(heading).toBeVisible();
  
  const headingText = await heading.textContent();
  console.log('Main heading:', headingText);
  
  // Check if there's a button to toggle animation
  const button = page.locator('button').first();
  await expect(button).toBeVisible();
  
  const buttonText = await button.textContent();
  console.log('Button text:', buttonText);
  
  // Check if there's an animated element
  const animatedElement = page.locator('[style*="transform"]').first();
  if (await animatedElement.count() > 0) {
    console.log('Found animated element');
    
    // Get initial transform
    const initialTransform = await animatedElement.evaluate(el => el.style.transform);
    console.log('Initial transform:', initialTransform);
    
    // Click the button to trigger animation
    await button.click();
    
    // Wait a bit for animation to start
    await page.waitForTimeout(100);
    
    // Check if transform changed
    const newTransform = await animatedElement.evaluate(el => el.style.transform);
    console.log('New transform:', newTransform);
    
    // The transform should have changed
    expect(newTransform).not.toBe(initialTransform);
  }
  
  // Check console for any errors
  const logs = [];
  page.on('console', msg => {
    if (msg.type() === 'error') {
      logs.push(msg.text());
    }
  });
  
  // Wait a bit more to catch any errors
  await page.waitForTimeout(1000);
  
  if (logs.length > 0) {
    console.log('Console errors:', logs);
  }
  
  // Take a screenshot for debugging
  await page.screenshot({ path: 'demo_screenshot.png' });
  
  console.log('Demo test completed successfully!');
});
