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
        // TODO: Implement GSAP export
        Err(StudioError::ExportError(
            "GSAP export not yet implemented".to_string(),
        ))
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
            AnimationProperty::Translation => "transform".to_string(),
            AnimationProperty::Rotation => "transform".to_string(),
            AnimationProperty::Scale => "transform".to_string(),
            AnimationProperty::Opacity => "opacity".to_string(),
            AnimationProperty::Color => "color".to_string(),
            AnimationProperty::Custom(name) => name.clone(),
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
        // TODO: Implement React component generation
        Ok("// React component generation not implemented yet".to_string())
    }

    fn generate_vue_component(&self, timeline: &Timeline3D) -> Result<String> {
        // TODO: Implement Vue component generation
        Ok("// Vue component generation not implemented yet".to_string())
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
