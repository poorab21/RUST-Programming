use std::io;

fn main() {
    let mut input = String::from("");
    let mut index = 0;
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    let mut input = input.as_str().split(" ");
    
    for (i,word) in input.clone().enumerate() {
        if word.len() > count { index = i; count = word.len(); }
    }
    
    println!("largest word = {}",input.nth(index).unwrap());
}