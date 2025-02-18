use std::f64::consts::PI;
use super::area::Area;

pub struct Circle {
    pub radius: f64,
    pub x: f64,
    pub y: f64
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

impl Default for Circle {
    fn default() -> Self {
        return Circle {
            radius: 10.0,
            x: 0.0,
            y: 0.0
        }
    }
}
