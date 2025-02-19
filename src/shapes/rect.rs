use std::fmt::Display;

use super::area::Area;

pub struct Rectangle {
    pub height: f64,
    pub width: f64,
    pub x: f64,
    pub y: f64
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Rectangle {
            x: 0.0,
            y: 0.0,
            height: 10.0,
            width: 10.0,
        }
    }
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Rectangle({}, {}): {}x{}", self.x, self.y, self.height, self.width);
    }
}

pub struct RectIter {
    points: [(f64, f64); 4],
    idx: usize
}

impl From<&Rectangle> for RectIter {
    fn from(rect: &Rectangle) -> Self {
        return RectIter {
            points: vec![
                (rect.x, rect.y),
                (rect.x + rect.width, rect.y),
                (rect.x, rect.y + rect.height),
                (rect.x + rect.width, rect.y + rect.height),
            ],
            idx: 0,
        }
    }
}

impl IntoIterator for Rectangle {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return (&self).into();
    }
}

impl IntoIterator for &Rectangle {
    type Item = (f64, f64);

    type IntoIter = RectIter;

    fn into_iter(self) -> Self::IntoIter {
        return self.into();
    }
}
