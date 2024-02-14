use super::{
    hittable::{HitRecord, Hittable},
    utility::Point,
    vec3::dot_product,
};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn from(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &super::ray::Ray,
        ray_tmin: f64,
        ray_tmax: f64,
        record: &mut HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot_product(&oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrt_disc = f64::sqrt(discriminant);
        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrt_disc) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (-half_b + sqrt_disc) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        record.t = root;
        record.pt = ray.at(record.t);
        let outward_normal = (record.pt - self.center) / self.radius;
        record.set_face_normal(ray, &outward_normal);

        true
    }
}
