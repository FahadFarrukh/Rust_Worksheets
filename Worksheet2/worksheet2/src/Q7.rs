
#[derive(Debug)]
enum  Status{
    Active,
    Inactive,
    Suspended
}


#[derive(Debug)]
struct Book {
title: String,
author: String,
pages: u32

}
impl Book {
    fn new(title: String,author: String,pages: u32) -> Self {
    Self{
        title,
        author,
        pages
        }
    }
    fn title(&self) -> &String {
        &self.title
    }
}
fn book_status(title:String,status:Status)->(String,Status){

    match status {
        Status::Active=>(title,status),
        Status::Inactive=>(title,status),
        Status::Suspended=>(title,status),

    }

}
fn main() {
    let book_1 = Book::new(String::from("Diary of a Wimpy Kid"),String::from("Jeff Kinney"),244);
    println!("{:#?}",book_1);
    println!("\nTitle: {}", book_1.title());

    let status=book_status(String::from("Diary of a Wimpy Kid"), Status::Active);
    println!("\nStatus: {:#?}", status); 
}

