use std::io;

fn main() {
    let mut input = String::new();
    let mut i = 0;
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    i = input.len() - 1;
    
    while i > 0 {
        if count == 2 { input.insert(i,',') ; count = -1; }
        count += 1;
        i -= 1;
    }
    
    println!("{}",input);
}