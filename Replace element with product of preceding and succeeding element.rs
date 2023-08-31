use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 9 ] = [ 0 ; 9 ];
    let mut prev = 0;
    let mut current = 0;
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len() {
        if (i as i32) - 1 < 0 {
            prev = arr[i] * arr[i+1];
        }
        else if (i as i32) - 1 >= 0 && i + 1 < arr.len() {
            current = arr[i-1] * arr[i+1];
            arr[i-1] = prev;
            prev = current;
        }
        else if i + 1 >= arr.len() {
            arr[i] = arr[i] * arr[i-1];
            arr[i-1] = prev;
        }
    }
    
    println!("{:?}",arr);
}