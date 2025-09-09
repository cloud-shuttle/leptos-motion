// Timeline Sequences Tests
//
// These tests define the expected behavior of the Timeline Sequences system
// for advanced animation orchestration using Test-Driven Development.

use leptos_motion_core::{AnimationValue, Easing, RepeatConfig, Transition};
use std::collections::HashMap;

/// Timeline sequence step
#[derive(Debug, Clone, PartialEq)]
pub struct TimelineStep {
    /// Step identifier
    pub id: String,
    /// Animation target for this step
    pub target: HashMap<String, AnimationValue>,
    /// Duration of this step
    pub duration: f64,
    /// Delay before this step starts
    pub delay: f64,
    /// Transition configuration
    pub transition: Option<Transition>,
}

impl TimelineStep {
    pub fn new(id: String, target: HashMap<String, AnimationValue>, duration: f64) -> Self {
        Self {
            id,
            target,
            duration,
            delay: 0.0,
            transition: None,
        }
    }

    pub fn with_delay(mut self, delay: f64) -> Self {
        self.delay = delay;
        self
    }

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
}

/// Timeline sequence player for executing sequences
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
}

/// Test basic timeline sequence functionality
#[test]
fn test_timeline_sequence_basic() {
    let mut sequence = TimelineSequence::new("test_sequence".to_string());

    // Create a simple step
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));

    let step = TimelineStep::new("step1".to_string(), target, 1.0);
    sequence.add_step(step);

    // Should have one step
    assert_eq!(sequence.step_count(), 1);
    assert_eq!(sequence.total_duration, 1.0);

    // Should be able to get the step
    let step = sequence.get_step("step1").unwrap();
    assert_eq!(step.duration, 1.0);
    assert_eq!(step.delay, 0.0);
}

/// Test timeline sequence with multiple steps
#[test]
fn test_timeline_sequence_multiple_steps() {
    let mut sequence = TimelineSequence::new("multi_step".to_string());

    // Add multiple steps
    let mut target1 = HashMap::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step1 = TimelineStep::new("step1".to_string(), target1, 1.0);
    sequence.add_step(step1);

    let mut target2 = HashMap::new();
    target2.insert("scale".to_string(), AnimationValue::Number(1.2));
    let step2 = TimelineStep::new("step2".to_string(), target2, 0.5);
    sequence.add_step(step2);

    let mut target3 = HashMap::new();
    target3.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
    let step3 = TimelineStep::new("step3".to_string(), target3, 0.8);
    sequence.add_step(step3);

    // Should have three steps
    assert_eq!(sequence.step_count(), 3);
    assert_eq!(sequence.total_duration, 1.0); // Max of all step durations

    // Should be able to get all steps
    assert!(sequence.get_step("step1").is_some());
    assert!(sequence.get_step("step2").is_some());
    assert!(sequence.get_step("step3").is_some());
    assert!(sequence.get_step("nonexistent").is_none());
}

/// Test timeline sequence with delays
#[test]
fn test_timeline_sequence_with_delays() {
    let mut sequence = TimelineSequence::new("delayed_sequence".to_string());

    // Add steps with delays
    let mut target1 = HashMap::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step1 = TimelineStep::new("step1".to_string(), target1, 1.0).with_delay(0.5);
    sequence.add_step(step1);

    let mut target2 = HashMap::new();
    target2.insert("scale".to_string(), AnimationValue::Number(1.2));
    let step2 = TimelineStep::new("step2".to_string(), target2, 0.5).with_delay(0.2);
    sequence.add_step(step2);

    // Total duration should account for delays
    assert_eq!(sequence.total_duration, 1.5); // Max of (0.5 + 1.0) and (0.2 + 0.5)

    // Should be able to get steps with delays
    let step1 = sequence.get_step("step1").unwrap();
    assert_eq!(step1.delay, 0.5);
    assert_eq!(step1.duration, 1.0);

    let step2 = sequence.get_step("step2").unwrap();
    assert_eq!(step2.delay, 0.2);
    assert_eq!(step2.duration, 0.5);
}

/// Test timeline sequence with transitions
#[test]
fn test_timeline_sequence_with_transitions() {
    let mut sequence = TimelineSequence::new("transition_sequence".to_string());

    // Create step with custom transition
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(0.0));

    let transition = Transition {
        duration: Some(0.5),
        ease: Easing::EaseInOut,
        delay: Some(0.1),
        repeat: RepeatConfig::Never,
        stagger: None,
    };

    let step =
        TimelineStep::new("fade_step".to_string(), target, 1.0).with_transition(transition.clone());
    sequence.add_step(step);

    // Should have the step with transition
    let fade_step = sequence.get_step("fade_step").unwrap();
    assert!(fade_step.transition.is_some());
    assert_eq!(fade_step.transition.as_ref().unwrap().duration, Some(0.5));
    assert_eq!(
        fade_step.transition.as_ref().unwrap().ease,
        Easing::EaseInOut
    );
}

/// Test timeline sequence repeat behavior
#[test]
fn test_timeline_sequence_repeat() {
    let mut sequence = TimelineSequence::new("repeat_sequence".to_string());

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("step1".to_string(), target, 1.0);
    sequence.add_step(step);

    // Set repeat behavior
    sequence.set_repeat(true);
    sequence.set_reverse(true);

    assert!(sequence.repeat);
    assert!(sequence.reverse);
}

/// Test timeline player basic functionality
#[test]
fn test_timeline_player_basic() {
    let mut sequence = TimelineSequence::new("player_test".to_string());

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("step1".to_string(), target, 1.0);
    sequence.add_step(step);

    let mut player = TimelinePlayer::new(sequence);

    // Should start stopped
    assert!(!player.is_playing());
    assert!(!player.is_paused());
    assert_eq!(player.current_step, 0);
    assert_eq!(player.current_time(), 0.0);

    // Start playing
    player.play();
    assert!(player.is_playing());
    assert!(!player.is_paused());

    // Update player
    let completed = player.update(0.5);
    assert!(!completed);
    assert_eq!(player.current_time(), 0.5);

    // Complete the step
    let completed = player.update(0.5);
    assert!(completed);
    assert!(!player.is_playing());
}

/// Test timeline player with multiple steps
#[test]
fn test_timeline_player_multiple_steps() {
    let mut sequence = TimelineSequence::new("multi_player".to_string());

    // Add multiple steps
    let mut target1 = HashMap::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step1 = TimelineStep::new("step1".to_string(), target1, 0.5);
    sequence.add_step(step1);

    let mut target2 = HashMap::new();
    target2.insert("scale".to_string(), AnimationValue::Number(1.2));
    let step2 = TimelineStep::new("step2".to_string(), target2, 0.5);
    sequence.add_step(step2);

    let mut player = TimelinePlayer::new(sequence);
    player.play();

    // First step
    let completed = player.update(0.5);
    assert!(!completed);
    assert_eq!(player.current_step, 1);

    // Second step
    let completed = player.update(0.5);
    assert!(completed);
    assert!(!player.is_playing());
}

/// Test timeline player pause and resume
#[test]
fn test_timeline_player_pause_resume() {
    let mut sequence = TimelineSequence::new("pause_test".to_string());

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("step1".to_string(), target, 1.0);
    sequence.add_step(step);

    let mut player = TimelinePlayer::new(sequence);
    player.play();

    // Update a bit
    player.update(0.3);
    assert_eq!(player.current_time(), 0.3);

    // Pause
    player.pause();
    assert!(player.is_paused());
    assert!(!player.is_playing());

    // Update while paused (should not change time)
    player.update(0.2);
    assert_eq!(player.current_time(), 0.3);

    // Resume
    player.play();
    assert!(!player.is_paused());
    assert!(player.is_playing());

    // Update again
    player.update(0.2);
    assert_eq!(player.current_time(), 0.5);
}

/// Test timeline player stop and reset
#[test]
fn test_timeline_player_stop_reset() {
    let mut sequence = TimelineSequence::new("stop_test".to_string());

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("step1".to_string(), target, 1.0);
    sequence.add_step(step);

    let mut player = TimelinePlayer::new(sequence);
    player.play();

    // Update a bit
    player.update(0.7);
    assert_eq!(player.current_time(), 0.7);

    // Stop
    player.stop();
    assert!(!player.is_playing());
    assert!(!player.is_paused());
    assert_eq!(player.current_step, 0);
    assert_eq!(player.current_time(), 0.0);
}

/// Test timeline player with repeat
#[test]
fn test_timeline_player_repeat() {
    let mut sequence = TimelineSequence::new("repeat_test".to_string());

    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("step1".to_string(), target, 0.5);
    sequence.add_step(step);

    sequence.set_repeat(true);
    let mut player = TimelinePlayer::new(sequence);
    player.play();

    // Complete first loop
    let completed = player.update(0.5);
    assert!(!completed);
    assert_eq!(player.loop_count(), 1);
    assert_eq!(player.current_step, 0);
    assert_eq!(player.current_time(), 0.0);

    // Complete second loop
    let completed = player.update(0.5);
    assert!(!completed);
    assert_eq!(player.loop_count(), 2);
}

/// Test timeline player seek functionality
#[test]
fn test_timeline_player_seek() {
    let mut sequence = TimelineSequence::new("seek_test".to_string());

    // Add steps with different durations
    let mut target1 = HashMap::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step1 = TimelineStep::new("step1".to_string(), target1, 1.0);
    sequence.add_step(step1);

    let mut target2 = HashMap::new();
    target2.insert("scale".to_string(), AnimationValue::Number(1.2));
    let step2 = TimelineStep::new("step2".to_string(), target2, 1.0);
    sequence.add_step(step2);

    let mut player = TimelinePlayer::new(sequence);

    // Seek to middle of first step
    player.seek(0.5);
    assert_eq!(player.current_time(), 0.5);
    assert_eq!(player.current_step, 0);

    // Seek to second step
    player.seek(1.5);
    assert_eq!(player.current_time(), 1.5);
    assert_eq!(player.current_step, 1);
}

/// Test timeline sequence performance with many steps
#[test]
fn test_timeline_sequence_performance() {
    let mut sequence = TimelineSequence::new("performance_test".to_string());

    // Add many steps
    for i in 0..1000 {
        let mut target = HashMap::new();
        target.insert(
            "opacity".to_string(),
            AnimationValue::Number(i as f64 / 1000.0),
        );

        let step = TimelineStep::new(format!("step{}", i), target, 0.1);
        sequence.add_step(step);
    }

    // Should have 1000 steps
    assert_eq!(sequence.step_count(), 1000);

    // Should be able to access steps quickly
    assert!(sequence.get_step("step500").is_some());
    let step = sequence.get_step("step500").unwrap();
    assert_eq!(
        step.target.get("opacity").unwrap(),
        &AnimationValue::Number(0.5)
    );
}

/// Test timeline player with complex sequence
#[test]
fn test_timeline_player_complex_sequence() {
    let mut sequence = TimelineSequence::new("complex_test".to_string());

    // Add steps with different properties
    let mut target1 = HashMap::new();
    target1.insert("opacity".to_string(), AnimationValue::Number(1.0));
    target1.insert("scale".to_string(), AnimationValue::Number(1.0));
    let step1 = TimelineStep::new("step1".to_string(), target1, 0.5).with_delay(0.1);
    sequence.add_step(step1);

    let mut target2 = HashMap::new();
    target2.insert("opacity".to_string(), AnimationValue::Number(0.5));
    target2.insert("scale".to_string(), AnimationValue::Number(1.2));
    target2.insert("rotateZ".to_string(), AnimationValue::Number(45.0));
    let step2 = TimelineStep::new("step2".to_string(), target2, 0.8).with_delay(0.2);
    sequence.add_step(step2);

    let mut player = TimelinePlayer::new(sequence);
    player.play();

    // Should start with first step
    let current_step = player.current_step().unwrap();
    assert_eq!(current_step.id, "step1");
    assert_eq!(
        current_step.target.get("opacity").unwrap(),
        &AnimationValue::Number(1.0)
    );

    // Update to second step
    player.update(0.7); // 0.1 delay + 0.5 duration + 0.1 buffer
    let current_step = player.current_step().unwrap();
    assert_eq!(current_step.id, "step2");
    assert_eq!(
        current_step.target.get("rotateZ").unwrap(),
        &AnimationValue::Number(45.0)
    );
}

/// Test timeline sequence edge cases
#[test]
fn test_timeline_sequence_edge_cases() {
    let mut sequence = TimelineSequence::new("edge_cases".to_string());

    // Test empty sequence
    assert_eq!(sequence.step_count(), 0);
    assert_eq!(sequence.total_duration, 0.0);

    // Test step with zero duration
    let mut target = HashMap::new();
    target.insert("opacity".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("zero_duration".to_string(), target, 0.0);
    sequence.add_step(step);

    assert_eq!(sequence.total_duration, 0.0);

    // Test step with negative delay
    let mut target = HashMap::new();
    target.insert("scale".to_string(), AnimationValue::Number(1.0));
    let step = TimelineStep::new("negative_delay".to_string(), target, 1.0).with_delay(-0.5);
    sequence.add_step(step);

    // Should handle negative delay gracefully
    let step = sequence.get_step("negative_delay").unwrap();
    assert_eq!(step.delay, -0.5);
}
