use std::io;

fn main() {
    let mut input = String::new();
    let mut newString : String = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    let mut words = input.split(" ");
    
    for word in words {
        if word.len() >= 3 { 
            newString = format!("{} {}",newString,word.chars().rev().collect::<String>()).trim().to_string();
        }
        else { newString = format!("{} {}",newString,word).trim().to_string(); }
    }
    
    println!("{}",newString);
}