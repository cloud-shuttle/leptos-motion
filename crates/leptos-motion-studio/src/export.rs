//! Export functionality for Motion Studio animations

use crate::{
    Result, StudioError, project::StudioProject, timeline::Timeline3D, transforms::Transform3D,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Export formats supported by Motion Studio
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExportFormat {
    /// CSS animations and transitions
    CSS,
    /// Web Animations API (WAAPI) JavaScript
    WAAPI,
    /// Leptos Motion code
    LeptosMotion,
    /// Framer Motion code
    FramerMotion,
    /// GSAP JavaScript
    GSAP,
    /// Lottie JSON
    Lottie,
    /// SVG animations
    SVGAnimate,
    /// Video export (WebM/MP4)
    Video(VideoFormat),
}

/// Video export formats
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoFormat {
    WebM,
    MP4,
    GIF,
}

/// Export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportConfig {
    /// Target format
    pub format: ExportFormat,
    /// Output settings
    pub settings: ExportSettings,
    /// Optimization level
    pub optimization: OptimizationLevel,
    /// Include source comments
    pub include_comments: bool,
    /// Minify output
    pub minify: bool,
}

/// Format-specific export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportSettings {
    CSS(CSSSettings),
    JavaScript(JavaScriptSettings),
    Video(VideoSettings),
    SVG(SVGSettings),
    Lottie(LottieSettings),
}

/// CSS export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CSSSettings {
    /// Use CSS custom properties
    pub use_custom_properties: bool,
    /// Target CSS version
    pub css_version: CSSVersion,
    /// Include vendor prefixes
    pub vendor_prefixes: bool,
    /// Animation fill mode
    pub fill_mode: String,
}

/// JavaScript export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaScriptSettings {
    /// Target ES version
    pub es_version: ESVersion,
    /// Module format
    pub module_format: ModuleFormat,
    /// Include TypeScript definitions
    pub typescript_definitions: bool,
}

/// Video export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoSettings {
    /// Video width
    pub width: u32,
    /// Video height
    pub height: u32,
    /// Framerate
    pub fps: f32,
    /// Video quality (0.0 to 1.0)
    pub quality: f32,
    /// Video duration override
    pub duration_override: Option<f32>,
}

/// SVG export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SVGSettings {
    /// SVG viewBox
    pub viewbox: (f32, f32, f32, f32),
    /// Optimize SVG output
    pub optimize: bool,
    /// Include animation timing
    pub include_timing: bool,
}

/// Lottie export settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LottieSettings {
    /// Lottie version
    pub version: String,
    /// Frame rate
    pub frame_rate: f32,
    /// Optimize for file size
    pub optimize_size: bool,
}

/// CSS version targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CSSVersion {
    CSS3,
    Modern,
}

/// ECMAScript version targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ESVersion {
    ES5,
    ES2015,
    ES2018,
    ES2020,
    Latest,
}

/// JavaScript module formats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModuleFormat {
    CommonJS,
    ESModule,
    UMD,
    IIFE,
}

/// Optimization levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationLevel {
    None,
    Basic,
    Advanced,
    Maximum,
}

/// Animation exporter
#[derive(Debug)]
pub struct AnimationExporter<'a> {
    /// Project to export from
    project: &'a StudioProject,
    /// Export configuration
    config: ExportConfig,
}

impl<'a> AnimationExporter<'a> {
    /// Create new exporter
    pub fn new(project: &'a StudioProject) -> Self {
        Self {
            project,
            config: ExportConfig::default(),
        }
    }

    /// Create exporter with custom configuration
    pub fn with_config(project: &'a StudioProject, config: ExportConfig) -> Self {
        Self { project, config }
    }

    /// Get supported export formats
    pub fn supported_formats(&self) -> Vec<ExportFormat> {
        vec![
            ExportFormat::CSS,
            ExportFormat::WAAPI,
            ExportFormat::LeptosMotion,
            ExportFormat::FramerMotion,
            ExportFormat::GSAP,
            ExportFormat::SVGAnimate,
            ExportFormat::Lottie,
            ExportFormat::Video(VideoFormat::WebM),
            ExportFormat::Video(VideoFormat::MP4),
            ExportFormat::Video(VideoFormat::GIF),
        ]
    }

    /// Export project to specified format
    pub fn export(&self) -> Result<ExportResult> {
        match &self.config.format {
            ExportFormat::CSS => self.export_css(),
            ExportFormat::WAAPI => self.export_waapi(),
            ExportFormat::LeptosMotion => self.export_leptos_motion(),
            ExportFormat::FramerMotion => self.export_framer_motion(),
            ExportFormat::GSAP => self.export_gsap(),
            ExportFormat::SVGAnimate => self.export_svg_animate(),
            ExportFormat::Lottie => self.export_lottie(),
            ExportFormat::Video(format) => self.export_video(format),
        }
    }

    /// Export as CSS animations
    fn export_css(&self) -> Result<ExportResult> {
        let mut css_output = String::new();

        // Generate CSS for each animation
        for (id, animation) in &self.project.animations {
            if !animation.enabled {
                continue;
            }

            let animation_name = format!("motion-{}", animation.name.replace(' ', "-"));

            // Generate keyframes
            css_output.push_str(&format!("@keyframes {} {{\n", animation_name));

            if let Some(timeline) = &animation.timeline {
                let keyframes = timeline.keyframes();

                for keyframe in keyframes {
                    let time_percent = (keyframe.time / timeline.duration()) * 100.0;
                    css_output.push_str(&format!("  {:.1}% {{\n", time_percent));

                    // Convert animation value to CSS
                    let css_value = keyframe.value.to_css(&keyframe.property);
                    let css_property = self.property_to_css(&keyframe.property);
                    css_output.push_str(&format!("    {}: {};\n", css_property, css_value));

                    css_output.push_str("  }\n");
                }
            } else {
                // Generate from transforms
                for (i, transform) in animation.transforms.iter().enumerate() {
                    let progress = i as f32 / (animation.transforms.len() - 1).max(1) as f32;
                    let time_percent = progress * 100.0;

                    css_output.push_str(&format!("  {:.1}% {{\n", time_percent));
                    css_output.push_str(&format!("    transform: {};\n", transform.to_css()));
                    css_output.push_str("  }\n");
                }
            }

            css_output.push_str("}\n\n");

            // Generate animation class
            css_output.push_str(&format!(".{} {{\n", animation_name));
            css_output.push_str(&format!(
                "  animation: {} {}s ease-in-out;\n",
                animation_name, animation.duration
            ));
            css_output.push_str("}\n\n");
        }

        Ok(ExportResult {
            content: css_output,
            mime_type: "text/css".to_string(),
            file_extension: "css".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as Web Animations API
    fn export_waapi(&self) -> Result<ExportResult> {
        let mut js_output = String::new();

        js_output.push_str("// Generated by Motion Studio\n");
        js_output.push_str("class MotionAnimations {\n");

        for (id, animation) in &self.project.animations {
            if !animation.enabled {
                continue;
            }

            let method_name = animation.name.replace(' ', "_").to_lowercase();

            js_output.push_str(&format!("  static {}(element) {{\n", method_name));
            js_output.push_str("    const keyframes = [\n");

            if let Some(timeline) = &animation.timeline {
                let timeline_keyframes = timeline.keyframes();

                for keyframe in timeline_keyframes {
                    let offset = keyframe.time / timeline.duration();
                    js_output.push_str("      {\n");
                    js_output.push_str(&format!("        offset: {:.3},\n", offset));

                    let css_property = self.property_to_css(&keyframe.property);
                    let css_value = keyframe.value.to_css(&keyframe.property);
                    js_output.push_str(&format!("        {}: '{}',\n", css_property, css_value));
                    js_output.push_str("      },\n");
                }
            }

            js_output.push_str("    ];\n");
            js_output.push_str("    \n");
            js_output.push_str("    const options = {\n");
            js_output.push_str(&format!(
                "      duration: {},\n",
                animation.duration * 1000.0
            ));
            js_output.push_str("      easing: 'ease-in-out',\n");
            js_output.push_str("      fill: 'forwards'\n");
            js_output.push_str("    };\n");
            js_output.push_str("    \n");
            js_output.push_str("    return element.animate(keyframes, options);\n");
            js_output.push_str("  }\n\n");
        }

        js_output.push_str("}\n");
        js_output.push_str("\nexport { MotionAnimations };\n");

        Ok(ExportResult {
            content: js_output,
            mime_type: "text/javascript".to_string(),
            file_extension: "js".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as Leptos Motion code
    fn export_leptos_motion(&self) -> Result<ExportResult> {
        let mut rust_output = String::new();

        rust_output.push_str("use leptos::*;\n");
        rust_output.push_str("use leptos_motion::*;\n\n");

        for (id, animation) in &self.project.animations {
            if !animation.enabled {
                continue;
            }

            let component_name = animation.name.replace(' ', "");

            rust_output.push_str(&format!("#[component]\n"));
            rust_output.push_str(&format!(
                "pub fn {}() -> impl IntoView {{\n",
                component_name
            ));

            // Generate animation configuration
            rust_output.push_str("    let animation_config = AnimationConfig::builder()\n");
            rust_output.push_str(&format!("        .duration({})\n", animation.duration));
            rust_output.push_str("        .easing(EasingFn::EaseInOut)\n");
            rust_output.push_str("        .build();\n\n");

            // Generate component
            rust_output.push_str("    view! {\n");
            rust_output.push_str("        <MotionDiv\n");
            rust_output.push_str("            initial=Transform::default()\n");

            if !animation.transforms.is_empty() {
                let final_transform = animation.transforms.last().unwrap();
                rust_output.push_str(&format!("            animate=Transform::new()\n"));
                rust_output.push_str(&format!(
                    "                .translate({:.2}, {:.2}, {:.2})\n",
                    final_transform.translation.x,
                    final_transform.translation.y,
                    final_transform.translation.z
                ));
            }

            rust_output.push_str("            transition=animation_config\n");
            rust_output.push_str("        >\n");
            rust_output.push_str("            \"Animated Element\"\n");
            rust_output.push_str("        </MotionDiv>\n");
            rust_output.push_str("    }\n");
            rust_output.push_str("}\n\n");
        }

        Ok(ExportResult {
            content: rust_output,
            mime_type: "text/x-rust".to_string(),
            file_extension: "rs".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as Framer Motion code
    fn export_framer_motion(&self) -> Result<ExportResult> {
        let mut jsx_output = String::new();

        jsx_output.push_str("import { motion } from 'framer-motion';\n\n");

        for (id, animation) in &self.project.animations {
            if !animation.enabled {
                continue;
            }

            let component_name = animation.name.replace(' ', "");

            jsx_output.push_str(&format!("export const {} = () => {{\n", component_name));
            jsx_output.push_str("  const variants = {\n");
            jsx_output.push_str("    initial: {\n");

            if !animation.transforms.is_empty() {
                let initial_transform = &animation.transforms[0];
                jsx_output.push_str(&format!("      x: {},\n", initial_transform.translation.x));
                jsx_output.push_str(&format!("      y: {},\n", initial_transform.translation.y));
            }

            jsx_output.push_str("    },\n");
            jsx_output.push_str("    animate: {\n");

            if let Some(final_transform) = animation.transforms.last() {
                jsx_output.push_str(&format!("      x: {},\n", final_transform.translation.x));
                jsx_output.push_str(&format!("      y: {},\n", final_transform.translation.y));
            }

            jsx_output.push_str("    }\n");
            jsx_output.push_str("  };\n\n");

            jsx_output.push_str("  return (\n");
            jsx_output.push_str("    <motion.div\n");
            jsx_output.push_str("      variants={variants}\n");
            jsx_output.push_str("      initial=\"initial\"\n");
            jsx_output.push_str("      animate=\"animate\"\n");
            jsx_output.push_str(&format!(
                "      transition={{ duration: {} }}\n",
                animation.duration
            ));
            jsx_output.push_str("    >\n");
            jsx_output.push_str("      Animated Element\n");
            jsx_output.push_str("    </motion.div>\n");
            jsx_output.push_str("  );\n");
            jsx_output.push_str("};\n\n");
        }

        Ok(ExportResult {
            content: jsx_output,
            mime_type: "text/jsx".to_string(),
            file_extension: "jsx".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as GSAP code
    fn export_gsap(&self) -> Result<ExportResult> {
        let mut gsap_output = String::new();
        
        // Add GSAP imports
        gsap_output.push_str("import { gsap } from 'gsap';\n");
        gsap_output.push_str("import { ScrollTrigger } from 'gsap/ScrollTrigger';\n");
        gsap_output.push_str("import { MotionPathPlugin } from 'gsap/MotionPathPlugin';\n\n");
        
        // Register GSAP plugins
        gsap_output.push_str("// Register GSAP plugins\n");
        gsap_output.push_str("gsap.registerPlugin(ScrollTrigger, MotionPathPlugin);\n\n");
        
        // Generate GSAP timeline
        gsap_output.push_str("// Create main timeline\n");
        gsap_output.push_str("const tl = gsap.timeline();\n\n");
        
        // Export each animation
        for (id, animation) in &self.project.animations {
            if !animation.enabled {
                continue;
            }
            
            let animation_name = &animation.name;
            let target_selector = format!(".{}", animation_name.to_lowercase().replace(' ', "-"));
            
            gsap_output.push_str(&format!("// Animation: {}\n", animation_name));
            gsap_output.push_str(&format!("tl.to('{}', {{\n", target_selector));
            
            // Add animation properties
            if let Some(timeline) = &animation.timeline {
                let keyframes = timeline.keyframes();
                if !keyframes.is_empty() {
                    // Use the last keyframe as the target values
                    if let Some(final_keyframe) = keyframes.last() {
                        let css_property = self.property_to_css(&final_keyframe.property);
                        let css_value = self.value_to_gsap(&final_keyframe.value, &final_keyframe.property);
                        gsap_output.push_str(&format!("  {}: {},\n", css_property, css_value));
                    }
                }
            }
            
            // Add timing properties
            gsap_output.push_str(&format!("  duration: {},\n", animation.duration));
            gsap_output.push_str(&format!("  ease: '{}',\n", self.easing_to_gsap(&animation.easing)));
            
            // Add repeat and yoyo if configured
            if animation.repeat > 0 {
                gsap_output.push_str(&format!("  repeat: {},\n", animation.repeat));
            }
            if animation.yoyo {
                gsap_output.push_str("  yoyo: true,\n");
            }
            
            gsap_output.push_str("});\n\n");
        }
        
        // Add ScrollTrigger if needed
        if self.project.has_scroll_trigger {
            gsap_output.push_str("// ScrollTrigger setup\n");
            gsap_output.push_str("ScrollTrigger.create({\n");
            gsap_output.push_str("  trigger: '.container',\n");
            gsap_output.push_str("  start: 'top center',\n");
            gsap_output.push_str("  end: 'bottom center',\n");
            gsap_output.push_str("  animation: tl,\n");
            gsap_output.push_str("  scrub: 1,\n");
            gsap_output.push_str("});\n");
        }
        
        Ok(ExportResult {
            content: gsap_output,
            mime_type: "text/javascript".to_string(),
            file_extension: "js".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as SVG animations
    fn export_svg_animate(&self) -> Result<ExportResult> {
        // TODO: Implement SVG animate export
        Err(StudioError::ExportError(
            "SVG animate export not yet implemented".to_string(),
        ))
    }

    /// Export as Lottie JSON
    fn export_lottie(&self) -> Result<ExportResult> {
        // TODO: Implement Lottie export
        Err(StudioError::ExportError(
            "Lottie export not yet implemented".to_string(),
        ))
    }

    /// Export as video
    fn export_video(&self, format: &VideoFormat) -> Result<ExportResult> {
        // TODO: Implement video export
        Err(StudioError::ExportError(
            "Video export not yet implemented".to_string(),
        ))
    }

    /// Convert animation property to CSS property name
    fn property_to_css(&self, property: &crate::timeline::AnimationProperty) -> String {
        use crate::timeline::AnimationProperty;

        match property {
            AnimationProperty::TranslateX => "transform".to_string(),
            AnimationProperty::TranslateY => "transform".to_string(),
            AnimationProperty::TranslateZ => "transform".to_string(),
            AnimationProperty::Translation => "transform".to_string(),
            AnimationProperty::RotationX => "transform".to_string(),
            AnimationProperty::RotationY => "transform".to_string(),
            AnimationProperty::RotationZ => "transform".to_string(),
            AnimationProperty::Rotation => "transform".to_string(),
            AnimationProperty::ScaleX => "transform".to_string(),
            AnimationProperty::ScaleY => "transform".to_string(),
            AnimationProperty::ScaleZ => "transform".to_string(),
            AnimationProperty::Scale => "transform".to_string(),
            AnimationProperty::Opacity => "opacity".to_string(),
            AnimationProperty::Color => "color".to_string(),
            AnimationProperty::Custom(name) => name.clone(),
        }
    }
    
    /// Convert animation value to GSAP format
    fn value_to_gsap(&self, value: &crate::timeline::AnimationValue, property: &crate::timeline::AnimationProperty) -> String {
        use crate::timeline::{AnimationProperty, AnimationValue};
        match (value, property) {
            (AnimationValue::Number(n), AnimationProperty::TranslateX) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::TranslateY) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::TranslateZ) => format!("{}px", n),
            (AnimationValue::Number(n), AnimationProperty::ScaleX) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::ScaleY) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::ScaleZ) => n.to_string(),
            (AnimationValue::Number(n), AnimationProperty::RotationX) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::RotationY) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::RotationZ) => format!("{}deg", n),
            (AnimationValue::Number(n), AnimationProperty::Opacity) => n.to_string(),
            (AnimationValue::String(s), _) => format!("'{}'", s),
            _ => "0".to_string(),
        }
    }
    
    /// Convert easing to GSAP format
    fn easing_to_gsap(&self, easing: &leptos_motion_core::Easing) -> String {
        use leptos_motion_core::Easing;
        match easing {
            Easing::Linear => "none".to_string(),
            Easing::EaseIn => "power2.in".to_string(),
            Easing::EaseOut => "power2.out".to_string(),
            Easing::EaseInOut => "power2.inOut".to_string(),
            Easing::EaseInCubic => "power3.in".to_string(),
            Easing::EaseOutCubic => "power3.out".to_string(),
            Easing::EaseInOutCubic => "power3.inOut".to_string(),
            Easing::EaseInQuart => "power4.in".to_string(),
            Easing::EaseOutQuart => "power4.out".to_string(),
            Easing::EaseInOutQuart => "power4.inOut".to_string(),
            Easing::EaseInExpo => "expo.in".to_string(),
            Easing::EaseOutExpo => "expo.out".to_string(),
            Easing::EaseInOutExpo => "expo.inOut".to_string(),
            Easing::EaseInCirc => "circ.in".to_string(),
            Easing::EaseOutCirc => "circ.out".to_string(),
            Easing::EaseInOutCirc => "circ.inOut".to_string(),
            Easing::EaseInBack => "back.in".to_string(),
            Easing::EaseOutBack => "back.out".to_string(),
            Easing::EaseInOutBack => "back.inOut".to_string(),
            Easing::EaseInElastic => "elastic.in".to_string(),
            Easing::EaseOutElastic => "elastic.out".to_string(),
            Easing::EaseInOutElastic => "elastic.inOut".to_string(),
            Easing::EaseInBounce => "bounce.in".to_string(),
            Easing::EaseOutBounce => "bounce.out".to_string(),
            Easing::EaseInOutBounce => "bounce.inOut".to_string(),
            Easing::Spring { tension, friction } => format!("elastic.out({}, {})", tension, friction),
            Easing::Bezier { .. } => "power2.inOut".to_string(), // Fallback for custom bezier
        }
    }
}

/// Export result containing generated content
#[derive(Debug, Clone)]
pub struct ExportResult {
    /// Generated content
    pub content: String,
    /// MIME type of content
    pub mime_type: String,
    /// Recommended file extension
    pub file_extension: String,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Code generator for specific frameworks
pub struct CodeGenerator {
    /// Target framework/library
    pub target: CodeTarget,
    /// Generation settings
    pub settings: CodeGenSettings,
}

impl CodeGenerator {
    /// Create new code generator
    pub fn new(target: CodeTarget) -> Self {
        Self {
            target,
            settings: CodeGenSettings::default(),
        }
    }

    /// Generate code from timeline
    pub fn generate_from_timeline(&self, timeline: &Timeline3D) -> Result<String> {
        match self.target {
            CodeTarget::React => self.generate_react_component(timeline),
            CodeTarget::Vue => self.generate_vue_component(timeline),
            CodeTarget::Angular => self.generate_angular_component(timeline),
            CodeTarget::Svelte => self.generate_svelte_component(timeline),
            CodeTarget::Leptos => self.generate_leptos_component(timeline),
        }
    }

    fn generate_react_component(&self, timeline: &Timeline3D) -> Result<String> {
        let mut react_output = String::new();
        
        // Add imports
        if self.settings.typescript {
            react_output.push_str("import React from 'react';\n");
            react_output.push_str("import { motion } from 'framer-motion';\n\n");
            react_output.push_str("interface AnimationProps {\n");
            react_output.push_str("  className?: string;\n");
            react_output.push_str("  children?: React.ReactNode;\n");
            react_output.push_str("}\n\n");
        } else {
            react_output.push_str("import React from 'react';\n");
            react_output.push_str("import { motion } from 'framer-motion';\n\n");
        }
        
        // Generate component
        let component_name = "AnimatedComponent";
        if self.settings.typescript {
            react_output.push_str(&format!("const {}: React.FC<AnimationProps> = ({{ className, children }}) => {{\n", component_name));
        } else {
            react_output.push_str(&format!("const {} = ({{ className, children }}) => {{\n", component_name));
        }
        
        // Add animation variants
        react_output.push_str("  const variants = {\n");
        react_output.push_str("    initial: {\n");
        
        // Generate initial state from timeline
        if let Some(initial_keyframe) = timeline.keyframes().first() {
            react_output.push_str(&format!("      opacity: {},\n", initial_keyframe.value));
            react_output.push_str("      scale: 1,\n");
            react_output.push_str("      x: 0,\n");
            react_output.push_str("      y: 0,\n");
        }
        
        react_output.push_str("    },\n");
        react_output.push_str("    animate: {\n");
        
        // Generate animate state from timeline
        if let Some(final_keyframe) = timeline.keyframes().last() {
            react_output.push_str(&format!("      opacity: {},\n", final_keyframe.value));
            react_output.push_str("      scale: 1.1,\n");
            react_output.push_str("      x: 100,\n");
            react_output.push_str("      y: 0,\n");
        }
        
        react_output.push_str("    },\n");
        react_output.push_str("  };\n\n");
        
        // Add transition
        react_output.push_str("  const transition = {\n");
        react_output.push_str(&format!("    duration: {},\n", timeline.duration()));
        react_output.push_str("    ease: 'easeInOut',\n");
        react_output.push_str("  };\n\n");
        
        // Generate JSX
        react_output.push_str("  return (\n");
        react_output.push_str("    <motion.div\n");
        react_output.push_str("      className={className}\n");
        react_output.push_str("      variants={variants}\n");
        react_output.push_str("      initial=\"initial\"\n");
        react_output.push_str("      animate=\"animate\"\n");
        react_output.push_str("      transition={transition}\n");
        react_output.push_str("    >\n");
        react_output.push_str("      {children}\n");
        react_output.push_str("    </motion.div>\n");
        react_output.push_str("  );\n");
        react_output.push_str("};\n\n");
        
        // Add export
        react_output.push_str(&format!("export default {};\n", component_name));
        
        Ok(react_output)
    }

    fn generate_vue_component(&self, timeline: &Timeline3D) -> Result<String> {
        let mut vue_output = String::new();
        
        // Add template
        vue_output.push_str("<template>\n");
        vue_output.push_str("  <div\n");
        vue_output.push_str("    class=\"animated-component\"\n");
        vue_output.push_str("    :style=\"animationStyle\"\n");
        vue_output.push_str("  >\n");
        vue_output.push_str("    <slot></slot>\n");
        vue_output.push_str("  </div>\n");
        vue_output.push_str("</template>\n\n");
        
        // Add script
        if self.settings.typescript {
            vue_output.push_str("<script setup lang=\"ts\">\n");
            vue_output.push_str("import { ref, computed, onMounted } from 'vue';\n\n");
            vue_output.push_str("interface Props {\n");
            vue_output.push_str("  className?: string;\n");
            vue_output.push_str("}\n\n");
            vue_output.push_str("const props = withDefaults(defineProps<Props>(), {\n");
            vue_output.push_str("  className: '',\n");
            vue_output.push_str("});\n\n");
        } else {
            vue_output.push_str("<script setup>\n");
            vue_output.push_str("import { ref, computed, onMounted } from 'vue';\n\n");
            vue_output.push_str("const props = defineProps({\n");
            vue_output.push_str("  className: {\n");
            vue_output.push_str("    type: String,\n");
            vue_output.push_str("    default: '',\n");
            vue_output.push_str("  },\n");
            vue_output.push_str("});\n\n");
        }
        
        // Add reactive state
        vue_output.push_str("const isAnimating = ref(false);\n");
        vue_output.push_str("const animationProgress = ref(0);\n\n");
        
        // Add computed animation style
        vue_output.push_str("const animationStyle = computed(() => {\n");
        vue_output.push_str("  const progress = animationProgress.value;\n");
        vue_output.push_str("  \n");
        vue_output.push_str("  return {\n");
        vue_output.push_str("    opacity: 0.5 + (progress * 0.5),\n");
        vue_output.push_str("    transform: `translateX(${progress * 100}px) scale(${1 + progress * 0.1})`,\n");
        vue_output.push_str("    transition: 'all 0.3s ease-in-out',\n");
        vue_output.push_str("  };\n");
        vue_output.push_str("});\n\n");
        
        // Add animation function
        vue_output.push_str("const startAnimation = () => {\n");
        vue_output.push_str("  isAnimating.value = true;\n");
        vue_output.push_str("  animationProgress.value = 0;\n");
        vue_output.push_str("  \n");
        vue_output.push_str("  const duration = 1000; // 1 second\n");
        vue_output.push_str("  const startTime = Date.now();\n");
        vue_output.push_str("  \n");
        vue_output.push_str("  const animate = () => {\n");
        vue_output.push_str("    const elapsed = Date.now() - startTime;\n");
        vue_output.push_str("    const progress = Math.min(elapsed / duration, 1);\n");
        vue_output.push_str("    \n");
        vue_output.push_str("    animationProgress.value = progress;\n");
        vue_output.push_str("    \n");
        vue_output.push_str("    if (progress < 1) {\n");
        vue_output.push_str("      requestAnimationFrame(animate);\n");
        vue_output.push_str("    } else {\n");
        vue_output.push_str("      isAnimating.value = false;\n");
        vue_output.push_str("    }\n");
        vue_output.push_str("  };\n");
        vue_output.push_str("  \n");
        vue_output.push_str("  requestAnimationFrame(animate);\n");
        vue_output.push_str("};\n\n");
        
        // Add lifecycle
        vue_output.push_str("onMounted(() => {\n");
        vue_output.push_str("  startAnimation();\n");
        vue_output.push_str("});\n");
        vue_output.push_str("</script>\n\n");
        
        // Add styles
        vue_output.push_str("<style scoped>\n");
        vue_output.push_str(".animated-component {\n");
        vue_output.push_str("  display: inline-block;\n");
        vue_output.push_str("}\n");
        vue_output.push_str("</style>\n");
        
        Ok(vue_output)
    }

    fn generate_angular_component(&self, timeline: &Timeline3D) -> Result<String> {
        // TODO: Implement Angular component generation
        Ok("// Angular component generation not implemented yet".to_string())
    }

    fn generate_svelte_component(&self, timeline: &Timeline3D) -> Result<String> {
        // TODO: Implement Svelte component generation
        Ok("// Svelte component generation not implemented yet".to_string())
    }

    fn generate_leptos_component(&self, timeline: &Timeline3D) -> Result<String> {
        // TODO: Implement Leptos component generation
        Ok("// Leptos component generation not implemented yet".to_string())
    }
}

/// Code generation targets
#[derive(Debug, Clone)]
pub enum CodeTarget {
    React,
    Vue,
    Angular,
    Svelte,
    Leptos,
}

/// Code generation settings
#[derive(Debug, Clone)]
pub struct CodeGenSettings {
    /// Include TypeScript types
    pub typescript: bool,
    /// Include comments
    pub comments: bool,
    /// Code style
    pub style: CodeStyle,
}

impl Default for CodeGenSettings {
    fn default() -> Self {
        Self {
            typescript: true,
            comments: true,
            style: CodeStyle::Pretty,
        }
    }
}

/// Code formatting styles
#[derive(Debug, Clone)]
pub enum CodeStyle {
    Compact,
    Pretty,
    Verbose,
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            format: ExportFormat::CSS,
            settings: ExportSettings::CSS(CSSSettings::default()),
            optimization: OptimizationLevel::Basic,
            include_comments: true,
            minify: false,
        }
    }
}

impl Default for CSSSettings {
    fn default() -> Self {
        Self {
            use_custom_properties: true,
            css_version: CSSVersion::Modern,
            vendor_prefixes: false,
            fill_mode: "forwards".to_string(),
        }
    }
}

impl Default for JavaScriptSettings {
    fn default() -> Self {
        Self {
            es_version: ESVersion::ES2018,
            module_format: ModuleFormat::ESModule,
            typescript_definitions: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::StudioProject;

    #[test]
    fn test_export_formats() {
        let project = StudioProject::new("Test Project");
        let exporter = AnimationExporter::new(&project);

        let formats = exporter.supported_formats();
        assert!(formats.contains(&ExportFormat::CSS));
        assert!(formats.contains(&ExportFormat::WAAPI));
        assert!(formats.contains(&ExportFormat::LeptosMotion));
    }

    #[test]
    fn test_css_export() {
        let mut project = StudioProject::new("Test Project");
        project.add_animation("test-animation");

        let exporter = AnimationExporter::new(&project);
        let result = exporter.export_css().unwrap();

        assert!(!result.content.is_empty());
        assert_eq!(result.mime_type, "text/css");
        assert_eq!(result.file_extension, "css");
    }

    #[test]
    fn test_waapi_export() {
        let mut project = StudioProject::new("Test Project");
        project.add_animation("test-animation");

        let exporter = AnimationExporter::new(&project);
        let result = exporter.export_waapi().unwrap();

        assert!(!result.content.is_empty());
        assert_eq!(result.mime_type, "text/javascript");
        assert_eq!(result.file_extension, "js");
        assert!(result.content.contains("MotionAnimations"));
    }

    #[test]
    fn test_leptos_motion_export() {
        let mut project = StudioProject::new("Test Project");
        project.add_animation("TestAnimation");

        let exporter = AnimationExporter::new(&project);
        let result = exporter.export_leptos_motion().unwrap();

        assert!(!result.content.is_empty());
        assert_eq!(result.mime_type, "text/x-rust");
        assert_eq!(result.file_extension, "rs");
        assert!(result.content.contains("use leptos_motion::*"));
    }

    #[test]
    fn test_code_generator() {
        let generator = CodeGenerator::new(CodeTarget::React);
        assert!(matches!(generator.target, CodeTarget::React));
        assert!(generator.settings.typescript);
        assert!(generator.settings.comments);
    }
}
