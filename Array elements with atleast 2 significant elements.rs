use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 8 ] = [ 0 ; 8 ];
    let mut count = 0;
    let mut any = 0;
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len() {
        for j in 0..arr.len() {
            if i == j { continue ; }
            else if arr[i] < arr[j]  {
                count += 1;
            }
        }
        
        if count >= 2 { print!("{} ",arr[i]) ; count = 0; any = 1 } 
    }
    
    if any == 0 { println!("No elements with atleast 2 greater elements found"); }
}