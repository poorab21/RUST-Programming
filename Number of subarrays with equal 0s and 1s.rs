use std::io;

fn main() {
    let mut size = 0;
    let mut arr : Vec<i32> = Vec::new();
    let mut input = String::new();
    let mut count = 0;
    
    println!("Enter Size:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for size of array");
    input.pop();
    
    size = input.trim().parse().unwrap();
    input.clear();
    
    for i in 0..size {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr.push(input.trim().parse().unwrap());
        input.clear();
    }
    
    for i in 0..arr.len() {
        for j in i..arr.len() {
            if equivalent(&arr,i,j) { count += 1 }
        }
    }
    println!("number of subarrays with equal number of 0s and 1s = {}",count);
}

fn equivalent( arr : &Vec<i32> , start : usize , end : usize ) -> bool {
    let mut zeros = 0;
    let mut ones = 0;
    for i in start..=end {
        if arr[i] == 0 { zeros += 1; }
        else if arr[i] == 1 { ones += 1; }
    }
    if zeros == ones { return true; }
    false
}