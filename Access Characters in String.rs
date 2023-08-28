use std::io;

fn main(){
    let mut value = String::from("");
    println!("Enter String:");
    io::stdin().read_line(&mut value).expect("Something went wrong");
    
    let chars = value.as_str().chars();
    
    for i in chars {
        print!("{} \n",i);
    }
}