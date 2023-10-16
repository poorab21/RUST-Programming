use std::io;

fn main(){
    let mut input = String::from("");
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong");
    input.pop();
    
    let reverseString : String = input.chars().rev().collect();
    
    for i in 0..input.len() {
        if &input[i..i+1] == &reverseString[i..i+1] && i == input.len() - 1 {
            println!("{} is a Pallindrome",input);
        } 
        else if &input[i..i+1] != &reverseString[i..i+1] {
            println!("{} is not a Pallindrome",input);
            break;
        }
    }
}