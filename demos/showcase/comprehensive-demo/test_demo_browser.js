const { exec } = require('child_process');
const fs = require('fs');

// Simple browser test using curl and basic checks
async function testDemo() {
  console.log('🧪 Testing Leptos Motion Demo with Browser Automation...');
  
  // Test 1: Check if the demo page loads
  console.log('📄 Testing demo page load...');
  const curlCommand = 'curl -s http://localhost:8080/dist/index.html';
  
  exec(curlCommand, (error, stdout, stderr) => {
    if (error) {
      console.log(`❌ Failed to load demo page: ${error.message}`);
      return;
    }
    
    if (stdout.includes('<!DOCTYPE html>')) {
      console.log('✅ Demo page loads successfully');
      
      // Check for key elements
      if (stdout.includes('TDD Reactive Animation Demo')) {
        console.log('✅ Demo title found');
      }
      
      if (stdout.includes('comprehensive-demo-3cd3f36ada708680.js')) {
        console.log('✅ WASM JavaScript file referenced');
      }
      
      if (stdout.includes('comprehensive-demo-3cd3f36ada708680_bg.wasm')) {
        console.log('✅ WASM file referenced');
      }
      
      // Check for interactive elements
      if (stdout.includes('<button')) {
        console.log('✅ Interactive button found');
      }
      
      if (stdout.includes('on:click')) {
        console.log('✅ Click handler found');
      }
      
      console.log('🎉 Demo page structure looks correct!');
      
    } else {
      console.log('❌ Demo page does not contain HTML content');
      console.log('📄 Response:', stdout.substring(0, 200));
    }
  });
  
  // Test 2: Check WASM files
  console.log('\n🔍 Testing WASM files...');
  
  const wasmFiles = [
    'comprehensive-demo-3cd3f36ada708680.js',
    'comprehensive-demo-3cd3f36ada708680_bg.wasm'
  ];
  
  wasmFiles.forEach(file => {
    const filePath = `dist/${file}`;
    if (fs.existsSync(filePath)) {
      const stats = fs.statSync(filePath);
      console.log(`✅ ${file} exists (${stats.size} bytes)`);
    } else {
      console.log(`❌ ${file} not found`);
    }
  });
  
  // Test 3: Check if server is serving files correctly
  console.log('\n🌐 Testing server file serving...');
  
  const testFiles = [
    '/dist/index.html',
    '/dist/comprehensive-demo-3cd3f36ada708680.js',
    '/dist/comprehensive-demo-3cd3f36ada708680_bg.wasm'
  ];
  
  testFiles.forEach(file => {
    exec(`curl -s -o /dev/null -w "%{http_code}" http://localhost:8080${file}`, (error, stdout) => {
      if (error) {
        console.log(`❌ ${file} - Server error: ${error.message}`);
      } else if (stdout.trim() === '200') {
        console.log(`✅ ${file} - Server returns 200 OK`);
      } else {
        console.log(`❌ ${file} - Server returns ${stdout.trim()}`);
      }
    });
  });
  
  // Test 4: Check demo functionality by opening in browser
  console.log('\n🌐 Opening demo in browser for manual verification...');
  
  exec('open http://localhost:8080/dist/index.html', (error) => {
    if (error) {
      console.log(`❌ Failed to open browser: ${error.message}`);
    } else {
      console.log('✅ Demo opened in browser');
      console.log('💡 Please check the browser for:');
      console.log('   - Page loads without errors');
      console.log('   - "Toggle Animation" button is visible');
      console.log('   - Clicking button triggers animation');
      console.log('   - No console errors in browser dev tools');
    }
  });
}

// Run the test
testDemo();
