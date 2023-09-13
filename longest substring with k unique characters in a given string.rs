use std::io;

fn main() {
    let mut s = String::new();
    let mut k = String::new();
    let mut vec1 : Vec<char> = Vec::new();
    let mut maximum = 0;
    let mut count = 0;
    let mut enough_uniques = false;

    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();

    println!("Enter K:");
    io::stdin().read_line(&mut k).expect("Something went wrong while processing input for K");
    k.pop();

    let k = k.trim().parse().unwrap();

    for i in 0..s.len() {
        for j in i..s.len() {
            if vec1.contains( &s.chars().nth(j).unwrap() ) { count += 1; }
            else if !vec1.contains( &s.chars().nth(j).unwrap() ) && vec1.len() < k { 
                vec1.push( s.chars().nth(j).unwrap() );
                count += 1;
            }
            else { break; }
        }
        if !enough_uniques && vec1.len() == k { enough_uniques = !enough_uniques; }

        if maximum < count && vec1.len() == k { maximum = count; }
        count = 0;

        vec1.clear();
    }

    if !enough_uniques { println!("Not enough unique characters"); return; }
    
    println!("length of the longest substring with {} unique characters = {}",k,maximum);
}