use std::net::{ TcpListener , TcpStream };
use std::thread;
use std::io::{ Write , Read };

fn handle_client( mut stream : TcpStream ) -> Result< () , Box< dyn std::error::Error>> {
    let mut buf = [ 0 ; 512 ];
    println!("Incoming connection from {}",stream.peer_addr()?);

    loop {
        let data_read_len = stream.read( &mut buf )?;

        if data_read_len == 0 { return Ok(()); }

        stream.write( &buf[..data_read_len] )?;
    }
}

fn main() {
    let listener = TcpListener::bind( "127.0.0.1:8080" ).expect("Could not bind server to 127.0.0.1:8888");

    for stream in listener.incoming() {
        match stream {
            Err(e) => eprintln!("{}",e.to_string()) ,
            Ok(stream) => {
                thread::spawn(
                    move || {
                        handle_client( stream ).unwrap_or_else( |error| println!("{}",error.to_string()) )
                    }
                );
            }    
        }
    }
}
