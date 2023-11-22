use std::net::TcpStream;
use std::io::{self, Write, BufRead , BufReader};
use std::str::from_utf8;
use crate::userfile::{UserFile, Access};

pub fn delete_folder( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut foldername = String::new();
    let mut buffer = Vec::new();

    println!("Enter Folder name:");
    io::stdin().read_line( &mut foldername ).expect("Could not process input for folder to delete");
    foldername.pop();

    if foldername.trim().is_empty() { return Err("Folder name must be provided".into()); }

    let userfile = UserFile::new(
        foldername.clone() , 
        "".to_string() , 
        "".to_string() , 
        ip_address.to_string() , 
        Access::PUBLIC , 
        6
    );

    let json_request = serde_json::to_string( &userfile )?;

    source.write_all( json_request.as_bytes() )?;
    source.write_all(b"\n")?;

    let mut reader = BufReader::new( source );
    reader.read_until( b'\0' , &mut buffer )?;

    Ok(
        from_utf8( &buffer )?.to_string()
    )
}
