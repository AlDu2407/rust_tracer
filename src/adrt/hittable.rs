use crate::adrt::interval::Interval;
use crate::adrt::utility::Point;

use super::{
    material::MaterialType,
    ray::Ray,
    vec3::{dot_product, Vec3},
};

#[derive(Debug, Clone, Copy)]
pub struct HitRecord {
    pub pt: Point,
    pub normal: Vec3,
    pub material: MaterialType,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            pt: Point::new(),
            normal: Vec3::new(),
            material: MaterialType::NONE,
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_noramal` is assumed to have unit length.

        self.front_face = dot_product(ray.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &Interval, record: &mut HitRecord) -> bool;
}
