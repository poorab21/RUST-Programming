use std::io;

fn main() {
    let mut number = String::new();
    let mut number_words = String::new();

    println!("Enter Number:");
    io::stdin().read_line(&mut number).expect("Something went wrong while processing input for number");
    number.pop();

    for digit in number.chars() {
        number_words += format!("{} ",toWord(digit)).as_str();
    }

    println!("Number in words = {}",number_words);
}

fn toWord( c : char ) -> &'static str {
    if c == '0' { return "zero" }
    else if c == '1' { return "one" }
    else if c == '2' { return "two" }
    else if c == '3' { return "three" }
    else if c == '4' { return "four" }
    else if c == '5' { return "five" }
    else if c == '6' { return "six" }
    else if c == '7' { return "seven" }
    else if c == '8' { return "eight" }
    else if c == '9' { return "nine" }
    ""
 }