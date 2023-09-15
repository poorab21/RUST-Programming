// The input must be in the format a/b
use std::io;

fn main() {
    let mut s = String::new();

    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing the input for string");
    s.pop();

    let numbers : Vec<i32> = s.trim().split("/").map(|x| {
        x.parse::<i32>().unwrap()
    }).collect();
    
    let lowest_term = lowest_terms( numbers.get(0).copied().unwrap() , numbers.get(1).copied().unwrap());

    if lowest_term.0 == 0 && lowest_term.1 == 0 { println!("fraction is invalid"); return; }

    println!("Lowest terms of fraction = {}/{}", lowest_term.0 , lowest_term.1 );
}

fn lowest_terms( num1 : i32 , num2 : i32 ) -> (i32,i32) {
    let mut i = 1;
    let mut number = 1;

    while i <= num1 {
        if num1 % i == 0 && num2 % i == 0 { number = i; }
        i += 1;
    } 

    ( num1 / number , num2 / number )
}