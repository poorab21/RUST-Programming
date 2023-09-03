use std::io;

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut str1).expect("Something went wrong while processing input for first string");
    str1.pop();
    
    println!("Enter String:");
    io::stdin().read_line(&mut str2).expect("Something went wrong while processing input for second string");
    str2.pop();
    
    str1 += format!("{}",str2).as_str();
    
    for i in 0..str1.len() {
        if hasOccurred( &str1 , i ) { continue; }
        count += 1;
    }
    
    println!("Total number of unique characters of the said two strings = {}",count);
}

fn hasOccurred( s : &String , index : usize ) -> bool {
    for i in 0..index {
        if s.chars().nth(i).unwrap() == s.chars().nth(index).unwrap() {
            return true;
        }
    }
    return false;
}