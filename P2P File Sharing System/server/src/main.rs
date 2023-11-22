use std::io::{ BufReader , BufRead , Write };
use std::net::{ TcpListener, TcpStream };
use std::thread::spawn;
use p2p_lib::userfile::UserFile;
use serde_json::from_slice;

mod helper;
use crate::helper::{search_file, download , upload_to_folder , retrieve_files , delete };

fn handle_client( stream : TcpStream ) -> Result< () , Box< dyn std::error::Error > > {
    let source = stream.peer_addr().unwrap();
    let mut buffer = Vec::new();
    let mut stream = BufReader::new( stream );
    
    println!("Request from {}",source);

    loop {
        buffer.clear();

        let bytes_read = stream.read_until( b'\n' , &mut buffer ).expect(format!("error in reading stream from {}",source).as_str() );
    
        if bytes_read == 0 { println!("Session with {} terminated",source) ; return Ok(()) }

        let file : UserFile = from_slice(&buffer)?;

        if file.category() == 1 || file.category() == 2 {
            let result = upload_to_folder( file )?;
            
            write!( stream.get_mut() , "{}\n" , result ).unwrap();
        }
        else if file.category() == 3 {
            let result = retrieve_files( file.ip_address() );
            
            match result {
                Ok(files) if files.len() == 0 => {
                    write!( stream.get_mut() , "Your Server Directory is empty" ).unwrap();
                    write!( stream.get_mut() , "\0" ).unwrap();
                }
                Ok(files) => {
                    write!( stream.get_mut() , "{:#?}\0" , files ).unwrap();
                },
                Err(e) if e.to_string() == "The system cannot find the path specified. (os error 3)" => {
                    write!( stream.get_mut() , "No Server directory found for {}\n" , file.ip_address() ).unwrap();
                    write!( stream.get_mut() , "\0" ).unwrap();
                },
                Err(e) => {
                    eprintln!("{}",e.to_string());
                }
            }
        }
        else if file.category() == 4 {
            let result = search_file( file.title().to_string() , file.ip_address() );
            
            match result {
                Ok(files) if files.len() == 0 => { 
                    write!( stream.get_mut() , "No file of such name found on the server\n\0" ).unwrap(); 
                    continue; 
                },
                Ok(files) => {
                    for file in files {
                        write!( stream.get_mut() , "{}\n" , file ).unwrap();
                    }
                },
                Err(e) => { eprintln!("{}",e.to_string()); }
            }

            write!( stream.get_mut() , "\0" ).unwrap();
        }
        else if file.category() == 5 {
            let result = download(
                file.description().split("=").nth(1).unwrap().trim() , 
                file.title().trim() , 
                file.ip_address().split("-").nth(1).unwrap().trim()  ,
                file.ip_address().split("-").nth(0).unwrap().trim()
            );

            match result {
                Ok(file_content) => {
                    write!( stream.get_mut() , "{}\n" , file_content ).unwrap();
                    write!( stream.get_mut() , "\0" ).unwrap();
                },
                Err(e) if e.to_string() == "The system cannot find the path specified. (os error 3)" || e.to_string() == "The system cannot find the file specified. (os error 2)" => {
                    write!( stream.get_mut() , "{}\n" , e.to_string() ).unwrap();
                    write!( stream.get_mut() , "\0" ).unwrap();
                },
                Err(e) => {
                    eprintln!("{}",e.to_string());
                }
            }
        }
        else if file.category() == 6 {
            let result = delete( file.ip_address() , file.title().trim() );

            match result {
                Ok(_) => {
                    write!( stream.get_mut() , "File successfully deleted from your directory in server" ).unwrap();
                    write!( stream.get_mut() , "\0" ).unwrap();
                },
                Err(e) if e.to_string() == "The system cannot find the file specified. (os error 2)" || e.to_string() == "The system cannot find the path specified. (os error 3)" => {
                    write!( stream.get_mut() , "{}\0" , e.to_string() ).unwrap();
                },
                Err(e) => {
                    eprint!("{}",e.to_string());
                }
            }
        }
    }
}

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").expect("Could not bind server to socket 127.0.0.1:8080");

    for stream in server.incoming() {
        match stream {
            Ok(mut stream) => { 
                spawn(
                    move || {
                        let result = handle_client( stream.try_clone().unwrap() );

                        if let Err(e) = result { 
                            stream.write_all( 
                            format!("{}\n",e.to_string()).as_bytes()
                            ).unwrap(); 
                        }
                    }
                );
            } ,
            Err(e) => { println!("{}",e.to_string()); }
        }
    }

}