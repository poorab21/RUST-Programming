use std::io;

fn main() {
	let mut s = String::new();
	let mut c1 = String::new();
	let mut c2 = String::new();

	println!("Enter String:");
	io::stdin().read_line(&mut s).expect("Something went wrong while processing input for string");
	s.pop();
	
	println!("Enter First character:");
	io::stdin().read_line(&mut c1).expect("Something went wrong while processing input for the first character");
	c1.pop();

	println!("Enter Second character:");
	io::stdin().read_line(&mut c2).expect("Something went wrong while processing input for the second character");
	c2.pop();

	let c1 = c1.trim().parse::<char>().unwrap();
	let c2 = c2.trim().parse::<char>().unwrap();
	
	println!("String after swapping = {}",swap_characters(&mut s,c1,c2));
}

fn swap_characters( s : &mut String , c1 : char , c2 : char ) -> &str {
	for i in 0..s.len() {
		if s.chars().nth(i).unwrap() == c1 { s.replace_range( i..i+1 , format!("{}",c2).as_str() ); }
		else if s.chars().nth(i).unwrap() == c2 { s.replace_range( i .. i + 1 , format!("{}",c1).as_str() ); }
	}
	&s[..]
}