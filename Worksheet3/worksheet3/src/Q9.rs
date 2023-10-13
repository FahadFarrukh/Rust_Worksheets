trait Area {
    fn calculate_area(&self) -> f64;
}

trait Perimeter {
    fn calculate_perimeter(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn calculate_area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Perimeter for Circle {
    fn calculate_perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Area for Rectangle {
    fn calculate_area(&self) -> f64 {
        self.width * self.height
    }
}

impl Perimeter for Rectangle {
    fn calculate_perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let rectangle = Rectangle {
        width: 4.0,
        height: 3.0,
    };

    println!("Circle area: {}", circle.calculate_area());
    println!("Circle perimeter: {}", circle.calculate_perimeter());

    println!("Rectangle area: {}", rectangle.calculate_area());
    println!("Rectangle perimeter: {}", rectangle.calculate_perimeter());
}
