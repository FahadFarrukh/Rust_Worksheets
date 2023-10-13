#![allow(unused_variables)]
#[derive(Debug)]


struct Person {
name: String,
age: u32,
}
impl Person {
    fn new(name: String,age: u32,) -> Self {
    Self{
        name,
        age
        }
    }
}

fn main() {
    let person1 = Person::new(String::from("Fahad"),22);
    println!("{:#?}",person1);
 
}


