fn main() {
    let array = ['l', 'o', 'l'];
    let mut a = 0;
    let mut y = 2;
    let mut is_palindrome = true; 

    while y >= a {
        let z = array[y];
        let f = array[a];

        if z != f { 
            is_palindrome = false; 
            break; 
        }

        a = a + 1;
        y = y - 1;
    }

    if is_palindrome {
        println!("Palindrome");
    } else {
        println!("Not Palindrome");
    }
}
