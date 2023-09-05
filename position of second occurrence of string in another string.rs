use std::io;

fn main() {
    let mut input = String::new();
    let mut secString = String::new();
    let mut i = 0;
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    println!("Enter string to check for occurrence:");
    io::stdin().read_line(&mut secString).expect("Something went wrong while processing input for string");
    secString.pop();
    
    while i < input.len() {
        if input.chars().nth(i).unwrap() == secString.chars().nth(0).unwrap() && (i + secString.len()) <= input.len() && &input[i..i+secString.len()] == secString {
            count += 1;
            if count == 2 { println!("second occurrence of {} occurs at index {}",secString,i); return; }
            
            i += secString.len();
            continue;
        }
        i += 1;
    }
    
    println!("{} does not not occur atleast twice in {}",secString,input);
}