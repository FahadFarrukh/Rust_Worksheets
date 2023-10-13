fn main() {
    let string1 = "abcde";
    let string2 = "helloooo";

    let first_char1 = &string1[0..1]; 
    let last_char1 = &string1[string1.len() - 1..]; 

    let first_char2 = &string2[0..1]; 
    let last_char2 = &string2[string2.len() - 1..]; 

    println!("String 1: {}", string1);
    println!("First character of String 1: {}", first_char1);
    println!("Last character of String 1: {}", last_char1);

    println!("String 2: {}", string2);
    println!("First character of String 2: {}", first_char2);
    println!("Last character of String 2: {}", last_char2);




}
