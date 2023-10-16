use std::io;

fn main(){
   let mut s1 = String::new();
   let mut s2 = String::new();

   println!("Enter First String:");
   io::stdin().read_line(&mut s1).expect("Something went wrong while processing input for the first string");
   s1.pop();

   println!("Enter Second String:");
   io::stdin().read_line(&mut s2).expect("Something went wrong while processing input for the second string");
   s2.pop();

   let larger_string = which_is_larger( &s1 , &s2 );
   
   println!("\n{}",larger_string);
}

fn which_is_larger<'a>( s1 : &'a str , s2 : &'a str ) -> &'a str {
	if s1.len() >= s2.len() { s1 }
	else { s2 }
}