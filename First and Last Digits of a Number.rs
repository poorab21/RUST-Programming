use std::io;

fn main() {
    let mut number = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input for number");
    number.pop();

    println!("First digit = {}",number.chars().nth(0).unwrap());
    println!("Last digit = {}",number.chars().nth(number.len() -1).unwrap());
}