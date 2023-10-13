#[derive(Debug)]
struct Book {
title: String,

}
impl Book {
    fn new(title: String) -> Self {
    Self{
        title,
        }
    }
}

fn main() {
    let book_1 = Book::new(String::from("Diary of a Wimpy Kid"));
    println!("{:#?}",book_1);
 
}
