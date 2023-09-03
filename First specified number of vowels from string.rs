use std::io;

fn main(){
    let mut input = String::new();
    let mut numOfVowels = 0;
    let mut vowels = String::new();
    let mut i = 0;
    
    println!("Enter number of vowels:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for number of vowels");
    input.pop();
    
    numOfVowels = input.trim().parse().unwrap();
    input.clear();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    while numOfVowels > 0 && i < input.len() {
        let c = input.chars().nth(i).unwrap();
        
        if isVowel(c) { 
            numOfVowels -= 1;
            vowels += format!("{} ",c).as_str();
        }
        i += 1;
    }
    if numOfVowels > 0 { println!("n is less than number of vowels present in the string!"); return; }
    println!("{}",vowels);
}

fn isVowel( c : char ) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
        return true;
    }
    false
}