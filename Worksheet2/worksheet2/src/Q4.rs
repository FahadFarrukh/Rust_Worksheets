#[derive(Debug)]
enum Option {
    Text(String),
    Number(u32),
}

fn print(option: Option) {
    match option {
        Option::Text(text) => println!("Text: {}", text),
        Option::Number(num) => println!("Number: {}", num),
    }
}

fn main() {
    let text_option = Option::Text(String::from("Hello, Rust!"));
    let number_option = Option::Number(42);

    print(text_option);
    print(number_option);
}
