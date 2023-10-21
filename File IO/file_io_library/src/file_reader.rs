use std::collections::HashMap;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs::OpenOptions;
use regex::Regex;

pub fn read_file( filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = File::open(filename)?;
    let mut content = String::new();

    file.read_to_string( &mut content )?;

    Ok(content)
}

pub fn occurrences_of_words_in_file( filename : &str ) -> Result< HashMap< String , u32 > , Box<dyn std::error::Error> > {
    let file = File::open(filename)?;
    let mut word_count : HashMap<String,u32> = HashMap::new();
    let reader = BufReader::new(file);
    let word_regex = Regex::new("^[a-zA-Z]+$").unwrap();

    for line in reader.lines() {
        if let Err(_) = line { return Err("File contents could not be read".into()) } 

        for word in line.unwrap().trim().split_whitespace() {
            if let None = word_regex.find(word.trim()) { continue; }
            *(word_count.entry(word.to_ascii_lowercase()).or_insert(0)) += 1;
        }
    }

    Ok( word_count )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_read_file() -> Result< () , Box< dyn std::error::Error > > {
        let content = read_file("C://Users//SAIF UR REHMAN//Desktop//RUST Projects//File IO//file_io_library//file2.txt")?;
        
        if content.trim() == "Hello There" {
            return Ok(());
        }
        
        Err("File content is erroneous".into())
    }

    #[test]
    fn check_occurrences() -> Result< () , Box< dyn std::error::Error > > {
        let occurrences = occurrences_of_words_in_file("C://Users//SAIF UR REHMAN//Desktop//RUST Projects//File IO//file_io_library//file2.txt")?;
        
        if occurrences.len() != 2 {
            return Err("HashMap does not contain all the words from the file".into());
        }

        Ok(())
    } 
}