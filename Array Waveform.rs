use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ i32 ; 8 ] = [ 0 ; 8 ];
    
    for i in 0..arr.len() {
        println!("Enter Number:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong processing input for index {} of array",i).as_str());
        input.pop();
        
        arr[i] = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..(arr.len() - 1 as usize) {
        if i % 2 == 0 && arr[i] < arr[i+1]  {
            swap(&mut arr[..],i,i+1);
        }
        else if i % 2 != 0 && arr[i] > arr[i+1] {
            swap(&mut arr[..],i,i+1);
        }
    }
    
    println!("{:?}",arr);
}

fn swap( arr : &mut [ i32 ] , index : usize , index2 : usize ) {
    let sub = arr[index];
    arr[index] = arr[index2];
    arr[index2] = sub;
}