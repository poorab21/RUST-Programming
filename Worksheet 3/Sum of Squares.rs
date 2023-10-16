fn sum_of_squares<T: std::cmp::PartialOrd + num::Num + std::ops::Mul + Clone + std::iter::Sum>( vec : &mut Vec<T> ) -> T {
    vec.iter().map(|x| x.clone() * x.clone()).sum()
    
}

fn main() {
    println!("{}",sum_of_squares(&mut vec![6u32,82u32,8u32,10u32,17u32]));
}