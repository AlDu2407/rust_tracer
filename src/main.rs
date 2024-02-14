mod adrt;

use crate::adrt::{utility::Point, vec3::Vec3};

use self::adrt::utility::{write_color, Color};
use adrt::{
    ray::Ray,
    vec3::{dot, dot_ref, unit_vector},
};
use indicatif::ProgressBar;
use std::{fs::File, io::Write};

fn hit_sphere_bool(center: &Point, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - center;
    let a = dot_ref(r.direction(), r.direction());
    let b = 2.0 * dot_ref(&oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc: Vec3 = r.origin() - center;
    let a = dot_ref(r.direction(), r.direction());
    let b = 2.0 * dot_ref(&oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        (-b - f64::sqrt(discriminant)) / (2.0 * a)
    } else {
        -1.0
    }
}

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point::from(0.0, 0.0, -1.0), 0.5, ray);
    if hit_sphere_bool(&Point::from(0.0, 0.0, -1.0), 0.5, ray) {
        let n = unit_vector(ray.at(t) - Vec3::from(0.0, 0.0, 1.0));
        return 0.5 * Color::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }
    let unit_direction = unit_vector(*ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Open file or creat new image file
    let file_path = "image.ppm";

    let mut file = match File::create(file_path) {
        Ok(f) => f,
        Err(_) => panic!("ERROR: could not open ${file_path}"),
    };

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };
    let duration: u64 = (image_width * image_height) as u64;
    let bar = ProgressBar::new(duration);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::from(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_width, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left =
        camera_center - Vec3::from(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("\n\nStarting render for rust tracer...");
    file.write(format!("P3\n{image_width} {image_height}\n255\n").as_bytes())?;

    for j in 0..image_height {
        for i in 0..image_width {
            bar.inc(1);

            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut file, pixel_color)?;
        }
    }
    file.flush()?;
    bar.finish();

    println!("Rendering finished successfully!");
    Ok(())
}
