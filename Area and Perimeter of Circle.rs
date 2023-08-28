use std::io;

fn main(){
    let mut radius = String::new();
    let mut area : f64 = 0 as f64;
    let mut perimeter : f64 = 0 as f64;
    
    println!("Enter Radius:");
    io::stdin().read_line(&mut radius).expect("Something went wrong");
    
    let radius : f64 = radius.trim().parse().unwrap();
    perimeter = (2 as f64) * (3.14 as f64) * radius;
    area = (3.14 as f64) * radius * radius;
    
    print!("Perimeter = {}\nArea = {}",perimeter,area);
}