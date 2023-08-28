use std::io;

fn main(){
    let mut number = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong when taking input");
    number.pop();
    
    let endIndexStart = number.len() - 1;
    
    let firstNumber = number.chars().nth(0).unwrap();
    let lastNumber = number.chars().nth(endIndexStart).unwrap();
    
    number.replace_range(0..1,format!("{}",lastNumber).as_str());
    number.replace_range(endIndexStart..endIndexStart+1,format!("{}",firstNumber).as_str());
    
    println!("{}",number);
}