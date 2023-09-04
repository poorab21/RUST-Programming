use std::io;

fn main() {
    let mut input = String::new();
    let c = 's';
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    for character in input.chars() {
        if character == c { count += 1; }
    }
    
    println!("Total Occurrances of {} = {}",c,count);
}