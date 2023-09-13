use std::io;

fn main() {
    let mut num = String::new();
    
    println!("Enter Number:");
    io::stdin().read_line(&mut num).expect("Something went wrong while processing input for number");
    num.pop();
    
    let mut num : u64 = num.trim().parse().unwrap();
    
    println!("{}",excel_column(num));
}

fn excel_column( num : u64 ) -> String {
    if num == 0 { return "".to_string(); }
    
    let mut i = 0;
    let mut n = 0;
    
    while 26 * i < num {
        i += 1;
    }
    i -= 1;
    
    n = num - (26 * i);
    
    excel_column(i) + to_corresponding_alphabet(n as u8)
    
    
}

fn to_corresponding_alphabet( num : u8 ) -> &'static str {
    if num == 1 { return "A" }
    if num == 2 { return "B" }
    if num == 3 { return "C" }
    if num == 4 { return "D" }
    if num == 5 { return "E" }
    if num == 6 { return "F" }
    if num == 7 { return "G" }
    if num == 8 { return "H" }
    if num == 9 { return "I" }
    if num == 10 { return "J" }
    if num == 11 { return "K" }
    if num == 12 { return "L" }
    if num == 13 { return "M" }
    if num == 14 { return "N" }
    if num == 15 { return "O "}
    if num == 16 { return "P" }
    if num == 17 { return "Q" }
    if num == 18 { return "R" }
    if num == 19 { return "S" }
    if num == 20 { return "T" }
    if num == 21 { return "U" }
    if num == 22 { return "V" }
    if num == 23 { return "W" }
    if num == 24 { return "X" }
    if num == 25 { return "Y" }
    if num == 26 { return "Z" }
    ""
}