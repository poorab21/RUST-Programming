fn remove_even( vec : &Vec<i32> ) -> Vec<i32> {
    let mut vec1 : Vec<i32> = Vec::new();
    
    for i in 0..vec.len() {
        if vec[i] % 2 != 0 { vec1.push( vec[i].clone() ); }
    }
    
    vec1
}

fn main() {
    let vec = vec![1,4,8,9,10,11,73,88];
    let new_vec = remove_even(&vec);
    println!("{:?}",new_vec);
}