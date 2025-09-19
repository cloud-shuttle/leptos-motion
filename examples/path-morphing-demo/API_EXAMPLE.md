# PathMorpher API - Simple and Intuitive

The `leptos-motion-studio` PathMorpher now has a much more intuitive API that handles all the complexity automatically.

## Simple One-Line Morphing

The easiest way to morph between two SVG paths:

```rust
use leptos_motion_studio::morphing::PathMorpher;

// Simple one-line morphing - handles everything automatically
let morphed_path = PathMorpher::morph(
    "M10 10 L20 10 L15 20 Z",  // source path
    "M5 5 L25 5 L15 25 Z",     // target path
    0.5                        // progress (0.0 to 1.0)
)?;

println!("Morphed path: {}", morphed_path.to_data());
```

## Advanced Usage (if needed)

For more control, you can still use the traditional approach:

```rust
// Create and prepare a morpher (now automatic)
let morpher = PathMorpher::new(source_path, target_path)?;

// Interpolate at any progress value
let result = morpher.interpolate(0.3)?;
```

## What Changed

### Before (Complex):
```rust
// Old way - required multiple steps and error handling
let mut morpher = PathMorpher::new(source, target)?;
morpher.prepare()?;  // This step was confusing
let result = morpher.interpolate(progress)?;
```

### After (Simple):
```rust
// New way - one line does everything
let result = PathMorpher::morph(source, target, progress)?;
```

## Benefits

1. **One-line morphing** - No need to understand preparation steps
2. **Automatic setup** - All complexity is handled internally
3. **Better error messages** - Clearer feedback when things go wrong
4. **Backward compatible** - Old API still works for advanced use cases

## Real Example

Here's how the demo now works:

```rust
// In the demo, morphing is now just:
match PathMorpher::morph(&paths[current_idx], &paths[next_idx], progress) {
    Ok(interpolated_path) => interpolated_path.to_data(),
    Err(e) => {
        // Fallback to simple switching
        if progress < 0.5 { paths[current_idx] } else { paths[next_idx] }
    }
}
```

The library is now much more intuitive to use!
