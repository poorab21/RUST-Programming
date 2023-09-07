use std::io;

fn main(){
    let mut arr = vec![1, 2, 3, 1, 5, 8, 9];
    let mut count = 1;
    let mut max = 0;
    let mut i = 0;
    
    for i in 0..arr.len() {
        for j in i+1..arr.len() {
            if arr[i] > arr[j] {
                arr[i] += arr[j];
                arr[j] = arr[i] - arr[j];
                arr[i] -= arr[j];
            }
        }
    }
    
    while i < arr.len() - 1 {
        if arr[i] + 1 == arr[i+1] {
            count += 1;
        }
        else if arr[i] == arr[i+1] {
            i += 1;
            continue;
        }
        else if count > max {
            max = count;
            count = 1;
        }
        i += 1;
    }
    
    if count > max { max = count; }
    
    println!("Length of the Longest consecutive subsequence is {}",max);
}