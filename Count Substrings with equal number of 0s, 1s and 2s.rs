use std::io;
use std::collections::HashMap;

fn main() {
    let mut s = String::new();
    let mut h1 = HashMap::new();
    let mut count = 0;

    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();

    for i in 0..s.len() {
        h1.insert(0,0);
        h1.insert(1,0);
        h1.insert(2,0);

        for j in i..s.len() {
            let current = *h1.get( &(s.chars().nth(j).unwrap() as i32 - '0' as i32) ).unwrap();
            h1.insert( s.chars().nth(j).unwrap() as i32 - '0' as i32 , current + 1  );

            if *h1.get(&0).unwrap() == *h1.get(&1).unwrap() && *h1.get(&0).unwrap() == *h1.get(&2).unwrap() {
                count += 1;
            }
        }
    }
    println!("{}",count);
}