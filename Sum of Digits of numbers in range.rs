use std::io;

fn main(){
    let mut startNum = String::new();
    let mut endNum = String::new();
    let mut sum = 0;
    
    println!("Enter Beginning Number:");
    io::stdin().read_line(&mut startNum).expect("Something went wrong while processing input for starting number");
    startNum.pop();
    
    println!("Enter End Number:");
    io::stdin().read_line(&mut endNum).expect("Something went wrong while processing input for end number");
    endNum.pop();
    
    let mut startNum : i32 = startNum.trim().parse().unwrap();
    let mut endNum : i32 = endNum.trim().parse().unwrap();
    
    for i in startNum..=endNum {
        sum += sumOfDigits(i);
    }
    println!("Sum of Digits = {}",sum);
}

fn sumOfDigits(num : i32) -> i32 {
    let mut sub = num;
    let mut remainder = 0;
    let mut sum = 0;
    
    while sub > 0 {
        remainder = sub % 10;
        sub /= 10;
        sum += remainder;
    }
    return sum;
}