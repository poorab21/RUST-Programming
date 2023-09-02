use std::io;

fn main() {
    let mut input = String::new();
    
    println!("Enter String:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for string");
    input.pop();
    
    let mut strRef = &mut input;
    let mut character1 = ' ';
    let mut character2 = ' ';
    
    for i in 0..strRef.len() {
        for j in i+1..strRef.len() {
            character1 = strRef.chars().nth(i).unwrap();
            character2 = strRef.chars().nth(j).unwrap();
            if isAlphabet(character1) && isAlphabet(character2) && character1 > character2 {
                Swap(strRef,i,j);
            }
        }
    }
    
    println!("String in alphabetical order = {}",input);
}

fn isAlphabet(character : char) -> bool {
    if character >= 'a' && character <= 'z' || character >= 'A' && character <= 'Z' { return true; }
    return false;
}

fn Swap(str : &mut String, i : usize, j : usize) {
    let sub = str.chars().nth(i).unwrap();
    str.replace_range(i..i+1,format!("{}", str.chars().nth(j).unwrap() ).as_str());
    str.replace_range(j..j+1,format!("{}",sub).as_str());
}