use std::io;

fn main() {
    let mut strArr  = [
        "Audie" ,
        "Andy"
    ];
    
    let mut str = 0;
    let mut commonPrefix : &str = "";
    
    if strArr.len() == 1 { println!("Atleast 2 Strings must be provided"); return; }

    while str < strArr.len() {
        if commonPrefix.len() > 0 {
            let mut count = 0;
            while count < commonPrefix.len() && &strArr[str][count..count+1] == &commonPrefix[count..count+1] { 
                count += 1;
            }
            if count < commonPrefix.len() { commonPrefix = &strArr[str][0..count]; }
            str += 1;
        }
        else {
            let mut index = 0;
            while strArr[str].chars().nth(index).unwrap() == strArr[str+1].chars().nth(index).unwrap() {
                index += 1;
            }
            commonPrefix = &strArr[str][0..index];
            str += 2;
        }
    }
    
    if commonPrefix.len() > 0 {
        println!("Longest Common Prefix in the Strings = {}",commonPrefix);
    }
    else { println!("No Longest Common Prefix found"); }
}