fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Division by zero is not allowed".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    let dividend = 10;
    let divisor = 0;
    match divide(dividend, divisor) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => eprintln!("Error: {}", error),
    }
}
