fn largest( arr : &[i32] ) -> i32 {
    let mut largest = 0;
    
    for i in 0..arr.len() {
        if largest < arr[i] {
            largest = arr[i];
        }
    }
    
    largest
}

fn main(){
    let mut arr = &[ 51 , 22 , 15 , 13 , 9 , 31 ];
    
    println!("Largest Element in Array = {}",largest(arr));
}