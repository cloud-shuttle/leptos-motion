//! Timing and interpolation utilities for animations

use leptos_motion_core::*;
use std::time::{Duration, Instant};

/// Timing utilities for animations
pub struct TimingUtils {
    /// Last frame time
    last_frame_time: Option<Instant>,
    /// Target frame rate
    target_fps: f64,
    /// Frame time accumulator
    frame_time_accumulator: f64,
    /// Fixed timestep for consistent updates
    fixed_timestep: f64,
}

impl TimingUtils {
    /// Create new timing utilities
    pub fn new(target_fps: f64) -> Self {
        Self {
            last_frame_time: None,
            target_fps,
            frame_time_accumulator: 0.0,
            fixed_timestep: 1.0 / target_fps,
        }
    }

    /// Update timing and return delta time
    pub fn update(&mut self) -> f64 {
        let now = Instant::now();
        let delta_time = if let Some(last_time) = self.last_frame_time {
            now.duration_since(last_time).as_secs_f64()
        } else {
            0.0
        };

        self.last_frame_time = Some(now);
        self.frame_time_accumulator += delta_time;

        // Cap delta time to prevent large jumps
        delta_time.min(1.0 / 30.0) // Cap at 30fps minimum
    }

    /// Get fixed timestep
    pub fn get_fixed_timestep(&self) -> f64 {
        self.fixed_timestep
    }

    /// Check if we should update with fixed timestep
    pub fn should_update_fixed(&mut self) -> bool {
        if self.frame_time_accumulator >= self.fixed_timestep {
            self.frame_time_accumulator -= self.fixed_timestep;
            true
        } else {
            false
        }
    }

    /// Reset timing
    pub fn reset(&mut self) {
        self.last_frame_time = None;
        self.frame_time_accumulator = 0.0;
    }

    /// Set target FPS
    pub fn set_target_fps(&mut self, fps: f64) {
        self.target_fps = fps.max(1.0);
        self.fixed_timestep = 1.0 / self.target_fps;
    }

    /// Get target FPS
    pub fn get_target_fps(&self) -> f64 {
        self.target_fps
    }
}

impl Default for TimingUtils {
    fn default() -> Self {
        Self::new(60.0)
    }
}

/// Interpolation utilities for smooth value transitions
pub struct InterpolationUtils;

impl InterpolationUtils {
    /// Linear interpolation between two values
    pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t.clamp(0.0, 1.0)
    }

    /// Smooth step interpolation
    pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * (3.0 - 2.0 * t)
    }

    /// Smoother step interpolation
    pub fn smootherstep(edge0: f64, edge1: f64, x: f64) -> f64 {
        let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    /// Cubic bezier interpolation
    pub fn cubic_bezier(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;

        uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
    }

    /// Ease in interpolation
    pub fn ease_in(t: f64) -> f64 {
        t * t
    }

    /// Ease out interpolation
    pub fn ease_out(t: f64) -> f64 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease in-out interpolation
    pub fn ease_in_out(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - 2.0 * (1.0 - t) * (1.0 - t)
        }
    }

    /// Apply easing function to progress
    pub fn apply_easing(progress: f64, easing: &Easing) -> f64 {
        match easing {
            Easing::Linear => progress,
            Easing::EaseIn => Self::ease_in(progress),
            Easing::EaseOut => Self::ease_out(progress),
            Easing::EaseInOut => Self::ease_in_out(progress),
            Easing::CircIn => 1.0 - (1.0 - progress * progress).sqrt(),
            Easing::CircOut => ((2.0 - progress) * progress).sqrt(),
            Easing::CircInOut => {
                if progress < 0.5 {
                    (1.0 - (1.0 - 4.0 * progress * progress).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - 4.0 * (1.0 - progress) * (1.0 - progress)).sqrt()) / 2.0
                }
            }
            Easing::BackIn => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                C3 * progress * progress * progress - C1 * progress * progress
            }
            Easing::BackOut => {
                const C1: f64 = 1.70158;
                const C3: f64 = C1 + 1.0;
                1.0 + C3 * (progress - 1.0).powi(3) + C1 * (progress - 1.0) * (progress - 1.0)
            }
            Easing::BackInOut => {
                const C1: f64 = 1.70158;
                const C2: f64 = C1 * 1.525;
                if progress < 0.5 {
                    (2.0 * progress).powi(2) * ((C2 + 1.0) * 2.0 * progress - C2) / 2.0
                } else {
                    ((2.0 * progress - 2.0).powi(2) * ((C2 + 1.0) * (2.0 * progress - 2.0) + C2) + 2.0) / 2.0
                }
            }
            Easing::Spring(_) => progress, // Handled separately in spring physics
            Easing::Bezier(x1, y1, x2, y2) => Self::cubic_bezier(0.0, *x1, *x2, 1.0, progress),
            Easing::CubicBezier(cb) => Self::cubic_bezier(0.0, cb.0, cb.2, 1.0, progress),
        }
    }

    /// Clamp value between min and max
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        value.max(min).min(max)
    }

    /// Map value from one range to another
    pub fn map_range(value: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
        let t = (value - in_min) / (in_max - in_min);
        Self::lerp(out_min, out_max, t)
    }

    /// Normalize value to 0-1 range
    pub fn normalize(value: f64, min: f64, max: f64) -> f64 {
        (value - min) / (max - min)
    }

    /// Denormalize value from 0-1 range
    pub fn denormalize(normalized: f64, min: f64, max: f64) -> f64 {
        min + normalized * (max - min)
    }
}

/// Animation timing controller
pub struct AnimationTimingController {
    /// Start time of the animation
    start_time: Option<Instant>,
    /// Duration of the animation
    duration: Duration,
    /// Delay before animation starts
    delay: Duration,
    /// Whether animation is paused
    is_paused: bool,
    /// Pause start time
    pause_start_time: Option<Instant>,
    /// Total pause duration
    total_pause_duration: Duration,
}

impl AnimationTimingController {
    /// Create new timing controller
    pub fn new(duration: Duration, delay: Duration) -> Self {
        Self {
            start_time: None,
            duration,
            delay,
            is_paused: false,
            pause_start_time: None,
            total_pause_duration: Duration::ZERO,
        }
    }

    /// Start the animation
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_paused = false;
        self.pause_start_time = None;
        self.total_pause_duration = Duration::ZERO;
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        if !self.is_paused {
            self.is_paused = true;
            self.pause_start_time = Some(Instant::now());
        }
    }

    /// Resume the animation
    pub fn resume(&mut self) {
        if self.is_paused {
            if let Some(pause_start) = self.pause_start_time {
                self.total_pause_duration += Instant::now().duration_since(pause_start);
            }
            self.is_paused = false;
            self.pause_start_time = None;
        }
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f64 {
        if let Some(start) = self.start_time {
            let elapsed = if self.is_paused {
                if let Some(pause_start) = self.pause_start_time {
                    pause_start.duration_since(start) - self.total_pause_duration
                } else {
                    Duration::ZERO
                }
            } else {
                Instant::now().duration_since(start) - self.total_pause_duration
            };

            let total_duration = self.duration + self.delay;
            let progress = elapsed.as_secs_f64() / total_duration.as_secs_f64();
            progress.clamp(0.0, 1.0)
        } else {
            0.0
        }
    }

    /// Check if animation is complete
    pub fn is_complete(&self) -> bool {
        self.get_progress() >= 1.0
    }

    /// Check if animation is in delay phase
    pub fn is_in_delay(&self) -> bool {
        if let Some(start) = self.start_time {
            let elapsed = if self.is_paused {
                if let Some(pause_start) = self.pause_start_time {
                    pause_start.duration_since(start) - self.total_pause_duration
                } else {
                    Duration::ZERO
                }
            } else {
                Instant::now().duration_since(start) - self.total_pause_duration
            };

            elapsed < self.delay
        } else {
            true
        }
    }

    /// Check if animation is active (not in delay and not complete)
    pub fn is_active(&self) -> bool {
        !self.is_in_delay() && !self.is_complete()
    }

    /// Get remaining time
    pub fn get_remaining_time(&self) -> Duration {
        if let Some(start) = self.start_time {
            let elapsed = if self.is_paused {
                if let Some(pause_start) = self.pause_start_time {
                    pause_start.duration_since(start) - self.total_pause_duration
                } else {
                    Duration::ZERO
                }
            } else {
                Instant::now().duration_since(start) - self.total_pause_duration
            };

            let total_duration = self.duration + self.delay;
            if elapsed >= total_duration {
                Duration::ZERO
            } else {
                total_duration - elapsed
            }
        } else {
            self.duration + self.delay
        }
    }

    /// Reset the timing controller
    pub fn reset(&mut self) {
        self.start_time = None;
        self.is_paused = false;
        self.pause_start_time = None;
        self.total_pause_duration = Duration::ZERO;
    }

    /// Set duration
    pub fn set_duration(&mut self, duration: Duration) {
        self.duration = duration;
    }

    /// Set delay
    pub fn set_delay(&mut self, delay: Duration) {
        self.delay = delay;
    }

    /// Get duration
    pub fn get_duration(&self) -> Duration {
        self.duration
    }

    /// Get delay
    pub fn get_delay(&self) -> Duration {
        self.delay
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}
