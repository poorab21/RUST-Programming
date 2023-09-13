use std::io;

fn main() {
    let mut number = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input for number");
    number.pop();

    let sum = to_number( number.chars().nth(0).unwrap() ) + to_number( number.chars().nth( number.len() - 1 ).unwrap() );

    println!("Sum of first and last digit = {}",sum);
}

fn to_number( c : char ) -> i32 {
    (c as i32) - ('0' as i32) 
}