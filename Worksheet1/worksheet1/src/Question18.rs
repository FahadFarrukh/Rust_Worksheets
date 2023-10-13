fn count_substring_occurrences(main_string: &str, substring: &str) -> usize {
    main_string
        .match_indices(substring)
        .count()
}

fn main() {
    let main_string = "Hello, hello, hello! How many hellos can you find in this hello world?";
    let substring = "hello";
    
    let count = count_substring_occurrences(main_string, substring);
    
    println!("The substring '{}' appears {} times in the main string.", substring, count);
}
