use std::io;

fn main(){
    let mut name = String::new();
    let mut sub = String::new();
    
    println!("Enter Name:");
    io::stdin().read_line(&mut name).expect("Something went wrong");
    name.pop();

    for i in name.trim().rsplit(" ") {
        sub += format!("{} ",i).as_str();                    
    }
    println!("{}",sub);
}