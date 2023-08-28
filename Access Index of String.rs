use std::io;

fn main(){
    let mut s = String::from("");
    
    println!("Enter String:");
    io::stdin().read_line(&mut s).expect("Something went wrong");
    
    for i in 0..s.len(){ // len() returns usize type so it can be deduced that usize can be used in loops
        print!("{} \n",&s[i..i+1])
    }
}