use crate::adrt::vec3::Vec3;

use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};

use super::interval::Interval;

pub type Color = Vec3;
pub type Point = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    f64::sqrt(linear_component)
}

pub fn write_color(
    file: &mut BufWriter<File>,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let intensity = Interval::from(0.000, 0.999);
    let mut pixels = Vec::<u8>::with_capacity(3);
    pixels.push((intensity.clamp(linear_to_gamma(pixel_color.x() * scale)) * 255.0) as u8);
    pixels.push((intensity.clamp(linear_to_gamma(pixel_color.y() * scale)) * 255.0) as u8);
    pixels.push((intensity.clamp(linear_to_gamma(pixel_color.z() * scale)) * 255.0) as u8);

    file.write(&pixels)?;
    Ok(())
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

#[allow(dead_code)]
pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
