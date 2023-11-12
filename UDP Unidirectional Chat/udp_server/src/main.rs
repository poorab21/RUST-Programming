use std::net::{ UdpSocket };
use std::str::{ from_utf8 };
fn main() {
    let server = UdpSocket::bind("127.0.0.1:8888").expect("Could not bind to 127.0.0.1:8888");

    loop {
        let mut buf = [ 0 ; 512 ];

        match server.recv_from( &mut buf ) {
            Ok(( _ , src )) => {
                println!("message from {} = {}",src,from_utf8(&buf).expect("Could not convert buffer to string"));
            } ,
            Err(e) => { println!("{}",e.to_string()); }
        }
    }
}
