use std::io;

fn main(){
    let mut arr : [ i32 ; 5 ] = [ 0 ; 5 ];
    let mut input = String::new();
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while inputting for index {} of array",i).as_str());
        input.pop();
        
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    println!("{:?}",arr);
}