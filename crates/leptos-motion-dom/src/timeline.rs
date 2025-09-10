//! Timeline Sequences
//!
//! Advanced orchestration system for complex animation sequences

use leptos::prelude::*;
use leptos::reactive::signal::signal;
use leptos_motion_core::{AnimationTarget, Transition};
use std::collections::HashMap;

/// Type alias for timeline hook return type
type UseTimelineReturn = (
    ReadSignal<Option<String>>,
    WriteSignal<Option<String>>,
    ReadSignal<Option<AnimationTarget>>,
    WriteSignal<HashMap<String, TimelineSequence>>,
);

/// Timeline sequence step
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineStep {
    /// Step identifier
    pub id: String,
    /// Animation target for this step
    pub target: AnimationTarget,
    /// Duration of this step
    pub duration: f64,
    /// Delay before this step starts
    pub delay: f64,
    /// Transition configuration
    pub transition: Option<Transition>,
}

impl TimelineStep {
    /// Create a new timeline step
    pub fn new(id: String, target: AnimationTarget, duration: f64) -> Self {
        Self {
            id,
            target,
            duration,
            delay: 0.0,
            transition: None,
        }
    }

    /// Add delay to the step
    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

    /// Add transition to the step
    pub fn with_transition(mut self, transition: Transition) -> Self {
        self.transition = Some(transition);
        self
    }
}

/// Timeline sequence for orchestrating multiple animation steps
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineSequence {
    /// Sequence identifier
    pub id: String,
    /// Steps in the sequence
    pub steps: Vec<TimelineStep>,
    /// Whether the sequence should repeat
    pub repeat: bool,
    /// Whether the sequence should reverse on repeat
    pub reverse: bool,
    /// Total duration of the sequence
    pub total_duration: f64,
}

impl TimelineSequence {
    /// Create a new timeline sequence
    pub fn new(id: String) -> Self {
        Self {
            id,
            steps: Vec::new(),
            repeat: false,
            reverse: false,
            total_duration: 0.0,
        }
    }

    /// Add a step to the sequence
    pub fn add_step(&mut self, step: TimelineStep) {
        self.steps.push(step);
        self.calculate_total_duration();
    }

    /// Calculate total duration based on steps
    fn calculate_total_duration(&mut self) {
        self.total_duration = self
            .steps
            .iter()
            .map(|step| step.delay + step.duration)
            .fold(0.0, f64::max);
    }

    /// Get step by ID
    pub fn get_step(&self, id: &str) -> Option<&TimelineStep> {
        self.steps.iter().find(|step| step.id == id)
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Set repeat behavior
    pub fn set_repeat(&mut self, repeat: bool) {
        self.repeat = repeat;
    }

    /// Set reverse behavior
    pub fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }

    /// Get step at index
    pub fn get_step_at(&self, index: usize) -> Option<&TimelineStep> {
        self.steps.get(index)
    }

    /// Remove step by ID
    pub fn remove_step(&mut self, id: &str) -> Option<TimelineStep> {
        if let Some(pos) = self.steps.iter().position(|step| step.id == id) {
            let step = self.steps.remove(pos);
            self.calculate_total_duration();
            Some(step)
        } else {
            None
        }
    }

    /// Clear all steps
    pub fn clear_steps(&mut self) {
        self.steps.clear();
        self.total_duration = 0.0;
    }
}

/// Timeline sequence player for executing sequences
#[derive(Debug, Clone)]
pub struct TimelinePlayer {
    sequence: TimelineSequence,
    current_step: usize,
    current_time: f64,
    is_playing: bool,
    is_paused: bool,
    is_reversed: bool,
    loop_count: usize,
}

impl TimelinePlayer {
    /// Create a new timeline player
    pub fn new(sequence: TimelineSequence) -> Self {
        Self {
            sequence,
            current_step: 0,
            current_time: 0.0,
            is_playing: false,
            is_paused: false,
            is_reversed: false,
            loop_count: 0,
        }
    }

    /// Start playing the sequence
    pub fn play(&mut self) {
        self.is_playing = true;
        self.is_paused = false;
    }

    /// Pause the sequence
    pub fn pause(&mut self) {
        self.is_paused = true;
    }

    /// Stop the sequence
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.is_paused = false;
        self.current_step = 0;
        self.current_time = 0.0;
        self.loop_count = 0;
    }

    /// Update the timeline player
    pub fn update(&mut self, delta_time: f64) -> bool {
        if !self.is_playing || self.is_paused {
            return false;
        }

        self.current_time += delta_time;

        // Check if current step is complete
        if let Some(step) = self.sequence.get_step_at(self.current_step) {
            let step_end_time = step.delay + step.duration;

            if self.current_time >= step_end_time {
                self.current_step += 1;

                // Check if sequence is complete
                if self.current_step >= self.sequence.step_count() {
                    if self.sequence.repeat {
                        self.loop_count += 1;
                        if self.sequence.reverse {
                            self.is_reversed = !self.is_reversed;
                        }
                        self.current_step = 0;
                        self.current_time = 0.0;
                    } else {
                        self.is_playing = false;
                        return true; // Sequence completed
                    }
                }
            }
        }

        false
    }

    /// Get current step
    pub fn current_step(&self) -> Option<&TimelineStep> {
        self.sequence.get_step_at(self.current_step)
    }

    /// Get current time
    pub fn current_time(&self) -> f64 {
        self.current_time
    }

    /// Check if playing
    pub fn is_playing(&self) -> bool {
        self.is_playing && !self.is_paused
    }

    /// Check if paused
    pub fn is_paused(&self) -> bool {
        self.is_paused
    }

    /// Get loop count
    pub fn loop_count(&self) -> usize {
        self.loop_count
    }

    /// Seek to specific time
    pub fn seek(&mut self, time: f64) {
        self.current_time = time;

        // Find current step based on time
        self.current_step = 0;
        for (index, step) in self.sequence.steps.iter().enumerate() {
            let step_start_time = step.delay;
            let step_end_time = step.delay + step.duration;
            if time >= step_start_time && time <= step_end_time {
                self.current_step = index;
                break;
            } else if time > step_end_time {
                self.current_step = index + 1;
            }
        }

        // Ensure we don't exceed the number of steps
        if self.current_step >= self.sequence.step_count() {
            self.current_step = self.sequence.step_count().saturating_sub(1);
        }
    }

    /// Get sequence
    pub fn sequence(&self) -> &TimelineSequence {
        &self.sequence
    }

    /// Get progress (0.0 to 1.0)
    pub fn progress(&self) -> f64 {
        if self.sequence.total_duration > 0.0 {
            (self.current_time / self.sequence.total_duration).min(1.0)
        } else {
            0.0
        }
    }
}

/// Timeline manager for multiple sequences
#[derive(Debug, Clone)]
pub struct TimelineManager {
    sequences: HashMap<String, TimelineSequence>,
    players: HashMap<String, TimelinePlayer>,
}

impl TimelineManager {
    /// Create a new timeline manager
    pub fn new() -> Self {
        Self {
            sequences: HashMap::new(),
            players: HashMap::new(),
        }
    }

    /// Add a sequence
    pub fn add_sequence(&mut self, sequence: TimelineSequence) {
        let id = sequence.id.clone();
        self.sequences.insert(id.clone(), sequence);
    }

    /// Get a sequence
    pub fn get_sequence(&self, id: &str) -> Option<&TimelineSequence> {
        self.sequences.get(id)
    }

    /// Create a player for a sequence
    pub fn create_player(&mut self, sequence_id: &str) -> Option<&mut TimelinePlayer> {
        if let Some(sequence) = self.sequences.get(sequence_id).cloned() {
            let player = TimelinePlayer::new(sequence);
            self.players.insert(sequence_id.to_string(), player);
            self.players.get_mut(sequence_id)
        } else {
            None
        }
    }

    /// Get a player
    pub fn get_player(&self, id: &str) -> Option<&TimelinePlayer> {
        self.players.get(id)
    }

    /// Get a mutable player
    pub fn get_player_mut(&mut self, id: &str) -> Option<&mut TimelinePlayer> {
        self.players.get_mut(id)
    }

    /// Update all players
    pub fn update_all(&mut self, delta_time: f64) -> Vec<String> {
        let mut completed = Vec::new();

        for (id, player) in self.players.iter_mut() {
            if player.update(delta_time) {
                completed.push(id.clone());
            }
        }

        completed
    }

    /// Remove a sequence and its player
    pub fn remove_sequence(
        &mut self,
        id: &str,
    ) -> (Option<TimelineSequence>, Option<TimelinePlayer>) {
        let sequence = self.sequences.remove(id);
        let player = self.players.remove(id);
        (sequence, player)
    }

    /// Clear all sequences and players
    pub fn clear(&mut self) {
        self.sequences.clear();
        self.players.clear();
    }

    /// Get sequence count
    pub fn sequence_count(&self) -> usize {
        self.sequences.len()
    }

    /// Get player count
    pub fn player_count(&self) -> usize {
        self.players.len()
    }
}

impl Default for TimelineManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Hook for using timeline sequences in Leptos components
pub fn use_timeline() -> UseTimelineReturn {
    let (current_sequence, set_current_sequence) = signal(None::<String>);
    let (current_target, set_current_target) = signal(None::<AnimationTarget>);
    let (sequences, set_sequences) = signal(HashMap::<String, TimelineSequence>::new());

    // Update current target when sequence changes
    Effect::new(move |_| {
        if let Some(seq_id) = current_sequence.get() {
            if let Some(sequence) = sequences.get().get(&seq_id)
                && let Some(step) = sequence.get_step_at(0)
            {
                set_current_target.set(Some(step.target.clone()));
            }
        } else {
            set_current_target.set(None);
        }
    });

    (
        current_sequence,
        set_current_sequence,
        current_target,
        set_sequences,
    )
}

/// Utility functions for common timeline patterns
pub mod timeline_utils {
    use super::*;

    /// Create a fade in sequence
    pub fn fade_in_sequence(id: String, duration: f64) -> TimelineSequence {
        let mut sequence = TimelineSequence::new(id);

        let initial_step = TimelineStep::new(
            "initial".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "opacity".to_string(),
                leptos_motion_core::AnimationValue::Number(0.0),
            )]),
            0.0,
        );

        let fade_step = TimelineStep::new(
            "fade".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "opacity".to_string(),
                leptos_motion_core::AnimationValue::Number(1.0),
            )]),
            duration,
        );

        sequence.add_step(initial_step);
        sequence.add_step(fade_step);

        sequence
    }

    /// Create a scale sequence
    pub fn scale_sequence(id: String, duration: f64) -> TimelineSequence {
        let mut sequence = TimelineSequence::new(id);

        let initial_step = TimelineStep::new(
            "initial".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "scale".to_string(),
                leptos_motion_core::AnimationValue::Number(0.0),
            )]),
            0.0,
        );

        let scale_step = TimelineStep::new(
            "scale".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "scale".to_string(),
                leptos_motion_core::AnimationValue::Number(1.0),
            )]),
            duration,
        );

        sequence.add_step(initial_step);
        sequence.add_step(scale_step);

        sequence
    }

    /// Create a bounce sequence
    pub fn bounce_sequence(id: String, duration: f64) -> TimelineSequence {
        let mut sequence = TimelineSequence::new(id);

        let initial_step = TimelineStep::new(
            "initial".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "scale".to_string(),
                leptos_motion_core::AnimationValue::Number(0.0),
            )]),
            0.0,
        );

        let bounce_step = TimelineStep::new(
            "bounce".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "scale".to_string(),
                leptos_motion_core::AnimationValue::Number(1.2),
            )]),
            duration * 0.3,
        );

        let settle_step = TimelineStep::new(
            "settle".to_string(),
            leptos_motion_core::AnimationTarget::from([(
                "scale".to_string(),
                leptos_motion_core::AnimationValue::Number(1.0),
            )]),
            duration * 0.7,
        )
        .with_delay(duration * 0.3);

        sequence.add_step(initial_step);
        sequence.add_step(bounce_step);
        sequence.add_step(settle_step);

        sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline_step_creation() {
        let target = leptos_motion_core::AnimationTarget::from([(
            "opacity".to_string(),
            leptos_motion_core::AnimationValue::Number(1.0),
        )]);

        let step = TimelineStep::new("step1".to_string(), target, 1.0).with_delay(0.5);

        assert_eq!(step.id, "step1");
        assert_eq!(step.duration, 1.0);
        assert_eq!(step.delay, 0.5);
    }

    #[test]
    fn test_timeline_sequence() {
        let mut sequence = TimelineSequence::new("test_sequence".to_string());

        let target = leptos_motion_core::AnimationTarget::from([(
            "opacity".to_string(),
            leptos_motion_core::AnimationValue::Number(1.0),
        )]);

        let step = TimelineStep::new("step1".to_string(), target, 1.0);
        sequence.add_step(step);

        assert_eq!(sequence.step_count(), 1);
        assert_eq!(sequence.total_duration, 1.0);

        sequence.set_repeat(true);
        assert!(sequence.repeat);
    }

    #[test]
    fn test_timeline_player() {
        let mut sequence = TimelineSequence::new("player_test".to_string());

        let target = leptos_motion_core::AnimationTarget::from([(
            "opacity".to_string(),
            leptos_motion_core::AnimationValue::Number(1.0),
        )]);

        let step = TimelineStep::new("step1".to_string(), target, 1.0);
        sequence.add_step(step);

        let mut player = TimelinePlayer::new(sequence);

        assert!(!player.is_playing());

        player.play();
        assert!(player.is_playing());

        let completed = player.update(0.5);
        assert!(!completed);

        let completed = player.update(0.5);
        assert!(completed);
        assert!(!player.is_playing());
    }

    #[test]
    fn test_timeline_manager() {
        let mut manager = TimelineManager::new();

        let sequence = timeline_utils::fade_in_sequence("fade".to_string(), 1.0);
        manager.add_sequence(sequence);

        assert_eq!(manager.sequence_count(), 1);

        let player = manager.create_player("fade");
        assert!(player.is_some());
        assert_eq!(manager.player_count(), 1);
    }
}
