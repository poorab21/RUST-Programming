use std::io;
use std::collections::HashMap;

fn main() {
    let mut seconds = String::new();
    let mut earth_year_in_seconds = 31557600 as f32 ;
    let mut planet_year_in_seconds = HashMap::from([
        ( "Mercury" ,  earth_year_in_seconds * 0.2408467 ) , 
        ( "Venus" , earth_year_in_seconds * 0.61519726 ) , 
        ( "Earth" , earth_year_in_seconds ) ,
        ( "Mars" , earth_year_in_seconds * 1.8808158 ) , 
        ( "Jupiter" , earth_year_in_seconds * 11.862615 ) ,
        ( "Saturn" , earth_year_in_seconds * 29.447498 ) ,
        ( "Uranus" , earth_year_in_seconds * 84.016846 ) ,
        ( "Neptune" , earth_year_in_seconds * 164.79132 )
    ]);

    println!("Enter Seconds:");
    io::stdin().read_line(&mut seconds).expect("Something went wrong while processing input for seconds");
    seconds.pop();

    let seconds : f32 = seconds.trim().parse().unwrap();

    for ( index , value ) in &planet_year_in_seconds {
        println!("Age on {} = {}",index,(seconds/value));
    }
}