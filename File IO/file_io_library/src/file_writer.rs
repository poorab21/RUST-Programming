use std::io::Write;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;

pub fn create_file( filename : &str ) -> Result< String , Box< dyn std::error::Error > > {
    File::create(filename)?;

    Ok( format!("File '{}' was created successfully",filename))
}

pub fn write_to_file( filename : &str , content : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = File::create(filename).expect("Filename could not be processed");

    let _ = File::write( &mut file , content.as_bytes() )?;
    Ok("Content was successfully written to file".into())
}

pub fn append_to_file( filename : &str , content : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut file = OpenOptions::new().read(true).write(true).open(filename)?;
    let mut file_content = String::new();
    let reader = BufReader::new(file.try_clone().unwrap());

    for line in reader.lines() {
        if let Err(_) = line { return Err("File content could not be retrieved".into()); }

        file_content += format!("{}\n",line.unwrap().trim()).as_str();
    }

    file_content += format!("{}",content).as_str();

    let mut file = File::create(filename);

    if let Err(_) = file { return Err("Something went wrong while processing file".into()); }

    let _ = File::write( &mut file.unwrap() , file_content.as_bytes() )?;

    Ok("Data was successfully appended to file".into())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_create_file() -> Result< () , Box<dyn std::error::Error > > {
        let result = create_file("file3.txt")?;

        Ok(())
    }

    #[test]
    fn check_write_to_file() -> Result< () , Box<dyn std::error::Error > > {
        let result = write_to_file("file2.txt","Hello There")?;

        Ok(())
    }

    #[test]
    fn check_append_to_file() -> Result< () , Box<dyn std::error::Error > > {
        let result = append_to_file("file2.txt","How are you \n fine")?;

        Ok(())
    }
}