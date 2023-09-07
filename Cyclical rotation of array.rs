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
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr.push(input.trim().parse().unwrap());
        input.clear();
    }
    
    let mut input = 0;
    
    if arr.len() == 1 { println!("{:?}",arr) ; return; }
    
    for i in (0..arr.len()).rev() {
        if i == arr.len() - 1 { input = arr[i] ; arr[i] = arr[i-1]; }
        else if i == 0 { arr[i] = input; }
        else { arr[i] = arr[i-1]; }
    }
    println!("{:?}",arr);
}