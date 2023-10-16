fn sum( tuple : ( i32 , i32 ) ) -> i32 {
    tuple.0 + tuple.1
}

fn main() {
    println!("{}",sum((14,6)));
}