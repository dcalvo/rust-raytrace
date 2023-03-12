use glam::Vec3;
use std::ops::{Add, Div, Mul, Sub};

pub fn map_range<T: Copy>(s: T, from_range: (T, T), to_range: (T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

pub fn lerp(t: f32, start_value: Vec3, end_value: Vec3) -> Vec3 {
    assert!(t >= 0.0, "t is less than zero");
    assert!(t <= 1.0, "t is greater than one");
    (1.0 - t) * start_value + t * end_value
}

pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}
