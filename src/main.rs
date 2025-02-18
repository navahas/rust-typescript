use shapes::{area::Area, circle::Circle, rect::Rectangle};

mod shapes;

fn main() {
    let rectangle = Rectangle::default();

    let circle = Circle::default();

    println!("{}", rectangle);
}
