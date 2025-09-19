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
        let mut svg_output = String::new();
        
        // SVG header
        svg_output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        svg_output.push_str("<svg width=\"1920\" height=\"1080\" viewBox=\"0 0 1920 1080\" xmlns=\"http://www.w3.org/2000/svg\">\n");
        svg_output.push_str("  <defs>\n");
        
        // Add gradients and filters
        svg_output.push_str("    <linearGradient id=\"gradient1\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\">\n");
        svg_output.push_str("      <stop offset=\"0%\" style=\"stop-color:#ff6b6b;stop-opacity:1\" />\n");
        svg_output.push_str("      <stop offset=\"100%\" style=\"stop-color:#4ecdc4;stop-opacity:1\" />\n");
        svg_output.push_str("    </linearGradient>\n");
        svg_output.push_str("  </defs>\n");
        
        // Generate animated elements
        for (i, (id, animation)) in self.project.animations.iter().enumerate() {
            if !animation.enabled {
                continue;
            }
            
            let element_id = format!("animated-element-{}", i);
            let animation_name = &animation.name;
            
            svg_output.push_str(&format!("  <!-- {} -->\n", animation_name));
            svg_output.push_str(&format!("  <g id=\"{}\">\n", element_id));
            
            // Add animated rectangle
            svg_output.push_str("    <rect\n");
            svg_output.push_str("      x=\"100\"\n");
            svg_output.push_str("      y=\"100\"\n");
            svg_output.push_str("      width=\"200\"\n");
            svg_output.push_str("      height=\"100\"\n");
            svg_output.push_str("      fill=\"url(#gradient1)\"\n");
            svg_output.push_str("      stroke=\"#333\"\n");
            svg_output.push_str("      stroke-width=\"2\"\n");
            svg_output.push_str("      rx=\"10\"\n");
            svg_output.push_str("    >\n");
            
            // Add animations
            svg_output.push_str("      <animateTransform\n");
            svg_output.push_str("        attributeName=\"transform\"\n");
            svg_output.push_str("        type=\"translate\"\n");
            svg_output.push_str("        values=\"0,0; 100,50; 0,0\"\n");
            svg_output.push_str("        dur=\"2s\"\n");
            svg_output.push_str("        repeatCount=\"indefinite\"\n");
            svg_output.push_str("      />\n");
            
            svg_output.push_str("      <animate\n");
            svg_output.push_str("        attributeName=\"opacity\"\n");
            svg_output.push_str("        values=\"1; 0.5; 1\"\n");
            svg_output.push_str("        dur=\"2s\"\n");
            svg_output.push_str("        repeatCount=\"indefinite\"\n");
            svg_output.push_str("      />\n");
            
            svg_output.push_str("      <animateTransform\n");
            svg_output.push_str("        attributeName=\"transform\"\n");
            svg_output.push_str("        type=\"scale\"\n");
            svg_output.push_str("        values=\"1; 1.2; 1\"\n");
            svg_output.push_str("        dur=\"2s\"\n");
            svg_output.push_str("        repeatCount=\"indefinite\"\n");
            svg_output.push_str("      />\n");
            
            svg_output.push_str("    </rect>\n");
            
            // Add text label
            svg_output.push_str("    <text\n");
            svg_output.push_str("      x=\"200\"\n");
            svg_output.push_str("      y=\"160\"\n");
            svg_output.push_str("      text-anchor=\"middle\"\n");
            svg_output.push_str("      font-family=\"Arial, sans-serif\"\n");
            svg_output.push_str("      font-size=\"16\"\n");
            svg_output.push_str("      fill=\"white\"\n");
            svg_output.push_str("    >\n");
            svg_output.push_str(&format!("      {}\n", animation_name));
            svg_output.push_str("    </text>\n");
            
            svg_output.push_str("  </g>\n");
        }
        
        // Add interactive elements
        svg_output.push_str("  <!-- Interactive elements -->\n");
        svg_output.push_str("  <circle\n");
        svg_output.push_str("    cx=\"960\"\n");
        svg_output.push_str("    cy=\"540\"\n");
        svg_output.push_str("    r=\"50\"\n");
        svg_output.push_str("    fill=\"#ff6b6b\"\n");
        svg_output.push_str("    stroke=\"#333\"\n");
        svg_output.push_str("    stroke-width=\"3\"\n");
        svg_output.push_str("  >\n");
        svg_output.push_str("    <animateTransform\n");
        svg_output.push_str("      attributeName=\"transform\"\n");
        svg_output.push_str("      type=\"rotate\"\n");
        svg_output.push_str("      values=\"0 960 540; 360 960 540\"\n");
        svg_output.push_str("      dur=\"3s\"\n");
        svg_output.push_str("      repeatCount=\"indefinite\"\n");
        svg_output.push_str("    />\n");
        svg_output.push_str("  </circle>\n");
        
        // Add path animation
        svg_output.push_str("  <path\n");
        svg_output.push_str("    d=\"M 100 800 Q 500 600 900 800 T 1700 800\"\n");
        svg_output.push_str("    stroke=\"#4ecdc4\"\n");
        svg_output.push_str("    stroke-width=\"4\"\n");
        svg_output.push_str("    fill=\"none\"\n");
        svg_output.push_str("  >\n");
        svg_output.push_str("    <animate\n");
        svg_output.push_str("      attributeName=\"stroke-dasharray\"\n");
        svg_output.push_str("      values=\"0,2000; 1000,1000; 0,2000\"\n");
        svg_output.push_str("      dur=\"4s\"\n");
        svg_output.push_str("      repeatCount=\"indefinite\"\n");
        svg_output.push_str("    />\n");
        svg_output.push_str("  </path>\n");
        
        svg_output.push_str("</svg>\n");
        
        Ok(ExportResult {
            content: svg_output,
            mime_type: "image/svg+xml".to_string(),
            file_extension: "svg".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as Lottie JSON
    fn export_lottie(&self) -> Result<ExportResult> {
        let mut lottie_output = String::new();
        
        // Lottie JSON structure
        lottie_output.push_str("{\n");
        lottie_output.push_str("  \"v\": \"5.7.4\",\n");
        lottie_output.push_str("  \"fr\": 60,\n");
        lottie_output.push_str("  \"ip\": 0,\n");
        lottie_output.push_str("  \"op\": 60,\n");
        lottie_output.push_str("  \"w\": 1920,\n");
        lottie_output.push_str("  \"h\": 1080,\n");
        lottie_output.push_str("  \"nm\": \"Motion Studio Animation\",\n");
        lottie_output.push_str("  \"ddd\": 0,\n");
        lottie_output.push_str("  \"assets\": [],\n");
        lottie_output.push_str("  \"layers\": [\n");
        
        // Generate layers for each animation
        for (i, (id, animation)) in self.project.animations.iter().enumerate() {
            if !animation.enabled {
                continue;
            }
            
            let layer_name = &animation.name;
            let layer_id = i + 1;
            
            lottie_output.push_str("    {\n");
            lottie_output.push_str("      \"ddd\": 0,\n");
            lottie_output.push_str("      \"ind\": ").push_str(&layer_id.to_string()).push_str(",\n");
            lottie_output.push_str("      \"ty\": 4,\n");
            lottie_output.push_str("      \"nm\": \"").push_str(layer_name).push_str("\",\n");
            lottie_output.push_str("      \"sr\": 1,\n");
            lottie_output.push_str("      \"ks\": {\n");
            
            // Transform properties
            lottie_output.push_str("        \"o\": {\n");
            lottie_output.push_str("          \"a\": 0,\n");
            lottie_output.push_str("          \"k\": 100\n");
            lottie_output.push_str("        },\n");
            
            lottie_output.push_str("        \"r\": {\n");
            lottie_output.push_str("          \"a\": 0,\n");
            lottie_output.push_str("          \"k\": 0\n");
            lottie_output.push_str("        },\n");
            
            lottie_output.push_str("        \"p\": {\n");
            lottie_output.push_str("          \"a\": 0,\n");
            lottie_output.push_str("          \"k\": [960, 540, 0]\n");
            lottie_output.push_str("        },\n");
            
            lottie_output.push_str("        \"a\": {\n");
            lottie_output.push_str("          \"a\": 0,\n");
            lottie_output.push_str("          \"k\": [0, 0, 0]\n");
            lottie_output.push_str("        },\n");
            
            lottie_output.push_str("        \"s\": {\n");
            lottie_output.push_str("          \"a\": 0,\n");
            lottie_output.push_str("          \"k\": [100, 100, 100]\n");
            lottie_output.push_str("        }\n");
            
            lottie_output.push_str("      },\n");
            lottie_output.push_str("      \"ao\": 0,\n");
            lottie_output.push_str("      \"shapes\": [\n");
            
            // Add shape data
            lottie_output.push_str("        {\n");
            lottie_output.push_str("          \"ty\": \"gr\",\n");
            lottie_output.push_str("          \"it\": [\n");
            lottie_output.push_str("            {\n");
            lottie_output.push_str("              \"d\": 1,\n");
            lottie_output.push_str("              \"ty\": \"el\",\n");
            lottie_output.push_str("              \"s\": {\n");
            lottie_output.push_str("                \"a\": 0,\n");
            lottie_output.push_str("                \"k\": [100, 100]\n");
            lottie_output.push_str("              },\n");
            lottie_output.push_str("              \"p\": {\n");
            lottie_output.push_str("                \"a\": 0,\n");
            lottie_output.push_str("                \"k\": [0, 0]\n");
            lottie_output.push_str("              }\n");
            lottie_output.push_str("            },\n");
            lottie_output.push_str("            {\n");
            lottie_output.push_str("              \"ty\": \"fl\",\n");
            lottie_output.push_str("              \"c\": {\n");
            lottie_output.push_str("                \"a\": 0,\n");
            lottie_output.push_str("                \"k\": [1, 0.5, 0.2, 1]\n");
            lottie_output.push_str("              }\n");
            lottie_output.push_str("            }\n");
            lottie_output.push_str("          ]\n");
            lottie_output.push_str("        }\n");
            
            lottie_output.push_str("      ],\n");
            lottie_output.push_str("      \"ip\": 0,\n");
            lottie_output.push_str("      \"op\": 60,\n");
            lottie_output.push_str("      \"st\": 0,\n");
            lottie_output.push_str("      \"bm\": 0\n");
            lottie_output.push_str("    }");
            
            // Add comma if not last layer
            if i < self.project.animations.len() - 1 {
                lottie_output.push_str(",");
            }
            lottie_output.push_str("\n");
        }
        
        lottie_output.push_str("  ]\n");
        lottie_output.push_str("}\n");
        
        Ok(ExportResult {
            content: lottie_output,
            mime_type: "application/json".to_string(),
            file_extension: "json".to_string(),
            metadata: HashMap::new(),
        })
    }

    /// Export as video
    fn export_video(&self, format: &VideoFormat) -> Result<ExportResult> {
        let mut video_output = String::new();
        
        match format {
            VideoFormat::WebM => {
                // Generate WebM export instructions
                video_output.push_str("// WebM Video Export Instructions\n");
                video_output.push_str("// This is a placeholder for WebM video export functionality\n");
                video_output.push_str("// In a real implementation, this would generate actual video data\n\n");
                
                video_output.push_str("const videoExport = {\n");
                video_output.push_str("  format: 'webm',\n");
                video_output.push_str("  width: 1920,\n");
                video_output.push_str("  height: 1080,\n");
                video_output.push_str("  fps: 60,\n");
                video_output.push_str("  duration: 10.0,\n");
                video_output.push_str("  quality: 0.8,\n");
                video_output.push_str("  codec: 'vp9',\n");
                video_output.push_str("  bitrate: 5000000,\n");
                video_output.push_str("};\n\n");
                
                video_output.push_str("// Animation frames data\n");
                video_output.push_str("const animationFrames = [\n");
                
                // Generate frame data
                for (i, (id, animation)) in self.project.animations.iter().enumerate() {
                    if !animation.enabled {
                        continue;
                    }
                    
                    video_output.push_str("  {\n");
                    video_output.push_str(&format!("    frame: {},\n", i));
                    video_output.push_str(&format!("    animation: '{}',\n", animation.name));
                    video_output.push_str("    properties: {\n");
                    video_output.push_str("      x: 100,\n");
                    video_output.push_str("      y: 100,\n");
                    video_output.push_str("      scale: 1.0,\n");
                    video_output.push_str("      rotation: 0,\n");
                    video_output.push_str("      opacity: 1.0,\n");
                    video_output.push_str("    },\n");
                    video_output.push_str("  },\n");
                }
                
                video_output.push_str("];\n\n");
                
                video_output.push_str("// Export function\n");
                video_output.push_str("function exportWebM() {\n");
                video_output.push_str("  // This would use WebCodecs API or similar\n");
                video_output.push_str("  console.log('Exporting WebM video...');\n");
                video_output.push_str("  console.log('Video settings:', videoExport);\n");
                video_output.push_str("  console.log('Animation frames:', animationFrames);\n");
                video_output.push_str("}\n");
            },
            
            VideoFormat::MP4 => {
                // Generate MP4 export instructions
                video_output.push_str("// MP4 Video Export Instructions\n");
                video_output.push_str("// This is a placeholder for MP4 video export functionality\n\n");
                
                video_output.push_str("const videoExport = {\n");
                video_output.push_str("  format: 'mp4',\n");
                video_output.push_str("  width: 1920,\n");
                video_output.push_str("  height: 1080,\n");
                video_output.push_str("  fps: 30,\n");
                video_output.push_str("  duration: 10.0,\n");
                video_output.push_str("  quality: 0.9,\n");
                video_output.push_str("  codec: 'h264',\n");
                video_output.push_str("  bitrate: 8000000,\n");
                video_output.push_str("};\n\n");
                
                video_output.push_str("// Export function\n");
                video_output.push_str("function exportMP4() {\n");
                video_output.push_str("  // This would use FFmpeg.wasm or similar\n");
                video_output.push_str("  console.log('Exporting MP4 video...');\n");
                video_output.push_str("  console.log('Video settings:', videoExport);\n");
                video_output.push_str("}\n");
            },
            
            VideoFormat::GIF => {
                // Generate GIF export instructions
                video_output.push_str("// GIF Export Instructions\n");
                video_output.push_str("// This is a placeholder for GIF export functionality\n\n");
                
                video_output.push_str("const gifExport = {\n");
                video_output.push_str("  format: 'gif',\n");
                video_output.push_str("  width: 800,\n");
                video_output.push_str("  height: 600,\n");
                video_output.push_str("  fps: 15,\n");
                video_output.push_str("  duration: 5.0,\n");
                video_output.push_str("  colors: 256,\n");
                video_output.push_str("  loop: true,\n");
                video_output.push_str("};\n\n");
                
                video_output.push_str("// Export function\n");
                video_output.push_str("function exportGIF() {\n");
                video_output.push_str("  // This would use gif.js or similar\n");
                video_output.push_str("  console.log('Exporting GIF...');\n");
                video_output.push_str("  console.log('GIF settings:', gifExport);\n");
                video_output.push_str("}\n");
            }
        }
        
        Ok(ExportResult {
            content: video_output,
            mime_type: "text/javascript".to_string(),
            file_extension: "js".to_string(),
            metadata: HashMap::new(),
        })
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
        let mut angular_output = String::new();
        
        // Add imports
        if self.settings.typescript {
            angular_output.push_str("import { Component, Input, OnInit, ElementRef, Renderer2 } from '@angular/core';\n");
            angular_output.push_str("import { trigger, state, style, transition, animate } from '@angular/animations';\n\n");
        } else {
            angular_output.push_str("import { Component, Input, OnInit, ElementRef, Renderer2 } from '@angular/core';\n");
            angular_output.push_str("import { trigger, state, style, transition, animate } from '@angular/animations';\n\n");
        }
        
        // Generate component decorator
        angular_output.push_str("@Component({\n");
        angular_output.push_str("  selector: 'app-animated-component',\n");
        angular_output.push_str("  template: `\n");
        angular_output.push_str("    <div \n");
        angular_output.push_str("      class=\"animated-component\"\n");
        angular_output.push_str("      [@animationState]=\"currentState\"\n");
        angular_output.push_str("      [ngClass]=\"className\"\n");
        angular_output.push_str("    >\n");
        angular_output.push_str("      <ng-content></ng-content>\n");
        angular_output.push_str("    </div>\n");
        angular_output.push_str("  `,\n");
        angular_output.push_str("  styles: [`\n");
        angular_output.push_str("    .animated-component {\n");
        angular_output.push_str("      display: inline-block;\n");
        angular_output.push_str("      will-change: transform, opacity;\n");
        angular_output.push_str("    }\n");
        angular_output.push_str("  `],\n");
        angular_output.push_str("  animations: [\n");
        angular_output.push_str("    trigger('animationState', [\n");
        angular_output.push_str("      state('initial', style({\n");
        angular_output.push_str("        opacity: 0.5,\n");
        angular_output.push_str("        transform: 'translateX(0px) scale(1)'\n");
        angular_output.push_str("      })),\n");
        angular_output.push_str("      state('animate', style({\n");
        angular_output.push_str("        opacity: 1,\n");
        angular_output.push_str("        transform: 'translateX(100px) scale(1.1)'\n");
        angular_output.push_str("      })),\n");
        angular_output.push_str("      transition('initial => animate', animate('1000ms ease-in-out')),\n");
        angular_output.push_str("      transition('animate => initial', animate('1000ms ease-in-out'))\n");
        angular_output.push_str("    ])\n");
        angular_output.push_str("  ]\n");
        angular_output.push_str("})\n");
        
        // Generate component class
        if self.settings.typescript {
            angular_output.push_str("export class AnimatedComponent implements OnInit {\n");
            angular_output.push_str("  @Input() className: string = '';\n");
            angular_output.push_str("  \n");
            angular_output.push_str("  currentState: string = 'initial';\n");
            angular_output.push_str("  \n");
            angular_output.push_str("  constructor(\n");
            angular_output.push_str("    private elementRef: ElementRef,\n");
            angular_output.push_str("    private renderer: Renderer2\n");
            angular_output.push_str("  ) {}\n");
        } else {
            angular_output.push_str("export class AnimatedComponent implements OnInit {\n");
            angular_output.push_str("  @Input() className = '';\n");
            angular_output.push_str("  \n");
            angular_output.push_str("  currentState = 'initial';\n");
            angular_output.push_str("  \n");
            angular_output.push_str("  constructor(\n");
            angular_output.push_str("    private elementRef: ElementRef,\n");
            angular_output.push_str("    private renderer: Renderer2\n");
            angular_output.push_str("  ) {}\n");
        }
        
        // Add lifecycle methods
        angular_output.push_str("  \n");
        angular_output.push_str("  ngOnInit(): void {\n");
        angular_output.push_str("    // Start animation after component initialization\n");
        angular_output.push_str("    setTimeout(() => {\n");
        angular_output.push_str("      this.startAnimation();\n");
        angular_output.push_str("    }, 100);\n");
        angular_output.push_str("  }\n");
        angular_output.push_str("  \n");
        angular_output.push_str("  startAnimation(): void {\n");
        angular_output.push_str("    this.currentState = 'animate';\n");
        angular_output.push_str("    \n");
        angular_output.push_str("    // Optional: Add custom animation logic here\n");
        angular_output.push_str("    // You can use the Renderer2 for more complex animations\n");
        angular_output.push_str("  }\n");
        angular_output.push_str("  \n");
        angular_output.push_str("  resetAnimation(): void {\n");
        angular_output.push_str("    this.currentState = 'initial';\n");
        angular_output.push_str("  }\n");
        angular_output.push_str("}\n");
        
        Ok(angular_output)
    }

    fn generate_svelte_component(&self, timeline: &Timeline3D) -> Result<String> {
        let mut svelte_output = String::new();
        
        // Add script section
        if self.settings.typescript {
            svelte_output.push_str("<script lang=\"ts\">\n");
            svelte_output.push_str("  import { onMount } from 'svelte';\n");
            svelte_output.push_str("  import { cubicOut } from 'svelte/easing';\n");
            svelte_output.push_str("  import { fly, scale } from 'svelte/transition';\n\n");
            svelte_output.push_str("  export let className: string = '';\n");
            svelte_output.push_str("  export let duration: number = 1000;\n");
            svelte_output.push_str("  export let delay: number = 0;\n\n");
            svelte_output.push_str("  let isVisible: boolean = false;\n");
            svelte_output.push_str("  let animationKey: number = 0;\n\n");
        } else {
            svelte_output.push_str("<script>\n");
            svelte_output.push_str("  import { onMount } from 'svelte';\n");
            svelte_output.push_str("  import { cubicOut } from 'svelte/easing';\n");
            svelte_output.push_str("  import { fly, scale } from 'svelte/transition';\n\n");
            svelte_output.push_str("  export let className = '';\n");
            svelte_output.push_str("  export let duration = 1000;\n");
            svelte_output.push_str("  export let delay = 0;\n\n");
            svelte_output.push_str("  let isVisible = false;\n");
            svelte_output.push_str("  let animationKey = 0;\n\n");
        }
        
        // Add animation functions
        svelte_output.push_str("  const startAnimation = () => {\n");
        svelte_output.push_str("    isVisible = true;\n");
        svelte_output.push_str("    animationKey += 1;\n");
        svelte_output.push_str("  };\n\n");
        svelte_output.push_str("  const resetAnimation = () => {\n");
        svelte_output.push_str("    isVisible = false;\n");
        svelte_output.push_str("    animationKey += 1;\n");
        svelte_output.push_str("  };\n\n");
        svelte_output.push_str("  onMount(() => {\n");
        svelte_output.push_str("    // Start animation after component mounts\n");
        svelte_output.push_str("    setTimeout(() => {\n");
        svelte_output.push_str("      startAnimation();\n");
        svelte_output.push_str("    }, delay);\n");
        svelte_output.push_str("  });\n");
        svelte_output.push_str("</script>\n\n");
        
        // Add template
        svelte_output.push_str("<div \n");
        svelte_output.push_str("  class=\"animated-component {className}\"\n");
        svelte_output.push_str("  class:visible={isVisible}\n");
        svelte_output.push_str("  in:fly={{ x: -100, duration, easing: cubicOut }}\n");
        svelte_output.push_str("  out:fly={{ x: 100, duration, easing: cubicOut }}\n");
        svelte_output.push_str("  key={animationKey}\n");
        svelte_output.push_str(">\n");
        svelte_output.push_str("  <div \n");
        svelte_output.push_str("    class=\"inner-content\"\n");
        svelte_output.push_str("    in:scale={{ duration: duration * 0.8, easing: cubicOut }}\n");
        svelte_output.push_str("    out:scale={{ duration: duration * 0.8, easing: cubicOut }}\n");
        svelte_output.push_str("  >\n");
        svelte_output.push_str("    <slot></slot>\n");
        svelte_output.push_str("  </div>\n");
        svelte_output.push_str("</div>\n\n");
        
        // Add styles
        svelte_output.push_str("<style>\n");
        svelte_output.push_str("  .animated-component {\n");
        svelte_output.push_str("    display: inline-block;\n");
        svelte_output.push_str("    will-change: transform, opacity;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  .inner-content {\n");
        svelte_output.push_str("    display: inline-block;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  .visible {\n");
        svelte_output.push_str("    opacity: 1;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  /* Custom animation classes */\n");
        svelte_output.push_str("  .animated-component.fade-in {\n");
        svelte_output.push_str("    opacity: 0;\n");
        svelte_output.push_str("    transition: opacity 0.3s ease-in-out;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  .animated-component.fade-in.visible {\n");
        svelte_output.push_str("    opacity: 1;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  .animated-component.slide-up {\n");
        svelte_output.push_str("    transform: translateY(20px);\n");
        svelte_output.push_str("    opacity: 0;\n");
        svelte_output.push_str("    transition: transform 0.3s ease-out, opacity 0.3s ease-out;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("  \n");
        svelte_output.push_str("  .animated-component.slide-up.visible {\n");
        svelte_output.push_str("    transform: translateY(0);\n");
        svelte_output.push_str("    opacity: 1;\n");
        svelte_output.push_str("  }\n");
        svelte_output.push_str("</style>\n");
        
        Ok(svelte_output)
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
