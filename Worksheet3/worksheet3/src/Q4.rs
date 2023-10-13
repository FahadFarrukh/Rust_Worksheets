use std::fs;

fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn main() {
    let file_path = "sample.txt";
    match read_file(file_path) {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
