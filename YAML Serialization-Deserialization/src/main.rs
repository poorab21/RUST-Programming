use std::io::{ self , Write , Read , BufReader , BufRead };
use std::net::{ TcpStream , TcpListener };
extern crate serde_yaml;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::{ env , str };
use std::thread;

#[derive(Debug,Serialize,Deserialize)]
struct Point {
    x : i32 ,
    y : i32 ,
    z : i32
}

fn handle_client( mut stream : TcpStream ) -> Result< () , Box<dyn std::error::Error> > {
    let mut buf = BufReader::new( stream );
    let mut data = Vec::new();
    let mut sum = 0.0;
    let mut bytes_read = 2;

    loop {
        data.clear();

        bytes_read = buf.read_until( b'\n' , &mut data )?;
        
        if bytes_read == 0 { break; }

        if data.len() > 1 {

            let mut number = str::from_utf8( &data )?.to_string().split(':').nth(1).unwrap().to_string();
            number.pop();
            sum += (number.trim().parse::<f32>().unwrap() * number.trim().parse::<f32>().unwrap());

        }
        else {
            write!( buf.get_mut() , "{}\n" , sum.sqrt() )?;
            sum = 0.0;
        }
        
    }
    
    Ok(())
}

fn main() {
    let args : Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Please specify client or server");
        std::process::exit(1);
    }
    else if args[1] == "client" {
        let mut stream = TcpStream::connect("127.0.0.1:8888").expect("could not connect to server at 127.0.0.1:8888");

        loop {
            let mut coords = String::new();
            let mut buf : Vec<u8> = Vec::new();

            io::stdin().read_line( &mut coords ).expect("Could not read input for coords");

            let coordinates : Vec<_> = coords.trim().split(',').collect();

            let point = Point {
                x : coordinates[0].trim().parse().unwrap() ,
                y : coordinates[1].trim().parse().unwrap() ,
                z : coordinates[2].trim().parse().unwrap()
            };
            
            stream.write_all( serde_yaml::to_string(&point).unwrap().as_bytes() ).expect("Could not send data to server");
            stream.write_all( b"\n" ).expect("Could not send newline to server");
            
            let mut reader = BufReader::new( &stream );

            reader.read_until( b'\n' , &mut buf ).expect("Could not read server message into buffer");

            println!("Response from Server = {}" , str::from_utf8(&buf).unwrap() );
        }
    }
    else if args[1] == "server" {
        let listener = TcpListener::bind("127.0.0.1:8888").expect("Could not bind server to socket 127.0.0.1:8888");

        for stream in listener.incoming() {
            match stream {
                Err(e) => { println!("{}",e.to_string()); } ,
                Ok(stream) => {
                    thread::spawn(
                        move || { 
                            let result = handle_client( stream );

                            if let Err(e) = result {
                                println!("{}",e.to_string());
                            }
                            else {
                                println!("response successfully send");
                            }
                        }
                    );
                }
            }
        }

    }
}
