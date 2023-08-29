use std::io;

fn main() {
    let mut input = String::new();
    let mut sum = 0;
    let mut sub = 0;
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input");
    input.pop();
    
    for i in input.chars() {
        if i >= '0' && i <= '9' {
            sub = ( sub * 10 ) + (i as u32 - '0' as u32);
        }
        else if i < '0' || i > '9' && sub > 0 {
            sum += sub;
            sub = 0;
        }
    }
    
    if sub > 0 { sum += sub ; sub = 0; }
    println!("{}",sum);
}