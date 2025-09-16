import { test, expect } from '@playwright/test';

test('confirm reactive animation fix is working', async ({ page }) => {
  const logs: string[] = [];

  // Capture console logs
  page.on('console', msg => {
    logs.push(`[${msg.type()}] ${msg.text()}`);
  });

  // Navigate to the demo
  await page.goto('http://localhost:8080', { waitUntil: 'domcontentloaded' });

  // Wait for the app to initialize
  await page.waitForTimeout(3000);

  // Look for animation logs
  const animationLogs = logs.filter(
    log => log.includes('Animation triggered') || log.includes('Returning')
  );

  console.log('=== All Animation Logs ===');
  animationLogs.forEach(log => console.log(log));

  // Check if we have the expected logs that prove our fix is working
  const hasAnimationTriggered = animationLogs.some(log => log.includes('Animation triggered'));

  const hasReturningAnimation = animationLogs.some(log => log.includes('Returning'));

  const hasIdleAnimation = animationLogs.some(log => log.includes('idle animation'));

  console.log('Has "Animation triggered" log:', hasAnimationTriggered);
  console.log('Has "Returning" log:', hasReturningAnimation);
  console.log('Has "idle animation" log:', hasIdleAnimation);

  // The fix is working if we have these logs
  expect(hasAnimationTriggered).toBe(true);
  expect(hasReturningAnimation).toBe(true);
  expect(hasIdleAnimation).toBe(true);

  console.log('✅ REACTIVE ANIMATION FIX CONFIRMED: The animation system is working!');
});
