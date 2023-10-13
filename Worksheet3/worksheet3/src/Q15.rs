trait Sortable {
    fn sort(&mut self);
}

impl Sortable for Vec<i32> {
    fn sort(&mut self) {
        self.sort();
    }
}

impl Sortable for Vec<String> {
    fn sort(&mut self) {
        self.sort();
    }
}

fn main() {
    let mut numbers = vec![4, 2, 8, 1, 7];
    let mut names = vec!["Alice", "Charlie", "Bob"];

    numbers.sort();
    names.sort();

    println!("{:?}", numbers);
    println!("{:?}", names);
}
