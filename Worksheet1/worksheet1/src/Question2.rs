use std::io;

fn main() {
   let mut name = String::new();

  
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");


    name = name.trim().to_string();
    println!("Hello {}! Welcome to the Rust program by JBS.", name);
    
}
