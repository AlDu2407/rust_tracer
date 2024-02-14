use super::vec3::Vec3;
use std::fs::File;
use std::io::Write;

pub type Color = Vec3;
pub type Point = Vec3;

pub fn write_color(file: &mut File, pixel_color: Color) -> std::io::Result<()> {
    let r = (255.999 * pixel_color.x()) as i64;
    let g = (255.999 * pixel_color.y()) as i64;
    let b = (255.999 * pixel_color.z()) as i64;

    file.write(format!("{r} {g} {b}\n").as_bytes())?;
    Ok(())
}
