//! Mathematical utilities for animations



/// Clamp a value between min and max
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if min > max {
        panic!("min must be less than or equal to max");
    }
    value.max(min).min(max)
}

/// Map a value from one range to another
pub fn map_range(value: f64, from_min: f64, from_max: f64, to_min: f64, to_max: f64) -> f64 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    
    if from_range == 0.0 {
        return to_min;
    }
    
    let normalized = (value - from_min) / from_range;
    to_min + (normalized * to_range)
}

/// Calculate distance between two 2D points
pub fn distance_2d(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

/// Smooth step function (3t² - 2t³)
pub fn smooth_step(t: f64) -> f64 {
    let t = clamp(t, 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Smoother step function (6t⁵ - 15t⁴ + 10t³)
pub fn smoother_step(t: f64) -> f64 {
    let t = clamp(t, 0.0, 1.0);
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(15.0, 0.0, 10.0), 10.0);
    }
    
    #[test]
    fn test_map_range() {
        assert_relative_eq!(map_range(0.5, 0.0, 1.0, 0.0, 100.0), 50.0);
        assert_relative_eq!(map_range(2.0, 0.0, 4.0, 10.0, 20.0), 15.0);
        assert_relative_eq!(map_range(0.0, 0.0, 0.0, 0.0, 100.0), 0.0); // Edge case
    }
    
    #[test]
    fn test_distance_2d() {
        assert_relative_eq!(distance_2d(0.0, 0.0, 3.0, 4.0), 5.0);
        assert_relative_eq!(distance_2d(0.0, 0.0, 0.0, 0.0), 0.0);
    }
    
    #[test]
    fn test_smooth_step() {
        assert_eq!(smooth_step(0.0), 0.0);
        assert_eq!(smooth_step(1.0), 1.0);
        assert_relative_eq!(smooth_step(0.5), 0.5);
        
        // Should be smoother than linear
        let linear_quarter = 0.25;
        let smooth_quarter = smooth_step(0.25);
        assert!(smooth_quarter < linear_quarter);
    }
    
    #[test]
    fn test_smoother_step() {
        assert_eq!(smoother_step(0.0), 0.0);
        assert_eq!(smoother_step(1.0), 1.0);
        assert_relative_eq!(smoother_step(0.5), 0.5);
    }
}