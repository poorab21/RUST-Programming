use std::io;
use std::net::TcpStream;
use clearscreen::clear;
use crate::download::download_file;
use crate::fileview::{ view_all , search_file_in_server };
use crate::upload::{file_create_and_upload, local_file_upload};
use crate::deletion::delete_folder;

pub fn menu( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut choice = String::new();

    println!("1) Create and Upload File");
    println!("2) Upload File");
    println!("3) Your Stored Files");
    println!("4) Search File");
    println!("5) Download File");
    println!("6) Delete Folder");
    println!("7) Exit");
    println!("Select your choice:");
    io::stdin().read_line( &mut choice ).expect("Could not process input for choice");

    clear().unwrap();
    let choice = choice.trim().parse::<i32>();

    if let Err(_) = choice {
        return Err("Please enter your choice in a valid format".into());
    }

    match choice.unwrap() {
        1 => { 
            return file_create_and_upload( source , ip_address );
        },
        2 => {
            return local_file_upload(source, ip_address);
        },
        3 => {
            return view_all(source, ip_address);
        },
        4 => {
            return search_file_in_server(source , ip_address);
        },
        5 => {
            return download_file(source , ip_address );
        },
        6 => {
            return delete_folder( source , ip_address );
        },
        7 => {
            return Ok("break".into());
        },
        _ => { Err("Please select a valid choice".into()) } 
    }
}