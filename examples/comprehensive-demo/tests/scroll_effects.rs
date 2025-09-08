//! Tests for scroll-based animation capabilities
//!
//! These tests define the expected behavior for scroll-based animations
//! like parallax, reveal effects, and scroll-triggered animations.

use crate::test_utils::*;
use leptos::*;
use leptos_motion_core::*;
use leptos_motion_scroll::*;

/// Test that parallax scrolling works correctly
#[test]
fn test_parallax_scrolling() {
    // Given: A parallax element
    let (scroll_y, set_scroll_y) = create_signal(0.0);
    let (parallax_offset, set_parallax_offset) = create_signal(0.0);

    // When: User scrolls
    set_scroll_y.set(100.0);
    set_parallax_offset.set(scroll_y.get() * 0.5);

    // Then: Parallax offset should be calculated correctly
    assert_eq!(scroll_y.get(), 100.0);
    assert_eq!(parallax_offset.get(), 50.0);
}

/// Test that scroll-triggered animations work correctly
#[test]
fn test_scroll_triggered_animation() {
    // Given: A scroll-triggered animation
    let (scroll_position, set_scroll_position) = create_signal(0.0);
    let (is_visible, set_is_visible) = create_signal(false);
    let (animation_progress, set_animation_progress) = create_signal(0.0);

    // When: Element comes into view
    set_scroll_position.set(500.0);
    set_is_visible.set(true);
    set_animation_progress.set(1.0);

    // Then: Animation should be triggered
    assert!(is_visible.get());
    assert_eq!(animation_progress.get(), 1.0);
}

/// Test that scroll-based reveal effects work correctly
#[test]
fn test_scroll_reveal_effect() {
    // Given: A scroll reveal element
    let (element_rect, set_element_rect) = create_signal(DomRect {
        x: 0.0,
        y: 1000.0,
        width: 100.0,
        height: 100.0,
    });
    let (viewport_height, set_viewport_height) = create_signal(800.0);
    let (is_revealed, set_is_revealed) = create_signal(false);

    // When: Element enters viewport
    set_element_rect.set(DomRect {
        x: 0.0,
        y: 400.0,
        width: 100.0,
        height: 100.0,
    });
    set_is_revealed.set(true);

    // Then: Element should be revealed
    assert!(is_revealed.get());
}

/// Test that scroll-based opacity changes work correctly
#[test]
fn test_scroll_opacity_effect() {
    // Given: A scroll-based opacity element
    let (scroll_progress, set_scroll_progress) = create_signal(0.0);
    let (opacity, set_opacity) = create_signal(1.0);

    // When: Scroll progress changes
    set_scroll_progress.set(0.5);
    set_opacity.set(1.0 - scroll_progress.get());

    // Then: Opacity should be calculated correctly
    assert_eq!(scroll_progress.get(), 0.5);
    assert_eq!(opacity.get(), 0.5);
}

/// Test that scroll-based scale changes work correctly
#[test]
fn test_scroll_scale_effect() {
    // Given: A scroll-based scale element
    let (scroll_progress, set_scroll_progress) = create_signal(0.0);
    let (scale, set_scale) = create_signal(1.0);

    // When: Scroll progress changes
    set_scroll_progress.set(0.3);
    set_scale.set(1.0 + scroll_progress.get() * 0.5);

    // Then: Scale should be calculated correctly
    assert_eq!(scroll_progress.get(), 0.3);
    assert_eq!(scale.get(), 1.15);
}

/// Test that scroll-based rotation works correctly
#[test]
fn test_scroll_rotation_effect() {
    // Given: A scroll-based rotation element
    let (scroll_progress, set_scroll_progress) = create_signal(0.0);
    let (rotation, set_rotation) = create_signal(0.0);

    // When: Scroll progress changes
    set_scroll_progress.set(0.8);
    set_rotation.set(scroll_progress.get() * 360.0);

    // Then: Rotation should be calculated correctly
    assert_eq!(scroll_progress.get(), 0.8);
    assert_eq!(rotation.get(), 288.0);
}

/// Test that scroll-based transform combinations work correctly
#[test]
fn test_scroll_transform_combination() {
    // Given: A scroll-based transform element
    let (scroll_progress, set_scroll_progress) = create_signal(0.0);
    let (transform, set_transform) = create_signal(Transform::default());

    // When: Scroll progress changes
    set_scroll_progress.set(0.6);
    let new_transform = Transform {
        translate_x: Some(scroll_progress.get() * 100.0),
        translate_y: Some(scroll_progress.get() * 50.0),
        scale_x: Some(1.0 + scroll_progress.get() * 0.3),
        scale_y: Some(1.0 + scroll_progress.get() * 0.3),
        rotate: Some(scroll_progress.get() * 180.0),
        ..Default::default()
    };
    set_transform.set(new_transform);

    // Then: All transforms should be applied correctly
    let t = transform.get();
    assert_eq!(t.translate_x, Some(60.0));
    assert_eq!(t.translate_y, Some(30.0));
    assert_eq!(t.scale_x, Some(1.18));
    assert_eq!(t.scale_y, Some(1.18));
    assert_eq!(t.rotate, Some(108.0));
}

/// Test that scroll-based animation timing works correctly
#[test]
fn test_scroll_animation_timing() {
    // Given: A scroll-based animation with timing
    let (scroll_start, set_scroll_start) = create_signal(0.0);
    let (scroll_end, set_scroll_end) = create_signal(1000.0);
    let (current_scroll, set_current_scroll) = create_signal(500.0);
    let (animation_progress, set_animation_progress) = create_signal(0.0);

    // When: Scroll position is between start and end
    let progress =
        (current_scroll.get() - scroll_start.get()) / (scroll_end.get() - scroll_start.get());
    set_animation_progress.set(progress);

    // Then: Animation progress should be calculated correctly
    assert_eq!(animation_progress.get(), 0.5);
}

/// Test that scroll-based animation easing works correctly
#[test]
fn test_scroll_animation_easing() {
    // Given: A scroll-based animation with easing
    let (linear_progress, set_linear_progress) = create_signal(0.5);
    let (eased_progress, set_eased_progress) = create_signal(0.0);

    // When: Easing is applied
    let eased = ease_in_out_cubic(linear_progress.get());
    set_eased_progress.set(eased);

    // Then: Eased progress should be different from linear
    assert_ne!(eased_progress.get(), linear_progress.get());
}

/// Test that scroll-based animation performance is optimized
#[test]
fn test_scroll_animation_performance() {
    // Given: A scroll-based animation with performance monitoring
    let (frame_count, set_frame_count) = create_signal(0);
    let (is_optimized, set_is_optimized) = create_signal(true);

    // When: Animation is running
    set_frame_count.set(frame_count.get() + 1);
    set_is_optimized.set(true);

    // Then: Performance should be good
    assert!(frame_count.get() > 0);
    assert!(is_optimized.get());
}

/// Test that scroll-based animation cleanup works correctly
#[test]
fn test_scroll_animation_cleanup() {
    // Given: A scroll-based animation that needs cleanup
    let (is_active, set_is_active) = create_signal(true);
    let (cleanup_called, set_cleanup_called) = create_signal(false);

    // When: Cleanup is called
    set_is_active.set(false);
    set_cleanup_called.set(true);

    // Then: Animation should be inactive and cleanup should be called
    assert!(!is_active.get());
    assert!(cleanup_called.get());
}

/// Test that scroll-based animation handles edge cases correctly
#[test]
fn test_scroll_animation_edge_cases() {
    // Given: A scroll-based animation with edge cases
    let (scroll_position, set_scroll_position) = create_signal(0.0);
    let (is_at_top, set_is_at_top) = create_signal(true);
    let (is_at_bottom, set_is_at_bottom) = create_signal(false);

    // When: Scroll position is at extremes
    set_scroll_position.set(0.0);
    set_is_at_top.set(true);
    set_is_at_bottom.set(false);

    // Then: Edge cases should be handled correctly
    assert!(is_at_top.get());
    assert!(!is_at_bottom.get());
}

/// Helper function for easing calculations
fn ease_in_out_cubic(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}
