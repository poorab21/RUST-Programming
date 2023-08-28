use std::io;

fn main() -> () {
    let mut num = String::new();
    let mut reverse = 0;
    let mut i = 1;
    let mut decrease = 0;
    
    println!("Enter Number:");
    io::stdin().read_line(&mut num).expect("Something went wrong when taking input");
    
    let mut num : i32 = num.trim().parse().unwrap();
    
    while i <= num  {
        for j in 0..num-decrease {
            print!(" ");
        }
        for j in 1..=i {
            print!("* ");
        }
        
        if reverse == 0 && i < num {
            i += 1;
            decrease += 1;
        }       
        else if reverse == 0 && i == num {
            reverse = 1;
            i -= 1;
            decrease -= 1; 
        }
        else if reverse == 1 && i > 1 {
            i -= 1;
            decrease -= 1;
        }
        else if reverse == 1 && i == 1 {
            break;
        }
        else {
            break;
        }
        
        print!("\n");
    }
}