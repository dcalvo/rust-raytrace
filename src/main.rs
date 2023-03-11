mod ray;

use crate::ray::Ray;
use glam::{vec3, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use std::ops::{Add, Div, Mul, Sub};

type Point3 = Vec3;
type Color = Vec3;

fn map_range<T: Copy>(s: T, from_range: (T, T), to_range: (T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn lerp(t: f32, start_value: Vec3, end_value: Vec3) -> Vec3 {
    assert!(t >= 0.0, "t is less than zero");
    assert!(t <= 1.0, "t is greater than one");
    (1.0 - t) * start_value + t * end_value
}

fn vec_to_pixel(vec: Color) -> Rgb<u8> {
    let r = map_range(vec.x, (0.0, 1.0), (0.0, 255.999)) as u8;
    let g = map_range(vec.y, (0.0, 1.0), (0.0, 255.999)) as u8;
    let b = map_range(vec.z, (0.0, 1.0), (0.0, 255.999)) as u8;

    Rgb([r, g, b])
}

fn ray_color(r: &Ray) -> Color {
    let unit_direction = r.direction.normalize();
    let t = 0.5 * (unit_direction.y + 1.0);
    let white: Color = vec3(1.0, 1.0, 1.0);
    let sky_blue: Color = vec3(0.5, 0.7, 1.0);

    lerp(t, white, sky_blue)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1280;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin: Point3 = vec3(0.0, 0.0, 0.0);
    let horizontal = vec3(viewport_width, 0.0, 0.0);
    let vertical = vec3(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - vec3(0.0, 0.0, focal_length);

    // Render

    let mut image_buffer: RgbImage = ImageBuffer::new(image_width, image_height);

    // Write image data, left to write, top to bottom
    for y in (0..image_height).rev() {
        println!("Scanlines remaining: {y}");
        for x in 0..image_width {
            let u = x as f32 / (image_width - 1) as f32;
            let v = y as f32 / (image_height - 1) as f32;

            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            let color = ray_color(&r);

            // ImageBuffer's origin is top-left, or (0, image_height), so subtract y to move it
            let pixel = image_buffer.get_pixel_mut(x, (image_height - y) - 1);
            *pixel = vec_to_pixel(color);
        }
    }

    image_buffer
        .save("output.png")
        .expect("Error writing to file");
}
