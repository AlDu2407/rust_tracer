mod adrt;

use crate::adrt::{hittable_list::HittableList, sphere::Sphere, utility::Point};

use adrt::camera::Camera;

fn main() -> std::io::Result<()> {
    let file_path = "image.ppm";

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(Point::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(
        Point::from(0.0, -100.5, -1.0),
        100.0,
    )));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 50;
    camera.max_depth = 10;

    camera.render(&file_path.to_string(), &world);

    Ok(())
}
