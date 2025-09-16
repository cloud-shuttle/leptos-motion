const http = require('http');

// Test if the server is running and serving the demo
function testDemo() {
  console.log('🧪 Testing Leptos Motion Demo...');
  
  // Test 1: Check if server is running
  const options = {
    hostname: 'localhost',
    port: 8080,
    path: '/dist/index.html',
    method: 'GET'
  };

  const req = http.request(options, (res) => {
    console.log(`✅ Server is running - Status: ${res.statusCode}`);
    
    let data = '';
    res.on('data', (chunk) => {
      data += chunk;
    });
    
    res.on('end', () => {
      if (res.statusCode === 200) {
        console.log('✅ Demo file is accessible');
        
        // Check if it's HTML content
        if (data.includes('<!DOCTYPE HTML>') || data.includes('<html')) {
          console.log('✅ File contains HTML content');
          
          // Check for key elements
          if (data.includes('TDD Reactive Animation Demo')) {
            console.log('✅ Demo title found');
          }
          
          if (data.includes('comprehensive-demo-3cd3f36ada708680.js')) {
            console.log('✅ WASM JavaScript file referenced');
          }
          
          if (data.includes('comprehensive-demo-3cd3f36ada708680_bg.wasm')) {
            console.log('✅ WASM file referenced');
          }
          
          console.log('🎉 Demo appears to be properly configured!');
          console.log('📊 Demo file size:', data.length, 'bytes');
          
        } else {
          console.log('❌ File does not contain HTML content');
          console.log('📄 First 200 characters:', data.substring(0, 200));
        }
      } else {
        console.log(`❌ Demo file not accessible - Status: ${res.statusCode}`);
        console.log('📄 Response:', data.substring(0, 200));
      }
    });
  });

  req.on('error', (e) => {
    console.log(`❌ Server connection failed: ${e.message}`);
    console.log('💡 Make sure the server is running with: python3 -m http.server 8080');
  });

  req.end();
}

// Test 2: Check if WASM files are accessible
function testWasmFiles() {
  console.log('\n🔍 Testing WASM files...');
  
  const wasmFiles = [
    '/dist/comprehensive-demo-3cd3f36ada708680.js',
    '/dist/comprehensive-demo-3cd3f36ada708680_bg.wasm'
  ];
  
  wasmFiles.forEach((file, index) => {
    const options = {
      hostname: 'localhost',
      port: 8080,
      path: file,
      method: 'HEAD' // Just check if file exists
    };

    const req = http.request(options, (res) => {
      if (res.statusCode === 200) {
        console.log(`✅ ${file} is accessible`);
      } else {
        console.log(`❌ ${file} not accessible - Status: ${res.statusCode}`);
      }
    });

    req.on('error', (e) => {
      console.log(`❌ ${file} connection failed: ${e.message}`);
    });

    req.end();
  });
}

// Run tests
testDemo();
setTimeout(testWasmFiles, 1000);
