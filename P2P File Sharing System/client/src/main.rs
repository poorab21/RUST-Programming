use std::{ net::TcpStream , env };
use p2p_lib::menu::menu;
use clearscreen::clear;

fn main() {
    let mut socket = TcpStream::connect("127.0.0.1:8080").expect("Could not connect to server at socket 127.0.0.1:8080");
    let ip_address = env::args().nth(1).unwrap();

    loop {
        let result = menu( &mut socket , &ip_address );
        
        match result {
            Err(e) => { clear().unwrap() ; println!("\n{}\n",e.to_string()); },
            Ok(msg) if msg.trim().contains("break") => { break; } ,
            Ok(msg) => { clear().unwrap() ; println!("\n{}\n",msg); }
        }
    }
    clear().unwrap();
}
