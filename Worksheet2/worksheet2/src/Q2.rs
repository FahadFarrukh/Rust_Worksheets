enum Color {
    Red,
    Green,
    Blue,
}

// Define a function to convert Color to RGB
fn color_to_rgb(color: Color) -> (u8, u8, u8) {
    match color {
        Color::Red => (255, 0, 0),
        Color::Green => (0, 255, 0),
        Color::Blue => (0, 0, 255),
    }
}

fn main() {
    let red_rgb = color_to_rgb(Color::Red);
    let green_rgb = color_to_rgb(Color::Green);
    let blue_rgb = color_to_rgb(Color::Blue);

    println!("Red: {:?}", red_rgb);
    println!("Green: {:?}", green_rgb);
    println!("Blue: {:?}", blue_rgb);
}

