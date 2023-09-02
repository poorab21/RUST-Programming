use std::io;

fn main() {
    let mut input = String::from("");
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    
    let input : String = input.chars().map(|x| {
        if x == 'z' { 'a' }
        else if x == 'Z' { 'A' } 
        else if x >= 'a' && x < 'z' { ((x as u8) + 1) as char }
        else if x >= 'A' && x < 'Z' { ((x as u8) + 1) as char }
        else { x }
    }).collect();
    
    println!("{}",input);
}