use std::io;

fn main() {
   let mut censored_string = String::new();
   let mut censored_vowels = String::new();
   
   println!("Enter Censored String:");
   io::stdin().read_line(&mut censored_string).expect("Something went wrong while processing input for censored string");
   censored_string.pop();

   println!("Enter Censored Vowels:");
   io::stdin().read_line(&mut censored_vowels).expect("Something went wrong while processing input for censored vowels");
   censored_vowels.pop();

   if total_censored_characters( &censored_string ) != censored_vowels.trim().len() as u8 { println!("Number of censored characters do not equal number of vowels"); return; }

   let uncensored_string = uncensor( &mut censored_string , &censored_vowels );

   println!("{}",uncensored_string);   

}

fn total_censored_characters( censored_string : &str ) -> u8 {
	let mut count = 0;

	for c in censored_string.chars() {
		if c == '*' { count += 1; }
	}
	count
}

fn uncensor<'a>( censored_string : &'a mut String , censored_vowels : &'a String ) -> &'a str {
	let mut count = 0;
	
	for i in 0..censored_string.len() {
		if censored_string.chars().nth(i).unwrap() == '*' { 
			censored_string.replace_range(i..i+1, format!("{}",censored_vowels.chars().nth(count).unwrap()).as_str() );
			count += 1; 
		}
	}
	
	&censored_string[..]
}