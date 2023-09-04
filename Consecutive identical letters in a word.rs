use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    
    for i in 0..input.len() - 1 {
        if input.chars().nth(i).unwrap() == input.chars().nth(i+1).unwrap() {
            println!("True");
            return;
        }
    }
    println!("False");
}