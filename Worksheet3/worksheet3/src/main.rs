use std::collections::HashMap;

fn filter_students_above_threshold(students: &mut HashMap<String, i32>, threshold: i32) {
    students.retain(|_, score| *score > threshold);
}


fn main() {
    let mut students = HashMap::new();
    students.insert("Alice".to_string(), 85);
    students.insert("Bob".to_string(), 72);
    students.insert("Charlie".to_string(), 90);
    students.insert("David".to_string(), 60);

    let threshold = 80;
    filter_students_above_threshold(&mut students, threshold);
    println!("{:?}", students);
}
