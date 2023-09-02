use std::io;

fn main() {
    let mut input = String::new();
    let mut arr : [ String ; 5 ] = [
        String::from("Hello") ,
        String:: from("Johnny") ,
        String::from("United States") ,
        String::from("Indian") ,
        String::from("Earthquake")
    ];
    
    let mut strRef = &mut arr[..];
    let mut i = 0;
    
    while i < strRef.len() {
        println!("{:?}",strRef.iter().nth(i).unwrap());
        i += 1;
    }
}