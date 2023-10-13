fn find_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let string1 = "abcde";
    let string2 = "helloooo";

    let longest = find_longest(string1, string2);
    
    println!("The longest string is: {}", longest);
}
