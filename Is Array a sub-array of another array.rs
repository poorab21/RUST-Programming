fn main() {
    let arr = [ 1 , 2 , 3 , 4 , 5 ];
    let arr2 = [ 1, 2, 3 ];
    let mut i = 0;

    if arr2.len() == arr.len() { println!("'{:?}' is not a sublist of '{:?}'",arr2,arr); return; }

    while i < arr.len() {
        if i + arr2.len() > arr.len() { break; }

        let sublist = &arr[i..i+arr2.len()];

        for (index,&num) in sublist.iter().enumerate() {
            if arr2[index] != num { break; }
            else if arr2[index] == num && index == arr2.len() - 1 { 
                println!("'{:?}' is a sublist of '{:?}'",arr2,arr) ; 
                return; 
            } 
        }

        i += 1;
    }

    println!("'{:?}' is not a sublist of '{:?}'",arr2,arr);
}