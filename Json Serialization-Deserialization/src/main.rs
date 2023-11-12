#[macro_use]
extern crate serde_derive;
extern crate serde_json;
use std::io::{ self , BufReader , BufRead , Write };
use std::{ self , str , env , process };
use std::net::{ TcpStream , TcpListener };
use std::thread;

#[derive(Serialize,Deserialize,Debug)]
struct Point {
    x : i32 , 
    y : i32 ,
    z : i32
}

fn handle_client( mut stream : TcpStream ) -> Result< () , Box<dyn std::error::Error> > {
    let mut buf : Vec<u8> = Vec::new();
    let mut stream = BufReader::new( stream );

    loop {
        buf.clear();

        let byte_read = stream.read_until( b'\n' , &mut buf )?;

        if byte_read == 0 { return Ok(()) }

        let point : Point = serde_json::from_slice( &buf )?;

        let distance = format!("{}",point.x.pow(2) + point.y.pow(2) + point.z.pow(2));

        write!( stream.get_mut() , "{}" , distance.as_str() )?;
        write!( stream.get_mut() , "\n");
    }
}

fn main() {
    let args : Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify either server or client");
        process::exit(1);
    }
    else if args[1] == "server" {
        let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind server to 127.0.0.1:8888");

        for stream in listener.incoming() {
            match stream {
                Err(e) => { println!("{}",e.to_string()); }
                Ok(stream) => {
                    thread::spawn(
                        move || {
                            println!("Connection from {}",stream.peer_addr().expect("could not process client address"));
                            let result = handle_client( stream );

                            if let Err(e) = result {
                                println!("{}",e.to_string());
                            } 
                        }
                    );
                }
            }
        }
    }
    else if args[1] == "client" {
        let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server socket at 127.0.0.1:8888");

        loop {
            let mut coordinates = String::new();
            let mut buf : Vec<u8> = Vec::new(); 

            io::stdin().read_line( &mut coordinates ).expect("Could not process input for coordinates");
            
            let coords : Vec<_> = coordinates.trim().split(',').collect();
            
            let point = Point {
                x : coords[0].trim().parse().unwrap() ,
                y : coords[1].trim().parse().unwrap() ,
                z : coords[2].trim().parse().unwrap()
            };

            stream.write_all( serde_json::to_string(&point).unwrap().as_bytes() ).expect("Could not write stream to server");
            stream.write_all(b"\n").expect("Could not write newline to server");

            let mut reader = BufReader::new( &stream );

            reader.read_until( b'\n' , &mut buf );

            println!("Response from server = {}", str::from_utf8(&buf[..]).expect("Could not convert buffer to string") );
        }
    }
}
