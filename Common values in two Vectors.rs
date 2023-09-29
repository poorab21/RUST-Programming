fn common_values( vec1 : &Vec<i32> , vec2 : &Vec<i32> ) -> Vec<i32> {
    let mut vec = Vec::new();
    
    for num in vec1 {
        if vec2.contains(num) && !vec.contains(num) {
            vec.push(num.clone());
        }
    }
    vec
}

fn main() {
    let mut vec1 = vec![3,2,3,5,8,7,11,11];
    let mut vec2 = vec![3,5,7,9,10,11];
    
    let mut common_vector = common_values( &vec1 , &vec2 );
    
    println!("{:#?}",common_vector);
}