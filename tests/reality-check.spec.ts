import { test, expect } from '@playwright/test';

test.describe('Leptos Motion - Reality Check', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for the app to load
    await page.waitForSelector('h1:has-text("Leptos Motion - Advanced Features!")');
  });

  test.describe('Basic Functionality', () => {
    test('should load the showcase page', async ({ page }) => {
      const title = await page.locator('h1').textContent();
      expect(title).toContain('Leptos Motion - Advanced Features!');
    });

    test('should display all demo sections', async ({ page }) => {
      const sections = await page.locator('.demo-section').count();
      expect(sections).toBeGreaterThanOrEqual(5); // We have multiple demo sections
    });
  });

  test.describe('Animation System', () => {
    test('should have animated boxes with proper styling', async ({ page }) => {
      const animatedBox = page.locator('.animated-box').first();
      await expect(animatedBox).toBeVisible();

      // Check if it has the expected styles
      const styles = await animatedBox.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return {
          background: computed.background,
          borderRadius: computed.borderRadius,
          boxShadow: computed.boxShadow,
        };
      });

      expect(styles.background).not.toBe('none');
      expect(styles.borderRadius).toBe('12px');
      expect(styles.boxShadow).not.toBe('none');
    });

    test('should apply initial animation states', async ({ page }) => {
      const animatedBox = page.locator('.animated-box').first();

      // Check if the element has transform styles (indicating animation)
      const transform = await animatedBox.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return computed.transform;
      });

      // Should have some transform applied
      expect(transform).not.toBe('none');
    });
  });

  test.describe('Layout Transitions (FLIP)', () => {
    test('should have layout demo section', async ({ page }) => {
      const layoutSection = page.locator('h2:has-text("FLIP Layout Animations")');
      await expect(layoutSection).toBeVisible();
    });

    test('should have layout toggle button', async ({ page }) => {
      const toggleButton = page.locator('button:has-text("Switch to Grid")');
      await expect(toggleButton).toBeVisible();
    });

    test('should have layout items', async ({ page }) => {
      const layoutItems = page.locator('.layout-item');
      await expect(layoutItems).toHaveCount(4);
    });

    test('should switch between grid and list layouts', async ({ page }) => {
      const toggleButton = page.locator('button:has-text("Switch to Grid")');
      const layoutContainer = page.locator('.list-layout, .grid-layout');

      // Initial state should be list
      await expect(layoutContainer).toHaveClass(/list-layout/);

      // Click to switch to grid
      await toggleButton.click();

      // Should now be grid layout
      await expect(layoutContainer).toHaveClass(/grid-layout/);

      // Click again to switch back
      await toggleButton.click();
      await expect(layoutContainer).toHaveClass(/list-layout/);
    });
  });

  test.describe('Gesture Integration', () => {
    test('should have gesture demo section', async ({ page }) => {
      const gestureSection = page.locator('h2:has-text("Gesture Integration")');
      await expect(gestureSection).toBeVisible();
    });

    test('should have interactive gesture box', async ({ page }) => {
      const gestureBox = page.locator('.gesture-box');
      await expect(gestureBox).toBeVisible();

      // Check if it has proper cursor styles
      const cursor = await gestureBox.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return computed.cursor;
      });

      expect(cursor).toBe('pointer');
    });

    test('should have touch demo section', async ({ page }) => {
      const touchSection = page.locator('h2:has-text("Multi-touch Support")');
      await expect(touchSection).toBeVisible();
    });
  });

  test.describe('Interactive Elements', () => {
    test('should have working buttons', async ({ page }) => {
      const buttons = page.locator('button');
      await expect(buttons).toHaveCount(6); // Count all buttons

      // Test that buttons are clickable
      for (let i = 0; i < Math.min(buttons.count(), 3); i++) {
        const button = buttons.nth(i);
        await expect(button).toBeEnabled();
      }
    });

    test('should have counter functionality', async ({ page }) => {
      const counterButton = page.locator('button:has-text("Count: 0")');
      await expect(counterButton).toBeVisible();

      // Click to increment
      await counterButton.click();
      await expect(counterButton).toHaveText(/Count: 1/);

      // Click again
      await counterButton.click();
      await expect(counterButton).toHaveText(/Count: 2/);
    });

    test('should have show/hide functionality', async ({ page }) => {
      const toggleButton = page.locator('button:has-text("Hide")');
      const contentBox = page.locator('.content-box');

      // Initially visible
      await expect(contentBox).toBeVisible();

      // Click to hide
      await toggleButton.click();
      await expect(toggleButton).toHaveText('Show');
      await expect(contentBox).not.toBeVisible();

      // Click to show again
      await toggleButton.click();
      await expect(toggleButton).toHaveText('Hide');
      await expect(contentBox).toBeVisible();
    });
  });

  test.describe('CSS and Styling', () => {
    test('should have proper CSS animations', async ({ page }) => {
      // Check if CSS animations are defined
      const animations = await page.evaluate(() => {
        const styleSheets = Array.from(document.styleSheets);
        let hasAnimations = false;

        for (const sheet of styleSheets) {
          try {
            const rules = Array.from(sheet.cssRules || []);
            for (const rule of rules) {
              if (rule instanceof CSSKeyframesRule) {
                hasAnimations = true;
                break;
              }
            }
          } catch (e) {
            // CORS issues with external stylesheets
          }
        }

        return hasAnimations;
      });

      expect(animations).toBe(true);
    });

    test('should have responsive design', async ({ page }) => {
      // Test mobile viewport
      await page.setViewportSize({ width: 375, height: 667 });

      // Check if layout adapts
      const app = page.locator('.app');
      await expect(app).toBeVisible();

      // Check if buttons stack vertically on mobile
      const buttonGroup = page.locator('.button-group');
      const flexDirection = await buttonGroup.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return computed.flexDirection;
      });

      // Should be column on mobile
      expect(flexDirection).toBe('column');
    });
  });

  test.describe('Performance and Technical Features', () => {
    test('should have hardware acceleration hints', async ({ page }) => {
      const animatedElements = page.locator(
        '.animated-box, .content-box, .layout-item, .gesture-box, .touch-box'
      );

      for (let i = 0; i < Math.min(animatedElements.count(), 3); i++) {
        const element = animatedElements.nth(i);
        const willChange = await element.evaluate(el => {
          const computed = window.getComputedStyle(el);
          return computed.willChange;
        });

        // Should have will-change property for performance
        expect(willChange).not.toBe('auto');
      }
    });

    test('should have proper transform styles', async ({ page }) => {
      const animatedBox = page.locator('.animated-box').first();

      const transform = await animatedBox.evaluate(el => {
        const computed = window.getComputedStyle(el);
        return {
          transform: computed.transform,
          backfaceVisibility: computed.backfaceVisibility,
          transformStyle: computed.transformStyle,
        };
      });

      // Should have 3D transform support
      expect(transform.backfaceVisibility).toBe('hidden');
      expect(transform.transformStyle).toBe('flat');
    });
  });

  test.describe('Error Handling and Robustness', () => {
    test('should handle rapid interactions gracefully', async ({ page }) => {
      const counterButton = page.locator('button:has-text("Count: 0")');

      // Rapid clicking
      for (let i = 0; i < 10; i++) {
        await counterButton.click();
      }

      // Should still be functional
      await expect(counterButton).toHaveText(/Count: \d+/);
    });

    test('should maintain state during layout changes', async ({ page }) => {
      const counterButton = page.locator('button:has-text("Count: 0")');
      const layoutToggle = page.locator('button:has-text("Switch to Grid")');

      // Increment counter
      await counterButton.click();
      await expect(counterButton).toHaveText('Count: 1');

      // Change layout
      await layoutToggle.click();
      await expect(page.locator('.grid-layout')).toBeVisible();

      // Counter should maintain state
      await expect(counterButton).toHaveText('Count: 1');
    });
  });
});
