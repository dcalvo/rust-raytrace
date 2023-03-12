mod camera;
mod ray;
mod sphere;
mod utils;

use crate::camera::Camera;
use crate::ray::{HitRecord, Hittable, Ray};
use crate::sphere::Sphere;
use crate::utils::{clamp, lerp, map_range, random_unit_vector};
use glam::{vec3, Vec3};
use image::{ImageBuffer, Rgb, RgbImage};
use rand::random;

type Point3 = Vec3;
type Color = Vec3;

fn vec_to_pixel(vec: Color) -> Rgb<u8> {
    // Gamma correction
    let r = vec.x.sqrt();
    let g = vec.y.sqrt();
    let b = vec.z.sqrt();

    // Map from [0, 1] to [0, 255)
    let r = map_range(r, (0.0, 1.0), (0.0, 255.999)) as u8;
    let g = map_range(g, (0.0, 1.0), (0.0, 255.999)) as u8;
    let b = map_range(b, (0.0, 1.0), (0.0, 255.999)) as u8;

    // Clamp between 0 and 255
    let r = clamp(r, 0, 255);
    let g = clamp(g, 0, 255);
    let b = clamp(b, 0, 255);

    Rgb([r, g, b])
}

fn hit_world(r: &Ray, world: &Vec<Box<dyn Hittable>>, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut closest_so_far = t_max;
    let mut hit_record = None;
    for geo in world {
        if let Some(hit) = geo.hit(r, t_min, closest_so_far) {
            closest_so_far = hit.t;
            hit_record = Some(hit);
        }
    }
    hit_record
}

fn ray_color(r: &Ray, world: &Vec<Box<dyn Hittable>>, depth: i32) -> Color {
    if depth <= 0 {
        return Color::ZERO;
    }

    let hit = hit_world(r, world, 0.001, f32::INFINITY);
    match hit {
        Some(hit_record) => {
            let scatter_direction = hit_record.normal + random_unit_vector();
            let scatter_ray = Ray {
                origin: hit_record.p,
                direction: scatter_direction,
            };
            0.5 * ray_color(&scatter_ray, world, depth - 1)
        }
        None => {
            // Skybox
            let unit_direction = r.direction.normalize();
            let t = 0.5 * (unit_direction.y + 1.0);
            let white: Color = Color::ONE;
            let sky_blue: Color = vec3(0.5, 0.7, 1.0);

            lerp(t, white, sky_blue)
        }
    }
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (image_width as f32 / aspect_ratio) as u32;
    let rays_per_pixel = 100;
    let max_depth = 50;

    // Camera
    let camera = Camera::new();

    // World
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere {
            center: vec3(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: vec3(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    // Render

    let mut image_buffer: RgbImage = ImageBuffer::new(image_width, image_height);

    // Write image data, left to write, top to bottom
    for y in (0..image_height).rev() {
        println!("Scanlines remaining: {y}");
        for x in 0..image_width {
            // Anti-aliasing
            let mut color: Color = Color::ZERO;
            for _ in 0..rays_per_pixel {
                let u = (x as f32 + random::<f32>()) / (image_width - 1) as f32;
                let v = (y as f32 + random::<f32>()) / (image_height - 1) as f32;

                let r = camera.get_ray(u, v);
                color += ray_color(&r, &world, max_depth);
            }

            color /= rays_per_pixel as f32;

            // ImageBuffer's origin is top-left, or (0, image_height), so subtract y to move it
            let pixel = image_buffer.get_pixel_mut(x, (image_height - y) - 1);
            *pixel = vec_to_pixel(color);
        }
    }

    image_buffer
        .save("output.png")
        .expect("Error writing to file");
}
