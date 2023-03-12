use glam::{vec3, Vec3};
use rand::random;
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

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    let p = vec3(random::<f32>(), random::<f32>(), random::<f32>());
    let min_p = Vec3::splat(min);
    let max_p = Vec3::splat(max);
    p.clamp(min_p, max_p)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec3(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
}

pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
