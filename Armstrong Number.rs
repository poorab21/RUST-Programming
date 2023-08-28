use std::io;

fn main(){
    let mut input = String::new();
    println!("Enter Number:");
    io::stdin().read_line(&mut input).expect("Something went wrong");
    let mut number = input.trim().parse().unwrap();
    
    if isArmstrong( number , (input.len() as u32) - 1 ) == true {
        println!("{} is an Armstrong Number",number);
    }
    else if isArmstrong( number , (input.len() as u32) - 1 ) == false {
        println!("{} is not an Armstrong number",number);
    }
}

fn isArmstrong(input : i32, length : u32) -> bool {
    let mut newNum : i32 = 0;
    let mut remainder : i32 = 0;
    let mut sub : i32 = input;
    
    while sub > 0 {
        remainder = sub % 10;
        newNum += remainder.pow(length);
        sub /= 10;
    } 
    
    if newNum == input {
        return true;
    }
    else {
        return false;
    }
}