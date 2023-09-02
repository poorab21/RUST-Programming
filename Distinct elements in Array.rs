use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 9 ] = [ 0 ; 9 ];
    let mut count = 0;
    
    for i in 0..arr.len() {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    print!("Distinct elements in array = ");
    for i in 0..arr.len() {
        if alreadyExists(&arr,i) { continue; }
        else { print!("{} ",arr[i]); }
    } 
}

fn alreadyExists( arr : &[ i32 ] , index : usize ) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    return false;
}