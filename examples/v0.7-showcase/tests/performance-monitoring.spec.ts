import { test, expect } from '@playwright/test';

test.describe('Performance Monitoring Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8080');
    await page.waitForLoadState('networkidle');
  });

  test('should monitor page load performance', async ({ page }) => {
    // Measure page load metrics with timeout
    const performanceMetrics = await page.evaluate(
      () => {
        const navigation = performance.getEntriesByType(
          'navigation'
        )[0] as PerformanceNavigationTiming;
        return {
          domContentLoaded:
            navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
          loadComplete: navigation.loadEventEnd - navigation.loadEventStart,
          totalLoadTime: navigation.loadEventEnd - navigation.fetchStart,
          firstContentfulPaint:
            performance.getEntriesByName('first-contentful-paint')[0]?.startTime || 0,
          largestContentfulPaint:
            performance.getEntriesByName('largest-contentful-paint')[0]?.startTime || 0,
        };
      },
      { timeout: 10000 }
    );

    // Log metrics for monitoring
    console.log('Performance Metrics:', performanceMetrics);

    // Set performance thresholds
    expect(performanceMetrics.domContentLoaded).toBeLessThan(2000); // 2 seconds
    expect(performanceMetrics.loadComplete).toBeLessThan(3000); // 3 seconds
    expect(performanceMetrics.totalLoadTime).toBeLessThan(5000); // 5 seconds
  });

  test('should monitor memory usage', async ({ page }) => {
    // Measure initial memory usage
    const initialMemory = await page.evaluate(() => {
      return (performance as any).memory
        ? {
            usedJSHeapSize: (performance as any).memory.usedJSHeapSize,
            totalJSHeapSize: (performance as any).memory.totalJSHeapSize,
            jsHeapSizeLimit: (performance as any).memory.jsHeapSizeLimit,
          }
        : null;
    });

    if (initialMemory) {
      console.log('Initial Memory Usage:', initialMemory);

      // Perform some interactions to test memory growth
      const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();
      if (await toggleButton.isVisible()) {
        for (let i = 0; i < 10; i++) {
          await toggleButton.click();
          await page.waitForTimeout(100);
        }
      }

      // Measure memory after interactions
      const finalMemory = await page.evaluate(() => {
        return (performance as any).memory
          ? {
              usedJSHeapSize: (performance as any).memory.usedJSHeapSize,
              totalJSHeapSize: (performance as any).memory.totalJSHeapSize,
              jsHeapSizeLimit: (performance as any).memory.jsHeapSizeLimit,
            }
          : null;
      });

      if (finalMemory) {
        console.log('Final Memory Usage:', finalMemory);

        // Check for memory leaks (memory shouldn't grow excessively)
        const memoryGrowth = finalMemory.usedJSHeapSize - initialMemory.usedJSHeapSize;
        const memoryGrowthPercent = (memoryGrowth / initialMemory.usedJSHeapSize) * 100;

        console.log(`Memory Growth: ${memoryGrowth} bytes (${memoryGrowthPercent.toFixed(2)}%)`);

        // Memory growth should be reasonable (less than 50% increase)
        expect(memoryGrowthPercent).toBeLessThan(50);
      }
    }
  });

  test('should monitor animation performance', async ({ page }) => {
    const motionDiv = page.locator('[data-motion]').first();
    if (await motionDiv.isVisible()) {
      const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();

      // Measure animation performance
      const animationMetrics = await page.evaluate(() => {
        const startTime = performance.now();
        return { startTime };
      });

      await toggleButton.click();
      await page.waitForTimeout(2000); // Wait for animation

      const animationPerformance = await page.evaluate(startTime => {
        const endTime = performance.now();
        const animationDuration = endTime - startTime;

        // Get frame rate information
        const frameCount = performance.getEntriesByType('measure').length;

        return {
          animationDuration,
          frameCount,
          averageFrameTime: animationDuration / frameCount,
          fps: frameCount / (animationDuration / 1000),
        };
      }, animationMetrics.startTime);

      console.log('Animation Performance:', animationPerformance);

      // Performance thresholds
      expect(animationPerformance.animationDuration).toBeLessThan(3000); // 3 seconds max
      expect(animationPerformance.fps).toBeGreaterThan(30); // At least 30 FPS
      expect(animationPerformance.averageFrameTime).toBeLessThan(33); // 33ms per frame max
    }
  });

  test('should monitor response times', async ({ page }) => {
    const responseTimes = [];

    // Test multiple interactions
    for (let i = 0; i < 5; i++) {
      const startTime = Date.now();

      // Test button click response
      const incrementButton = page.locator(
        '.showcase-card:has-text("Simple Test") button:has-text("Increment Counter")'
      );
      if (await incrementButton.isVisible()) {
        await incrementButton.click();
        await page.waitForTimeout(100);

        const responseTime = Date.now() - startTime;
        responseTimes.push(responseTime);

        console.log(`Interaction ${i + 1} response time: ${responseTime}ms`);
      }
    }

    // Calculate statistics
    const avgResponseTime = responseTimes.reduce((a, b) => a + b, 0) / responseTimes.length;
    const maxResponseTime = Math.max(...responseTimes);
    const minResponseTime = Math.min(...responseTimes);

    console.log(
      `Response Time Stats: Avg: ${avgResponseTime.toFixed(2)}ms, Max: ${maxResponseTime}ms, Min: ${minResponseTime}ms`
    );

    // Performance thresholds
    expect(avgResponseTime).toBeLessThan(500); // Average response time under 500ms
    expect(maxResponseTime).toBeLessThan(1000); // Max response time under 1 second
  });

  test('should monitor CPU usage patterns', async ({ page }) => {
    // Monitor CPU usage during interactions
    const cpuMetrics = await page.evaluate(() => {
      const startTime = performance.now();
      const startMemory = (performance as any).memory?.usedJSHeapSize || 0;

      return { startTime, startMemory };
    });

    // Perform intensive operations
    const toggleButton = page.locator('.showcase-card:has([data-motion]) button').first();
    if (await toggleButton.isVisible()) {
      for (let i = 0; i < 20; i++) {
        await toggleButton.click();
        await page.waitForTimeout(50);
      }
    }

    const finalMetrics = await page.evaluate(startData => {
      const endTime = performance.now();
      const endMemory = (performance as any).memory?.usedJSHeapSize || 0;

      return {
        totalTime: endTime - startData.startTime,
        memoryDelta: endMemory - startData.startMemory,
        operationsPerSecond: 20 / ((endTime - startData.startTime) / 1000),
      };
    }, cpuMetrics);

    console.log('CPU Usage Metrics:', finalMetrics);

    // Performance thresholds
    expect(finalMetrics.totalTime).toBeLessThan(5000); // Should complete within 5 seconds
    expect(finalMetrics.operationsPerSecond).toBeGreaterThan(2); // At least 2 operations per second
  });

  test('should monitor network performance', async ({ page }) => {
    // Monitor network requests
    const networkMetrics = await page.evaluate(() => {
      const resources = performance.getEntriesByType('resource');
      const wasmResources = resources.filter(r => r.name.includes('.wasm'));
      const jsResources = resources.filter(r => r.name.includes('.js'));

      return {
        totalResources: resources.length,
        wasmResources: wasmResources.length,
        jsResources: jsResources.length,
        totalTransferSize: resources.reduce((sum, r) => sum + (r as any).transferSize, 0),
        wasmSize: wasmResources.reduce((sum, r) => sum + (r as any).transferSize, 0),
        jsSize: jsResources.reduce((sum, r) => sum + (r as any).transferSize, 0),
      };
    });

    console.log('Network Metrics:', networkMetrics);

    // Performance thresholds
    expect(networkMetrics.totalTransferSize).toBeLessThan(10 * 1024 * 1024); // Less than 10MB
    expect(networkMetrics.wasmSize).toBeLessThan(5 * 1024 * 1024); // Less than 5MB for WASM
  });

  test('should generate performance report', async ({ page }) => {
    // Collect all performance metrics
    const performanceReport = await page.evaluate(() => {
      const navigation = performance.getEntriesByType(
        'navigation'
      )[0] as PerformanceNavigationTiming;
      const memory = (performance as any).memory;
      const resources = performance.getEntriesByType('resource');

      return {
        timestamp: new Date().toISOString(),
        pageLoad: {
          domContentLoaded:
            navigation.domContentLoadedEventEnd - navigation.domContentLoadedEventStart,
          loadComplete: navigation.loadEventEnd - navigation.loadEventStart,
          totalLoadTime: navigation.loadEventEnd - navigation.fetchStart,
        },
        memory: memory
          ? {
              usedJSHeapSize: memory.usedJSHeapSize,
              totalJSHeapSize: memory.totalJSHeapSize,
              jsHeapSizeLimit: memory.jsHeapSizeLimit,
            }
          : null,
        network: {
          totalResources: resources.length,
          totalSize: resources.reduce((sum, r) => sum + (r as any).transferSize, 0),
        },
        userAgent: navigator.userAgent,
      };
    });

    // Log comprehensive report
    console.log('=== PERFORMANCE REPORT ===');
    console.log(JSON.stringify(performanceReport, null, 2));
    console.log('==========================');

    // Store report for historical tracking
    expect(performanceReport.timestamp).toBeDefined();
    expect(performanceReport.pageLoad.totalLoadTime).toBeLessThan(5000);
  });
});
