use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 8 ] = [ 0 ; 8 ];
    let mut value = String::from("");
    
    for i in 0..arr.len() {
        println!("Enter element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear(); 
    }
    println!("Enter Value:");
    io::stdin().read_line(&mut input).expect("Something went while processing the value variable");
    input.pop();
    
    let mut input : i32 = input.trim().parse().unwrap();
    
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if already_Exists(&arr,i) { continue; }
            
            if arr[i] + arr[j] == input && already_Used(&arr,i,j) == false { 
                value.push_str(format!("({} , {})",arr[i],arr[j]).as_str());
            }
        }
    }
    if value.len() == 0 { value.push_str("None") }
    println!("Pairs in array with sum equal to {} = {}",input,value);
}

fn already_Exists( arr : &[ i32 ] , index : usize ) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    return false;
}

fn already_Used( arr : &[ i32 ] , index : usize , index2 : usize ) -> bool {
    for i in 0..index2 {
        if i != index && arr[i] == arr[index2] { return true; }
    }
    return false;
}