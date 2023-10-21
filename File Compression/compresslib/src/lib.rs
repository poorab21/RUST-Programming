use std::fs::{ self , File };
use std::io::Write;
use std::io::Read;
use std::io::BufRead;
use std::io::BufReader;

pub fn create( filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let _ = File::create(filename)?;

    Ok(format!("File {} successfully created",filename))
}

pub fn write_to_file( filename : &str , content : &str ) -> Result< String , Box<dyn std::error::Error > > {
    File::open(filename)?;
    
    let mut file = File::create(filename)?;
    let _ = file.write( content.as_bytes() )?;

    Ok( format!("File {} successfully updated with content",filename) )
}

pub fn read_file( filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = File::open(filename)?;
    let mut content = String::new();

    let _ = file.read_to_string( &mut content )?;
    Ok(content)
}

fn compression( word : &str ) -> String {
    let mut compressed_str = String::new();
    let mut i = 0;
    let mut character = word.chars().nth(0).unwrap();
    
    while i < word.len() {
        let c = word.chars().nth(i).unwrap();
        
        if character != c { 
            compressed_str.push(character);
            character = c; 
            continue;
        }
        else if i == word.len() - 1 {
            compressed_str.push(character);
            break;
        }
        
        i += 1;
    }
    compressed_str
}

pub fn compress( filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    let reader = BufReader::new(file);
    let mut original = 0;

    for line in reader.lines() {
        if let Err(e) = line { return Err(e.into()); }
        
        let mut line_in_file = line.unwrap();

        original += line_in_file.len();

        for word in line_in_file.trim().split_whitespace() {
            content += format!("{} ",compression( word )).as_str();
        }
        content += "\n";
    }
    

    let mut file = File::create(filename)?;
    let _ = file.write( content.trim().as_bytes() )?;

    Ok( format!("File {} was successfully compressed from length of {} to {}",filename,original,content.trim().len()) )
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_file_creation() -> Result< () , Box<dyn  std::error::Error > > {
        let result = create("file.txt");

        if let Err(e) = result { return Err(e); }

        Ok(())
    }
}
