#!/usr/bin/env node

const { exec } = require('child_process');
const path = require('path');

// Test configurations with timeouts
const testConfigs = [
  {
    name: 'Responsiveness Tests',
    command: 'npx playwright test tests/simple-responsiveness-test.spec.ts --reporter=line',
    timeout: 60000, // 1 minute
    description: 'Tests for page responsiveness and basic interactions',
  },
  {
    name: 'Visual Regression Tests',
    command: 'npx playwright test tests/visual-regression.spec.ts --reporter=line',
    timeout: 120000, // 2 minutes
    description: 'Tests for visual consistency and screenshot comparisons',
  },
  {
    name: 'Performance Monitoring',
    command: 'npx playwright test tests/performance-monitoring.spec.ts --reporter=line',
    timeout: 180000, // 3 minutes
    description: 'Tests for performance metrics and monitoring',
  },
  {
    name: 'Component Coverage',
    command: 'npx playwright test tests/component-coverage.spec.ts --reporter=line',
    timeout: 240000, // 4 minutes
    description: 'Tests for comprehensive component coverage',
  },
];

// Function to run a test with timeout
function runTestWithTimeout(config) {
  return new Promise((resolve, reject) => {
    console.log(`\n🧪 Running ${config.name}...`);
    console.log(`📝 ${config.description}`);
    console.log(`⏱️  Timeout: ${config.timeout / 1000} seconds`);
    console.log(`🔧 Command: ${config.command}`);
    console.log('─'.repeat(60));

    const startTime = Date.now();

    const child = exec(config.command, {
      cwd: process.cwd(),
      timeout: config.timeout,
    });

    let output = '';
    let errorOutput = '';

    child.stdout.on('data', data => {
      output += data;
      process.stdout.write(data);
    });

    child.stderr.on('data', data => {
      errorOutput += data;
      process.stderr.write(data);
    });

    child.on('close', code => {
      const endTime = Date.now();
      const duration = endTime - startTime;

      console.log('─'.repeat(60));

      if (code === 0) {
        console.log(`✅ ${config.name} PASSED (${duration}ms)`);
        resolve({
          name: config.name,
          status: 'PASSED',
          duration,
          output,
          errorOutput,
        });
      } else {
        console.log(`❌ ${config.name} FAILED (${duration}ms)`);
        resolve({
          name: config.name,
          status: 'FAILED',
          duration,
          output,
          errorOutput,
          exitCode: code,
        });
      }
    });

    child.on('error', error => {
      const endTime = Date.now();
      const duration = endTime - startTime;

      console.log('─'.repeat(60));
      console.log(`💥 ${config.name} ERROR (${duration}ms)`);
      console.log(`Error: ${error.message}`);

      reject({
        name: config.name,
        status: 'ERROR',
        duration,
        error: error.message,
      });
    });
  });
}

// Function to run all tests
async function runAllTests() {
  console.log('🚀 Starting Comprehensive Test Suite with Timeouts');
  console.log('='.repeat(60));

  const results = [];
  const startTime = Date.now();

  for (const config of testConfigs) {
    try {
      const result = await runTestWithTimeout(config);
      results.push(result);
    } catch (error) {
      results.push(error);
    }
  }

  const endTime = Date.now();
  const totalDuration = endTime - startTime;

  // Generate summary
  console.log('\n📊 TEST SUMMARY');
  console.log('='.repeat(60));

  const passed = results.filter(r => r.status === 'PASSED').length;
  const failed = results.filter(r => r.status === 'FAILED').length;
  const errors = results.filter(r => r.status === 'ERROR').length;

  console.log(`Total Tests: ${results.length}`);
  console.log(`✅ Passed: ${passed}`);
  console.log(`❌ Failed: ${failed}`);
  console.log(`💥 Errors: ${errors}`);
  console.log(`⏱️  Total Duration: ${totalDuration}ms`);

  console.log('\n📋 DETAILED RESULTS');
  console.log('─'.repeat(60));

  results.forEach(result => {
    const status = result.status === 'PASSED' ? '✅' : result.status === 'FAILED' ? '❌' : '💥';
    console.log(`${status} ${result.name}: ${result.status} (${result.duration}ms)`);

    if (result.status === 'FAILED' && result.exitCode) {
      console.log(`   Exit Code: ${result.exitCode}`);
    }

    if (result.status === 'ERROR' && result.error) {
      console.log(`   Error: ${result.error}`);
    }
  });

  // Exit with appropriate code
  if (failed > 0 || errors > 0) {
    console.log('\n💥 Some tests failed or errored. Check the output above.');
    process.exit(1);
  } else {
    console.log('\n🎉 All tests passed successfully!');
    process.exit(0);
  }
}

// Handle process termination
process.on('SIGINT', () => {
  console.log('\n🛑 Test execution interrupted by user');
  process.exit(130);
});

process.on('SIGTERM', () => {
  console.log('\n🛑 Test execution terminated');
  process.exit(143);
});

// Run the tests
if (require.main === module) {
  runAllTests().catch(error => {
    console.error('💥 Fatal error:', error);
    process.exit(1);
  });
}

module.exports = { runTestWithTimeout, runAllTests };
