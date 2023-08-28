use std::io;

fn main(){
    let mut num = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut num).expect("Something went wrong");
    
    let num : f64 = num.trim().parse().unwrap();
    
    if isNegative(num) {
        println!("{} is a negative number",num);
    }
    else if isPositive(num) {
        println!("{} is a positive number",num);
    }
    else {
        println!("{} is Zero",num);
    }
}

fn isNegative(number : f64) -> bool {
    if number < (0 as f64) { return true; }
    else { return false; }
}

fn isPositive(number : f64) -> bool {
    if number > (0 as f64) { return true; }
    else { return false; }
}

