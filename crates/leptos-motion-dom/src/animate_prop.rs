use leptos::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

use crate::AnimationValue;

/// Flexible animation property that accepts both static and reactive values
#[derive(Clone)]
pub enum AnimateProp {
    Static(HashMap<String, AnimationValue>),
    Reactive(Signal<HashMap<String, AnimationValue>>),
    Derived(Memo<HashMap<String, AnimationValue>>),
    Fn(Rc<dyn Fn() -> HashMap<String, AnimationValue>>),
    Variants(crate::variants::Variants, String), // (variants, variant_name)
}

/// Extension trait for automatic conversion
pub trait IntoAnimateProp {
    fn into_animate_prop(self) -> AnimateProp;
}

impl IntoAnimateProp for HashMap<String, AnimationValue> {
    fn into_animate_prop(self) -> AnimateProp {
        AnimateProp::Static(self)
    }
}

impl IntoAnimateProp for Signal<HashMap<String, AnimationValue>> {
    fn into_animate_prop(self) -> AnimateProp {
        AnimateProp::Reactive(self)
    }
}

impl IntoAnimateProp for leptos::prelude::ReadSignal<HashMap<String, AnimationValue>> {
    fn into_animate_prop(self) -> AnimateProp {
        AnimateProp::Reactive(self.into())
    }
}

impl IntoAnimateProp for Memo<HashMap<String, AnimationValue>> {
    fn into_animate_prop(self) -> AnimateProp {
        AnimateProp::Derived(self)
    }
}

impl<F> IntoAnimateProp for F 
where 
    F: Fn() -> HashMap<String, AnimationValue> + 'static
{
    fn into_animate_prop(self) -> AnimateProp {
        AnimateProp::Fn(Rc::new(self))
    }
}

impl AnimateProp {
    /// Resolve the animation property to its current values
    pub fn resolve(&self) -> HashMap<String, AnimationValue> {
        match self {
            AnimateProp::Static(map) => map.clone(),
            AnimateProp::Reactive(signal) => signal.get(),
            AnimateProp::Derived(memo) => memo.get(),
            AnimateProp::Fn(f) => f(),
            AnimateProp::Variants(variants, variant_name) => {
                variants.get(variant_name).cloned().unwrap_or_default()
            },
        }
    }
    
    /// Check if this property is reactive (will change over time)
    pub fn is_reactive(&self) -> bool {
        match self {
            AnimateProp::Static(_) | AnimateProp::Variants(_, _) => false,
            AnimateProp::Reactive(_) | AnimateProp::Derived(_) | AnimateProp::Fn(_) => true,
        }
    }
}

/// Helper function to resolve an optional AnimateProp
pub fn resolve_animate_prop(prop: &Option<AnimateProp>) -> HashMap<String, AnimationValue> {
    prop.as_ref()
        .map(|p| p.resolve())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_static_animate_prop() {
        let mut values = HashMap::new();
        values.insert("opacity".to_string(), AnimationValue::Number(1.0));
        
        let prop = values.into_animate_prop();
        assert!(!prop.is_reactive());
        assert_eq!(prop.resolve()["opacity"], AnimationValue::Number(1.0));
    }
    
    #[test]
    fn test_reactive_animate_prop() {
        let (signal, set_signal) = signal(HashMap::new());
        let prop = signal.into_animate_prop();
        
        assert!(prop.is_reactive());
        
        let mut values = HashMap::new();
        values.insert("opacity".to_string(), AnimationValue::Number(0.5));
        set_signal.set(values);
        
        assert_eq!(prop.resolve()["opacity"], AnimationValue::Number(0.5));
    }
}
