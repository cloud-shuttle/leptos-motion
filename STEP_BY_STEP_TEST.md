# Step-by-Step Test Guide

## 🧪 **Manual Testing Steps**

Since the shell has issues, let's test each demo individually:

### **Step 1: Test SSR Demo**
```bash
cd demos/ssr-demo
cargo check
```

**Expected Result**: Should compile without errors
**If Errors**: Note the specific error messages

### **Step 2: Test CSR Demo**
```bash
cd ../csr-demo
cargo check
```

**Expected Result**: Should compile without errors
**If Errors**: Note the specific error messages

### **Step 3: Test Native Demo**
```bash
cd ../native-test
cargo check
```

**Expected Result**: Should compile without errors
**If Errors**: Note the specific error messages

### **Step 4: Test Workspace**
```bash
cd ../..
cargo check --workspace
```

**Expected Result**: Should compile without errors
**If Errors**: Note the specific error messages

## 📋 **Error Analysis**

If you encounter errors, please share:

1. **Which demo failed** (SSR, CSR, Native, or Workspace)
2. **Specific error messages** (copy the full error output)
3. **Any warnings** (especially from leptos-motion-dom)

## 🔧 **Common Issues & Fixes**

### **If SSR Demo Fails**
- Check for missing imports
- Verify leptos_axum API usage
- Look for async/await issues

### **If CSR Demo Fails**
- Check for MotionDiv import issues
- Verify leptos-motion-dom dependencies
- Look for WASM compatibility issues

### **If Native Demo Fails**
- Check workspace configuration
- Verify path dependencies
- Look for missing features

### **If Workspace Fails**
- Check for circular dependencies
- Verify all crates compile individually
- Look for feature flag conflicts

## 🚀 **Next Steps**

1. **Run each test step by step**
2. **Note any errors that occur**
3. **Share the error details**
4. **I'll provide specific fixes for each issue**

This approach will help us identify exactly which demos are working and which need additional fixes.
