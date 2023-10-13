use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn read_file_content( filepath : &str ) -> Result< String , std::io::Error > {
    let mut file = File::open(filepath)?;
    let mut content = String::new();

    file.read_to_string( &mut content )?;

    Ok(content)
}

fn main() {
    let file_content = read_file_content("file.txt");

    if let Ok(content) = file_content {
        println!("{}",content);
    }
    else if let Err(e) = file_content {
        match e.kind() {
            ErrorKind::NotFound =>  println!("file not found") ,
            _ => println!("Some other error occurred")
        }
    }
}