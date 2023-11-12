use std::net::{ UdpSocket };
use std::io;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:8081").expect("Could not bind to 127.0.0.1:8080");

    socket.connect("127.0.0.1:8888").expect("Could not connect to 127.0.0.1:8888");

    loop {
        let mut input = String::new();

        io::stdin().read_line( &mut input ).expect("Could not read user input");

        socket.send( input.as_bytes() ).expect("Could not write to server");
    }
}
