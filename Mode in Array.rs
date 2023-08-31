use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 8 ] = [ 0 ; 8 ];
    let mut count = 0;
    let mut largest = 0;
    let mut index = 0;
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len() {
        
        if alreadyExists( arr , i ) { continue; }
        
        for j in i..arr.len() {
            if arr[i] == arr[j] {
                count += 1;
            }
        }
        
        if count > largest { largest = count ; index = i; }
        
        count = 0;
    }
    
    print!("\nMode = {} ",arr[index]);
    
    for i in 0..arr.len() {
        
        if alreadyExists( arr , i ) { continue; }
        if i == index { continue; }
        
        for j in i..arr.len() {
            if arr[i] == arr[j] { count += 1; }
        }
        
        if count == largest { print!("{} ",arr[i]); }
        count = 0;
    }
}

fn alreadyExists(arr : [ i32 ; 8 ] , index : usize) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    return false;
}