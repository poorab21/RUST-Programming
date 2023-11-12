use std::net::{ TcpStream , TcpListener };
use std::io::{ self , BufRead , BufReader , Write };
use std::str;

fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to Server at 127.0.0.1:8888");

    loop {
        let mut message = String::new();
        let mut buf : Vec<u8> = Vec::new();

        io::stdin().read_line( &mut message ).expect("Message could not be sent");

        socket.write( message.as_bytes() ).expect("Could not send message to server");

        let mut reader = BufReader::new( &socket );

        reader.read_until( b'\n' , &mut buf );
        println!("{}",str::from_utf8( &buf ).expect("Could not write buffer as string"));
    }
}
