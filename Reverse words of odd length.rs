use std::io;

fn main() {
    let mut input = String::new();
    let mut newString = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    for word in input.trim().split(" ") {
        if word.len() % 2 != 0 {
            newString += format!("{} ",word.chars().rev().collect::<String>().as_str()).as_str();
        }
        else {
            newString += format!("{} ",word).as_str();
        }
    }
    
    println!("{}",newString);
}