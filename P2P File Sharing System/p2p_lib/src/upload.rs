use std::fs::File;
use std::net::TcpStream;
use std::io::{ self , Write, BufReader , BufRead, Read };
use std::str::from_utf8;
use crate::userfile::{UserFile, Access};

pub fn upload( source : &mut TcpStream , user_file : UserFile ) -> Result< String , Box< dyn std::error::Error > > {
    let json_data = serde_json::to_string( &user_file )?;
    let mut buf : Vec<u8> = Vec::new(); 
    
    source.write_all( json_data.as_bytes() ).expect("File data could not all be written to server");
    source.write_all( b"\n" ).expect("Could not send newline character to server");

    let mut reader = BufReader::new( source );
    reader.read_until( b'\n' , &mut buf )?;

    Ok( from_utf8(&buf)?.to_string() )
}

pub fn file_create_and_upload( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    
    let mut filename = String::new();
    let mut content = String::new();
    let mut description = String::new();
    let mut visibility = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename ).expect("Could not process the inputted filename");
    filename.pop();

    if filename.trim().len() == 0 { return Err("Filename must be provided".into()); }

    println!("Enter Description for file:");
    io::stdin().read_line( &mut  description).expect("Could not process the inputted description for file");
    description.pop();

    if description.trim().len() == 0 { return Err("File description must be provided".into()); }

    println!("Should file be Private (1) or Public (2):");
    io::stdin().read_line( &mut visibility ).expect("Could not process the inputted visibility for file");
    visibility.pop();

    let visibility = visibility.trim().parse::<u8>();

    if let Err(_) = visibility { 
        return Err("Please specify visibility for the file in proper format".into()); 
    }

    let visibility = match visibility.unwrap() { 
        1 => Access::PRIVATE,
        2 => Access::PUBLIC,
        _ => return Err("Please specify a valid visibility setting for the file".into()) 
    };

    println!("Enter File Content:");
    let reader = io::stdin().lock();

    for line in reader.lines() {
        if let Ok(line) = line {  
            if line.is_empty() { break; }
            content += format!("{}\n",line.trim()).as_str();
        }
    }

    let user_file = UserFile::new( 
        filename , 
        description, 
        content, 
        ip_address.to_string() ,
        visibility ,  
        1 );

    Ok( upload( source , user_file )?.into() )
}

pub fn local_file_upload( source : &mut TcpStream , ip_address : &str ) -> Result< String , Box< dyn std::error::Error > > {
    let mut filepath = String::new();
    let mut content = String::new();
    let mut description = String::new();
    let mut visibility = String::new();

    println!("Enter absolute path of file:");
    io::stdin().read_line( &mut filepath ).expect("Could not process input for absolute path of local file");
    filepath.pop();

    filepath = filepath.replace("\\" , r"//" );

    let file = File::open( filepath.trim().clone() );

    if let Ok(mut file) = file {
        file.read_to_string( &mut content ).expect(
            format!("could not read content from {} to variable",filepath).as_str()
        );
    }
    else if let Err(e) = file {
        return Err( e.to_string().into() );
    }

    println!("Enter Description for file:");
    io::stdin().read_line( &mut description ).expect("Could not process input for local file description");
    description.pop();

    if description.trim().len() == 0 { return Err("Description must be provided".into()) }

    println!("Should file be Private (1) or Public (2):");
    io::stdin().read_line( &mut visibility ).expect("Could not process the inputted visibility for file");
    visibility.pop();

    let visibility = visibility.trim().parse::<u8>();

    if let Err(_) = visibility { 
        return Err("Please specify visibility for the file in proper format".into()); 
    }

    let visibility = match visibility.unwrap() { 
        1 => Access::PRIVATE,
        2 => Access::PUBLIC,
        _ => return Err("Please specify a valid visibility setting for the file".into()) 
    };

    let data = UserFile::new(
        filepath.split(r"//").last().unwrap().to_string() , 
        description, 
        content, 
        ip_address.to_string() , 
        visibility, 
        2);

    Ok(
        upload( source , data )?
    )
}