use std::io;

fn main() {
    let mut s = String::new();
    let mut s2 = String::new();
    let mut sub = String::new();
    let mut count = 1;

    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for the first string");
    s.pop();

    println!("Enter String:");
    io::stdin().read_line(&mut s2).expect("Something went wrong while processing input for the second string");
    s2.pop();

    sub = s.clone();   

    for c in s2.chars() {
        if !s.contains(format!("{}",c).as_str()) {
            println!("'{}' cannot be a substring of '{}'",s2,s);
            return;
        }
    }    

    while !s.contains(s2.as_str()) {
        s += sub.as_str();
        count += 1;
    }

    println!("String has to be repeated {} times",count);
}