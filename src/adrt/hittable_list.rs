use crate::adrt::hittable::{HitRecord, Hittable};
use std::vec::Vec;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn from(object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &self,
        ray: &super::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        record: &mut HitRecord,
    ) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter() {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *record = temp_rec;
            }
        }

        hit_anything
    }
}
