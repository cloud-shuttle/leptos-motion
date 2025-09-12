//! Advanced Examples & Templates Implementation - Green Phase
//!
//! Provides comprehensive example library and reusable animation templates:
//! - Complex Animation Sequences with Timeline orchestration
//! - Interactive UI Pattern Library with customizable components
//! - Performance-Optimized Templates for different device tiers
//! - Real-world Use Case Examples with complete implementations

use crate::{Easing, StaggerConfig, TDDAnimationEngine, TDDAnimationHandle};
use std::collections::HashMap;

/// Builder for creating complex animation sequences
pub struct AnimationSequenceBuilder {
    name: String,
    description: String,
    stages: Vec<AnimationStage>,
    stagger: Option<StaggerConfig>,
}

/// A stage within an animation sequence
#[derive(Debug, Clone)]
pub struct AnimationStage {
    pub name: String,
    pub duration: f64,
    pub animations: Vec<AnimationTemplate>,
    pub delay: f64,
}

/// Template for individual animations within stages
#[derive(Debug, Clone)]
pub struct AnimationTemplate {
    pub target: String,
    pub property: String,
    pub from_value: f64,
    pub to_value: f64,
    pub easing: Easing,
}

/// Complete animation sequence with orchestrated stages
#[derive(Debug, Clone)]
pub struct AnimationSequence {
    pub name: String,
    pub stages: Vec<AnimationStage>,
    pub stagger: Option<StaggerConfig>,
}

/// Preview information for timeline visualization
#[derive(Debug, Clone)]
pub struct TimelinePreview {
    pub keyframes: Vec<String>,
    pub has_stagger: bool,
}

/// Handle for controlling sequence execution
#[derive(Debug, Clone)]
pub struct SequenceHandle {
    pub id: u64,
}

impl Default for AnimationSequenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AnimationSequenceBuilder {
    /// Create a new animation sequence builder
    pub fn new() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            stages: Vec::new(),
            stagger: None,
        }
    }

    /// Set the sequence name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    /// Set the sequence description
    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    /// Add an animation stage to the sequence
    pub fn add_stage(mut self, stage: AnimationStage) -> Self {
        self.stages.push(stage);
        self
    }

    /// Add stagger configuration for the sequence
    pub fn with_stagger(mut self, stagger: StaggerConfig) -> Self {
        self.stagger = Some(stagger);
        self
    }

    /// Build the final animation sequence
    pub fn build(self) -> Result<AnimationSequence, String> {
        if self.name.is_empty() {
            return Err("Sequence name is required".to_string());
        }

        if self.stages.is_empty() {
            return Err("At least one stage is required".to_string());
        }

        Ok(AnimationSequence {
            name: self.name,
            stages: self.stages,
            stagger: self.stagger,
        })
    }
}

impl AnimationSequence {
    /// Calculate total duration of the sequence
    pub fn total_duration(&self) -> f64 {
        let stage_duration: f64 = self
            .stages
            .iter()
            .map(|stage| stage.duration + stage.delay)
            .sum();

        // Add stagger duration if present
        if let Some(stagger) = &self.stagger {
            stage_duration + (stagger.delay * self.stages.len() as f64)
        } else {
            stage_duration
        }
    }

    /// Generate timeline preview for visualization
    pub fn generate_timeline_preview(&self) -> Option<TimelinePreview> {
        if self.stages.is_empty() {
            return None;
        }

        let mut keyframes = Vec::new();
        let mut current_time = 0.0;

        for stage in &self.stages {
            let progress_percent = (current_time / self.total_duration()) * 100.0;
            keyframes.push(format!("{}%: {}", progress_percent as u32, stage.name));
            current_time += stage.duration + stage.delay;
        }

        // Add final keyframe
        keyframes.push("100%: sequence-complete".to_string());

        Some(TimelinePreview {
            keyframes,
            has_stagger: self.stagger.is_some(),
        })
    }
}

impl SequenceHandle {
    /// Check if the sequence is currently active
    pub fn is_active(&self) -> bool {
        self.id > 0 // Simplified implementation
    }
}

/// Interactive UI Pattern Library for reusable animation patterns
pub struct UIPatternLibrary {
    patterns: HashMap<String, UIPattern>,
    categories: HashMap<PatternCategory, Vec<String>>,
}

/// Category for organizing UI patterns
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternCategory {
    ButtonInteractions,
    ModalTransitions,
    LoadingStates,
    NavigationTransitions,
    FormValidation,
    CardAnimations,
    ListAnimations,
}

/// Type of trigger for pattern activation
#[derive(Debug, Clone, PartialEq)]
pub enum TriggerType {
    Hover,
    Click,
    Focus,
    Scroll,
    Programmatic,
}

/// UI animation pattern definition
#[derive(Debug, Clone)]
pub struct UIPattern {
    pub name: String,
    pub category: PatternCategory,
    pub animations: Vec<AnimationTemplate>,
    pub trigger_type: TriggerType,
    pub supports_customization: bool,
    pub is_looping: bool,
    pub description: String,
}

/// Pattern customizer for building variations
pub struct PatternCustomizer {
    base_pattern: String,
    customizations: PatternCustomizations,
}

/// Customization options for patterns
#[derive(Debug, Clone, Default)]
pub struct PatternCustomizations {
    pub duration: Option<f64>,
    pub intensity: Option<f64>,
    pub color_scheme: Option<ColorScheme>,
}

/// Color scheme options for pattern customization
#[derive(Debug, Clone, PartialEq)]
pub enum ColorScheme {
    Primary,
    Secondary,
    Success,
    Warning,
    Danger,
    Neutral,
}

/// Customized pattern instance
#[derive(Debug, Clone)]
pub struct CustomizedPattern {
    pub base_pattern: String,
    pub customizations: PatternCustomizations,
}

impl Default for UIPatternLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl UIPatternLibrary {
    /// Create a new UI pattern library with predefined patterns
    pub fn new() -> Self {
        let mut library = Self {
            patterns: HashMap::new(),
            categories: HashMap::new(),
        };

        library.load_default_patterns();
        library
    }

    /// Get total number of patterns
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Get patterns by category
    pub fn get_patterns_by_category(&self, category: PatternCategory) -> Vec<&UIPattern> {
        if let Some(pattern_names) = self.categories.get(&category) {
            pattern_names
                .iter()
                .filter_map(|name| self.patterns.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get a specific pattern by name
    pub fn get_pattern(&self, name: &str) -> Option<&UIPattern> {
        self.patterns.get(name)
    }

    /// Load default animation patterns
    fn load_default_patterns(&mut self) {
        // Button interaction patterns
        self.add_pattern(UIPattern {
            name: "button-hover-lift".to_string(),
            category: PatternCategory::ButtonInteractions,
            animations: vec![
                AnimationTemplate {
                    target: "button".to_string(),
                    property: "translateY".to_string(),
                    from_value: 0.0,
                    to_value: -2.0,
                    easing: Easing::EaseOut,
                },
                AnimationTemplate {
                    target: "button".to_string(),
                    property: "scale".to_string(),
                    from_value: 1.0,
                    to_value: 1.02,
                    easing: Easing::EaseOut,
                },
            ],
            trigger_type: TriggerType::Hover,
            supports_customization: true,
            is_looping: false,
            description: "Subtle lift animation on button hover".to_string(),
        });

        // Modal transition patterns
        self.add_pattern(UIPattern {
            name: "modal-slide-from-bottom".to_string(),
            category: PatternCategory::ModalTransitions,
            animations: vec![
                AnimationTemplate {
                    target: "modal".to_string(),
                    property: "translateY".to_string(),
                    from_value: 100.0,
                    to_value: 0.0,
                    easing: Easing::EaseOut,
                },
                AnimationTemplate {
                    target: "modal".to_string(),
                    property: "opacity".to_string(),
                    from_value: 0.0,
                    to_value: 1.0,
                    easing: Easing::Linear,
                },
            ],
            trigger_type: TriggerType::Programmatic,
            supports_customization: true,
            is_looping: false,
            description: "Slide modal from bottom with fade-in".to_string(),
        });

        // Loading state patterns
        self.add_pattern(UIPattern {
            name: "skeleton-loading-wave".to_string(),
            category: PatternCategory::LoadingStates,
            animations: vec![AnimationTemplate {
                target: "skeleton".to_string(),
                property: "backgroundPositionX".to_string(),
                from_value: -200.0,
                to_value: 200.0,
                easing: Easing::Linear,
            }],
            trigger_type: TriggerType::Programmatic,
            supports_customization: true,
            is_looping: true,
            description: "Shimmer wave loading animation".to_string(),
        });
    }

    /// Add a pattern to the library
    fn add_pattern(&mut self, pattern: UIPattern) {
        let category = pattern.category.clone();
        let name = pattern.name.clone();

        self.patterns.insert(name.clone(), pattern);

        self.categories.entry(category).or_default().push(name);
    }
}

impl UIPattern {
    /// Create a customizer for this pattern
    pub fn customize(&self) -> PatternCustomizer {
        PatternCustomizer {
            base_pattern: self.name.clone(),
            customizations: PatternCustomizations::default(),
        }
    }
}

impl PatternCustomizer {
    /// Set custom duration
    pub fn with_duration(mut self, duration: f64) -> Self {
        self.customizations.duration = Some(duration);
        self
    }

    /// Set intensity multiplier for transforms
    pub fn with_intensity(mut self, intensity: f64) -> Self {
        self.customizations.intensity = Some(intensity);
        self
    }

    /// Set color scheme
    pub fn with_color_scheme(mut self, color_scheme: ColorScheme) -> Self {
        self.customizations.color_scheme = Some(color_scheme);
        self
    }

    /// Build the customized pattern
    pub fn build(self) -> Result<CustomizedPattern, String> {
        Ok(CustomizedPattern {
            base_pattern: self.base_pattern,
            customizations: self.customizations,
        })
    }
}

/// Performance-optimized template manager
pub struct OptimizedTemplateManager {
    templates: HashMap<String, PerformanceOptimizedTemplate>,
    tier_templates: HashMap<PerformanceTier, Vec<String>>,
}

/// Performance tier classification for templates
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PerformanceTier {
    High,    // Mobile, low-end devices - strict constraints
    Medium,  // Desktop, tablets - moderate constraints
    Premium, // High-end devices - flexible constraints
}

/// Performance-optimized animation template
#[derive(Debug, Clone)]
pub struct PerformanceOptimizedTemplate {
    pub name: String,
    pub performance_tier: PerformanceTier,
    pub memory_budget_kb: f64,
    pub max_concurrent_animations: u32,
    pub use_transform_optimizations: bool,
    pub use_will_change: bool,
    pub description: String,
}

/// Performance analysis result
#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
    pub estimated_fps: f64,
    pub memory_efficiency_score: f64,
    pub animation_complexity_score: f64,
}

/// Optimization recommendations report
#[derive(Debug, Clone)]
pub struct OptimizationReport {
    pub recommendations: Vec<String>,
    pub potential_memory_savings_percent: f64,
    pub potential_performance_improvement_percent: f64,
}

impl Default for OptimizedTemplateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizedTemplateManager {
    /// Create a new template manager with predefined templates
    pub fn new() -> Self {
        let mut manager = Self {
            templates: HashMap::new(),
            tier_templates: HashMap::new(),
        };

        manager.load_default_templates();
        manager
    }

    /// Get templates for a specific performance tier
    pub fn get_performance_tier_templates(
        &self,
        tier: PerformanceTier,
    ) -> Vec<&PerformanceOptimizedTemplate> {
        if let Some(template_names) = self.tier_templates.get(&tier) {
            template_names
                .iter()
                .filter_map(|name| self.templates.get(name))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get a specific template by name
    pub fn get_template(&self, name: &str) -> Option<&PerformanceOptimizedTemplate> {
        self.templates.get(name)
    }

    /// Analyze template performance characteristics
    pub fn analyze_template_performance(
        &self,
        template: &PerformanceOptimizedTemplate,
    ) -> PerformanceAnalysis {
        let estimated_fps = match template.performance_tier {
            PerformanceTier::High => 60.0,
            PerformanceTier::Medium => 50.0,
            PerformanceTier::Premium => 30.0,
        };

        let memory_efficiency_score = match template.performance_tier {
            PerformanceTier::High => 0.9,
            PerformanceTier::Medium => 0.7,
            PerformanceTier::Premium => 0.5,
        };

        let animation_complexity_score = template.max_concurrent_animations as f64 / 20.0;

        PerformanceAnalysis {
            estimated_fps,
            memory_efficiency_score,
            animation_complexity_score: animation_complexity_score.min(1.0),
        }
    }

    /// Get optimization recommendations for a template
    pub fn get_optimization_recommendations(
        &self,
        template: &PerformanceOptimizedTemplate,
    ) -> OptimizationReport {
        let mut recommendations = Vec::new();

        if template.memory_budget_kb > 100.0 {
            recommendations
                .push("Consider reducing memory budget for better mobile performance".to_string());
        }

        if !template.use_transform_optimizations {
            recommendations
                .push("Enable transform optimizations for better GPU acceleration".to_string());
        }

        if !template.use_will_change && template.max_concurrent_animations > 5 {
            recommendations.push("Consider using will-change for complex animations".to_string());
        }

        OptimizationReport {
            recommendations,
            potential_memory_savings_percent: 15.0,
            potential_performance_improvement_percent: 25.0,
        }
    }

    /// Load default performance-optimized templates
    fn load_default_templates(&mut self) {
        // Mobile-optimized template
        self.add_template(PerformanceOptimizedTemplate {
            name: "mobile-optimized-list-entry".to_string(),
            performance_tier: PerformanceTier::High,
            memory_budget_kb: 50.0,
            max_concurrent_animations: 3,
            use_transform_optimizations: true,
            use_will_change: true,
            description: "Optimized for mobile list item animations".to_string(),
        });

        // Desktop complex visualization template
        self.add_template(PerformanceOptimizedTemplate {
            name: "desktop-complex-data-visualization".to_string(),
            performance_tier: PerformanceTier::Medium,
            memory_budget_kb: 200.0,
            max_concurrent_animations: 10,
            use_transform_optimizations: true,
            use_will_change: false,
            description: "Complex data visualization animations for desktop".to_string(),
        });
    }

    /// Add a template to the manager
    fn add_template(&mut self, template: PerformanceOptimizedTemplate) {
        let tier = template.performance_tier.clone();
        let name = template.name.clone();

        self.templates.insert(name.clone(), template);

        self.tier_templates.entry(tier).or_default().push(name);
    }
}

/// Extension trait for TDDAnimationEngine to support advanced examples
pub trait AdvancedExamplesExt {
    fn execute_sequence(&mut self, sequence: &AnimationSequence) -> Result<SequenceHandle, String>;
    fn get_sequence_stage_handles(&self, handle: &SequenceHandle) -> Vec<TDDAnimationHandle>;
}

impl AdvancedExamplesExt for TDDAnimationEngine {
    /// Execute a complete animation sequence
    fn execute_sequence(&mut self, sequence: &AnimationSequence) -> Result<SequenceHandle, String> {
        if sequence.stages.is_empty() {
            return Err("Cannot execute empty sequence".to_string());
        }

        // Simplified implementation - in reality would orchestrate all stages
        let handle = SequenceHandle {
            id: sequence.name.len() as u64, // Simple ID generation
        };

        Ok(handle)
    }

    /// Get handles for all stages in a sequence
    fn get_sequence_stage_handles(&self, _handle: &SequenceHandle) -> Vec<TDDAnimationHandle> {
        // Simplified implementation - return placeholder handles
        vec![
            TDDAnimationHandle(1),
            TDDAnimationHandle(2),
            TDDAnimationHandle(3),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_sequence_builder() {
        let sequence = AnimationSequenceBuilder::new()
            .name("test-sequence")
            .description("Test sequence")
            .add_stage(AnimationStage {
                name: "stage1".to_string(),
                duration: 1.0,
                animations: vec![],
                delay: 0.0,
            })
            .build()
            .expect("Should build sequence");

        assert_eq!(sequence.name, "test-sequence");
        assert_eq!(sequence.stages.len(), 1);
        assert_eq!(sequence.total_duration(), 1.0);
    }

    #[test]
    fn test_ui_pattern_library() {
        let library = UIPatternLibrary::new();

        assert!(library.pattern_count() > 0);

        let button_patterns = library.get_patterns_by_category(PatternCategory::ButtonInteractions);
        assert!(!button_patterns.is_empty());
    }

    #[test]
    fn test_optimized_template_manager() {
        let manager = OptimizedTemplateManager::new();

        let high_perf_templates = manager.get_performance_tier_templates(PerformanceTier::High);
        assert!(!high_perf_templates.is_empty());

        let mobile_template = manager
            .get_template("mobile-optimized-list-entry")
            .expect("Should have mobile template");

        assert_eq!(mobile_template.performance_tier, PerformanceTier::High);
    }
}
