use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 10 ] = [ 0 ; 10 ];
    let mut zeros = 0;
    let mut ones = 0;
    
    for i in 0..arr.len() {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len() {
        if arr[i] == 0 { zeros += 1; }
        else if arr[i] == 1 { ones += 1; }
    }
    
    for i in 0..=zeros-1 {
        arr[i] = 0;
    }
    
    for i in zeros+1..arr.len() {
        arr[i] = 1;
    }
    
    println!("{:?}",arr);
}