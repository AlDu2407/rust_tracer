use super::{
    hittable::HitRecord,
    ray::Ray,
    utility::Color,
    vec3::{random_unit_vector, reflect, unit_vector},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum MaterialType {
    NONE,
    LAMBERTIAN(Color),
    METAL(Color),
}

impl MaterialType {
    pub fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialType::LAMBERTIAN(c) => {
                MaterialType::lambertian_scatter(&c, hit_record, attenuation, scattered)
            }
            MaterialType::METAL(c) => {
                MaterialType::metal_scatter(&c, ray_in, hit_record, attenuation, scattered)
            }
            MaterialType::NONE => false,
        }
    }

    fn lambertian_scatter(
        color: &Color,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        *scattered = Ray::from(hit_record.pt, scatter_direction);
        *attenuation = *color;
        true
    }

    fn metal_scatter(
        color: &Color,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(*ray_in.direction()), &hit_record.normal);
        *scattered = Ray::from(hit_record.pt, reflected);
        *attenuation = *color;
        true
    }
}
