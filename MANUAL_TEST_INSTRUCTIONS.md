# Manual Test Instructions for MotionDiv Demos

## 🎯 **Quick Test Commands**

Since the shell has issues, here are the manual commands to test the demos:

### **1. Test Native Demo Compilation**
```bash
cd demos/native-test
cargo check
```

### **2. Test CSR Demo Compilation**
```bash
cd demos/csr-demo  
cargo check
```

### **3. Test SSR Demo Compilation**
```bash
cd demos/ssr-demo
cargo check
```

## 🧪 **Expected Results**

### **✅ Success Indicators**
- All demos should compile without errors
- No "missing field" or "trait not found" errors
- Clean compilation output

### **❌ Failure Indicators**
- Compilation errors about missing types
- Import/export errors
- Trait bound errors

## 🚀 **Run the Demos**

### **Native Test Demo**
```bash
cd demos/native-test
trunk serve
# Should open at http://localhost:8080
```

### **CSR Demo**
```bash
cd demos/csr-demo
trunk serve  
# Should open at http://localhost:8080
```

### **SSR Demo**
```bash
cd demos/ssr-demo
cargo run
# Should start server on http://localhost:3000
```

## 🎯 **Interactive Testing**

Once the demos are running:

1. **Click "Toggle Animation"** - Should animate the MotionDiv
2. **Click "Increment Counter"** - Should update the counter
3. **Hover over MotionDiv** - Should show hover effects
4. **Click MotionDiv directly** - Should trigger tap effects
5. **Check for console errors** - Should be no panics or errors

## 📊 **Test Results Template**

### **Compilation Results**
- [ ] Native Test: ✅/❌
- [ ] CSR Demo: ✅/❌  
- [ ] SSR Demo: ✅/❌

### **Runtime Results**
- [ ] Native Test runs: ✅/❌
- [ ] CSR Demo runs: ✅/❌
- [ ] SSR Demo runs: ✅/❌
- [ ] Animations work: ✅/❌
- [ ] Interactions work: ✅/❌
- [ ] No console errors: ✅/❌

## 🎯 **What to Look For**

### **✅ Success Signs**
- Smooth animations
- Responsive interactions
- No console errors
- Clean UI rendering

### **❌ Problem Signs**
- Console panics about SystemTime
- RefCell borrowing errors
- Animation not working
- UI not rendering

## 📝 **Report Results**

Please report:
1. **Compilation status** for each demo
2. **Runtime behavior** (animations, interactions)
3. **Any console errors** or panics
4. **Overall assessment** of MotionDiv functionality

This will help us determine if MotionDiv works in native mode and plan next steps!
