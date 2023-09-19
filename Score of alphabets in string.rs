use std::io;
use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    let mut sum = 0;
    let scores = HashMap::from([
        ( 'a' , 1 ) ,
        ( 'e' , 1 ) ,
        ( 'i' , 1 ) ,
        ( 'o' , 1 ) ,
        ( 'u' , 1 ) ,
        ( 'l' , 1 ) ,
        ( 'n' , 1 ) ,
        ( 'r' , 1 ) ,
        ( 's' , 1 ) ,
        ( 't' , 1 ) ,
        ( 'd' , 2 ) ,
        ( 'g' , 2 ) ,
        ( 'b' , 3 ) ,
        ( 'c' , 3 ) ,
        ( 'm' , 3 ) ,
        ( 'p' , 3 ) ,
        ( 'f' , 4 ) ,
        ( 'h' , 4 ) ,
        ( 'v' , 4 ) ,
        ( 'w' , 4 ) ,
        ( 'y' , 4 ) ,
        ( 'k' , 5 ) ,
        ( 'j' , 8 ) ,
        ( 'x' , 8 ) ,
        ( 'q' , 10 ) ,
        ( 'z' , 10 )
    ]);
    
    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();
    
    for c in s.to_ascii_lowercase().chars() {
        if scores.contains_key(&c) { sum += scores[ &c ]; }
    }
    
    println!("Score = {}",sum);
}