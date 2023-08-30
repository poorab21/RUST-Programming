use std::io;

fn main(){
    let mut arr : [ i32 ; 5 ] = [ 0 ; 5 ];
    let mut input = String::new();
    let mut sub = 0;
    let mut k = String::new();
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    println!("Find how K large elements in array ? ");
    io::stdin().read_line(&mut k).expect("Something went wrong while processing value of K");
    
    let k : i32 = k.trim().parse().unwrap();
    
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] < arr[j] {
                sub = arr[i];
                arr[i] = arr[j];
                arr[j] = sub;
            }
        }
    }
    
    println!("Second largest element in array = {:?}",&arr[0 as usize..k as usize]);
}