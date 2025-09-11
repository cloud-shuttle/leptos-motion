import { test, expect, Page } from '@playwright/test';

// Test configuration
test.describe.configure({ mode: 'parallel' });

// Helper functions
async function waitForStudioToLoad(page: Page) {
    await page.waitForSelector('.studio-demo', { timeout: 10000 });
    await page.waitForLoadState('networkidle');
}

async function clickNavItem(page: Page, navItem: string) {
    await page.click(`.studio-demo__nav-item:has-text("${navItem}")`);
    await page.waitForTimeout(500); // Allow for transitions
}

async function expectNavItemActive(page: Page, navItem: string) {
    const navButton = page.locator(`.studio-demo__nav-item:has-text("${navItem}")`);
    await expect(navButton).toHaveClass(/active/);
}

// Test suite for Motion Studio Demo
test.describe('Motion Studio Demo', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
    });

    test('should load the studio demo successfully', async ({ page }) => {
        // Check that the main components are present
        await expect(page.locator('.studio-demo')).toBeVisible();
        await expect(page.locator('.studio-demo__header h1')).toContainText('Motion Studio Demo');
        
        // Check that navigation is present
        await expect(page.locator('.studio-demo__nav')).toBeVisible();
        await expect(page.locator('.studio-demo__nav-item:has-text("Timeline")')).toBeVisible();
        await expect(page.locator('.studio-demo__nav-item:has-text("3D Transforms")')).toBeVisible();
        await expect(page.locator('.studio-demo__nav-item:has-text("SVG Morphing")')).toBeVisible();
        await expect(page.locator('.studio-demo__nav-item:has-text("Export")')).toBeVisible();
    });

    test('should have Timeline mode active by default', async ({ page }) => {
        await expectNavItemActive(page, 'Timeline');
        await expect(page.locator('.timeline-demo')).toBeVisible();
    });

    test('should switch between demo modes', async ({ page }) => {
        // Test switching to 3D Transforms
        await clickNavItem(page, '3D Transforms');
        await expectNavItemActive(page, '3D Transforms');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();

        // Test switching to SVG Morphing
        await clickNavItem(page, 'SVG Morphing');
        await expectNavItemActive(page, 'SVG Morphing');
        await expect(page.locator('.morphing-demo')).toBeVisible();

        // Test switching to Export
        await clickNavItem(page, 'Export');
        await expectNavItemActive(page, 'Export');
        await expect(page.locator('.export-demo')).toBeVisible();

        // Test switching back to Timeline
        await clickNavItem(page, 'Timeline');
        await expectNavItemActive(page, 'Timeline');
        await expect(page.locator('.timeline-demo')).toBeVisible();
    });
});

// Test suite for Timeline functionality
test.describe('Timeline Demo', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
        await clickNavItem(page, 'Timeline');
    });

    test('should display timeline controls', async ({ page }) => {
        await expect(page.locator('.timeline-demo__controls h3')).toContainText('Timeline Controls');
        await expect(page.locator('.property-selector')).toBeVisible();
        await expect(page.locator('.timeline-actions')).toBeVisible();
    });

    test('should have property selector with correct options', async ({ page }) => {
        const propertySelect = page.locator('.property-selector select');
        await expect(propertySelect).toBeVisible();
        
        // Check that all expected properties are present
        await expect(propertySelect.locator('option[value="TranslateX"]')).toBeVisible();
        await expect(propertySelect.locator('option[value="TranslateY"]')).toBeVisible();
        await expect(propertySelect.locator('option[value="RotateZ"]')).toBeVisible();
        await expect(propertySelect.locator('option[value="ScaleX"]')).toBeVisible();
        await expect(propertySelect.locator('option[value="Opacity"]')).toBeVisible();
    });

    test('should have timeline action buttons', async ({ page }) => {
        await expect(page.locator('.timeline-actions button:has-text("Add Keyframe")')).toBeVisible();
        await expect(page.locator('.timeline-actions button:has-text("Play")')).toBeVisible();
        await expect(page.locator('.timeline-actions button:has-text("Pause")')).toBeVisible();
    });

    test('should display timeline editor', async ({ page }) => {
        await expect(page.locator('.timeline-demo__editor')).toBeVisible();
        await expect(page.locator('.timeline-editor')).toBeVisible();
    });

    test('should handle property selection', async ({ page }) => {
        const propertySelect = page.locator('.property-selector select');
        
        // Select different properties
        await propertySelect.selectOption('TranslateY');
        await propertySelect.selectOption('RotateZ');
        await propertySelect.selectOption('Opacity');
        
        // Should not cause any errors
        await expect(page.locator('.timeline-demo')).toBeVisible();
    });

    test('should handle button clicks without errors', async ({ page }) => {
        // Test Add Keyframe button
        await page.click('.timeline-actions button:has-text("Add Keyframe")');
        await expect(page.locator('.timeline-demo')).toBeVisible();

        // Test Play button
        await page.click('.timeline-actions button:has-text("Play")');
        await expect(page.locator('.timeline-demo')).toBeVisible();

        // Test Pause button
        await page.click('.timeline-actions button:has-text("Pause")');
        await expect(page.locator('.timeline-demo')).toBeVisible();
    });
});

// Test suite for 3D Transform functionality
test.describe('3D Transform Demo', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
        await clickNavItem(page, '3D Transforms');
    });

    test('should display 3D transform controls', async ({ page }) => {
        await expect(page.locator('.transform-3d-demo__controls h3')).toContainText('3D Transform Controls');
        await expect(page.locator('.transform-controls')).toBeVisible();
    });

    test('should have transform control sliders', async ({ page }) => {
        await expect(page.locator('.control-group:has-text("Translation X:")')).toBeVisible();
        await expect(page.locator('.control-group:has-text("Translation Y:")')).toBeVisible();
        await expect(page.locator('.control-group:has-text("Rotation Z:")')).toBeVisible();
    });

    test('should display 3D preview', async ({ page }) => {
        await expect(page.locator('.transform-3d-demo__preview')).toBeVisible();
        await expect(page.locator('.transform-preview')).toBeVisible();
        await expect(page.locator('.transform-cube')).toBeVisible();
    });

    test('should have all cube faces', async ({ page }) => {
        await expect(page.locator('.cube-face.front')).toBeVisible();
        await expect(page.locator('.cube-face.back')).toBeVisible();
        await expect(page.locator('.cube-face.right')).toBeVisible();
        await expect(page.locator('.cube-face.left')).toBeVisible();
        await expect(page.locator('.cube-face.top')).toBeVisible();
        await expect(page.locator('.cube-face.bottom')).toBeVisible();
    });

    test('should handle slider interactions', async ({ page }) => {
        const translationXSlider = page.locator('.control-group:has-text("Translation X:") input[type="range"]');
        
        // Move slider to different positions
        await translationXSlider.fill('50');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();
        
        await translationXSlider.fill('-25');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();
        
        await translationXSlider.fill('100');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();
    });
});

// Test suite for SVG Morphing functionality
test.describe('SVG Morphing Demo', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
        await clickNavItem(page, 'SVG Morphing');
    });

    test('should display morphing controls', async ({ page }) => {
        await expect(page.locator('.morphing-demo__controls h3')).toContainText('SVG Path Morphing');
        await expect(page.locator('.path-inputs')).toBeVisible();
        await expect(page.locator('.morph-controls')).toBeVisible();
    });

    test('should have path input fields', async ({ page }) => {
        await expect(page.locator('.path-input:has-text("Source Path:")')).toBeVisible();
        await expect(page.locator('.path-input:has-text("Target Path:")')).toBeVisible();
        
        const sourceTextarea = page.locator('.path-input:has-text("Source Path:") textarea');
        const targetTextarea = page.locator('.path-input:has-text("Target Path:") textarea');
        
        await expect(sourceTextarea).toBeVisible();
        await expect(targetTextarea).toBeVisible();
    });

    test('should have morph progress control', async ({ page }) => {
        await expect(page.locator('.morph-controls:has-text("Morph Progress:")')).toBeVisible();
        await expect(page.locator('.morph-controls input[type="range"]')).toBeVisible();
    });

    test('should display morphing preview', async ({ page }) => {
        await expect(page.locator('.morphing-demo__preview')).toBeVisible();
        await expect(page.locator('.svg-morphing-editor')).toBeVisible();
    });

    test('should handle path input changes', async ({ page }) => {
        const sourceTextarea = page.locator('.path-input:has-text("Source Path:") textarea');
        const targetTextarea = page.locator('.path-input:has-text("Target Path:") textarea');
        
        // Change source path
        await sourceTextarea.fill('M 0 0 L 100 0 L 100 100 Z');
        await expect(page.locator('.morphing-demo')).toBeVisible();
        
        // Change target path
        await targetTextarea.fill('M 0 0 L 50 50 L 100 100 Z');
        await expect(page.locator('.morphing-demo')).toBeVisible();
    });

    test('should handle morph progress changes', async ({ page }) => {
        const progressSlider = page.locator('.morph-controls input[type="range"]');
        
        // Test different progress values
        await progressSlider.fill('0.25');
        await expect(page.locator('.morphing-demo')).toBeVisible();
        
        await progressSlider.fill('0.5');
        await expect(page.locator('.morphing-demo')).toBeVisible();
        
        await progressSlider.fill('0.75');
        await expect(page.locator('.morphing-demo')).toBeVisible();
    });
});

// Test suite for Export functionality
test.describe('Export Demo', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
        await clickNavItem(page, 'Export');
    });

    test('should display export controls', async ({ page }) => {
        await expect(page.locator('.export-demo__controls h3')).toContainText('Export Animation');
        await expect(page.locator('.format-selector')).toBeVisible();
        await expect(page.locator('.export-demo__controls button:has-text("Export")')).toBeVisible();
    });

    test('should have format selector with correct options', async ({ page }) => {
        const formatSelect = page.locator('.format-selector select');
        await expect(formatSelect).toBeVisible();
        
        // Check that all expected formats are present
        await expect(formatSelect.locator('option[value="CSS"]')).toBeVisible();
        await expect(formatSelect.locator('option[value="WAAPI"]')).toBeVisible();
        await expect(formatSelect.locator('option[value="LeptosMotion"]')).toBeVisible();
        await expect(formatSelect.locator('option[value="FramerMotion"]')).toBeVisible();
    });

    test('should display export result area', async ({ page }) => {
        await expect(page.locator('.export-demo__result')).toBeVisible();
        await expect(page.locator('.export-demo__result h4')).toContainText('Exported Code:');
        await expect(page.locator('.export-code')).toBeVisible();
    });

    test('should handle format selection', async ({ page }) => {
        const formatSelect = page.locator('.format-selector select');
        
        // Select different formats
        await formatSelect.selectOption('WAAPI');
        await formatSelect.selectOption('LeptosMotion');
        await formatSelect.selectOption('FramerMotion');
        
        // Should not cause any errors
        await expect(page.locator('.export-demo')).toBeVisible();
    });

    test('should handle export button click', async ({ page }) => {
        const exportButton = page.locator('.export-demo__controls button:has-text("Export")');
        
        // Click export button
        await exportButton.click();
        
        // Should not cause any errors and should show some result
        await expect(page.locator('.export-demo')).toBeVisible();
        await expect(page.locator('.export-code')).toBeVisible();
    });
});

// Test suite for error handling and edge cases
test.describe('Error Handling and Edge Cases', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
    });

    test('should handle rapid navigation switching', async ({ page }) => {
        // Rapidly switch between modes
        for (let i = 0; i < 10; i++) {
            await clickNavItem(page, 'Timeline');
            await clickNavItem(page, '3D Transforms');
            await clickNavItem(page, 'SVG Morphing');
            await clickNavItem(page, 'Export');
        }
        
        // Should still be functional
        await expect(page.locator('.studio-demo')).toBeVisible();
    });

    test('should handle invalid input gracefully', async ({ page }) => {
        await clickNavItem(page, 'SVG Morphing');
        
        const sourceTextarea = page.locator('.path-input:has-text("Source Path:") textarea');
        
        // Test with invalid SVG path
        await sourceTextarea.fill('INVALID SVG PATH');
        await expect(page.locator('.morphing-demo')).toBeVisible();
        
        // Test with empty path
        await sourceTextarea.fill('');
        await expect(page.locator('.morphing-demo')).toBeVisible();
    });

    test('should handle extreme slider values', async ({ page }) => {
        await clickNavItem(page, '3D Transforms');
        
        const translationXSlider = page.locator('.control-group:has-text("Translation X:") input[type="range"]');
        
        // Test extreme values
        await translationXSlider.fill('-100');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();
        
        await translationXSlider.fill('100');
        await expect(page.locator('.transform-3d-demo')).toBeVisible();
    });

    test('should handle multiple export operations', async ({ page }) => {
        await clickNavItem(page, 'Export');
        
        const exportButton = page.locator('.export-demo__controls button:has-text("Export")');
        
        // Click export multiple times
        for (let i = 0; i < 5; i++) {
            await exportButton.click();
            await page.waitForTimeout(100);
        }
        
        // Should still be functional
        await expect(page.locator('.export-demo')).toBeVisible();
    });
});

// Test suite for performance and responsiveness
test.describe('Performance and Responsiveness', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
    });

    test('should load within reasonable time', async ({ page }) => {
        const startTime = Date.now();
        await waitForStudioToLoad(page);
        const loadTime = Date.now() - startTime;
        
        // Should load within 10 seconds
        expect(loadTime).toBeLessThan(10000);
    });

    test('should be responsive on mobile viewport', async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 667 }); // iPhone SE size
        
        await expect(page.locator('.studio-demo')).toBeVisible();
        await expect(page.locator('.studio-demo__header')).toBeVisible();
        await expect(page.locator('.studio-demo__nav')).toBeVisible();
    });

    test('should be responsive on tablet viewport', async ({ page }) => {
        await page.setViewportSize({ width: 768, height: 1024 }); // iPad size
        
        await expect(page.locator('.studio-demo')).toBeVisible();
        await expect(page.locator('.studio-demo__content')).toBeVisible();
    });

    test('should handle window resize gracefully', async ({ page }) => {
        // Test various viewport sizes
        const viewports = [
            { width: 320, height: 568 },   // iPhone 5
            { width: 375, height: 667 },   // iPhone SE
            { width: 768, height: 1024 },  // iPad
            { width: 1024, height: 768 },  // iPad landscape
            { width: 1920, height: 1080 }, // Desktop
        ];
        
        for (const viewport of viewports) {
            await page.setViewportSize(viewport);
            await page.waitForTimeout(100);
            await expect(page.locator('.studio-demo')).toBeVisible();
        }
    });
});

// Test suite for accessibility
test.describe('Accessibility', () => {
    test.beforeEach(async ({ page }) => {
        await page.goto('/');
        await waitForStudioToLoad(page);
    });

    test('should have proper heading structure', async ({ page }) => {
        const h1 = page.locator('h1');
        await expect(h1).toHaveCount(1);
        await expect(h1).toContainText('Motion Studio Demo');
    });

    test('should have accessible form controls', async ({ page }) => {
        // Check that form controls have proper labels
        await expect(page.locator('label')).toHaveCount(4); // At least 4 labels in timeline mode
        
        // Check that buttons are accessible
        const buttons = page.locator('button');
        await expect(buttons).toHaveCount(4); // At least 4 buttons in timeline mode
    });

    test('should support keyboard navigation', async ({ page }) => {
        // Test tab navigation
        await page.keyboard.press('Tab');
        await page.keyboard.press('Tab');
        await page.keyboard.press('Tab');
        
        // Should not cause any errors
        await expect(page.locator('.studio-demo')).toBeVisible();
    });
});

