use std::net::TcpStream;
use std::io::{ self , Write };

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to port 8888 on loopback address");

    loop {
        let mut input = String::new();

        io::stdin().read_line( &mut input ).expect("Could not process input message");

        stream.write( input.as_bytes() ).expect("Could not write message to server");
        
    }
}
