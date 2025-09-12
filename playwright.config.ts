import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './',
  testMatch: [
    'tdd_demo_test.spec.ts',
    'simple_demo_test.spec.ts',
    'fixed_demo_test.spec.ts',
    'minimal_test.spec.ts',
    'phase4a_demo_test.spec.ts',
    'simple_phase4a_test.spec.ts',
    'transition_config_test.spec.ts',
    'memoization_test.spec.ts',
    'batched_updates_test.spec.ts',
    'comprehensive_e2e_test.spec.ts',
  ],
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  workers: process.env.CI ? 1 : undefined,
  timeout: 30000, // 30 seconds per test
  expect: {
    timeout: 10000, // 10 seconds for assertions
  },
  reporter: [
    ['html', { outputFolder: 'playwright-report' }],
    ['json', { outputFile: 'test-results.json' }],
    ['junit', { outputFile: 'test-results.xml' }],
    ...(process.env.CI ? [['github']] : []),
  ],
  use: {
    baseURL: 'http://localhost:8080',
    trace: 'on-first-retry',
    screenshot: 'only-on-failure',
    video: 'retain-on-failure',
    actionTimeout: 10000,
    navigationTimeout: 30000,
    // Performance monitoring
    launchOptions: {
      args: [
        '--disable-web-security',
        '--disable-features=VizDisplayCompositor',
        '--disable-background-timer-throttling',
        '--disable-backgrounding-occluded-windows',
        '--disable-renderer-backgrounding',
      ],
    },
  },

  projects: [
    {
      name: 'chromium',
      use: { 
        ...devices['Desktop Chrome'],
        // Enhanced performance monitoring
        launchOptions: {
          args: [
            '--enable-gpu-benchmarking',
            '--enable-threaded-compositing',
            '--enable-accelerated-2d-canvas',
          ],
        },
      },
    },
    {
      name: 'firefox',
      use: { 
        ...devices['Desktop Firefox'],
        // Firefox-specific optimizations
        launchOptions: {
          firefoxUserPrefs: {
            'dom.webgpu.enabled': true,
            'gfx.webrender.all': true,
          },
        },
      },
    },
    {
      name: 'webkit',
      use: { 
        ...devices['Desktop Safari'],
        // WebKit-specific settings
        launchOptions: {
          args: ['--enable-webgl2-compute-context'],
        },
      },
    },
    {
      name: 'Mobile Chrome',
      use: { 
        ...devices['Pixel 5'],
        // Mobile-specific optimizations
        launchOptions: {
          args: [
            '--enable-gpu-benchmarking',
            '--disable-background-timer-throttling',
          ],
        },
      },
    },
    {
      name: 'Mobile Safari',
      use: { 
        ...devices['iPhone 12'],
        // iOS-specific settings
        launchOptions: {
          args: ['--enable-webgl2-compute-context'],
        },
      },
    },
    // Performance testing project
    {
      name: 'performance',
      use: { 
        ...devices['Desktop Chrome'],
        // Performance-focused configuration
        launchOptions: {
          args: [
            '--enable-gpu-benchmarking',
            '--enable-threaded-compositing',
            '--enable-accelerated-2d-canvas',
            '--disable-background-timer-throttling',
            '--disable-backgrounding-occluded-windows',
            '--disable-renderer-backgrounding',
          ],
        },
      },
      testMatch: ['**/performance*.spec.ts'],
    },
  ],

  // webServer: {
  //   command: 'cd examples/comprehensive-demo && python3 serve.py',
  //   url: 'http://localhost:8080',
  //   reuseExistingServer: !process.env.CI,
  //   timeout: 120 * 1000,
  // },
});
