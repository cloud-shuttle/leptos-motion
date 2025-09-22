# Macro System Component Design

## Overview
Production-ready procedural macros for generating motion components and simplifying animation API usage.

## Current Issues
- **CRITICAL**: `#[derive(MotionComponent)]` implements non-existent trait
- `create_motion_elements` returns empty token stream
- `motion_target` builds invalid `AnimationTarget::new()` calls
- All macro implementations are placeholders

## Design Goals
- Type-safe animation component generation
- Simplified API for common animation patterns
- Compile-time validation of animation properties
- Zero runtime overhead
- Clear error messages for invalid usage

## API Design

### Core Macros
```rust
// Derive macro for creating motion components
#[derive(MotionComponent)]
#[motion(animate = "x, y, opacity", transition = "spring")]
struct FadeInBox {
    initial: MotionState,
    animate: MotionState,
    exit: Option<MotionState>,
}

// Function-like macro for creating motion elements
motion_div! {
    initial: { x: -100, opacity: 0 },
    animate: { x: 0, opacity: 1 },
    transition: spring(0.8),
    children: {
        h1 { "Hello Animation!" }
    }
}

// Target selection macro
motion_target!(
    ".my-class" => animate: { scale: 1.2 },
    "#my-id" => animate: { rotate: 360 }
)
```

### Generated Code
```rust
impl MotionComponent for FadeInBox {
    fn create_animation(&self) -> AnimationConfig {
        AnimationConfig::builder()
            .with_properties(vec!["x", "y", "opacity"])
            .with_initial_state(&self.initial)
            .with_animate_state(&self.animate)
            .with_transition(Transition::spring())
            .build()
    }
    
    fn render(&self, cx: Scope) -> impl IntoView {
        view! { cx,
            div(motion_props=self.create_animation()) {
                (self.children)
            }
        }
    }
}
```

## Implementation Plan

### Phase 1: MotionComponent Derive (Week 3, Day 1-2)
**File**: `crates/leptos-motion-macros/src/motion_component.rs`
**Target Lines**: <200

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, Lit};

pub fn derive_motion_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    // Parse motion attributes
    let motion_attrs = parse_motion_attributes(&input.attrs)?;
    
    // Generate MotionComponent implementation
    let expanded = quote! {
        impl MotionComponent for #name {
            fn create_animation(&self) -> AnimationConfig {
                let mut builder = AnimationConfig::builder();
                
                // Add properties from attributes
                #(builder = builder.with_property(#motion_attrs.properties);)*
                
                // Set transition if specified
                if let Some(transition) = &#motion_attrs.transition {
                    builder = builder.with_transition(transition.clone());
                }
                
                builder.build()
            }
            
            fn render(&self, cx: leptos::Scope) -> impl leptos::IntoView {
                use leptos::*;
                
                view! { cx,
                    div(
                        class=self.class.clone().unwrap_or_default(),
                        style=self.style.clone().unwrap_or_default()
                    ) {
                        (self.children.clone())
                    }
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[derive(Debug)]
struct MotionAttributes {
    properties: Vec<String>,
    transition: Option<syn::Expr>,
    initial: Option<syn::Expr>,
    animate: Option<syn::Expr>,
}
```

### Phase 2: Motion Element Macro (Week 3, Day 3)
**File**: `crates/leptos-motion-macros/src/motion_elements.rs`
**Target Lines**: <150

```rust
pub fn create_motion_elements(input: TokenStream) -> TokenStream {
    let motion_config = parse_macro_input!(input as MotionElementConfig);
    
    let initial = motion_config.initial.unwrap_or_else(|| quote! { MotionState::default() });
    let animate = motion_config.animate;
    let transition = motion_config.transition.unwrap_or_else(|| quote! { Transition::default() });
    let children = motion_config.children;
    let element_type = motion_config.element.unwrap_or_else(|| quote! { div });
    
    let expanded = quote! {
        {
            let animation_config = AnimationConfig::builder()
                .with_initial(#initial)
                .with_animate(#animate)
                .with_transition(#transition)
                .build();
            
            view! { cx,
                #element_type(
                    motion_config=animation_config,
                    ref=motion_ref
                ) {
                    #children
                }
            }
        }
    };
    
    TokenStream::from(expanded)
}

#[derive(Parse)]
struct MotionElementConfig {
    #[parse(if = peek_keyword("initial"))]
    initial: Option<syn::Expr>,
    
    #[parse(if = peek_keyword("animate"))]
    animate: syn::Expr,
    
    #[parse(if = peek_keyword("transition"))]
    transition: Option<syn::Expr>,
    
    #[parse(if = peek_keyword("children"))]
    children: Option<syn::Block>,
    
    #[parse(if = peek_keyword("element"))]
    element: Option<syn::Ident>,
}
```

### Phase 3: Motion Target Macro (Week 3, Day 4)
**File**: `crates/leptos-motion-macros/src/motion_target.rs`
**Target Lines**: <100

```rust
pub fn motion_target(input: TokenStream) -> TokenStream {
    let targets = parse_macro_input!(input as MotionTargets);
    
    let target_configs: Vec<_> = targets.rules.iter().map(|rule| {
        let selector = &rule.selector;
        let config = &rule.config;
        
        quote! {
            (#selector.to_string(), #config)
        }
    }).collect();
    
    let expanded = quote! {
        {
            let mut target_map = std::collections::HashMap::new();
            #(target_map.insert #target_configs;)*
            AnimationTargetMap::new(target_map)
        }
    };
    
    TokenStream::from(expanded)
}

#[derive(Parse)]
struct MotionTargets {
    rules: Vec<TargetRule>,
}

#[derive(Parse)]
struct TargetRule {
    selector: syn::LitStr,
    _arrow: syn::Token![=>],
    config: syn::Expr,
}
```

## File Structure
```
crates/leptos-motion-macros/src/
├── lib.rs                    # Main macro exports (<100 lines)
├── motion_component.rs       # MotionComponent derive (<200 lines)
├── motion_elements.rs        # Element creation macros (<150 lines)
├── motion_target.rs          # Target selection (<100 lines)
├── parsing/
│   ├── attributes.rs         # Attribute parsing (<100 lines)
│   ├── expressions.rs        # Expression parsing (<80 lines)
│   └── validation.rs         # Compile-time validation (<120 lines)
└── codegen/
    ├── component_gen.rs      # Component code generation (<150 lines)
    └── view_gen.rs          # View code generation (<100 lines)
```

## Trait Definitions

### MotionComponent Trait (in leptos-motion core)
**File**: `crates/leptos-motion/src/traits.rs`
**Target Lines**: <50

```rust
use leptos::*;

pub trait MotionComponent {
    fn create_animation(&self) -> AnimationConfig;
    fn render(&self, cx: Scope) -> impl IntoView;
    
    fn with_class(self, class: impl Into<String>) -> Self 
    where Self: Sized;
    
    fn with_style(self, style: impl Into<String>) -> Self 
    where Self: Sized;
}

pub trait AnimationTargetMap {
    fn new(targets: HashMap<String, AnimationConfig>) -> Self;
    fn apply_to_elements(&self, cx: Scope) -> Result<(), AnimationError>;
}
```

## Usage Examples

### Basic Component
```rust
#[derive(MotionComponent)]
#[motion(
    initial = "{ opacity: 0, scale: 0.8 }",
    animate = "{ opacity: 1, scale: 1 }",
    transition = "spring(mass: 0.8, stiffness: 100)"
)]
struct FadeInCard {
    title: String,
    content: String,
}

// Generated usage
let card = FadeInCard {
    title: "Hello".to_string(),
    content: "World".to_string(),
};
```

### Motion Elements
```rust
fn my_component(cx: Scope) -> impl IntoView {
    motion_div! {
        initial: { x: -50, opacity: 0 },
        animate: { x: 0, opacity: 1 },
        transition: spring(0.6),
        class: "my-animated-div",
        children: {
            h1 { "Animated Title" }
            p { "This content slides in smoothly" }
        }
    }
}
```

### Target-Based Animation
```rust
fn animate_existing_elements(cx: Scope) {
    motion_target! {
        ".card" => animate: { y: -20, opacity: 1 },
        ".button" => animate: { scale: 1.05 },
        "#header" => animate: { 
            background_color: "#ff6b6b",
            color: "#ffffff"
        }
    }.apply_to_elements(cx)?;
}
```

## Error Handling

### Compile-Time Validation
```rust
// In validation.rs
pub fn validate_motion_properties(properties: &[String]) -> syn::Result<()> {
    for property in properties {
        if !is_valid_css_property(property) {
            return Err(syn::Error::new_spanned(
                property,
                format!("Invalid CSS property: {}", property)
            ));
        }
    }
    Ok(())
}

pub fn validate_transition_syntax(transition: &syn::Expr) -> syn::Result<()> {
    // Validate transition expressions at compile time
}
```

### Runtime Error Handling
```rust
impl MotionComponent for GeneratedComponent {
    fn create_animation(&self) -> AnimationConfig {
        AnimationConfig::builder()
            .with_validation() // Enable runtime validation
            .build()
            .unwrap_or_else(|e| {
                leptos::logging::error!("Animation config error: {}", e);
                AnimationConfig::default()
            })
    }
}
```

## Testing Strategy
- Macro expansion tests using `macrotest`
- Generated code compilation tests
- Runtime behavior tests
- Error message quality tests
- Performance benchmarks for macro expansion

## Dependencies
```rust
[dependencies]
proc-macro2 = "1.0"
quote = "1.0"
syn = { version = "2.0", features = ["full", "parsing"] }
darling = "0.20" # For easier attribute parsing
```

## Success Criteria
- [ ] All placeholder implementations replaced
- [ ] MotionComponent trait properly defined
- [ ] Derive macro generates valid code
- [ ] Element macros create proper Leptos views
- [ ] Target macros build valid HashMap
- [ ] Compile-time validation working
- [ ] Clear error messages
- [ ] All files under 200 lines
- [ ] Performance acceptable (<10ms macro expansion)
