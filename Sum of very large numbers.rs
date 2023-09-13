use std::io;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut sum = String::new();
    let mut carry = 0;

    println!("Enter First Number:");
    io::stdin().read_line(&mut num1).expect("Something went wrong while processing input for string");
    num1.pop();

    println!("Enter Second Number:");
    io::stdin().read_line(&mut num2).expect("Something went wrong while processing input for string");
    num2.pop();

    if num1.len() > num2.len() { 
        let length = (num1.len() as i32 ) - (num2.len() as i32 );
        prepend_zeros( &mut num2 , length );
    }
    else if num1.len() < num2.len() { 
        let length = (num2.len() as i32) - (num1.len() as i32 );
        prepend_zeros( &mut num1 , length ); 
    }
    
    for i in (0..num1.len()).rev() {
       let addition : u8 = (Sum( num1.chars().nth(i).unwrap() , num2.chars().nth(i).unwrap() ) + carry) as u8;
       if addition >= 10 { sum.insert( 0 , toASCII(addition % 10) as char ) ; carry = 1; }
        else { sum.insert( 0 , toASCII(addition) as char ) ; carry = 0; }
    }

    if carry == 1 { sum.insert(0,'1'); }

    println!("Sum = {}",sum);
}

fn prepend_zeros( num : &mut String , num_of_zeros : i32 ) {
    for i in 0..num_of_zeros {
        num.insert(0,'0');
    }
}

fn Sum( num1 : char , num2 : char ) -> i32 {
    ((num1 as i32) - ('0' as i32)) + ((num2 as i32) - ('0' as i32))
}

fn toASCII( num : u8 ) -> u8 {
    match num {
        0 => 48 ,
        1 => 49 ,
        2 => 50 ,
        3 => 51 ,
        4 => 52 ,
        5 => 53 ,
        6 => 54 ,
        7 => 55 ,
        8 => 56 ,
        9 => 57 ,
        _ => 0
    }
}