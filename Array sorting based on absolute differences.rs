use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 7 ] = [ 0 ; 7 ];
    
    for i in 0..arr.len() {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    println!("Enter value:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing value variable");
    input.pop();
    
    let input : i32 = input.trim().parse().unwrap();
    
    if repeatingElements(&arr[..]) {
        panic!("Error! elements in array must be distinct");
    }
    else {
        calculate_Differences(&mut arr,input);    
    }
    
    descending_Order(&mut arr);
    
    println!("{:?}",arr);
}

fn repeatingElements( arr : &[ i32 ] ) -> bool {
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] == arr[j] { return true; }
        }
    }
    return false;
}

fn calculate_Differences( arr : &mut [i32] , value : i32 ) {
    for i in 0..arr.len() {
        arr[i] = (arr[i] - value).abs();
    }
} 

fn descending_Order( arr : &mut [i32] ){
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] < arr[j] {
                arr[i] += arr[j];
                arr[j] = arr[i] - arr[j];
                arr[i] -= arr[j];
            }
        }
    }
}