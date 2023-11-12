use std::net::UdpSocket;
use std::thread;

fn main() {
    let server_socket = UdpSocket::bind("127.0.0.1:8080").expect("Could not bind server socket to 127.0.0.1:8080");

    loop {
        let mut buf = [ 0 ; 1500 ];
        let sock = server_socket.try_clone().unwrap();

        match server_socket.recv_from( &mut buf ) {
            Err(e) => println!("{}",e.to_string()) ,
            Ok(( _ , src )) => {
                thread::spawn(
                    move || {
                        println!("Incoming connection from {}",src);
                        sock.send_to( &buf , &src ).expect("Could not send data to client");
                    }
                );
            }
        }
    }
}
