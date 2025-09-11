const fs = require('fs');
const path = require('path');

console.log('🔍 Testing WASM files...');

// Check if WASM files exist
const pkgDir = path.join(__dirname, 'examples/comprehensive-demo/pkg');
const files = ['comprehensive_demo.js', 'comprehensive_demo_bg.wasm', 'comprehensive_demo_bg.js'];

files.forEach(file => {
  const filePath = path.join(pkgDir, file);
  if (fs.existsSync(filePath)) {
    const stats = fs.statSync(filePath);
    console.log(`✅ ${file}: ${(stats.size / 1024).toFixed(1)}KB`);
  } else {
    console.log(`❌ ${file}: Missing`);
  }
});

// Check the main JS file content
const mainJsPath = path.join(pkgDir, 'comprehensive_demo.js');
if (fs.existsSync(mainJsPath)) {
  const content = fs.readFileSync(mainJsPath, 'utf8');
  console.log('\n📄 Main JS file analysis:');
  console.log(`- File size: ${(content.length / 1024).toFixed(1)}KB`);
  console.log(`- Contains 'default': ${content.includes('export default')}`);
  console.log(`- Contains 'wasm': ${content.includes('wasm')}`);
  console.log(`- Contains 'init': ${content.includes('init')}`);

  // Look for potential issues
  if (content.includes('import.meta.url')) {
    console.log('⚠️  Contains import.meta.url (may cause issues in some environments)');
  }
  if (content.includes('__dirname')) {
    console.log('⚠️  Contains __dirname (Node.js specific)');
  }
}

console.log('\n🌐 HTTP Server Test:');
console.log('Run: curl -s http://localhost:8000/pkg/comprehensive_demo.js | head -5');
