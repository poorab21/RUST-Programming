use std::io;

fn main() {
    let mut s = String::new();
    let mut k = String::new();
    let mut i = 0;
    let mut count = 0;

    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();

    println!("Enter K:");
    io::stdin().read_line(&mut k).expect("Something went wrong while processing input for K");
    k.pop();

    let k : i32 = k.trim().parse().unwrap();

    while i + k <= ( s.trim().len() as i32 ) {
        if is_unique(&s[ i as usize .. (i + k) as usize ]) {
            count += 1; 
        }
        i += 1;
    }

    println!("Count of substrings of length {k} with exactly {k} distinct characters = {count}");
}

fn is_unique( substr : &str ) -> bool {
    for i in 0..substr.len() {
        if has_occurred(substr, i) { return false }
    }    
    true
}

fn has_occurred( s : &str , index : usize ) -> bool {
    for i in 0..index {
        if s.chars().nth(i).unwrap() == s.chars().nth(index).unwrap() {
            return true;
        }
    }
    false
}