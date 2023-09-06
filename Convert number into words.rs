use std::io;

fn main(){
    let mut input = String::new();
    let mut count = 0;
    let mut i = 0;
    let mut wordNum = String::from("");
    
    println!("Enter Number:");
    io::stdin().read_line(&mut input).expect("Something went wrong while processing input for number");
    input.pop();
    
    i = (input.len() - 1) as i32 ;
    
    while i >= 0 {
        if i - 1 == 0 && (count % 3) == 0 && input.chars().nth((i as usize) - 1).unwrap() != '0' {
            wordNum = format!( "{} {} {}" , toWord( format!( "{}{}" , input.chars().nth((i as usize)-1).unwrap() , input.chars().nth(i as usize).unwrap() ).trim().parse().unwrap() ) , zeroCount(count) , wordNum );
            break;
        }   
        else if i == 0 && (count % 3) == 0 && input.chars().nth(i as usize).unwrap() != '0' {
            wordNum = format!( "{} {} {}" , toWord( format!( "{}" , input.chars().nth(i as usize).unwrap() ).trim().parse().unwrap() ) , zeroCount(count) , wordNum );
            break;
        }
        else if (count + 1) % 3 == 0 && input.chars().nth(i as usize).unwrap() == '0' {
            count += 1;
            i -= 1;
        }
        else if (count + 1) % 3 == 0 && input.chars().nth(i as usize).unwrap() != '0' {
            wordNum = format!( "{} hundred {}" , toWord( format!( "{}" , input.chars().nth(i as usize).unwrap() ).trim().parse().unwrap() ) , wordNum );
            count += 1;
            i -= 1;
        }
        else if i - 1 >= 0 && (count % 3) == 0 && input.chars().nth((i as usize) - 1).unwrap() != '0' || input.chars().nth(i as usize).unwrap() != '0' {
            wordNum = format!( "{} {} {}" , toWord(format!( "{}{}" , input.chars().nth((i as usize) - 1).unwrap() , input.chars().nth(i as usize).unwrap() ).trim().parse().unwrap()) , zeroCount(count) , wordNum );
            count += 2;
            i -= 2;
        }
        else if i - 2 >= 0 && (count % 3) == 0 && input.chars().nth((i as usize) - 2).unwrap() == '0' && input.chars().nth((i as usize) - 1).unwrap() == '0' && input.chars().nth(i as usize).unwrap() == '0' {
            count += 2;
            i -= 2;
        }
        else if i - 2 >= 0 && (count % 3) == 0 && input.chars().nth((i as usize) - 2).unwrap() != '0' && input.chars().nth((i as usize) - 1).unwrap() == '0' && input.chars().nth(i as usize).unwrap() == '0' {
            wordNum = format!( "{} {}" , zeroCount(count) , wordNum );
            count += 2;
            i -= 2;
        }
    }
    println!("{}",wordNum.trim());
}

fn zeroCount( zeros : i32 ) -> &'static str {
    if zeros == 3 { "thousand" }
    else if zeros == 6 { "million" }
    else if zeros == 9 { "billion" }
    else { "" }
}

fn toWord( number : i32 ) -> String {
    if number >= 0 && number <= 19 { processTill19( format!("{}",number).as_str() ).to_string() }
    else if number >= 20 && number <= 99 { processTill99( format!("{}",number).as_str() ).to_string() }
    else { "".to_string() }
}

fn processTill19( number : &str ) -> &'static str {
    if number == "1" { "one" }
    else if number == "2" { "two" }
    else if number == "3" { "three" }
    else if number == "4" { "four" }
    else if number == "5" { "five" }
    else if number == "6" { "six" }
    else if number == "7" { "seven" }
    else if number == "8" { "eight" }
    else if number == "9" { "nine" }
    else if number == "10" { "ten" }
    else if number == "11" { "eleven" }
    else if number == "12" { "twelve" }
    else if number == "13" { "thirteen" }
    else if number == "14" { "fourteen" }
    else if number == "15" { "fifteen" }
    else if number == "16" { "sixteen" }
    else if number == "17" { "seventeen" }
    else if number == "18" { "eighteen" }
    else if number == "19" { "ninteen" }
    else { "" }
}

fn processTill99(number : &str ) -> String {
    let mut num = String::from("");
    
    for i in 0..number.len() {
        if i == 0 && number.chars().nth(i).unwrap() == '2' { num += "twenty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '3' { num += "thirty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '4' { num += "forty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '5' { num += "fifty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '6' { num += "sixty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '7' { num += "seventy "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '8' { num += "eighty "; }
        else if i == 0 && number.chars().nth(i).unwrap() == '9' { num += "ninty "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '1' { num += "one "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '2' { num += "two "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '3' { num += "three "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '4' { num += "four "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '5' { num += "five "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '6' { num += "six "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '7' { num += "seven "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '8' { num += "eight "; }
        else if i == 1 && number.chars().nth(i).unwrap() == '9' { num += "nine "; }
    }
    num
}