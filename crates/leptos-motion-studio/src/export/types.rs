//! Export types and configuration structures

use serde::{Deserialize, Serialize};

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

/// Code generation targets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeTarget {
    /// React with Framer Motion
    React,
    /// Vue.js
    Vue,
    /// Angular
    Angular,
    /// Svelte
    Svelte,
    /// Leptos Motion (Rust)
    Leptos,
}

/// Code generation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGenSettings {
    /// Include comments
    pub include_comments: bool,
    /// Pretty print output
    pub pretty_print: bool,
    /// Include type annotations
    pub include_types: bool,
}

impl Default for CodeGenSettings {
    fn default() -> Self {
        Self {
            include_comments: true,
            pretty_print: true,
            include_types: true,
        }
    }
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
            css_version: CSSVersion::CSS3,
            vendor_prefixes: true,
            fill_mode: "forwards".to_string(),
        }
    }
}
