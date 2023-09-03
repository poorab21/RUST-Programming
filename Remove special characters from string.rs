use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    let mut strRef = &mut input;
    let mut c : char =  ' ';
    
    for i in 0..strRef.len() {
        c = strRef.chars().nth(i).unwrap();
        
        if c == '.' || c == '!' || c == '@' || c == '#' || c == '$' || c == '%' || c == '^' || c == '&' || c == '\\' || c == '*' || c == '(' || c == ')' {
            strRef.replace_range(i..i+1,"\0");
        }
    }
    println!("String without special characters = {}",input);
}