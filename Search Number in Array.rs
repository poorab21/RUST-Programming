use std::io;

fn main(){
    let mut input = String::new();
    let mut search = String::new();
    let mut arr : [ i32 ; 5 ] = [ 0 ; 5 ]; 
    let mut exists = 0;
    
    for i in 0..arr.len() {
        println!("Enter number for index {}:",i);
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while taking input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    println!("Enter number to search:");
    io::stdin().read_line(&mut search).expect("Something went wrong while taking input of the number to search");
    
    let search : i32 = search.trim().parse().unwrap();
    
    for ( index , &value ) in arr.iter().enumerate(){
        if value == search { println!("found at index {}",index) ; exists = 1; }
    }
    
    if exists == 0 { println!("Number does not exist in Array"); }
}