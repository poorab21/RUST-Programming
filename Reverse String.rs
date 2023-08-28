use std::io;

fn main(){
    let mut input = String::from("");
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong");
    
    let mut input : String = input.chars().rev().collect();
    println!("{}",input);
    
}