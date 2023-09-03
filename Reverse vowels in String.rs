use std::io;

fn main() {
    let mut input = String::new();
    let mut frontptr = 0;
    let mut endptr = 0;
    let mut frontChar : char = ' ';
    let mut endChar : char = ' ';
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for String");
    input.pop();
    
    
    if input.len() <= 0 { return; }
    
    endptr = input.len() - 1;

    while frontptr < endptr {
        frontChar = input.chars().nth(frontptr).unwrap();
        endChar = input.chars().nth(endptr).unwrap();
        
        if isVowel(frontChar) && isVowel(endChar) {
            Swap(&mut input,frontptr,endptr);
            frontptr += 1;
            endptr -= 1;
        }
        else if isVowel(frontChar) && !isVowel(endChar) {
            endptr -= 1;
        }
        else if !isVowel(frontChar) && isVowel(endChar) {
            frontptr += 1;
        }
        else if !isVowel(frontChar) && !isVowel(endChar) {
            frontptr += 1;
            endptr -= 1;
        }
    }
    println!("String with reversed vowels = {}",input);
}

fn isVowel(c : char) -> bool {
    if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'A' || c == 'E' || c == 'I' || c == 'O' || c == 'U' {
        return true;
    }
    false
}

fn Swap(s : &mut String , index1 : usize , index2 : usize ) {
    let sub = s.chars().nth(index1).unwrap();
    s.replace_range(index1..index1+1,format!("{}",s.chars().nth(index2).unwrap()).as_str());
    s.replace_range(index2..index2+1,format!("{}",sub).as_str());
}