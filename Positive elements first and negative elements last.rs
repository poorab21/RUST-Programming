use std::io;

fn main() {
    let mut arr : [ i32 ; 7 ] = [ 0 ; 7 ];
    let mut input = String::new();
    let mut j = 0;
    let mut k = 0;
    let mut count = 0;
    
    for i in 0..arr.len() {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    while j < arr.len() {
        if arr[j] < 0 {
            k = j + 1;
            
            while k < arr.len() && arr[k] < 0 {
                count += 1;
                k += 1;
            }
            
            if k == arr.len() { break; }
            if count == 0 { Swap(&mut arr,j,k); }
            else if count > 0 { replaceAndSwap(&mut arr,j,k); count = 0; }
        }
        j += 1;
    }
    println!("{:?}",arr);
}

fn Swap( arr : &mut [ i32 ] , index1 : usize , index2 : usize ) {
    arr[index1] += arr[index2];
    arr[index2] = arr[index1] - arr[index2];
    arr[index1] -= arr[index2];
}

fn replaceAndSwap( arr : &mut [ i32 ] , index1 : usize , index2 : usize ) {
    let value = arr[index2];
    
    for i in (index1+1..=index2).rev() {
        arr[i] = arr[i-1];
    }
    arr[index1] = value;
}