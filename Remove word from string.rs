use std::io;

fn main() {
    let mut input = String::new();
    let mut newString = String::new();
    let mut wordToRemove = "Exercises";
    
    println!("Enter string:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    for word in input.trim().split(" ") {
        if word != wordToRemove {
            newString += format!("{} ",word).as_str();
        }
    }
    
    println!("{}",newString);
}