fn parse_as_integer(input: &str) -> Option<i32> {
    input.parse().ok()
}

fn main() {
    let input = "42";
    match parse_as_integer(input) {
        Some(num) => println!("Parsed integer: {}", num),
        None => println!("Failed to parse as an integer"),
    }
}
