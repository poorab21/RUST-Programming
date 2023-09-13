use std::io;

fn main() {
    let mut s = String::new();
    let mut i = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();
    
    while i < s.len() {
        if isVowel(s.chars().nth(i).unwrap()) {
            s.replace_range(i.. i+1,"");
            continue;
        }
        i += 1;
    }
    println!("String without vowels = {}",s);
}

fn isVowel( c : char ) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' {
        return true;
    }
    false
}