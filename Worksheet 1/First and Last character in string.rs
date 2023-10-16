use std::io;

fn main() {
    let mut s = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();
    
    if s.len() == 0 { return; }
    
    println!("First character = {}",&s[0..1]);
    println!("Last character = {}",&s[s.len() - 1 .. s.len()]);
}