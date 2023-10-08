use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let word_regex = Regex::new("^[a-zA-Z]+$").unwrap();
    let file = File::open("file.txt").expect("Something went wrong while trying to open the specified file");    
    let mut words : HashMap< String , u32 > = HashMap::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        for word in line.unwrap().split(" ") {
            if let Some(_) =  word_regex.find(word) { *(words.entry(word.to_ascii_lowercase()).or_insert(0)) += 1; }
        }
    }

    println!("{:?}",words);
}