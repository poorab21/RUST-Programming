use std::io;

fn main() {
    let mut input = String::new();
    let mut size = 0;
    let mut arr : Vec<i32> = Vec::new();
    
    println!("Enter Size:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for array size");
    input.pop();
    
    size = input.trim().parse().unwrap();
    input.clear();
    
    for i in 0..size {
        println!("Enter element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr.push(input.trim().parse().unwrap());
        input.clear();
    }
    
    for i in 0..arr.len() {
        if !alreadyExists(&arr,i) && !occursAgain(&arr,i) { 
            println!("First non-repeating element = {}",arr[i]); 
            return;
        }
    }
    println!("No distinct elements in array");
}

fn alreadyExists( arr : &Vec<i32> , index : usize ) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    false
}

fn occursAgain( arr : &Vec<i32> , index : usize ) -> bool {
    for i in index+1..arr.len() {
        if arr[i] == arr[index] { return true; }
    }
    false
}