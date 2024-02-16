use crate::adrt::vec3::Vec3;
use rand::rngs::ThreadRng;
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::io::Write;

use super::interval::Interval;

pub type Color = Vec3;
pub type Point = Vec3;

pub fn write_color(
    file: &mut File,
    pixel_color: Color,
    samples_per_pixel: i32,
) -> std::io::Result<()> {
    let scale = 1.0 / samples_per_pixel as f64;
    let intensity = Interval::from(0.000, 0.999);
    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    file.write(
        format!(
            "{} {} {}\n",
            (256.0 * intensity.clamp(r)) as u64,
            (256.0 * intensity.clamp(g)) as u64,
            (256.0 * intensity.clamp(b)) as u64
        )
        .as_bytes(),
    )?;
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
