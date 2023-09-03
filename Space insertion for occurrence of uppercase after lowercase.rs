use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    for i in 0..input.len() - 1 {
        if isLowercase( input.chars().nth(i).unwrap() ) && isUppercase( input.chars().nth(i+1).unwrap() ) {
            input.insert(i+1,' ');
        }
    }
    
    println!("{}",input);
}

fn isUppercase( c : char ) -> bool {
    if c >= 'A' && c <= 'Z' {
        return true;
    }
    false
}

fn isLowercase( c : char ) -> bool {
    if c >= 'a' && c <= 'z' {
        return true;
    }
    false
}