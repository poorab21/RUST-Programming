use std::io;

fn main() {
    let mut s = String::new();
    let mut startptr = 0;
    let mut endptr = 0;
    let mut count = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
    s.pop();
    
    endptr = s.len() - 1;
    let mut str_len = s.len();
    
    while startptr < endptr {
        if &s[0..startptr+1] == &s[endptr..str_len] {
            count = startptr + 1;
        }
        startptr += 1;
        endptr -= 1;
    }
    println!("Length of the Longest prefix which is also a suffix = {}",count);
}