use std::io;

fn main() {
    let mut vec1 = vec![-3, 4];
    let mut position = String::new();
    let mut sub : Option<i32> = None;
    
    println!("How many positions to rotate array ? ");
    io::stdin().read_line(&mut position).expect("Something went wrong while processing input for position");
    position.pop();
    
    let mut position : i32 = position.trim().parse().unwrap();
    position %= vec1.len() as i32;
    
    for i in 0..position {
        for j in (0..vec1.len()).rev() {
            if let None = sub {    
                sub = Some(vec1[j]);
                vec1[j] = vec1[(j+1) % vec1.len()];
            }
            else {
                sub = Some(sub.unwrap() + vec1[j]);
                vec1[j] = sub.unwrap() - vec1[j];
                sub = Some(sub.unwrap() - vec1[j]);
            }
        }
        sub = None;
    }
    
    println!("{:?}",vec1);
}