# 🚀 Quick Start Guide - Leptos Motion Capability Showcase

## 🎯 What You'll See

This showcase demonstrates **all the major capabilities** of Leptos Motion v0.6.0:

- **🎨 Basic Animations** - Scale, rotate, fade, and color transitions
- **🖱️ Gesture Interactions** - Hover, tap, and drag interactions
- **📱 Layout Animations** - FLIP-based smooth layout transitions
- **🎬 Keyframe Animations** - Multi-step animations with precise control
- **⚡ Stagger Animations** - Sequential animations with configurable delays
- **🎯 Drag Constraints** - Axis and boundary constraints with elastic behavior
- **🚀 Performance Demo** - Multiple concurrent animations at 60fps
- **🔧 Advanced Features** - Spring physics, 3D transforms, and complex easing

## 🛠️ Running the Showcase

### Option 1: Python Server (Recommended)

```bash
# From the showcase directory
python3 serve.py
```

This will:

- Start a local server at `http://localhost:8000`
- Automatically open your browser
- Serve the showcase with proper CORS headers

### Option 2: Manual Server

```bash
# From the showcase directory
python3 -m http.server 8000
# Then open http://localhost:8000 in your browser
```

### Option 3: Other Servers

Any HTTP server will work. Just serve the files from this directory.

## 🔧 Building from Source

If you want to modify the showcase:

```bash
# Install wasm-pack if you haven't already
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the showcase
wasm-pack build --target web --out-dir pkg

# Then serve as above
python3 serve.py
```

## 📱 Browser Compatibility

The showcase works in all modern browsers:

- ✅ Chrome 80+
- ✅ Firefox 75+
- ✅ Safari 13+
- ✅ Edge 80+

## 🎮 Interactive Features

### Basic Animations

- Click "Hide/Show" to toggle visibility
- Click "Mode 1/2/3" to switch animation types

### Gesture Interactions

- Hover over the element to see hover effects
- Click to see tap animations
- Watch the tap counter increment

### Layout Animations

- Click "Add Item" to add new elements
- Click "Remove Item" to remove elements
- Click "Shuffle" to reorder elements

### Keyframe Animations

- Click "Play/Stop" to control the animation

### Stagger Animations

- Click "Show/Hide Staggered" to see sequential animations

### Drag Constraints

- Drag the element around
- Click the mode button to change constraints
- Try "Free Drag", "X Only", and "Constrained" modes

### Performance Demo

- Click "Start/Stop" to control animations
- Click "Count" to change the number of concurrent animations
- Watch the smooth 60fps performance

### Advanced Features

- Click the mode button to see different advanced animations
- Try "Spring Physics", "Color Transitions", and "3D Transforms"

## 🎯 Key Takeaways

This showcase demonstrates that Leptos Motion is:

1. **Production Ready** - All features work smoothly and reliably
2. **High Performance** - 60fps with 100+ concurrent animations
3. **Developer Friendly** - Clean, intuitive API
4. **Feature Complete** - Comprehensive animation capabilities
5. **Well Tested** - 500+ tests ensure reliability

## 🔗 Next Steps

- **Try the examples** in the `examples/` directory
- **Read the documentation** at [docs.rs/leptos-motion](https://docs.rs/leptos-motion)
- **Check out the source** on [GitHub](https://github.com/cloud-shuttle/leptos-motion)
- **Start building** your own animations!

## 🆘 Troubleshooting

### "pkg directory not found"

Run `wasm-pack build --target web --out-dir pkg` first

### "Module not found" errors

Make sure you're serving from the showcase directory with an HTTP server

### Animations not working

Check browser console for errors and ensure you're using a modern browser

### Performance issues

Try reducing the animation count in the Performance Demo section
