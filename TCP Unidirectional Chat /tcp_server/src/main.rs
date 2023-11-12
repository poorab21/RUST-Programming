use std::net::{ TcpListener , TcpStream };
use std::thread;
use std::io::{ Read , Write };
use std::str;

fn view_message( mut stream : TcpStream ) -> Result< () , Box<dyn std::error::Error> > {
    let mut buf = [ 0 ; 200 ];

    loop {
        let bytes_read = stream.read( &mut buf )?;
        if bytes_read == 0 { return Ok(()) }
        println!("Message from {} = {}",stream.peer_addr()?,str::from_utf8(&buf[..bytes_read])?);
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind server to 127.0.0.1:8888");

    for stream in listener.incoming() {
        match stream {
            Err(e) => { println!("{}",e.to_string()); } ,
            Ok(stream) => {
                thread::spawn(
                    move || {
                        view_message( stream );
                    }
                );
            }
        }
    }
}
