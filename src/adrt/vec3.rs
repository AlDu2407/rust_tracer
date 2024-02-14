use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}

pub fn dot_product(u: &Vec3, v: &Vec3) -> f64 {
    u.e0 * v.e0 + u.e1 * v.e1 + u.e2 * v.e2
}

#[allow(dead_code)]
pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::from(
        u.e1 * v.e2 - u.e2 * v.e1,
        u.e2 * v.e0 - u.e0 * v.e2,
        u.e0 * v.e1 - u.e1 * v.e0,
    )
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length();
    v / length
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Self {
            e0: 0.0,
            e2: 0.0,
            e1: 0.0,
        }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Self { e0, e1, e2 }
    }

    pub fn from_i32(e0: i32, e1: i32, e2: i32) -> Vec3 {
        Self {
            e0: e0 as f64,
            e1: e1 as f64,
            e2: e2 as f64,
        }
    }

    pub fn from_i64(e0: i64, e1: i64, e2: i64) -> Vec3 {
        Self {
            e0: e0 as f64,
            e1: e1 as f64,
            e2: e2 as f64,
        }
    }

    pub fn x(&self) -> f64 {
        self.e0
    }

    pub fn y(&self) -> f64 {
        self.e1
    }

    pub fn z(&self) -> f64 {
        self.e2
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            e0: self.e0 * (-1 as f64),
            e1: self.e1 * (-1 as f64),
            e2: self.e2 * (-1 as f64),
        }
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
        }
    }
}

impl<'a> ops::Add<&'a Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'a Vec3) -> Self::Output {
        *self + *rhs
    }
}

impl<'a> ops::Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        *self + &rhs
    }
}

impl<'a> ops::Add<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: &'a Vec3) -> Self::Output {
        self + *rhs
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e0 = self.e0 + rhs.e0;
        self.e1 = self.e1 + rhs.e1;
        self.e2 = self.e2 + rhs.e2;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
        }
    }
}

impl<'a> ops::Sub<&'a Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'a Vec3) -> Self::Output {
        *self - *rhs
    }
}

impl<'a> ops::Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        *self - &rhs
    }
}

impl<'a> ops::Sub<&'a Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'a Vec3) -> Self::Output {
        self - *rhs
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl<'a> ops::Mul<&'a Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &'a Vec3) -> Self::Output {
        *rhs * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e0 = self.e0 * rhs;
        self.e1 = self.e1 * rhs;
        self.e2 = self.e2 * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e0 = self.e0 / rhs;
        self.e1 = self.e1 / rhs;
        self.e2 = self.e2 / rhs;
    }
}

impl ops::Index<i32> for Vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            1 => &self.e0,
            2 => &self.e1,
            3 => &self.e2,
            _ => panic!("ERROR: IndexOutOufBound for {index}"),
        }
    }
}
