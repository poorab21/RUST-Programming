use std::io;

fn main() {
    let mut license_plate = String::new();
    let mut k = String::new();
    let mut count = 0;
    println!("Enter License Plate:");
    io::stdin().read_line(&mut license_plate).expect("Something went wrong while processing input for license plate");
    license_plate.pop();

    println!("Enter K:");
    io::stdin().read_line(&mut k).expect("Something went wrong while processing input for value of k");
    k.pop();

    let mut k : usize = k.trim().parse().unwrap();

    license_plate = license_plate.to_ascii_uppercase().replace("-","");
    
    let mut i : i32 = (license_plate.len() - 1) as i32;
    while i >= 0 {
        if count == k {
            license_plate.insert((i as usize)+1,'-');
            count = 0;
        }
        
        count += 1;
        i -= 1;
    }
    
    println!("{}",license_plate);
}