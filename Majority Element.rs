fn main() {
    let mut vec1 = vec![ 6, 6, 6, 6, 6, 6, 6,6,6];
    let mut count = 0;
    let mut max = 0;
    let mut element = 0;
    
    for i in 0..vec1.len() {
        if alreadyExists( &vec1 , i ) { continue; }
        
        for j in i..vec1.len() {
            if vec1[i] == vec1[j] { count += 1; }
        }
        
        if count > max {
            max = count;
            element = vec1[i];
        }
        count = 0;
    }
    
    if max > (vec1.len()/2) { println!("Majority element = {}",element); }
    else { println!("No majority element"); }
}

fn alreadyExists( arr : &Vec<i32> , index : usize ) -> bool {
    for i in 0..index {
        if arr[i] == arr[index] { return true; }
    }
    false
}