use std::io;

fn main(){
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong when processing input");
    input.pop();
    
    for i in 0..input.len() {
        if input.bytes().nth(i).unwrap() >= 65 && input.bytes().nth(i).unwrap() <= 90 {
            let newChar = input.bytes().nth(i).unwrap() + 32;
            input.replace_range(i..i+1,format!("{}",newChar as char).as_str());
        } 
        else if input.bytes().nth(i).unwrap() >= 97 && input.bytes().nth(i).unwrap() <= 122 {
            let newChar = input.bytes().nth(i).unwrap() - 32;
            input.replace_range(i..i+1,format!("{}",newChar as char).as_str());
        }
    }
    println!("{}",input);
}