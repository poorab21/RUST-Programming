fn to_uppercase( vec : &mut Vec<String> ) {
    *vec = vec.iter().map( |val| val.to_ascii_uppercase() ).collect::<Vec<String>>();
}

fn main() {
    let mut vec = vec![
        String::from("hello") ,
        String::from("there") ,
        String::from("johNNy") ,
        String::from("Barney") ,
        String::from("tHEODORE")
    ];
    
    to_uppercase( &mut vec );
    println!("{:#?}",vec);
}