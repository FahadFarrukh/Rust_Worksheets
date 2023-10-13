
#[derive(Debug)]
pub enum  Status{
    Active,
    Inactive,
    Suspended
}


#[derive(Debug)]
pub struct Book {
    title: String,
    author: String,
    pages: u32

}
impl Book {
    pub fn new(title: String,author: String,pages: u32) -> Self {
    Self{
        title,
        author,
        pages
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn author(&self) -> &String {
        &self.author
    }

    pub fn get_title(self) -> String {
        self.title
    }

    pub fn modify_title(&mut self, new_title: String) {
        self.title = new_title;
    }
}
pub fn book_status(title:String,status:Status)->(String,Status){

    match status {
        Status::Active=>(title,status),
        Status::Inactive=>(title,status),
        Status::Suspended=>(title,status),

    }

}

pub fn print_option(option: Option<i32>) {
    match option {
        Some(_) => println!("Has a value"),
        None => println!("No value"),
    }
}

