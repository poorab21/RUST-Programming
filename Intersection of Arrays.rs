use std::io;

fn main() {
    let mut size = 0;
    let mut input = String::new();
    let mut arr1 : Vec<i32> = Vec::new();
    let mut arr2 : Vec<i32> = Vec::new();
    
    println!("Enter size (for both arrays):");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for array size");
    input.pop();
    
    size = input.trim().parse().unwrap();
    input.clear();
    
    for i in 0..size {
        println!("Enter element (array 1):");
        io::stdin().read_line(&mut input).expect(format!("Something went wron while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr1.push(input.trim().parse().unwrap());
        input.clear();
    }
    
    for i in 0..size {
        println!("Enter Element (array 2):");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for {} of array",i).as_str());
        input.pop();
        
        arr2.push(input.trim().parse().unwrap());
        input.clear();
    }
    
    println!("{:?}",arr1);
    println!("{:?}",arr2);
    for i in 0..size {
        if InArray(&arr2 , arr1[i]) && !alreadyExists( &arr1 , i ) {
            print!("{} ",arr1[i]);
        }
    }
}

fn InArray( arr : &Vec<i32> , element : i32 ) -> bool {
    for i in arr {
        if *i == element { return true; }
    }
    false
}

fn alreadyExists( arr : &Vec<i32> , index : usize ) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    false
}