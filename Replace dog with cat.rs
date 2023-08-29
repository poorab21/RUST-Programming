use std::io;

fn main(){
    let mut value = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut value).expect("Something went wrong while processing input");
    value.pop();
    
    value = value.replace("dog","cat");
    
    println!("{}",value);
}