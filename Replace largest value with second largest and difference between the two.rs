use std::io;
use std::mem;

fn main() {
    let mut arr : Vec<i32> = Vec::new();
    let mut size = String::new();
    let mut input = String::new();
    let mut maximum = 0;
    let mut position = 0;

    println!("Enter Size:");
    io::stdin().read_line(&mut size).expect("Something went wrong while processing input for array size");
    size.pop();
    
    let size : i32 = size.trim().parse().unwrap();
    
    for i in 0..size {
        println!("Enter element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr.push( input.trim().parse::<i32>().unwrap() );
        input.clear();
    }
    
    let mut input : i32 = 0;
    
    for (index,value) in arr.iter().enumerate() {
        if maximum < *value {
            input = maximum;
            maximum = *value;
            position = index
        }
        else if *value > input {
            input = *value;
        }
    }
    std::mem::replace(&mut arr[position],input);
    arr.insert(position+1,maximum-input);
    
    println!("{:?}",arr);
}