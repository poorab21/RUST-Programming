use std::io;

fn main(){
    let mut value = String::from("");
    println!("Enter String:");
    io::stdin().read_line(&mut value).expect("Something went wrong");
    
    let str_bytes = value.bytes();
    
    for i in str_bytes {
        println!("{}",i as char);
    }
}