use std::net::UdpSocket;
use std::io;
use std::str::from_utf8;

fn main() {
    let client_socket = UdpSocket::bind("127.0.0.1:8888").expect("Could not bind client to socket 127.0.0.1:8888");
    client_socket.connect("127.0.0.1:8888").expect("Could not connect to server socket at 127.0.0.1:8888");

    loop {
        let mut input = String::new();
        let mut buf = [ 0 ; 512 ];

        io::stdin().read_line( &mut input ).expect("could not process user input");

        client_socket.send(input.as_bytes()).expect("could not write to server");

        client_socket.recv_from( &mut buf ).expect("Could not write server content to buffer");

        println!("{}",from_utf8(&buf).expect("Could not convert buffer to string"));
    }
}
