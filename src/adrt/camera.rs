use std::{fs::File, io::Write};

use indicatif::ProgressBar;

use crate::adrt::{
    camera, hittable::HitRecord, interval::Interval, utility::write_color, vec3::unit_vector,
};

use super::{
    hittable::Hittable,
    ray::Ray,
    utility::{Color, Point},
    vec3::Vec3,
};

#[derive(Debug, Copy, Clone)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    camera_config: CameraConfig,
}

#[derive(Debug, Copy, Clone)]
struct CameraConfig {
    image_height: i32,
    center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 0.0,
            image_width: 0,
            camera_config: CameraConfig::new(),
        }
    }
    pub fn render(&mut self, file_path: &String, world: &impl Hittable) {
        let mut file = match File::create(file_path) {
            Ok(f) => f,
            Err(_) => panic!("ERROR: could not open ${file_path}"),
        };
        let camera_config = self.initialize();
        self.camera_config = camera_config;

        let duration: u64 = (self.image_width * self.camera_config.image_height) as u64;
        if duration == 0 {
            panic!("Duration cannot be null, check the image size!");
        }

        let bar = ProgressBar::new(duration);

        println!("\n\nStarting render for rust tracer...");
        file.write(
            format!(
                "P3\n{} {}\n255\n",
                self.image_width, self.camera_config.image_height
            )
            .as_bytes(),
        )
        .expect(format!("Cannot write to file '{}'", file_path).as_str());

        for j in 0..self.camera_config.image_height {
            for i in 0..self.image_width {
                bar.inc(1);

                let pixel_center = self.camera_config.pixel00_loc
                    + (i as f64 * self.camera_config.pixel_delta_u)
                    + (j as f64 * self.camera_config.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_config.center;
                let r = Ray::from(self.camera_config.center, ray_direction);

                let pixel_color = self.ray_color(&r, world);
                write_color(&mut file, pixel_color).expect(
                    format!("Cannot write {:?} to file '{}'", pixel_color, file_path).as_str(),
                );
            }
        }
        file.flush()
            .expect(format!("Could not flush data to '{}'", file_path).as_str());
        bar.finish_with_message("Rendering finished successfully!");
    }

    fn initialize(&self) -> CameraConfig {
        // Calculate the image height, and ensure that it's at least 1
        let mut image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        image_height = if image_height < 1 { 1 } else { image_height };
        let center = Point::from(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / image_height as f64);
        let camera_center = Point::from(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edgese
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center
            - Vec3::from(0.0, 0.0, focal_length)
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        CameraConfig::from(
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        )
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable) -> Color {
        let mut record = HitRecord::new();

        if world.hit(ray, &Interval::from(0.0, f64::INFINITY), &mut record) {
            return 0.5 * (record.normal + Color::from(1.0, 1.0, 1.0));
        }

        let unit_direction = unit_vector(*ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::from(1.0, 1.0, 1.0) + a * Color::from(0.5, 0.7, 1.0)
    }
}

impl CameraConfig {
    fn new() -> Self {
        Self {
            image_height: 0,
            center: Point::new(),
            pixel00_loc: Point::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
        }
    }

    fn from(
        image_height: i32,
        center: Point,
        pixel00_loc: Point,
        pixel_delta_u: Vec3,
        pixel_delta_v: Vec3,
    ) -> Self {
        Self {
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
