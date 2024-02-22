mod adrt;

use crate::adrt::{hittable_list::HittableList, sphere::Sphere, utility::Point};

use adrt::{camera::Camera, material::MaterialType, utility::Color};

fn main() -> std::io::Result<()> {
    let file_path = "image.ppm";

    let material_ground = MaterialType::LAMBERTIAN(Color::from(0.8, 0.8, 0.0));
    let material_center = MaterialType::LAMBERTIAN(Color::from(0.7, 0.3, 0.3));
    let material_left = MaterialType::METAL(Color::from(0.8, 0.8, 0.8));
    let material_right = MaterialType::METAL(Color::from(0.8, 0.6, 0.2));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(
        Point::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::from(
        Point::from(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::from(
        Point::from(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::from(
        Point::from(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut camera = Camera::new();

    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 50;
    camera.max_depth = 10;

    camera.render(&file_path.to_string(), &world);

    Ok(())
}
