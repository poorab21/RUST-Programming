use std::io;

fn main(){
    let mut num1 = String::new() ; 
    let mut num2 = String::new() ;
    let mut num3 = String::new() ;
    let mut num4 = String::new() ;
    
    println!("Enter first Number:");
    io::stdin().read_line(&mut num1).expect("Something went wrong when inputting first number");
    
    println!("Enter second Number:");
    io::stdin().read_line(&mut num2).expect("Something went wrong when inputting second number");
    
    println!("Enter third Number:");
    io::stdin().read_line(&mut num3).expect("Something went wrong when inputting third number");

    println!("Enter fourth Number:");
    io::stdin().read_line(&mut num4).expect("Something went wrong when inputting fourth number");
    
    
    let average : f32 = (ToNumber(&num1) + ToNumber(&num2) + ToNumber(&num3) + ToNumber(&num4)) / 4 as f32;
    let total : f32 = ToNumber(&num1) + ToNumber(&num2) + ToNumber(&num3) + ToNumber(&num4);
    
    print!("Total = {} \nAverage = {}",total,average);
}

fn ToNumber(num : &String) -> f32 {
    num.trim().parse().unwrap()
}