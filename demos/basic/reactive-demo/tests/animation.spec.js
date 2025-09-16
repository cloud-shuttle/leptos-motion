const { test, expect } = require('@playwright/test');

test.describe('Leptos Motion Animation Demo', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the WASM to load
    await page.waitForLoadState('networkidle');
  });

  test('should load the demo page', async ({ page }) => {
    await expect(page.locator('h1').first()).toContainText('Phase 2: Reactive Animation Demo');
  });

  test('should have animation controls', async ({ page }) => {
    // Check that all control sliders are present
    await expect(page.locator('input[type="range"]')).toHaveCount(5);
    
    // Check that the animated element is present
    await expect(page.getByText('Animated!')).toBeVisible();
  });

  test('should animate when scale slider is moved', async ({ page }) => {
    const animatedElement = page.getByText('Animated!');
    const scaleSlider = page.locator('input[type="range"]').nth(0); // Scale slider
    
    // Get the ReactiveMotionDiv element (parent of the "Animated!" element)
    const reactiveMotionDiv = animatedElement.locator('..');
    
    // Check if the element exists and get its info
    const elementInfo = await reactiveMotionDiv.evaluate(el => {
      return {
        tagName: el.tagName,
        className: el.className,
        parentTagName: el.parentElement?.tagName,
        parentClassName: el.parentElement?.className,
        style: el.style.cssText,
        computedStyle: window.getComputedStyle(el).cssText
      };
    });
    
    console.log('Element info:', elementInfo);
    
    // Get initial transform from the ReactiveMotionDiv
    const initialTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    console.log('Initial transform:', initialTransform);
    
    // Move the scale slider (max is 2.0)
    await scaleSlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '2.0');
    
    // Wait for animation to start and capture intermediate values
    let hasAnimationStarted = false;
    let finalTransform = initialTransform;
    
    // Check for animation progress over time
    for (let i = 0; i < 20; i++) {
      await page.waitForTimeout(100);
      const currentTransform = await reactiveMotionDiv.evaluate(el => {
        return window.getComputedStyle(el).transform;
      });
      
      console.log(`Frame ${i}: ${currentTransform}`);
      
      if (currentTransform !== initialTransform) {
        hasAnimationStarted = true;
        finalTransform = currentTransform;
        // Check if we have a scale value that's not 1
        if (currentTransform.includes('scale(') && !currentTransform.includes('scale(1)')) {
          break;
        }
      }
    }
    
    console.log('Animation started:', hasAnimationStarted);
    console.log('Final transform:', finalTransform);
    
    // Verify animation started
    expect(hasAnimationStarted).toBe(true);
    expect(finalTransform).not.toBe(initialTransform);
    // The transform should be a matrix representing scale(2)
    expect(finalTransform).toContain('matrix(2, 0, 0, 2, 0, 0)');
    
    // Wait a bit more for animation to complete
    await page.waitForTimeout(1000);
    
    // Check final state
    const completedTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    console.log('Completed transform:', completedTransform);
    
    // The final transform should be a matrix representing scale(2)
    expect(completedTransform).toContain('matrix(2, 0, 0, 2, 0, 0)');
  });

  test('should animate when rotation slider is moved', async ({ page }) => {
    const animatedElement = page.getByText('Animated!');
    const rotationSlider = page.locator('input[type="range"]').nth(1); // Rotation slider (second slider)
    
    // Get the ReactiveMotionDiv element (parent of the "Animated!" element)
    const reactiveMotionDiv = animatedElement.locator('..');
    
    // Get initial transform from the ReactiveMotionDiv
    const initialTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    // Move the rotation slider (step is 10, so use 50)
    await rotationSlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '50');
    
    // Wait for animation to start and capture intermediate values
    let hasAnimationStarted = false;
    let finalTransform = initialTransform;
    
    // Check for animation progress over time
    for (let i = 0; i < 20; i++) {
      await page.waitForTimeout(100);
      const currentTransform = await reactiveMotionDiv.evaluate(el => {
        return window.getComputedStyle(el).transform;
      });
      
      if (currentTransform !== initialTransform) {
        hasAnimationStarted = true;
        finalTransform = currentTransform;
        // Check if we have a rotate value that's not 0 (matrix will change for rotation)
        if (currentTransform !== initialTransform) {
          break;
        }
      }
    }
    
    // Verify animation started
    expect(hasAnimationStarted).toBe(true);
    expect(finalTransform).not.toBe(initialTransform);
    // The transform should be a matrix representing rotation (not the identity matrix)
    expect(finalTransform).not.toBe('matrix(1, 0, 0, 1, 0, 0)');
    
    // Wait a bit more for animation to complete
    await page.waitForTimeout(1000);
    
    // Check final state
    const completedTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    // The final transform should be a matrix representing rotation (not the identity matrix)
    expect(completedTransform).not.toBe('matrix(1, 0, 0, 1, 0, 0)');
  });

  test('should animate when position sliders are moved', async ({ page }) => {
    const animatedElement = page.getByText('Animated!');
    const xSlider = page.locator('input[type="range"]').nth(3); // X position slider (fourth slider)
    const ySlider = page.locator('input[type="range"]').nth(4); // Y position slider (fifth slider)
    
    // Get the ReactiveMotionDiv element (parent of the "Animated!" element)
    const reactiveMotionDiv = animatedElement.locator('..');
    
    // Get initial transform from the ReactiveMotionDiv
    const initialTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    // Move both position sliders (range is -200 to 200, step 10)
    await xSlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '50');
    await ySlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '50');
    
    // Wait for animation to start and capture intermediate values
    let hasAnimationStarted = false;
    let finalTransform = initialTransform;
    
    // Check for animation progress over time
    for (let i = 0; i < 20; i++) {
      await page.waitForTimeout(100);
      const currentTransform = await reactiveMotionDiv.evaluate(el => {
        return window.getComputedStyle(el).transform;
      });
      
      if (currentTransform !== initialTransform) {
        hasAnimationStarted = true;
        finalTransform = currentTransform;
        // Check if we have translate values that are not 0 (matrix will change for translation)
        if (currentTransform !== initialTransform) {
          break;
        }
      }
    }
    
    // Verify animation started
    expect(hasAnimationStarted).toBe(true);
    expect(finalTransform).not.toBe(initialTransform);
    // The transform should be a matrix representing translation (not the identity matrix)
    expect(finalTransform).not.toBe('matrix(1, 0, 0, 1, 0, 0)');
    
    // Wait a bit more for animation to complete
    await page.waitForTimeout(1000);
    
    // Check final state
    const completedTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    // The final transform should be a matrix representing translation (not the identity matrix)
    expect(completedTransform).not.toBe('matrix(1, 0, 0, 1, 0, 0)');
  });

  test('should animate opacity when opacity slider is moved', async ({ page }) => {
    const animatedElement = page.getByText('Animated!');
    const opacitySlider = page.locator('input[type="range"]').nth(2); // Opacity slider (third slider)
    
    // Get the ReactiveMotionDiv element (parent of the "Animated!" element)
    const reactiveMotionDiv = animatedElement.locator('..');
    
    // Get initial opacity from the ReactiveMotionDiv
    const initialOpacity = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).opacity;
    });
    
    // Move the opacity slider (range is 0.0 to 1.0, step 0.1)
    await opacitySlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '0.5');
    
    // Wait for animation to start and capture intermediate values
    let hasAnimationStarted = false;
    let finalOpacity = initialOpacity;
    
    // Check for animation progress over time
    for (let i = 0; i < 20; i++) {
      await page.waitForTimeout(100);
      const currentOpacity = await reactiveMotionDiv.evaluate(el => {
        return window.getComputedStyle(el).opacity;
      });
      
      if (currentOpacity !== initialOpacity) {
        hasAnimationStarted = true;
        finalOpacity = currentOpacity;
        // Check if we have an opacity value that's not 1
        if (parseFloat(currentOpacity) !== 1.0) {
          break;
        }
      }
    }
    
    // Verify animation started
    expect(hasAnimationStarted).toBe(true);
    expect(finalOpacity).not.toBe(initialOpacity);
    
    // Wait a bit more for animation to complete
    await page.waitForTimeout(1000);
    
    // Check final state
    const completedOpacity = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).opacity;
    });
    
    // The final opacity should be close to 0.5
    expect(parseFloat(completedOpacity)).toBeCloseTo(0.5, 1);
  });

  test('should have smooth animations', async ({ page }) => {
    const animatedElement = page.getByText('Animated!');
    const scaleSlider = page.locator('input[type="range"]').nth(0);
    
    // Get the ReactiveMotionDiv element (parent of the "Animated!" element)
    const reactiveMotionDiv = animatedElement.locator('..');
    
    // Get initial transform from the ReactiveMotionDiv
    const initialTransform = await reactiveMotionDiv.evaluate(el => {
      return window.getComputedStyle(el).transform;
    });
    
    // Move slider and capture multiple frames
    await scaleSlider.evaluate((el, value) => {
      el.value = value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    }, '1.5');
    
    const transforms = [];
    let hasAnimationStarted = false;
    
    // Capture frames during animation
    for (let i = 0; i < 20; i++) {
      await page.waitForTimeout(50);
      const transform = await reactiveMotionDiv.evaluate(el => {
        return window.getComputedStyle(el).transform;
      });
      transforms.push(transform);
      
      if (transform !== initialTransform) {
        hasAnimationStarted = true;
      }
    }
    
    // Verify animation started
    expect(hasAnimationStarted).toBe(true);
    
    // Check that we have intermediate values (smooth animation)
    const uniqueTransforms = [...new Set(transforms)];
    expect(uniqueTransforms.length).toBeGreaterThan(1);
    
    // Verify that the transforms contain scale values (matrix values that are not identity)
    const hasScaleTransforms = transforms.some(transform => 
      transform !== 'matrix(1, 0, 0, 1, 0, 0)'
    );
    expect(hasScaleTransforms).toBe(true);
  });

  test('should not have reactive tracking warnings in console', async ({ page }) => {
    const consoleMessages = [];
    
    page.on('console', msg => {
      if (msg.type() === 'warn' && msg.text().includes('reactive tracking context')) {
        consoleMessages.push(msg.text());
      }
    });
    
    // Interact with sliders to trigger potential warnings
    const scaleSlider = page.locator('input[type="range"]').nth(0);
    await scaleSlider.fill('1.5');
    await page.waitForTimeout(100);
    
    const rotationSlider = page.locator('input[type="range"]').nth(4);
    await rotationSlider.fill('30');
    await page.waitForTimeout(100);
    
    // Check that no reactive tracking warnings were logged
    expect(consoleMessages).toHaveLength(0);
  });
});
