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

fn vec_to_pixel(vec: Color) -> Rgb<u8> {
    let r = map_range(vec.x, (0.0, 1.0), (0.0, 255.999)) as u8;
    let g = map_range(vec.y, (0.0, 1.0), (0.0, 255.999)) as u8;
    let b = map_range(vec.z, (0.0, 1.0), (0.0, 255.999)) as u8;

    image::Rgb([r, g, b])
}

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut image_buffer: RgbImage = ImageBuffer::new(image_width, image_height);

    // Write image data, left to write, top to bottom
    for y in (0..image_height).rev() {
        println!("Scanlines remaining: {y}");
        for x in 0..image_width {
            let r = x as f32 / (image_width - 1) as f32;
            let g = y as f32 / (image_height - 1) as f32;
            let b = 0.25;

            let vec: Color = vec3(r, g, b);

            // ImageBuffer's origin is top-left, or (0, image_height), so subtract y to move it
            let pixel = image_buffer.get_pixel_mut(x, (image_height - y) - 1);
            *pixel = vec_to_pixel(vec);
        }
    }

    image_buffer
        .save("output.png")
        .expect("Error writing to file");
}
