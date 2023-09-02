use std::io;

fn main() {
    let mut arr = [
        String::from("Spiderman") 
    ];
    
    let mut i = 0;
    let mut j = 1;
    let mut strRef = &mut arr[..];
    
    while i < strRef.len() - 1 {
        if strRef.iter().nth(i).unwrap().len() > strRef.iter().nth(j).unwrap().len() {
            let sub = strRef.iter().nth(i).unwrap().to_string();
            strRef[i] = strRef.iter().nth(j).unwrap().to_string();
            strRef[j] = sub;
        }
        
        j += 1;
        if j == strRef.len() {
            i += 1;
            j = i + 1;
        }
    }
    
    println!("{:?}",arr);
}