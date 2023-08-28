use std::io;
#[allow(unused_assignments)]

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Something went wrong");
    
    let mut nth : i32 = input.trim().parse().unwrap();
    
    let mut first = 0;
    let mut second = 1;
    let mut term = 3;
    
    if nth >= 3 {
      while term <= nth {
        let sub = first;
        first = second;
        second = sub + second;
        term += 1;
      }
     nth = second;
   }
   else if nth == 2 {
       nth = second;
   }
   else if nth == 1 {
       nth = first;
   }
    println!("value = {}",nth);
}
