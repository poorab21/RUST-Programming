use generator::generator::{ lowercase_character , uppercase_character , special_characters , digit };
use std::io;
use rand::Rng;

fn main() {
    let mut password = String::from("");
    let mut length = String::new();
    let mut digits = String::new();
    let mut special = String::new();
    let mut type_of_char = rand::thread_rng();

    println!("What should be length of password ?");
    io::stdin().read_line( &mut length ).expect("Something went wrong while processing input for length");
    length.pop();

    println!("Should password contain digits (Y/N):");
    io::stdin().read_line( &mut digits ).expect("Something went wrong while processing answer for digits in password");
    digits.pop();

    println!("Should password contains Special Characters (Y/N):");
    io::stdin().read_line( &mut special ).expect("Something went wrong while processing answer for special characters in password");
    special.pop();

    if let Err(_) = length.trim().parse::<u8>() {
        println!("Please enter valid length for password");
        return;
    }

    if digits.is_empty() || ( digits.chars().nth(0).unwrap() != 'Y' && digits.chars().nth(0).unwrap() != 'N' )  { 
        println!("Please submit proper answer for whether password should contain digits or not"); 
        return; 
    }
    
    if special.is_empty() || ( special.chars().nth(0).unwrap() != 'Y' && special.chars().nth(0).unwrap() != 'N' ) {
        println!("Please submit proper answer for whether password should contain special characters or not"); 
        return; 
    }
    
    let length : u32 = length.trim().parse().unwrap();
    
    let digits = if digits.chars().nth(0).unwrap() == 'Y' { true } else { false };

    let special = if special.chars().nth(0).unwrap() == 'Y' { true } else { false };

    while password.len() < (length as usize) {
        let x = type_of_char.gen_range(1..4);
        password.push( if x == 1 && digits == true {
            digit()
        }
        else if x == 2 && special == true {
            special_characters()
        }
        else if x == 3 {
            lowercase_character()
        }
        else {
            uppercase_character()
        })
    }

    println!("{}",password);
}
