use image::{ImageBuffer, RgbImage};
use std::ops::{Add, Div, Mul, Sub};

fn map_range<T: Copy>(s: T, from_range: (T, T), to_range: (T, T)) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T>,
{
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
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

            let ir = map_range(r, (0.0, 1.0), (0.0, 255.999)) as u8;
            let ig = map_range(g, (0.0, 1.0), (0.0, 255.999)) as u8;
            let ib = map_range(b, (0.0, 1.0), (0.0, 255.999)) as u8;

            // ImageBuffer's origin is top-left, or (0, image_height), so subtract y to move it
            let pixel = image_buffer.get_pixel_mut(x, (image_height - y) - 1);
            *pixel = image::Rgb([ir, ig, ib]);
            // println!("{ir} {ig} {ib}");
        }
    }

    image_buffer
        .save("output.png")
        .expect("Error writing to file");
}
