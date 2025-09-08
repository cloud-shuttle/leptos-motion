//! Leptos v0.8 Compatibility Layer
//!
//! This module provides trait implementations to make leptos-motion compatible
//! with Leptos v0.8's attribute system.

use leptos::prelude::*;
use leptos_motion_core::*;
use std::collections::HashMap;

/// Implement IntoClass for AnimationTarget to work with Leptos v0.8 class attributes
impl IntoClass for AnimationTarget {
    fn into_class(self) -> Class {
        // Convert animation target to CSS class string
        let class_string = self
            .iter()
            .map(|(property, value)| match (property.as_str(), value) {
                ("opacity", AnimationValue::Number(n)) => {
                    format!("opacity-{}", (*n * 100.0) as u8)
                }
                ("scale", AnimationValue::Number(n)) => {
                    format!("scale-{}", (*n * 100.0) as u8)
                }
                ("rotate", AnimationValue::Degrees(d)) => {
                    format!("rotate-{}", *d as i16)
                }
                ("translateX", AnimationValue::Pixels(p)) => {
                    format!("translate-x-{}", *p as i16)
                }
                ("translateY", AnimationValue::Pixels(p)) => {
                    format!("translate-y-{}", *p as i16)
                }
                ("width", AnimationValue::Pixels(p)) => {
                    format!("w-{}", *p as u16)
                }
                ("height", AnimationValue::Pixels(p)) => {
                    format!("h-{}", *p as u16)
                }
                ("backgroundColor", AnimationValue::Color(c)) => {
                    format!("bg-{}", c)
                }
                ("color", AnimationValue::Color(c)) => {
                    format!("text-{}", c)
                }
                _ => format!("{}-{}", property, value.to_string()),
            })
            .collect::<Vec<_>>()
            .join(" ");

        Class::new(class_string)
    }
}

/// Implement IntoClass for Option<AnimationTarget>
impl IntoClass for Option<AnimationTarget> {
    fn into_class(self) -> Class {
        match self {
            Some(target) => target.into_class(),
            None => Class::new(""),
        }
    }
}

/// Implement IntoAttributeValue for AnimationValue to work with Leptos v0.8 attributes
impl IntoAttributeValue for AnimationValue {
    fn into_attribute_value(self) -> AttributeValue {
        match self {
            AnimationValue::Number(n) => AttributeValue::String(n.to_string()),
            AnimationValue::Pixels(p) => AttributeValue::String(format!("{}px", p)),
            AnimationValue::Percentage(p) => AttributeValue::String(format!("{}%", p)),
            AnimationValue::Degrees(d) => AttributeValue::String(format!("{}deg", d)),
            AnimationValue::Radians(r) => AttributeValue::String(format!("{}rad", r)),
            AnimationValue::Color(c) => AttributeValue::String(c),
            AnimationValue::String(s) => AttributeValue::String(s),
            AnimationValue::Transform(t) => AttributeValue::String(t.to_css_string()),
            AnimationValue::Complex(c) => AttributeValue::String(match &c.data {
                #[cfg(feature = "serde-support")]
                data => serde_json::to_string(data).unwrap_or_default(),
                #[cfg(not(feature = "serde-support"))]
                data => data.clone(),
            }),
        }
    }
}

/// Implement IntoAttributeValue for Option<AnimationValue>
impl IntoAttributeValue for Option<AnimationValue> {
    fn into_attribute_value(self) -> AttributeValue {
        match self {
            Some(value) => value.into_attribute_value(),
            None => AttributeValue::String("".to_string()),
        }
    }
}

/// Implement IntoProperty for Transition to work with Leptos v0.8 properties
impl IntoProperty for Transition {
    fn into_property(self) -> Property {
        let mut properties = Vec::new();

        if let Some(duration) = self.duration {
            properties.push(format!("duration: {}s", duration));
        }

        if let Some(delay) = self.delay {
            properties.push(format!("delay: {}s", delay));
        }

        let ease_string = match self.ease {
            Easing::Linear => "linear",
            Easing::EaseIn => "ease-in",
            Easing::EaseOut => "ease-out",
            Easing::EaseInOut => "ease-in-out",
            Easing::Spring(config) => {
                format!(
                    "cubic-bezier({}, {}, {}, {})",
                    config.stiffness / 1000.0,
                    config.damping / 100.0,
                    config.mass / 10.0,
                    config.velocity / 100.0
                )
            }
            Easing::Bezier(x1, y1, x2, y2) => {
                format!("cubic-bezier({}, {}, {}, {})", x1, y1, x2, y2)
            }
            Easing::Custom(name) => &name,
        };
        properties.push(format!("ease: {}", ease_string));

        match self.repeat {
            RepeatConfig::Never => {}
            RepeatConfig::Count(n) => {
                properties.push(format!("iteration-count: {}", n));
            }
            RepeatConfig::Infinite => {
                properties.push("iteration-count: infinite".to_string());
            }
            RepeatConfig::InfiniteReverse => {
                properties.push("iteration-count: infinite".to_string());
                properties.push("direction: alternate".to_string());
            }
        }

        if let Some(stagger) = self.stagger {
            properties.push(format!("stagger: {}s", stagger.delay));
        }

        Property::new(properties.join("; "))
    }
}

/// Implement IntoProperty for Option<Transition>
impl IntoProperty for Option<Transition> {
    fn into_property(self) -> Property {
        match self {
            Some(transition) => transition.into_property(),
            None => Property::new(""),
        }
    }
}

/// Implement IntoProperty for AnimationTarget to work with style attributes
impl IntoProperty for AnimationTarget {
    fn into_property(self) -> Property {
        let style_string = self
            .iter()
            .map(|(property, value)| {
                let css_property = match property.as_str() {
                    "opacity" => "opacity",
                    "scale" => "transform",
                    "rotate" => "transform",
                    "translateX" => "transform",
                    "translateY" => "transform",
                    "translateZ" => "transform",
                    "scaleX" => "transform",
                    "scaleY" => "transform",
                    "scaleZ" => "transform",
                    "rotateX" => "transform",
                    "rotateY" => "transform",
                    "rotateZ" => "transform",
                    "skewX" => "transform",
                    "skewY" => "transform",
                    "width" => "width",
                    "height" => "height",
                    "backgroundColor" => "background-color",
                    "color" => "color",
                    "borderRadius" => "border-radius",
                    "margin" => "margin",
                    "padding" => "padding",
                    "top" => "top",
                    "left" => "left",
                    "right" => "right",
                    "bottom" => "bottom",
                    _ => property,
                };

                let css_value = match value {
                    AnimationValue::Number(n) => n.to_string(),
                    AnimationValue::Pixels(p) => format!("{}px", p),
                    AnimationValue::Percentage(p) => format!("{}%", p),
                    AnimationValue::Degrees(d) => format!("{}deg", d),
                    AnimationValue::Radians(r) => format!("{}rad", r),
                    AnimationValue::Color(c) => c.clone(),
                    AnimationValue::String(s) => s.clone(),
                    AnimationValue::Transform(t) => t.to_css_string(),
                    AnimationValue::Complex(c) => match &c.data {
                        #[cfg(feature = "serde-support")]
                        data => serde_json::to_string(data).unwrap_or_default(),
                        #[cfg(not(feature = "serde-support"))]
                        data => data.clone(),
                    },
                };

                format!("{}: {}", css_property, css_value)
            })
            .collect::<Vec<_>>()
            .join("; ");

        Property::new(style_string)
    }
}

/// Implement IntoProperty for Option<AnimationTarget>
impl IntoProperty for Option<AnimationTarget> {
    fn into_property(self) -> Property {
        match self {
            Some(target) => target.into_property(),
            None => Property::new(""),
        }
    }
}

/// Implement IntoClass for Easing to work with CSS classes
impl IntoClass for Easing {
    fn into_class(self) -> Class {
        let class_name = match self {
            Easing::Linear => "ease-linear",
            Easing::EaseIn => "ease-in",
            Easing::EaseOut => "ease-out",
            Easing::EaseInOut => "ease-in-out",
            Easing::Spring(_) => "ease-spring",
            Easing::Bezier(_, _, _, _) => "ease-bezier",
            Easing::Custom(name) => return Class::new(format!("ease-{}", name)),
        };
        Class::new(class_name)
    }
}

/// Implement IntoClass for SpringConfig
impl IntoClass for SpringConfig {
    fn into_class(self) -> Class {
        let stiffness_class = format!("spring-stiffness-{}", (self.stiffness / 10.0) as u8);
        let damping_class = format!("spring-damping-{}", (self.damping / 10.0) as u8);
        let mass_class = format!("spring-mass-{}", (self.mass * 10.0) as u8);

        Class::new(format!(
            "{} {} {}",
            stiffness_class, damping_class, mass_class
        ))
    }
}

/// Extension trait for Transform to provide CSS string conversion
pub trait TransformExt {
    fn to_css_string(&self) -> String;
}

impl TransformExt for Transform {
    fn to_css_string(&self) -> String {
        let mut transforms = Vec::new();

        if let Some(x) = self.x {
            transforms.push(format!("translateX({}px)", x));
        }
        if let Some(y) = self.y {
            transforms.push(format!("translateY({}px)", y));
        }
        if let Some(z) = self.z {
            transforms.push(format!("translateZ({}px)", z));
        }
        if let Some(rotate_x) = self.rotate_x {
            transforms.push(format!("rotateX({}deg)", rotate_x));
        }
        if let Some(rotate_y) = self.rotate_y {
            transforms.push(format!("rotateY({}deg)", rotate_y));
        }
        if let Some(rotate_z) = self.rotate_z {
            transforms.push(format!("rotateZ({}deg)", rotate_z));
        }
        if let Some(scale) = self.scale {
            transforms.push(format!("scale({})", scale));
        }
        if let Some(scale_x) = self.scale_x {
            transforms.push(format!("scaleX({})", scale_x));
        }
        if let Some(scale_y) = self.scale_y {
            transforms.push(format!("scaleY({})", scale_y));
        }
        if let Some(skew_x) = self.skew_x {
            transforms.push(format!("skewX({}deg)", skew_x));
        }
        if let Some(skew_y) = self.skew_y {
            transforms.push(format!("skewY({}deg)", skew_y));
        }

        transforms.join(" ")
    }
}

/// Extension trait for ComplexValue to provide string conversion
pub trait ComplexValueExt {
    fn to_string(&self) -> String;
}

impl ComplexValueExt for ComplexValue {
    fn to_string(&self) -> String {
        match &self.data {
            #[cfg(feature = "serde-support")]
            serde_json::Value::String(s) => s.clone(),
            #[cfg(feature = "serde-support")]
            _ => serde_json::to_string(&self.data).unwrap_or_default(),
            #[cfg(not(feature = "serde-support"))]
            data => data.clone(),
        }
    }
}

/// Extension trait for AnimationValue to provide string conversion
pub trait AnimationValueExt {
    fn to_string_value(&self) -> String;
}

impl AnimationValueExt for AnimationValue {
    fn to_string_value(&self) -> String {
        match self {
            AnimationValue::Number(n) => n.to_string(),
            AnimationValue::Pixels(p) => format!("{}px", p),
            AnimationValue::Percentage(p) => format!("{}%", p),
            AnimationValue::Degrees(d) => format!("{}deg", d),
            AnimationValue::Radians(r) => format!("{}rad", r),
            AnimationValue::Color(c) => c.clone(),
            AnimationValue::String(s) => s.clone(),
            AnimationValue::Transform(t) => t.to_css_string(),
            AnimationValue::Complex(c) => match &c.data {
                #[cfg(feature = "serde-support")]
                data => serde_json::to_string(data).unwrap_or_default(),
                #[cfg(not(feature = "serde-support"))]
                data => data.clone(),
            },
        }
    }
}
