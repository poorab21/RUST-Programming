use std::io;

fn main(){
    let mut binaryNum1 = String::new();
    let mut binaryNum2 = String::new();
    let mut addition = String::from("");
    let mut n = 0;
    
    println!("Enter first binary number (must be larger or equal):");
    io::stdin().read_line(&mut binaryNum1).expect("Something went wrong when inputting first binary number");
    binaryNum1.pop();
    
    println!("Enter second binary number (must be smaller or equal):");
    io::stdin().read_line(&mut binaryNum2).expect("Something went wrong when inputting second binary number");
    binaryNum2.pop();
    
    
    if binaryNum1.len() > binaryNum2.len() {
        for i in 1..=(binaryNum1.len() - binaryNum2.len()) {
            binaryNum2.insert_str(0,"0");
        }
    }
    
    let mut b1_startingIndex : i32 = binaryNum1.len() as i32 - 1;
    let mut b2_startingIndex : i32 = binaryNum2.len() as i32 - 1;
    
    while b1_startingIndex >= 0 {
        let sub1 : usize = b1_startingIndex as usize ;
        let sub2 : usize = b2_startingIndex as usize ;
        if &binaryNum1[sub1..sub1 + 1 as usize ] == "1" && &binaryNum2[sub2..sub2 + 1 as usize] == "1" && n == 0 {
            addition.insert_str(0,"0");
            n = 1;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "1" && &binaryNum2[sub2..sub2 + 1 as usize] == "1" && n == 1 {
            addition.insert_str(0,"1");
            n = 1;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "1" && &binaryNum2[sub2..sub2 + 1 as usize] == "0" && n == 0 {
            addition.insert_str(0,"1");
            n = 0;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "1" && &binaryNum2[sub2..sub2 + 1 as usize] == "0" && n == 1 {
            addition.insert_str(0,"0");
            n = 1;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "0" && &binaryNum2[sub2..sub2 + 1 as usize] == "1" && n == 0 {
            addition.insert_str(0,"1");
            n = 0;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "0" && &binaryNum2[sub2..sub2 + 1 as usize] == "1" && n == 1 {
            addition.insert_str(0,"0");
            n = 1;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "0" && &binaryNum2[sub2..sub2 + 1 as usize] == "0" && n == 0 {
            addition.insert_str(0,"0");
            n = 0;
        }
        else if &binaryNum1[sub1..sub1 + 1 as usize] == "0" && &binaryNum2[sub2..sub2 + 1 as usize] == "0" && n == 1 {
            addition.insert_str(0,"1");
            n = 0;
        }
        
        b1_startingIndex -= 1;
        b2_startingIndex -= 1;
    }
    
    if n == 1 {
        addition.insert_str(0,"1");    
    }
    
    println!("Binary Addition = {}",addition);
}