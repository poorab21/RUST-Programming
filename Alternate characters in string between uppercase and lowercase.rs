use std::io;

fn main() {
    let mut input = String::new();
    let mut sub = 0;
    let mut typeOfCase = 0;
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    let mut strRef = &mut input;
    
    for i in 0..strRef.len() {
        sub = strRef.bytes().nth(i).unwrap();
        
        if typeOfCase == 0 && sub >= 65 && sub <= 90 {
            strRef.replace_range(i..i+1,format!("{}", (sub+32) as char ).as_str());
        }
        else if typeOfCase == 1 && sub >= 97 && sub <= 122 {
            strRef.replace_range(i..i+1,format!("{}", (sub-32) as char ).as_str());
        }
        
        if (sub >= 65 && sub <= 90) || (sub >= 97 && sub <= 122) && typeOfCase == 0 { typeOfCase = 1; }
        else if (sub >= 65 && sub <= 90) || (sub >= 97 && sub <= 122) && typeOfCase == 1 { typeOfCase = 0; }
    }
    
    println!("{}",input);
}