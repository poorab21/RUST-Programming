use std::io;
fn main() {
    let mut name = String::new();
    
    println!("Enter name:");
    io::stdin().read_line(&mut name).expect("Something went wrong while processing input for name");
    name.pop();
    
    println!("greetings {}",name);
}