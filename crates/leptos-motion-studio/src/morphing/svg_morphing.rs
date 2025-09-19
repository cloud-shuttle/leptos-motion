//! SVG morphing system

use super::*;

/// SVG morphing system
pub struct SvgMorphing {
    /// Source SVG path
    source_path: SvgPath,
    /// Target SVG path
    target_path: SvgPath,
    /// Path morpher
    morpher: PathMorpher,
    /// Current morphing state
    state: MorphingState,
}

/// Morphing state
#[derive(Debug, Clone)]
struct MorphingState {
    /// Is currently morphing
    is_morphing: bool,
    /// Current progress
    progress: f64,
    /// Animation start time
    start_time: Option<f64>,
    /// Animation duration
    duration: f64,
}

impl SvgMorphing {
    /// Create a new SVG morphing system
    pub fn new(source_path: SvgPath, target_path: SvgPath, config: MorphConfig) -> Self {
        Self {
            source_path,
            target_path,
            morpher: PathMorpher::new(config),
            state: MorphingState {
                is_morphing: false,
                progress: 0.0,
                start_time: None,
                duration: 1.0,
            },
        }
    }

    /// Start morphing animation
    pub fn start_morphing(&mut self, duration: f64) {
        self.state.is_morphing = true;
        self.state.progress = 0.0;
        self.state.duration = duration;
        self.state.start_time = Some(0.0); // In real implementation, this would be current time
    }

    /// Stop morphing animation
    pub fn stop_morphing(&mut self) {
        self.state.is_morphing = false;
    }

    /// Update morphing animation
    pub fn update(&mut self, current_time: f64) -> bool {
        if !self.state.is_morphing {
            return false;
        }

        if let Some(start_time) = self.state.start_time {
            let elapsed = current_time - start_time;
            self.state.progress = (elapsed / self.state.duration).clamp(0.0, 1.0);

            if self.state.progress >= 1.0 {
                self.state.is_morphing = false;
                return false; // Animation complete
            }
        }

        true // Animation still running
    }

    /// Get current morphed path
    pub fn get_current_path(&mut self) -> Vec<PathCommand> {
        self.morpher.morph(
            &self.source_path.commands,
            &self.target_path.commands,
            self.state.progress,
        )
    }

    /// Get current progress
    pub fn progress(&self) -> f64 {
        self.state.progress
    }

    /// Check if currently morphing
    pub fn is_morphing(&self) -> bool {
        self.state.is_morphing
    }

    /// Set source path
    pub fn set_source_path(&mut self, path: SvgPath) {
        self.source_path = path;
    }

    /// Set target path
    pub fn set_target_path(&mut self, path: SvgPath) {
        self.target_path = path;
    }

    /// Get source path
    pub fn source_path(&self) -> &SvgPath {
        &self.source_path
    }

    /// Get target path
    pub fn target_path(&self) -> &SvgPath {
        &self.target_path
    }
}
