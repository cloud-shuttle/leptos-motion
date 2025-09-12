# TDD Examples

## Basic TDD Example

### Step 1: Red - Write Failing Test
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_creation() {
        let animation = Animation::new("fade-in");
        assert_eq!(animation.name(), "fade-in");
    }
}
```

### Step 2: Green - Write Minimal Code
```rust
pub struct Animation {
    name: String,
}

impl Animation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
}
```

### Step 3: Refactor - Improve Code
```rust
pub struct Animation {
    name: String,
    duration: f64,
}

impl Animation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            duration: 1.0, // Default duration
        }
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn duration(&self) -> f64 {
        self.duration
    }
}
```
