fn factorial( n : i32 ) -> i32 {
    if n == 1 { return 1; }
    
    n * factorial(n-1)
}

fn main() {
    println!("{}",factorial(5));
    println!("{}",factorial(4));
    println!("{}",factorial(6));
}