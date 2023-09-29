fn average( vec : &Vec<i32> ) -> f32 {
    vec.iter().sum::<i32>() as f32 / vec.len() as f32
}
fn main() {
    let vec1 = vec![5,18,3,5,19];
    let average = average(&vec1);
    println!("{}",average);
}