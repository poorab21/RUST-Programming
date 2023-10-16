use std::io;
use calculib::operations::{ add , subtract , multiply , divide , modulo };
use regex::Regex;
use clearscreen::clear;

enum Operation {
    Add ,
    Subtract , 
    Divide ,
    Multiply , 
    Modulo
}

fn operation( operation : Operation ) -> Result< f64 , Box< dyn std::error::Error > > {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let number_only = Regex::new(r"^(-)?[0-9]+(\.[0-9]+)?$").unwrap();

    println!("Enter First Number:");
    io::stdin().read_line( &mut num1 ).expect("Something went wrong while processing input for the first number");
    num1.pop();

    println!("Enter Second Number:");
    io::stdin().read_line( &mut num2 ).expect("Something went wrong while processing input for the second number");
    num2.pop();

    match ( number_only.find( num1.trim() ).is_some() , number_only.find( num2.trim() ).is_some() , operation ) {
        ( true , true , Operation::Add ) => {
            Ok( add(num1.trim().parse::<f64>().unwrap() , num2.trim().parse::<f64>().unwrap() ))
        },
        ( true , true , Operation::Subtract ) => {
            Ok( subtract( num1.trim().parse::<f64>().unwrap() , num2.trim().parse::<f64>().unwrap() ) )
        },
        ( true , true , Operation::Multiply ) => {
            Ok( multiply( num1.trim().parse::<f64>().unwrap() , num2.trim().parse::<f64>().unwrap() ) )
        },
        ( true , true , Operation::Divide ) => {
            divide( num1.trim().parse::<f64>().unwrap() , num2.trim().parse::<f64>().unwrap() )
        },
        ( true , true , Operation::Modulo ) => {
            Ok( modulo( num1.trim().parse::<u32>().unwrap() , num2.trim().parse::<u32>().unwrap() ) as f64 )
        },
        ( _ , _ , _ ) => Err("\nPlease enter valid numbers\n".into())
    }
}

fn main() {
    let mut operator = String::from("");

    loop {
        operator.clear();

        println!("Enter Operation (+,-,/,*,%):");
        io::stdin().read_line(&mut operator).expect("Something went wrong while processing input for operator");
        operator.pop();

        clear();

        match operator.trim() {
            "+" => { 
                let add_result = operation( Operation::Add );

                clear();

                if let Err(e) = add_result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(result) = add_result {
                    println!("\n{}\n",result);
                } 
            } ,
            "-" => {
                let sub_result = operation( Operation::Subtract );

                clear();

                if let Err(e) = sub_result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(result) = sub_result {
                    println!("\n{}\n",result);
                } 
            },
            "/" => {
                let div_result = operation( Operation::Divide );

                clear();

                if let Err(e) = div_result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(result) = div_result {
                    println!("\n{}\n",result);
                }
            },
            "*" => {
                let mul_result = operation( Operation::Multiply );

                clear();

                if let Err(e) = mul_result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(result) = mul_result {
                    println!("\n{}\n",result);
                }
            },
            "%" => {
                let mod_result = operation( Operation::Modulo );

                clear();

                if let Err(e) = mod_result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(result) = mod_result {
                    println!("\n{}\n",result);
                }
            }
            _ => { clear() ; println!("\nInvalid operator entered\n"); }
        }
    }
}
