use std::io::{ self , BufRead };
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;

fn main() {
    let mut input = String::new();
    let reader = std::io::stdin().lock();
    let mut file = OpenOptions::new().write(true).read(true).open("file.txt").expect("Something went wrong while opening file.txt");

    file.read_to_string( &mut input ).expect("Something went wrong while retrieving contents of file to input variable");

    println!("Enter String to append:");
    for line in reader.lines() {
        if let Ok(s) = line {
            if s.is_empty() { break; }
            input += format!("\n{}",s).as_str();
        }
    }

    std::mem::drop(file);

    let mut file = File::create("file.txt").expect("Something went wrong while overwriting file");
    file.write_all( input.as_str().as_bytes() ).expect("Something went wrong while writing to file.txt");
}