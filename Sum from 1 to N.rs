use std::io;

fn main(){
    let mut num = String::new();
    let mut sum : i32 = 0; 
    println!("Enter Number:");
    io::stdin().read_line(&mut num).expect("Something went wrong when inputting number");
    
    let num : i32 = num.trim().parse().unwrap();
    
    for i in 1..=num {
        sum += i;
    }
    
    println!("Sum = {}",sum);
}