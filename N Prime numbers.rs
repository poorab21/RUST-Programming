use std::io;

fn main(){
    let mut num = String::new();
    let mut count : i32 = 0;
    let mut i : i32 = 2;
    
    println!("Print how many first prime numbers ?");
    io::stdin().read_line(&mut num).expect("Something went wrong");
    
    let num : i32 = num.trim().parse().unwrap();
    
    while count < num {
        for j in 1..=i {
            if i%j == 0 && j > 1 && j < i {
                break;
            }
            else if j == i {
                count += 1;
                print!("{} ",i);
            }
        }
        i += 1;
    }
}