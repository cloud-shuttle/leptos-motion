//! Morphing transition management

use super::*;

/// Morphing transition system
pub struct MorphTransition {
    /// Transition configuration
    config: TransitionConfig,
    /// Current transition state
    state: TransitionState,
    /// Transition callbacks
    callbacks: TransitionCallbacks,
}

/// Transition configuration
#[derive(Debug, Clone)]
pub struct TransitionConfig {
    /// Transition duration
    pub duration: f64,
    /// Transition delay
    pub delay: f64,
    /// Transition easing
    pub easing: EasingFunction,
    /// Whether to reverse on completion
    pub reverse: bool,
    /// Number of iterations (0 = infinite)
    pub iterations: u32,
}

/// Transition state
#[derive(Debug, Clone)]
struct TransitionState {
    /// Current progress
    progress: f64,
    /// Current iteration
    iteration: u32,
    /// Is transitioning
    is_transitioning: bool,
    /// Is reversed
    is_reversed: bool,
}

/// Transition callbacks
pub struct TransitionCallbacks {
    /// On start callback
    pub on_start: Option<Box<dyn Fn() + Send + Sync>>,
    /// On progress callback
    pub on_progress: Option<Box<dyn Fn(f64) + Send + Sync>>,
    /// On complete callback
    pub on_complete: Option<Box<dyn Fn() + Send + Sync>>,
}

impl Default for TransitionConfig {
    fn default() -> Self {
        Self {
            duration: 1.0,
            delay: 0.0,
            easing: EasingFunction::EaseInOut,
            reverse: false,
            iterations: 1,
        }
    }
}

impl MorphTransition {
    /// Create a new morphing transition
    pub fn new(config: TransitionConfig) -> Self {
        Self {
            config,
            state: TransitionState {
                progress: 0.0,
                iteration: 0,
                is_transitioning: false,
                is_reversed: false,
            },
            callbacks: TransitionCallbacks {
                on_start: None,
                on_progress: None,
                on_complete: None,
            },
        }
    }

    /// Start the transition
    pub fn start(&mut self) {
        self.state.is_transitioning = true;
        self.state.progress = 0.0;
        self.state.iteration = 0;
        self.state.is_reversed = false;

        if let Some(callback) = &self.callbacks.on_start {
            callback();
        }
    }

    /// Update the transition
    pub fn update(&mut self, delta_time: f64) -> bool {
        if !self.state.is_transitioning {
            return false;
        }

        // Apply delay
        if self.state.progress < self.config.delay {
            self.state.progress += delta_time;
            return true;
        }

        // Calculate transition progress
        let transition_progress = (self.state.progress - self.config.delay) / self.config.duration;
        let eased_progress = self.config.easing.apply(transition_progress.clamp(0.0, 1.0));

        // Apply reverse if needed
        let final_progress = if self.state.is_reversed {
            1.0 - eased_progress
        } else {
            eased_progress
        };

        // Call progress callback
        if let Some(callback) = &self.callbacks.on_progress {
            callback(final_progress);
        }

        // Check if transition is complete
        if transition_progress >= 1.0 {
            self.state.iteration += 1;

            if self.config.iterations == 0 || self.state.iteration < self.config.iterations {
                // Continue with next iteration
                if self.config.reverse {
                    self.state.is_reversed = !self.state.is_reversed;
                }
                self.state.progress = self.config.delay; // Reset for next iteration
            } else {
                // Transition complete
                self.state.is_transitioning = false;
                if let Some(callback) = &self.callbacks.on_complete {
                    callback();
                }
                return false;
            }
        } else {
            self.state.progress += delta_time;
        }

        true // Transition still running
    }

    /// Stop the transition
    pub fn stop(&mut self) {
        self.state.is_transitioning = false;
    }

    /// Check if transitioning
    pub fn is_transitioning(&self) -> bool {
        self.state.is_transitioning
    }

    /// Get current progress
    pub fn progress(&self) -> f64 {
        self.state.progress
    }

    /// Get current iteration
    pub fn iteration(&self) -> u32 {
        self.state.iteration
    }

    /// Set callbacks
    pub fn set_callbacks(&mut self, callbacks: TransitionCallbacks) {
        self.callbacks = callbacks;
    }
}
