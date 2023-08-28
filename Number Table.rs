use std::io;

fn main(){
    let mut number = String::new();
    let mut count = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong when taking input of number variable");
    
    println!("How many multiples ? ");
    io::stdin().read_line(&mut count).expect("Something went wrong when taking input of count variable");
    
    let mut number : i32 = number.trim().parse().unwrap();
    let mut count : i32 = count.trim().parse().unwrap();
    
    for i in 1..=count {
        println!("{} x {} = {}",number,i,(number*i))
    }
}