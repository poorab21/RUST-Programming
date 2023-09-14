use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut a_ptr = 0;
    let mut b_ptr = 0;
    let mut c_ptr = 0;
    
    println!("Enter A:");
    io::stdin().read_line(&mut a).expect("Something went wrong while processing input for A");
    a.pop();
    
    println!("Enter B:");
    io::stdin().read_line(&mut b).expect("Something went wrong while processing input for B");
    b.pop();
    
    println!("Enter C:");
    io::stdin().read_line(&mut c).expect("Something went wrong while processing input for C");
    c.pop();
    
    if a.len() + b.len() != c.len() { println!("{c} is not an interleaved string of {a} and {b}"); return; }
    
    while c_ptr < c.len() {
      let mut sub_a = a_ptr;
      let mut sub_b = b_ptr;
      let mut sub_c = c_ptr;

      while sub_a < a.len() && a.chars().nth(sub_a).unwrap() == c.chars().nth(sub_c).unwrap() {
         sub_a += 1;
         sub_c += 1;
      }

      sub_c = c_ptr;

      while sub_b < b.len() && b.chars().nth(sub_b).unwrap() == c.chars().nth(sub_c).unwrap() {
         sub_b += 1;
         sub_c += 1;  
      }

      if sub_a - a_ptr == 0 && sub_b - b_ptr == 0 { println!("{c} is not an interleaved string of {a} and {b}") ; return; }
      else if sub_a - a_ptr >= sub_b - b_ptr { c_ptr += sub_a - a_ptr ; a_ptr = sub_a;  }
      else if sub_a - a_ptr < sub_b - b_ptr { c_ptr += sub_b - b_ptr ; b_ptr = sub_b; }  
    }
    
    println!("{c} is an interleaved string of {a} and {b}");
}