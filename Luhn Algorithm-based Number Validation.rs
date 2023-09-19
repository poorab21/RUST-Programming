use std::io;

fn main() {
    let mut number = String::new();
    let mut new_number = String::from("");
    let mut sum = 0;

    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input for number");
    number.pop();

    number = number.trim().to_string();

    if number.len() <= 1 { println!("Number must have length greater than 1") ; return ; }
    if !is_only_numeric( &number ) { println!("Number must only contain digits") ; return ; }

    for num in number.split(" "){
        new_number += format!("{} ",change_numbers(num).as_str()).as_str();
    }
    
    for c in new_number.chars() {
        if c >= '0' && c <= '9' {
            sum += ( c as i32 ) - ( '0' as i32 );
        }
    }
    
    if sum % 10 == 0 { println!("Valid Number"); }
    else { println!("Invalid Number"); }
}

fn is_only_numeric( num : &str ) -> bool {
    for c in num.chars() {
        if (c < '0' || c > '9') && c != ' ' { return false; }
    }
    true
}

fn change_numbers( num : &str ) -> String {
    let mut new_number = String::new();
    let mut count = 0;
    
    while count < num.len() {
        if count % 2 != 0 { new_number += format!("{}",num.chars().nth(count).unwrap()).as_str(); }
        else {
            let mut product : i32 = (num.chars().nth(count).unwrap() as i32) - ('0' as i32);
            product *= 2;

            if product > 9 { product -= 9; }

            new_number += format!("{}",product).as_str();
        }
        count += 1;
    }

    new_number
}