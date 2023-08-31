use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 9 ] = [ 0 ; 9 ];
    
    for i in 0..arr.len() {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..arr.len()-1 {
        if i % 2 == 0 && arr[i] >= arr[i+1] {
            Swap(&mut arr,i);
        }    
        else if i % 2 != 0 && arr[i] <= arr[i+1] {
            Swap(&mut arr,i);
        }
    }
    
    println!("{:?}",arr);
}

fn Swap(arr : &mut [ i32 ], index : usize){
    arr[index] += arr[index+1];
    arr[index+1] = arr[index] - arr[index+1];
    arr[index] -= arr[index+1];
}