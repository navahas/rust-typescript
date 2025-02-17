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

impl Rectangle {
    fn new(height: f64, width: f64, x: f64, y: f64) -> Self {
        Self { height, width, x, y }
    }
    
    fn area(&self) -> f64 {
        let area = self.height * self.width;
        return area;
    }
}

impl Circle {
    fn new(radius: f64, x: f64, y: f64) -> Self {
        Self { radius, x, y }
    }
    
    fn area(&self) -> f64 {
        return self.radius * PI;
    }
}

fn main() {
    let rectangle = Rectangle::new(5.0, 10.0, 0.0, 0.0);
    let area = rectangle.area();
    println!("{}", area);

    let circle = Circle::new(12.0, 0.0, 0.0);
    let area_circle = circle.area();
    println!("{}", area_circle);
}
