trait Drawable {
    fn draw(&self);
}

struct Shape {
    name: String,
}

impl Drawable for Shape {
    fn draw(&self) {
        println!("Drawing a {}.", self.name);
    }
}

fn main() {
    let shape = Shape {
        name: String::from("Circle"),
    };
    shape.draw();
}
