use std::io;

fn sum_of_n_numbers( num : i32 ) -> i32 {
    if num == 1 || num == 0 { return num; }
    
    num + sum_of_n_numbers(num-1)
}

fn main() {
    let mut num = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut num).expect("Something went wrong while processing input for number");
    num.pop();
    
    let mut num = num.trim().parse::<i32>().unwrap();
    println!("Sum = {}",sum_of_n_numbers(num));
    
}