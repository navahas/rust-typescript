use std::f64::consts::PI;

struct Rectangle {
    height: f64,
    width: f64,
    x: f64,
    y: f64
}

struct Circle {
    radius: f64,
    x: f64,
    y: f64
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        return self.width * self.height;
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI;
    }
}

fn main() {
    let rectangle = Rectangle {
        height: 10.0,
        width: 10.0,
        x: 0.0,
        y: 0.0,
    };

    let circle = Circle {
        radius: 10.0,
        x: 0.0,
        y: 0.0
    };

    println!("{}", rectangle.area());
    println!("{}", circle.area());
}
