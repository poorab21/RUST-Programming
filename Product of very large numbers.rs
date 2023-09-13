use std::io;
use std::collections::HashMap;

fn main() {
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut product = String::from("0");
    let mut i = 0;
    let mut count = 0;
    let digits : HashMap<i32,char> = HashMap::from([
        (1,'1') ,
        (0,'0') ,
        (2,'2') ,
        (3,'3') ,
        (4,'4') ,
        (5,'5') ,
        (6,'6') ,
        (7,'7') ,
        (8,'8') ,
        (9,'9')
    ]);
    
    println!("Enter First Number:");
    io::stdin().read_line(&mut num1).expect("Something went wrong while processing input for the first number");
    num1.pop();
    
    println!("Enter Second Number:");
    io::stdin().read_line(&mut num2).expect("Something went wrong while processing input for the second number");
    num2.pop();

    num2 = num2.trim().to_string();
    num1 = num1.trim().to_string();
    
    i = (num2.len() - 1) as i32;
    
    while i >= 0 {
       let mut p = multiply( &num1 , num2.chars().nth(i as usize).unwrap() , &digits );
       p = append_with_zeros(&p,count);
    
       if p.len() > product.len() { product = prepend_with_zeros( &product , (p.len() - product.len()) as i32 ) }
       else { p = prepend_with_zeros( &p , (product.len() - p.len()) as i32 ) }

       let s = add(&product,&p);
       product = s;

       count += 1;
       i -= 1;
    }
    
    println!("Product = {}",product);
}

fn multiply( num1 : &str , num2 : char , digits : &HashMap<i32,char> ) -> String {
    let mut carry = 0;
    let mut i : i32 = (num1.len() as i32) - 1;
    let mut product = String::new();
    let number2 = ( num2 as i32 ) - ( '0' as i32) ;
    
    while i >= 0 {
        let result = (((num1.chars().nth(i as usize).unwrap() as i32) - '0' as i32) * number2) + carry;
        carry = result / 10;
        product.insert(0 , digits.get(&(result % 10)).copied().unwrap());
        i -= 1;
    }
    if carry > 0 { product.insert( 0 , digits.get(&carry).copied().unwrap() ); }
    product
}

fn add( num1 : &str , num2 : &str ) -> String {
    let mut carry = 0;
    let mut sum = String::new();
    let mut n1 = 0;
    let mut n2 = 0;
    let mut i = (num1.len() - 1) as i32;

    while i >= 0 {
        n1 = ( num1.chars().nth(i as usize).unwrap() as i32 ) - ( '0' as i32 );
        n2 = ( num2.chars().nth(i as usize).unwrap() as i32 ) - ( '0' as i32 ) ;
        
        let num = ( n1 + n2 + carry ) % 10;
        
        if num == 0 { sum.insert( 0 , '0' ); }
        else if num == 1 { sum.insert( 0 , '1' ); }
        else if num == 2 { sum.insert( 0 , '2' ); }
        else if num == 3 { sum.insert( 0 , '3' ); }
        else if num == 4 { sum.insert( 0 , '4' ); }
        else if num == 5 { sum.insert( 0 , '5' ); }
        else if num == 6 { sum.insert( 0 , '6' ); }
        else if num == 7 { sum.insert( 0 , '7' ); }
        else if num == 8 { sum.insert( 0 , '8' ); }
        else if num == 9 { sum.insert( 0 , '9' ); }

        carry = (n1 + n2 + carry ) / 10;

        i -= 1;  
    }
    if carry > 0 { sum.insert(0,'1') }
    sum
}

fn append_with_zeros( num : &str , num_of_zeros : i32 ) -> String {
    let mut number = num.to_string();
    
    for i in 0..num_of_zeros {
        number += "0";
    }
    
    number
}

fn prepend_with_zeros( num : &str , num_of_zeros : i32 ) -> String {
    let mut number = String::from(num);

    for i in 0..num_of_zeros {
        number.insert(0,'0');
    }

    number
}