use std::io;

fn main() {
    let mut input = String::new();
    let mut uppercase = false;
    let mut lowercase = false;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong processing input for string");
    input.pop();
    
    for c in input.chars() {
        if c >= 'a' &&  c <= 'z' && uppercase == true || c >= 'A' && c <= 'Z' && lowercase == true { 
            println!("False") ; return;
        }
        else if c >= 'a' && c <= 'z' { lowercase = true; }
        else if c >= 'A' && c <= 'Z' { uppercase = true; }
    }
    println!("True");
}