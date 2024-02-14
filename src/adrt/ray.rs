use super::{utility::Point, vec3::Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        &self.origin + t * &self.direction
    }
}
