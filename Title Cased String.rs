use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    if input.len() <= 0 { return; }
    
    for word in input.trim().split(" ") {
        if word.chars().nth(0).unwrap() < 'A' || word.chars().nth(0).unwrap() > 'Z' {
            println!("{} is not a title cased string",input);
            return;
        }
    }
    println!("{} is a title cased string",input);
}