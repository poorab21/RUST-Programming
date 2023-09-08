use std::io;

fn main() {
    let mut input = String::new();
    let mut size = 0;
    let mut vec1 : Vec<f32> = Vec::new();
    
    println!("Enter Size:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for array size");
    input.pop();
    
    size = input.trim().parse().unwrap();
    input.clear();
    
    for i in 0..size {
        println!("Enter Element:");
        io::stdin().read_line(&mut input).expect(format!("Something went wrong while processing input for index {} of array",i).as_str());
        input.pop();
        
        vec1.push(input.trim().parse().unwrap());
        input.clear();
    }
    println!("{:?}",vec1);
}