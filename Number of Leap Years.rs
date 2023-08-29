use std::io;

fn main() {
    let mut year1 = String::new();
    let mut year2 = String::new();
    
    println!("Enter Year : ");
    io::stdin().read_line(&mut year1).expect("Something went wrong while processing input for Year 1");
    year1.pop();
    
    println!("Enter Year : ");
    io::stdin().read_line(&mut year2).expect("Something went wrong while processing input for Year 2");
    year2.pop();
    
    let year1 : i32 = year1.trim().parse().unwrap();
    let year2 : i32 = year2.trim().parse().unwrap();
    
    if year1 <= 0 || year2 >= 3000 {
        println!("Error! please make sure the inputted years satisfy the following conditions \n( 0 < year1 = year2 < 3,000)");
        return;
    }
    
    for i in year1+1..year2 {
        if i % 4 == 0 {
            print!("{} ",i);
        }
    }
    
    
}