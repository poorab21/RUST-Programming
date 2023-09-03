use std::io;

fn main(){
    let mut input = String::new();
    let mut largest = 0;
    let mut startptr = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    if input.len() <= 0 { return; }
    
    let mut endptr = input.len() - 1;
    
    for i in 0..input.len() {
        for j in (i..input.len()).rev() {
            if input.chars().nth(i).unwrap() == input.chars().nth(j).unwrap() && isPallindrome(&input,i,j) && j - i + 1 > largest {
               largest = j - i + 1;
               break;
            }
        }
    }
    println!("Length of the largest pallindrome in the string = {}",largest);
}

fn isPallindrome( s : &String , mut i : usize , mut j : usize ) -> bool {
    while s.chars().nth(i).unwrap() == s.chars().nth(j).unwrap() && i < j {
        i += 1;
        j -= 1;
    }
    
    if i >= j { return true; }
    return false;
}