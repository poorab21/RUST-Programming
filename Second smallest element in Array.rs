use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 5 ] = [ 0 ; 5 ];
    let mut sub = 0;
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] > arr[j] {
                sub = arr[i];
                arr[i] = arr[j];
                arr[j] = sub;
            }
        }
    }
    
    println!("2nd Smallest element in Array = {}",arr[1]);
}