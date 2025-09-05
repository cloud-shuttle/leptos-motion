# CSS Class Animation Showcase

This example demonstrates the new CSS class-based animation system in Leptos Motion, showcasing how to create performant animations using CSS classes instead of complex JavaScript animations.

## Features

- **CSS Class-Based Animations**: Uses CSS keyframes for better performance
- **Interactive Showcase**: Select different animation types and see them in action
- **Tailwind CSS Integration**: Beautiful, responsive design with Tailwind CSS
- **Real-time Animation**: Click to trigger animations and see immediate results
- **Multiple Animation Types**: Fade, slide, bounce, scale, and rotate animations

## Running the Example

1. **Build the example**:
   ```bash
   cd examples/css-class-showcase
   wasm-pack build --target web --dev
   ```

2. **Serve the files**:
   ```bash
   # Using Python
   python -m http.server 8000
   
   # Or using Node.js
   npx serve .
   
   # Or using any other static file server
   ```

3. **Open in browser**:
   Navigate to `http://localhost:8000` to see the showcase.

## How It Works

### CSS Animation Classes

The example defines several CSS animation classes:

```css
.fade-in {
    animation: fadeIn 0.6s ease-out;
}

.slide-in {
    animation: slideIn 0.8s ease-out;
}

.bounce-in {
    animation: bounceIn 1s ease-out;
}
```

### Leptos Integration

The Leptos components use conditional classes to trigger animations:

```rust
<div class="bg-white rounded-lg shadow-md p-6 border-2 transition-all duration-300"
     class:border-blue-500=is_active
     class:border-gray-200=move || !is_active()
>
```

### Animation Triggering

Animations are triggered by changing the `is_active` signal:

```rust
let start_animation = move |_| {
    set_is_animating(true);
    set_timeout(
        move || {
            set_is_animating(false);
        },
        std::time::Duration::from_millis(1000),
    );
};
```

## Benefits of CSS Class-Based Animations

1. **Performance**: CSS animations are hardware-accelerated and more performant than JavaScript animations
2. **Simplicity**: Easier to implement and maintain than complex JavaScript animation systems
3. **Consistency**: Works well with CSS frameworks like Tailwind CSS
4. **Accessibility**: Better support for reduced motion preferences
5. **Bundle Size**: Smaller JavaScript bundle since animations are handled by CSS

## Customization

You can easily customize the animations by:

1. **Modifying CSS keyframes**: Change the animation timing, easing, or effects
2. **Adding new animation types**: Create new keyframes and classes
3. **Integrating with Tailwind**: Use Tailwind's built-in animation classes
4. **Conditional animations**: Use Leptos signals to control when animations trigger

## Integration with Leptos Motion

This example demonstrates how the new CSS animation system integrates with Leptos Motion's core functionality, providing a bridge between simple CSS animations and the more advanced JavaScript-based animation system for users who need both approaches.
