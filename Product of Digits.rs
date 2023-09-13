use std::io;

fn main() {
    let mut number = String::new();
    let mut product = 1;

    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input for string");
    number.pop();

    for digit in number.chars() {
        product *= to_number(digit);
    }

    println!("Product of the digits = {}",product);
}

fn to_number( c : char ) -> i32 {
    (c as i32) - ('0' as i32)
}