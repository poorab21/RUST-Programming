use std::io;

fn main() {
    let mut input = String::new();
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for String");
    input.pop();
    
    for i in input.chars() {
        if i == 'a' || i == 'e' || i == 'i' || i == 'o' || i == 'u' || i == 's' {
            count += 1;        
        }
    }
    
    println!("Total number of vowels in String = {}",count);
}