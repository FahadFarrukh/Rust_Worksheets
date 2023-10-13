fn unique_words(sentence: &str) -> Vec<String> {
    let mut words: Vec<String> = sentence
        .split_whitespace()
        .map(|word| word.to_lowercase())
        .collect();

    words.sort();
    words.dedup();
    words
}

fn main() {
    let sentence = "This is a sample sentence with sample words";
    let unique = unique_words(sentence);
    println!("{:?}", unique);
}
