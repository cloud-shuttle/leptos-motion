import { test, expect, Page, Locator } from '@playwright/test';

// E2E Test Configuration
const E2E_CONFIG = {
  timeout: 10000,
  animationWait: 500,
  viewport: { width: 1280, height: 720 },
  performanceThreshold: 16, // 60fps threshold
};

// Test Helper Functions
class E2ETestHelper {
  constructor(private page: Page) {}

  async waitForAnimation(
    element: Locator,
    property: string,
    expectedValue: string,
    timeout = E2E_CONFIG.animationWait
  ) {
    const startTime = Date.now();

    while (Date.now() - startTime < timeout) {
      const currentValue = await element.evaluate(el =>
        window.getComputedStyle(el).getPropertyValue(property)
      );

      if (currentValue.includes(expectedValue)) {
        return true;
      }

      await this.page.waitForTimeout(10);
    }

    throw new Error(
      `Animation for property '${property}' did not complete to '${expectedValue}' within ${timeout}ms`
    );
  }

  async assertAnimationCompleted(element: Locator, property: string, expectedValue: string) {
    await this.waitForAnimation(element, property, expectedValue);
  }

  async getComputedStyle(element: Locator, property: string): Promise<string> {
    return await element.evaluate(el => window.getComputedStyle(el).getPropertyValue(property));
  }

  async simulateGesture(
    element: Locator,
    gestureType: 'swipe-left' | 'swipe-right' | 'pinch' | 'rotate'
  ) {
    const boundingBox = await element.boundingBox();
    if (!boundingBox) throw new Error('Element not found');

    const centerX = boundingBox.x + boundingBox.width / 2;
    const centerY = boundingBox.y + boundingBox.height / 2;

    switch (gestureType) {
      case 'swipe-left':
        await this.page.mouse.move(centerX, centerY);
        await this.page.mouse.down();
        await this.page.mouse.move(centerX - 100, centerY);
        await this.page.mouse.up();
        break;
      case 'swipe-right':
        await this.page.mouse.move(centerX, centerY);
        await this.page.mouse.down();
        await this.page.mouse.move(centerX + 100, centerY);
        await this.page.mouse.up();
        break;
      case 'pinch':
        // Simulate pinch with two touch points
        await this.page.touchscreen.tap(centerX - 20, centerY);
        await this.page.touchscreen.tap(centerX + 20, centerY);
        break;
      case 'rotate':
        // Simulate rotation gesture
        await this.page.mouse.move(centerX, centerY);
        await this.page.mouse.down();
        await this.page.mouse.move(centerX + 50, centerY - 50);
        await this.page.mouse.move(centerX + 100, centerY);
        await this.page.mouse.up();
        break;
    }
  }

  async measurePerformance(
    callback: () => Promise<void>
  ): Promise<{ fps: number; memoryUsage: number }> {
    const startTime = performance.now();
    let frameCount = 0;
    let memoryUsage = 0;

    // Start performance monitoring
    const performanceObserver = new PerformanceObserver(list => {
      for (const entry of list.getEntries()) {
        if (entry.entryType === 'measure') {
          frameCount++;
        }
      }
    });
    performanceObserver.observe({ entryTypes: ['measure'] });

    // Monitor memory usage
    const memoryObserver = setInterval(() => {
      if ('memory' in performance) {
        memoryUsage = (performance as any).memory.usedJSHeapSize / 1024 / 1024; // MB
      }
    }, 100);

    await callback();

    const endTime = performance.now();
    const duration = endTime - startTime;
    const fps = frameCount / (duration / 1000);

    performanceObserver.disconnect();
    clearInterval(memoryObserver);

    return { fps, memoryUsage };
  }
}

test.describe('Complete E2E Workflows', () => {
  let helper: E2ETestHelper;

  test.beforeEach(async ({ page }) => {
    await page.setViewportSize(E2E_CONFIG.viewport);
    await page.goto('/test-app.html');
    await page.waitForSelector('h1', { timeout: E2E_CONFIG.timeout });

    helper = new E2ETestHelper(page);
  });

  test('E-commerce Product Interaction Workflow', async ({ page }) => {
    // Create product grid
    await page.evaluate(() => {
      const container = document.createElement('div');
      container.id = 'product-grid';
      container.style.cssText =
        'display: grid; grid-template-columns: repeat(3, 1fr); gap: 20px; padding: 20px;';

      for (let i = 0; i < 6; i++) {
        const product = document.createElement('div');
        product.id = `product-${i}`;
        product.style.cssText =
          'background: white; border-radius: 8px; padding: 16px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); cursor: pointer;';

        const image = document.createElement('div');
        image.style.cssText =
          'width: 100%; height: 200px; background: #e0e0e0; border-radius: 4px; margin-bottom: 12px;';
        product.appendChild(image);

        const title = document.createElement('h3');
        title.textContent = `Product ${i + 1}`;
        title.style.cssText = 'margin: 0 0 8px 0; font-size: 16px;';
        product.appendChild(title);

        const price = document.createElement('p');
        price.textContent = `$${(i + 1) * 25}`;
        price.style.cssText = 'margin: 0; font-weight: bold; color: #2c3e50;';
        product.appendChild(price);

        container.appendChild(product);
      }

      document.body.appendChild(container);
    });

    // Test product hover workflow
    for (let i = 0; i < 6; i++) {
      const product = page.locator(`#product-${i}`);

      // Get initial transform
      const initialTransform = await helper.getComputedStyle(product, 'transform');

      // Hover over product
      await product.hover();
      await helper.assertAnimationCompleted(product, 'transform', 'scale(1.05)');

      // Click product
      await product.click();
      await helper.assertAnimationCompleted(product, 'transform', 'scale(0.98)');

      // Unhover
      await product.hover({ position: { x: -10, y: -10 } }); // Move away
      await helper.assertAnimationCompleted(product, 'transform', 'scale(1)');
    }
  });

  test('Form Validation and Submission Workflow', async ({ page }) => {
    // Create form
    await page.evaluate(() => {
      const form = document.createElement('form');
      form.id = 'contact-form';
      form.style.cssText =
        'max-width: 500px; margin: 0 auto; padding: 20px; background: white; border-radius: 8px;';

      const fields = [
        { id: 'name', type: 'text', label: 'Full Name' },
        { id: 'email', type: 'email', label: 'Email Address' },
        { id: 'phone', type: 'tel', label: 'Phone Number' },
        { id: 'message', type: 'textarea', label: 'Message' },
      ];

      fields.forEach(field => {
        const fieldContainer = document.createElement('div');
        fieldContainer.style.cssText = 'margin-bottom: 20px;';

        const label = document.createElement('label');
        label.textContent = field.label;
        label.style.cssText = 'display: block; margin-bottom: 5px; font-weight: bold;';
        fieldContainer.appendChild(label);

        const input = document.createElement(field.type === 'textarea' ? 'textarea' : 'input');
        input.id = field.id;
        input.style.cssText =
          'width: 100%; padding: 10px; border: 2px solid #ddd; border-radius: 4px; font-size: 14px;';
        if (field.type !== 'textarea') {
          input.setAttribute('type', field.type);
        }
        fieldContainer.appendChild(input);

        const error = document.createElement('div');
        error.id = `${field.id}-error`;
        error.textContent = 'This field is required';
        error.style.cssText = 'color: red; font-size: 12px; margin-top: 5px; display: none;';
        fieldContainer.appendChild(error);

        form.appendChild(fieldContainer);
      });

      const submitButton = document.createElement('button');
      submitButton.id = 'submit-btn';
      submitButton.textContent = 'Submit Form';
      submitButton.type = 'submit';
      submitButton.style.cssText =
        'width: 100%; padding: 12px; background: #3498db; color: white; border: none; border-radius: 4px; font-size: 16px; cursor: pointer;';
      form.appendChild(submitButton);

      document.body.appendChild(form);
    });

    // Test form validation workflow
    const submitButton = page.locator('#submit-btn');

    // Submit empty form
    await submitButton.click();

    // Check validation errors
    const nameError = page.locator('#name-error');
    const emailError = page.locator('#email-error');
    const phoneError = page.locator('#phone-error');
    const messageError = page.locator('#message-error');

    await expect(nameError).toBeVisible();
    await expect(emailError).toBeVisible();
    await expect(phoneError).toBeVisible();
    await expect(messageError).toBeVisible();

    // Fill out form fields
    await page.fill('#name', 'John Doe');
    await page.fill('#email', 'john@example.com');
    await page.fill('#phone', '555-1234');
    await page.fill('#message', 'This is a test message');

    // Test focus animations
    const nameInput = page.locator('#name');
    await nameInput.focus();
    await helper.assertAnimationCompleted(nameInput, 'border-color', 'rgb(52, 152, 219)');

    // Submit valid form
    await submitButton.click();

    // Form should submit successfully (no validation errors)
    await expect(nameError).toBeHidden();
    await expect(emailError).toBeHidden();
    await expect(phoneError).toBeHidden();
    await expect(messageError).toBeHidden();
  });

  test('Image Gallery with Lightbox Workflow', async ({ page }) => {
    // Create image gallery
    await page.evaluate(() => {
      const gallery = document.createElement('div');
      gallery.id = 'image-gallery';
      gallery.style.cssText =
        'display: grid; grid-template-columns: repeat(4, 1fr); gap: 10px; padding: 20px;';

      for (let i = 0; i < 12; i++) {
        const thumbnail = document.createElement('div');
        thumbnail.id = `thumb-${i}`;
        thumbnail.style.cssText =
          'width: 100%; height: 150px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px; cursor: pointer; position: relative; overflow: hidden;';

        const overlay = document.createElement('div');
        overlay.style.cssText =
          'position: absolute; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.3); display: flex; align-items: center; justify-content: center; color: white; font-weight: bold;';
        overlay.textContent = `Image ${i + 1}`;
        thumbnail.appendChild(overlay);

        gallery.appendChild(thumbnail);
      }

      // Create lightbox
      const lightbox = document.createElement('div');
      lightbox.id = 'lightbox';
      lightbox.style.cssText =
        'position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.9); display: none; align-items: center; justify-content: center; z-index: 1000;';

      const lightboxContent = document.createElement('div');
      lightboxContent.style.cssText = 'position: relative; max-width: 90%; max-height: 90%;';

      const lightboxImage = document.createElement('div');
      lightboxImage.id = 'lightbox-image';
      lightboxImage.style.cssText =
        'width: 600px; height: 400px; background: linear-gradient(45deg, #ff6b6b, #4ecdc4); border-radius: 8px;';

      const closeButton = document.createElement('button');
      closeButton.id = 'close-lightbox';
      closeButton.textContent = '×';
      closeButton.style.cssText =
        'position: absolute; top: -40px; right: 0; background: none; border: none; color: white; font-size: 30px; cursor: pointer;';

      lightboxContent.appendChild(lightboxImage);
      lightboxContent.appendChild(closeButton);
      lightbox.appendChild(lightboxContent);

      document.body.appendChild(gallery);
      document.body.appendChild(lightbox);
    });

    // Test gallery workflow
    for (let i = 0; i < 3; i++) {
      const thumbnail = page.locator(`#thumb-${i}`);
      const lightbox = page.locator('#lightbox');
      const closeButton = page.locator('#close-lightbox');

      // Hover over thumbnail
      await thumbnail.hover();
      await helper.assertAnimationCompleted(thumbnail, 'transform', 'scale(1.1)');

      // Click thumbnail to open lightbox
      await thumbnail.click();

      // Show lightbox
      await page.evaluate(() => {
        const lightbox = document.getElementById('lightbox');
        if (lightbox) {
          lightbox.style.display = 'flex';
        }
      });

      // Wait for lightbox animation
      await helper.assertAnimationCompleted(lightbox, 'opacity', '1');

      // Close lightbox
      await closeButton.click();

      // Hide lightbox
      await page.evaluate(() => {
        const lightbox = document.getElementById('lightbox');
        if (lightbox) {
          lightbox.style.display = 'none';
        }
      });

      // Unhover thumbnail
      await thumbnail.hover({ position: { x: -10, y: -10 } });
      await helper.assertAnimationCompleted(thumbnail, 'transform', 'scale(1)');
    }
  });

  test('Dashboard Navigation Workflow', async ({ page }) => {
    // Create dashboard
    await page.evaluate(() => {
      const dashboard = document.createElement('div');
      dashboard.id = 'dashboard';
      dashboard.style.cssText =
        'display: grid; grid-template-columns: 250px 1fr; grid-template-rows: 60px 1fr; gap: 0; height: 100vh;';

      // Header
      const header = document.createElement('div');
      header.id = 'dashboard-header';
      header.style.cssText =
        'grid-column: 1 / -1; background: #2c3e50; color: white; display: flex; align-items: center; padding: 0 20px;';

      const headerTitle = document.createElement('h1');
      headerTitle.textContent = 'Dashboard';
      headerTitle.style.cssText = 'margin: 0; font-size: 24px;';
      header.appendChild(headerTitle);

      // Sidebar
      const sidebar = document.createElement('div');
      sidebar.id = 'sidebar';
      sidebar.style.cssText = 'background: #34495e; color: white; padding: 20px;';

      const menuItems = ['Dashboard', 'Analytics', 'Users', 'Settings', 'Reports'];
      menuItems.forEach((item, i) => {
        const menuItem = document.createElement('div');
        menuItem.id = `menu-${i}`;
        menuItem.textContent = item;
        menuItem.style.cssText =
          'padding: 12px 16px; margin: 4px 0; border-radius: 4px; cursor: pointer; transition: all 0.2s ease;';
        sidebar.appendChild(menuItem);
      });

      // Main content
      const mainContent = document.createElement('div');
      mainContent.id = 'main-content';
      mainContent.style.cssText = 'background: #ecf0f1; padding: 20px; overflow-y: auto;';

      const contentSection = document.createElement('div');
      contentSection.id = 'content-section';
      contentSection.style.cssText =
        'background: white; padding: 20px; border-radius: 8px; margin-bottom: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);';

      const sectionTitle = document.createElement('h2');
      sectionTitle.textContent = 'Dashboard Overview';
      sectionTitle.style.cssText = 'margin: 0 0 10px 0; color: #2c3e50;';
      contentSection.appendChild(sectionTitle);

      const sectionDesc = document.createElement('p');
      sectionDesc.textContent = 'Welcome to your dashboard';
      sectionDesc.style.cssText = 'margin: 0; color: #7f8c8d;';
      contentSection.appendChild(sectionDesc);

      mainContent.appendChild(contentSection);

      dashboard.appendChild(header);
      dashboard.appendChild(sidebar);
      dashboard.appendChild(mainContent);

      document.body.appendChild(dashboard);
    });

    // Test sidebar navigation
    const menuItems = ['Dashboard', 'Analytics', 'Users', 'Settings', 'Reports'];

    for (let i = 0; i < menuItems.length; i++) {
      const menuItem = page.locator(`#menu-${i}`);

      // Hover over menu item
      await menuItem.hover();
      await helper.assertAnimationCompleted(menuItem, 'background-color', 'rgb(52, 152, 219)');

      // Click menu item
      await menuItem.click();
      await helper.assertAnimationCompleted(menuItem, 'transform', 'scale(0.95)');

      // Unhover
      await menuItem.hover({ position: { x: -10, y: -10 } });
      await helper.assertAnimationCompleted(menuItem, 'background-color', 'rgba(0, 0, 0, 0)');
    }
  });

  test('Mobile Gesture Workflow', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    // Create mobile app
    await page.evaluate(() => {
      const mobileContainer = document.createElement('div');
      mobileContainer.id = 'mobile-container';
      mobileContainer.style.cssText =
        'width: 375px; height: 667px; margin: 0 auto; background: #f8f9fa; border-radius: 20px; overflow: hidden; position: relative;';

      // Header
      const header = document.createElement('div');
      header.id = 'mobile-header';
      header.style.cssText =
        'height: 60px; background: #007AFF; color: white; display: flex; align-items: center; justify-content: center; font-weight: bold;';
      header.textContent = 'Mobile App';

      // Swipeable content
      const swipeContainer = document.createElement('div');
      swipeContainer.id = 'swipe-container';
      swipeContainer.style.cssText =
        'height: calc(100% - 60px); position: relative; overflow: hidden;';

      const pages = [
        { id: 'page-1', color: '#FF6B6B', title: 'Page 1' },
        { id: 'page-2', color: '#4ECDC4', title: 'Page 2' },
        { id: 'page-3', color: '#45B7D1', title: 'Page 3' },
        { id: 'page-4', color: '#96CEB4', title: 'Page 4' },
      ];

      pages.forEach((page, i) => {
        const pageElement = document.createElement('div');
        pageElement.id = page.id;
        pageElement.style.cssText = `position: absolute; top: 0; left: ${i * 100}%; width: 100%; height: 100%; background: ${page.color}; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px; font-weight: bold;`;
        pageElement.textContent = page.title;
        swipeContainer.appendChild(pageElement);
      });

      // Page indicators
      const indicators = document.createElement('div');
      indicators.id = 'page-indicators';
      indicators.style.cssText =
        'position: absolute; bottom: 20px; left: 50%; transform: translateX(-50%); display: flex; gap: 8px; z-index: 10;';

      pages.forEach((_, i) => {
        const indicator = document.createElement('div');
        indicator.id = `indicator-${i}`;
        indicator.style.cssText =
          'width: 8px; height: 8px; border-radius: 50%; background: rgba(255,255,255,0.5); cursor: pointer;';
        indicators.appendChild(indicator);
      });

      mobileContainer.appendChild(header);
      mobileContainer.appendChild(swipeContainer);
      mobileContainer.appendChild(indicators);

      document.body.appendChild(mobileContainer);
    });

    // Test page indicators
    for (let i = 0; i < 4; i++) {
      const indicator = page.locator(`#indicator-${i}`);

      // Tap indicator
      await indicator.tap();
      await helper.assertAnimationCompleted(indicator, 'transform', 'scale(1.2)');
    }

    // Test swipe gestures
    const swipeContainer = page.locator('#swipe-container');

    // Swipe left
    await helper.simulateGesture(swipeContainer, 'swipe-left');
    await page.waitForTimeout(300);

    // Swipe right
    await helper.simulateGesture(swipeContainer, 'swipe-right');
    await page.waitForTimeout(300);
  });

  test('Performance Under Load', async ({ page }) => {
    // Create many animated elements
    await page.evaluate(() => {
      const container = document.createElement('div');
      container.id = 'performance-container';
      container.style.cssText = 'display: grid; grid-template-columns: repeat(10, 1fr); gap: 10px;';

      for (let i = 0; i < 100; i++) {
        const element = document.createElement('div');
        element.id = `perf-${i}`;
        element.style.cssText =
          'width: 50px; height: 50px; background: purple; display: flex; align-items: center; justify-content: center; color: white;';
        element.textContent = i.toString();
        container.appendChild(element);
      }

      document.body.appendChild(container);
    });

    // Measure performance during rapid interactions
    const performance = await helper.measurePerformance(async () => {
      for (let i = 0; i < 10; i++) {
        const element = page.locator(`#perf-${i * 10}`);

        // Hover
        await element.hover();
        await page.waitForTimeout(50);

        // Click
        await element.click();
        await page.waitForTimeout(50);

        // Unhover
        await element.hover({ position: { x: -10, y: -10 } });
        await page.waitForTimeout(50);
      }
    });

    // Assert performance thresholds
    expect(performance.fps).toBeGreaterThan(30); // Should maintain 30+ FPS
    expect(performance.memoryUsage).toBeLessThan(100); // Should use less than 100MB
  });

  test('Accessibility and Reduced Motion', async ({ page }) => {
    // Set reduced motion preference
    await page.addInitScript(() => {
      Object.defineProperty(window, 'matchMedia', {
        writable: true,
        value: jest.fn().mockImplementation(query => ({
          matches: query === '(prefers-reduced-motion: reduce)',
          media: query,
          onchange: null,
          addListener: jest.fn(),
          removeListener: jest.fn(),
          addEventListener: jest.fn(),
          removeEventListener: jest.fn(),
          dispatchEvent: jest.fn(),
        })),
      });
    });

    // Reload page with reduced motion preference
    await page.reload();

    // Create animated element
    await page.evaluate(() => {
      const animatedElement = document.createElement('div');
      animatedElement.id = 'animated-element';
      animatedElement.style.cssText =
        'width: 100px; height: 100px; background: blue; transition: all 0.3s ease;';
      document.body.appendChild(animatedElement);
    });

    const animatedElement = page.locator('#animated-element');

    // Get initial state
    const initialOpacity = await helper.getComputedStyle(animatedElement, 'opacity');

    // Trigger animation
    await animatedElement.click();
    await page.waitForTimeout(100);

    // Check that animation was minimal or disabled
    const finalOpacity = await helper.getComputedStyle(animatedElement, 'opacity');
    const opacityChange = Math.abs(parseFloat(finalOpacity) - parseFloat(initialOpacity));

    // With reduced motion, the change should be minimal
    expect(opacityChange).toBeLessThan(0.1);
  });

  test('Cross-Browser Compatibility', async ({ page, browserName }) => {
    // Test basic functionality across different browsers
    await page.evaluate(() => {
      const testElement = document.createElement('div');
      testElement.id = 'compatibility-test';
      testElement.style.cssText =
        'width: 100px; height: 100px; background: green; transition: transform 0.3s ease;';
      document.body.appendChild(testElement);
    });

    const testElement = page.locator('#compatibility-test');

    // Test hover animation
    await testElement.hover();
    await helper.assertAnimationCompleted(testElement, 'transform', 'scale(1.1)');

    // Test click animation
    await testElement.click();
    await helper.assertAnimationCompleted(testElement, 'transform', 'scale(0.95)');

    // Browser-specific assertions
    if (browserName === 'chromium') {
      // Chrome-specific tests
      expect(await helper.getComputedStyle(testElement, 'transform')).toContain('matrix');
    } else if (browserName === 'firefox') {
      // Firefox-specific tests
      expect(await helper.getComputedStyle(testElement, 'transform')).toContain('matrix');
    } else if (browserName === 'webkit') {
      // Safari-specific tests
      expect(await helper.getComputedStyle(testElement, 'transform')).toContain('matrix');
    }
  });
});
