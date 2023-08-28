use std::io;

fn main(){
    let mut num1 = String::new();
    let mut num2 = String::new();
    
    println!("Enter First Number");
    io::stdin().read_line(&mut num1).expect("Something went wrong when inputting first number");
    
    println!("Enter Second Number:");
    io::stdin().read_line(&mut num2).expect("Something went wrong when inputting second number");
    
    let mut num1 : i32 = num1.trim().parse().unwrap();
    let mut num2 : i32 = num2.trim().parse().unwrap();
    
    println!("Before Swapping = {} {}",num1,num2);
    
    num1 += num2;
    num2 = num1 - num2;
    num1 -= num2;
    
    print!("After Swapping = {} {}",num1,num2);
}