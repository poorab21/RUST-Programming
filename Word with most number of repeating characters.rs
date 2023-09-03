use std::io;

fn main() {
    let mut input = String::new();
    let mut count = 0;
    let mut largest = 0;
    let mut index : i32 = -1;
    let mut numOfRepeats = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for String");
    input.pop();
    
    let mut words = input.split(" ");
    
    for ( i , word ) in words.clone().enumerate() {
        for j in 0..word.len() {
            if hasOccurred(word,j) { continue; }
            
            for k in j..word.len() {
                if word.chars().nth(k).unwrap() == word.chars().nth(j).unwrap() { count += 1; }
            }
            
            if count > 1 { numOfRepeats += 1; }
            count = 0;
        }
        
        if numOfRepeats > largest { index = i as i32; largest = numOfRepeats }
        
        numOfRepeats = 0;
    }
    
        
    if index != -1 {
        println!("Word which has the highest number of repeated letters = {}",words.nth(index as usize).unwrap());
    }
    else  { println!("Word which has the highest number of repeated letters = None"); }
}

fn hasOccurred( word : &str , index : usize ) -> bool {
    for i in 0..index {
        if word.chars().nth(i).unwrap() == word.chars().nth(index).unwrap() { return true }
    }
    false
}