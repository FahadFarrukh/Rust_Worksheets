fn reverse_string(input: &str) -> String {
    let reversed: String = input.chars().rev().collect();
    reversed
}

fn main() {
    let input_string = "Hello, World!";
    let reversed_string = reverse_string(input_string);
    println!("Original: {}", input_string);
    println!("Reversed: {}", reversed_string);
}