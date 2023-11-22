use std::{net::TcpStream, io::{Write, BufReader, BufRead}, fs::OpenOptions };
use std::str::from_utf8;
use std::io;
use regex::Regex;

use crate::userfile::{ UserFile, Access };

pub fn download_file( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut filename = String::new();
    let mut folder = String::new();
    let mut downloader_src = String::new();
    let mut buffer : Vec<u8> = Vec::new();
    let ip_format = Regex::new("^(([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])\\.){3}([0-9]|[1-9][0-9]|1[0-9][0-9]|2[0-4][0-9]|25[0-5])$").unwrap();

    println!("From whom to download the file (IP Address):");
    io::stdin().read_line( &mut downloader_src ).expect("Could not process the input for the source from which to download file");
    downloader_src.pop();

    if ip_format.captures(downloader_src.trim()).is_none() { return Err("Source IP must be specified in proper format".into()); }

    println!("Enter Folder name:");
    io::stdin().read_line( &mut folder ).expect("Could not process input for the folder to download from");
    folder.pop();

    if folder.trim().len() == 0 { return Err("Folder name must be specified".into()); }

    println!("Enter Filename (with extension):");
    io::stdin().read_line( &mut filename ).expect("Could not process the input for file to be downloaded");
    filename.pop();

    if filename.trim().len() == 0 { return Err("Filename must be specified".into()); }

    let request : UserFile = UserFile::new(
        filename.clone() , 
        format!("Folder = {}",folder) , 
        "".to_string() , 
        format!("{}-{}",ip_address,downloader_src) , 
        Access::PUBLIC, 
        5
    );

    source.write_all( serde_json::to_string(&request)?.as_bytes() )?;
    source.write_all( b"\n" )?;

    let mut reader = BufReader::new(source);
    reader.read_until( b'\0', &mut buffer).unwrap();

    let response = from_utf8( &buffer )?.to_string();

    if response.contains("The system cannot find the path specified. (os error 3)") || response.contains("The system cannot find the file specified. (os error 2)") {
        return Err(response.into());
    }
    
    OpenOptions::new().create(true).write(true).open(
        format!("C:/Users/SAIF UR REHMAN/Desktop/{}",filename.trim())
    )?.write_all( &buffer )?;

    Ok(
        "File successfully downloaded to your Local Device".into()
    )
}