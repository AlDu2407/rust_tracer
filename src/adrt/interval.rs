pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = Self {
        min: f64::INFINITY,
        max: -f64::INFINITY,
    };

    pub const WORLD: Interval = Self {
        min: -f64::INFINITY,
        max: f64::INFINITY,
    };

    pub fn new() -> Self {
        Self {
            min: f64::INFINITY,
            max: -f64::INFINITY,
        }
    }

    pub fn from(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
