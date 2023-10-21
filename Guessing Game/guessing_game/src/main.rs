use guessing_game_lib::{ guess , secret };
use std::io;
use std::io::ErrorKind;

fn main() {
    let secret_num = secret::generate_num_to_guess();
    let mut guess_num = String::from("");
    
    println!("I thought of a number ({secret_num}) ! Guess what it is ?");

    loop {
        guess_num.clear();

        println!("Enter your guess:");
        io::stdin().read_line( &mut guess_num ).expect("Something went wrong while processing your guess");
        guess_num.pop();

        if let Err(e) = guess_num.trim().parse::<u32>() {
            if e.to_string() == "invalid digit found in string".to_string() { println!("\n{}\n",e.to_string()); }
            else { println!("Some error in your input"); }
            continue; 
        }


        let guess = guess::Guess::new( guess_num.trim().parse().unwrap() );
        
        if let Err(e) = guess { println!("\n{:?}\n",e.to_string()); continue; }
        
        let guess = guess.unwrap();

        if guess.num() == secret_num { println!("you guessed correctly"); break; }
        else if guess.num() > secret_num { println!("Guessed number is greater than the number thought"); }
        else if guess.num() < secret_num { println!("Guessed number is lesser than the number thought"); }

    }
}
