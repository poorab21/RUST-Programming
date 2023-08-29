use std::io;

fn main(){
    let mut number = String::new();
    let mut hour = 0;
    
    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input");
    number.pop();
    
    let mut number : i32 = number.trim().parse().unwrap();
    
    while number > 60 {
        number -= 60;
        hour += 1;
    }
    println!("{}:{}",hour,number);
}