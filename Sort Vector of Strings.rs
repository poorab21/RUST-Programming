fn sort( vec : &Vec<String> ) -> Vec<String> {
    let mut vec1 = vec.clone();
    let mut temp = None;
    let mut temp2 = None;
    
    for i in 0..vec1.len() {
        for j in i..vec1.len() {
            if &vec1[i].to_ascii_lowercase() > &vec1[j].to_ascii_lowercase() {
                temp = Some(vec1[i].clone());
                temp2 = Some(vec1[j].clone());
                std::mem::replace( &mut vec1[i] , temp2.unwrap() );
                std::mem::replace( &mut vec1[j] , temp.unwrap() );
            }
        }
    }
    vec1
}

fn main() {
    let vec = vec![
        String::from("Drabs") ,
        String::from("Steve") ,
        String::from("Xerox") ,
        String::from("Devon") ,
        String::from("Divya Narendra")
    ];
    
    let sorted_vec = sort( &vec );
    
    println!("{:#?}",sorted_vec);
}