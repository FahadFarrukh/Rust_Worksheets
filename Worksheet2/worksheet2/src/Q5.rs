fn append_world(input: &mut String) {
    input.push_str(" World!");
}

fn main() {
    let mut my_string = String::from("Hello");
    append_world(&mut my_string);
    println!("{}", my_string); // Prints "Hello World!"
}

