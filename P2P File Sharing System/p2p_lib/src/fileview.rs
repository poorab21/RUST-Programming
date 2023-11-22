use std::{net::TcpStream, io::Write};
use std::io::{ BufRead , BufReader , self };
use std::str::from_utf8;

use crate::userfile::{UserFile, Access};

pub fn format_files( files : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut i = 0;
    let mut formatted = String::new();

    while i < files.len() {
        
        if i < files.len() - 1 && files.chars().nth(i).unwrap() == '\\' && files.chars().nth(i+1).unwrap() == 'n' {
            formatted.push('\n');
            i += 2;
        }
        else if i < files.len() - 1 && files.chars().nth(i).unwrap() == '\\' && files.chars().nth(i+1).unwrap() == '\"' {
            i += 2;
        }
        else if i < files.len() - 1 && files.chars().nth(i).unwrap() == '\\' && files.chars().nth(i+1).unwrap() == '0' {
            i += 2;
        }
        else {
            formatted.push(files.chars().nth(i).unwrap());
            i += 1;
        }
    }

    Ok(formatted)
}

pub fn view_all( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let json_data = UserFile::new( 
        "".to_string() , 
        "".to_string(), 
        "".to_string(), 
        ip_address.to_string() , 
        Access::PUBLIC , 
        3
    );

    let request = serde_json::to_string(&json_data);
    let mut response : Vec<u8> = Vec::new();

    source.write_all(request?.as_bytes() )?;
    source.write_all( b"\n" )?;

    let mut reader = BufReader::new( source );
    reader.read_until( b'\0' , &mut response )?;

    Ok( format_files(from_utf8(&response)?)? )
}

pub fn search_file_in_server( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut response : Vec<u8> = Vec::new();
    let mut filename = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename).expect("Could not process input for file to search");
    filename.pop();

    if filename.trim().len() == 0 { return Err("Filename must be specified".into()); }

    let request = UserFile::new( 
        filename.trim().to_string()
        , 
        "".to_string() , 
        "".to_string() , 
        ip_address.to_string() , 
        Access::PUBLIC , 
        4
    );
    
    source.write_all( serde_json::to_string(&request)?.as_bytes() )?;
    source.write_all( b"\n" )?;

    let mut reader = BufReader::new(source);
    reader.read_until( b'\0', &mut response).unwrap();

    Ok( from_utf8( &response )?.to_string() )
}