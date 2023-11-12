use std::{ thread , str };
use std::net::{ TcpStream , TcpListener };
use std::io::{ self , Write , BufRead , BufReader };
use csv::WriterBuilder;
use serde_derive::{ Serialize , Deserialize };

#[macro_use]
use serde_derive;

#[derive(Debug,Serialize,Deserialize)]
struct Point {
    x : i32 , 
    y : i32 , 
    z : i32
}

fn serialize_to_csv( point : &Point ) -> Result< String , Box<dyn std::error::Error> > {
    let mut serialized = Vec::new();
    {
        let mut writer = WriterBuilder::new().from_writer( &mut serialized );
        writer.serialize( point )?;
    }

    Ok( str::from_utf8(&serialized)?.into() )
}

fn deserialize_from_csv( serialized : &str )  -> Result< Point , std::io::Error >  {
    let serialized = serialized.to_string();

    let mut point = Point {
        x : 0 ,
        y : 0 , 
        z : 0
    };

    let mut index = 0;
    let mut temp = 0;
    let mut count = 1;
    while index < serialized.len() {
        let val = serialized.chars().nth(index).unwrap();

        if val >= '0' && val <= '9' {
            temp = ( temp * 10 ) + ((('0' as i32) - (val as i32)) as i32);
        }
        else if val == ',' && count == 1 {
            point.x = temp;
            count += 1;
            temp = 0;
        }
        else if val == ',' && count == 2 {
            point.y = temp;
            count += 1;
            temp = 0;
        }

        index += 1;
    }

    point.z = temp;
    
    Ok(point)
}

fn handle_client( stream : TcpStream ) -> Result< () , std::io::Error> {
    println!("Incoming connection from: {}", stream.peer_addr()?);
    let mut data = Vec::new();
    let mut stream = BufReader::new(stream);
    let mut count = 0;

    loop {
        data.clear();

        let bytes_read = stream.read_until(b'\n', &mut data)?;
 
        if bytes_read == 0 {
            return Ok(());

        }
        
        let input: Point = deserialize_from_csv( str::from_utf8(&data).expect("Could not convert buffer to string") )?;
        
        let value = input.x.pow(2) + input.y.pow(2) + input.z.pow(2);
        
        if count % 2 != 0 { 
            write!(stream.get_mut(), "{}", f64::from(value).sqrt())?;
            write!(stream.get_mut(), "{}", "\n")?;
        }
        count += 1;
    }
}

fn main() {
    let args : Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Please only specify client or server");
    }
    else if args[1] == "client" {
        let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to server at 127.0.0.1:8888");

        loop {
            let mut input = String::new();
            let mut buf : Vec<u8> = Vec::new();

            io::stdin().read_line( &mut input ).expect("Could not process input");
            input.pop();

            let coords : Vec<_> = input.trim().split(',').collect();
            let point = Point {
                x : coords[0].trim().parse().unwrap() ,
                y : coords[1].trim().parse().unwrap() ,
                z : coords[2].trim().parse().unwrap()
            };

            let serialized = serialize_to_csv( &point );
            
            if let Err(e) = serialized {
                println!("{}",e.to_string());
                continue;
            }

            stream.write( serialized.unwrap().trim().as_bytes() ).expect("Could not send data to server");
            stream.write( b"\n" ).expect("Could not sent newline to server");

            let mut reader = BufReader::new( &stream );
            reader.read_until( b'\n' , &mut buf ).expect("Could not read data into buffer");
            
            println!("Response from Server = {}", str::from_utf8(&buf).expect("Could not convert buffer to string") );
        }
    }
    else if args[1] == "server" {
        let socket = TcpListener::bind("127.0.0.1:8888").expect("Could not bind server to socket 127.0.0.1:8888");

        for stream in socket.incoming() {
            match stream {
                Err(e) => { println!("{}",e.to_string()); } ,
                Ok(stream) => { 
                    thread::spawn(
                        move || handle_client( stream )
                    );
                }
            }
        }
    }
}
