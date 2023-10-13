
fn main(){

    let mut y = 0;

    while y < 100 {
        y = y + 1;
        if y % 3 == 0 {
            println!("Fizz");
        }
        if y % 5 == 0 {
            println!("Buzz");
        }
        if y % 3 == 0 && y % 5 == 0 {
            println!("FizzBuzz");
        } else {
            println!("{y}");
        }
    }
}