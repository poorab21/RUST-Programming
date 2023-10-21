use compresslib::{ create , read_file , write_to_file , compress };
use clearscreen::clear;
use std::io::BufRead;
use std::io;

fn create_file() -> Result< String , Box< dyn std::error::Error > > {
    let mut filename = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename ).expect("Something went wrong while processing filename");
    filename.pop();

    Ok( create(filename.trim())? )
}

fn read_File() -> Result< String , Box< dyn std::error::Error > > {
    let mut filename = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename ).expect("Something went wrong while processing filename");
    filename.pop();

    Ok( read_file(filename.trim())? )
}

fn write_to_File() -> Result< String , Box< dyn std::error::Error > > {
    let mut filename = String::new();
    let mut content = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename ).expect("Something went wrong while processing input for file");
    filename.pop();

    let reader = io::stdin().lock();
    println!("Enter Content:");
    for line in reader.lines() {
        let data = line?;
        if data.is_empty() { break; }
        content += format!("{}\n",data).as_str();
    }

    Ok( write_to_file( filename.trim() , content.trim() )? )
}

fn compress_file() -> Result< String , Box< dyn std::error::Error > > {
    let mut filename = String::new();

    println!("Enter Filename:");
    io::stdin().read_line( &mut filename ).expect("Something went wrong while processing input for filename");
    filename.pop();

    Ok( compress( filename.trim() )? )
    
}

fn main() {
    let mut choice = String::from("");

    loop {

        choice.clear();

        println!("1. Create File");
        println!("2. Read File");
        println!("3. Write to File");
        println!("4. Compress File");
        println!("5. Exit");
        println!("Select Choice:");
        io::stdin().read_line( &mut choice ).expect("Something went wrong while processing your choice selection");
        choice.pop();

        clear();

        if let Err(_) = choice.trim().parse::<u32>() {
            println!("Please enter your choice properly");
            continue;
        }

        match choice.trim().parse().unwrap() {
            1 => {
                let result = create_file();

                clear();

                if let Err(e) = result {
                    eprintln!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            },
            2 => {
                let result = read_File();

                clear();

                if let Err(e) = result {
                    eprintln!("\n{}\n",e.to_string());
                }
                else if let Ok(content) = result {
                    println!("\n{}\n",content);
                }
            },
            3 => {
                let result = write_to_File();

                clear();

                if let Err(e) = result {
                    eprintln!("\n{}\n",e.to_string());
                }
                else if let Ok(content) = result {
                    println!("\n{}\n",content);
                }
            },
            4 => {
                let result = compress_file();

                clear();

                if let Err(e) = result {
                    eprintln!("\n{}\n",e.to_string());
                }
                else if let Ok(content) = result {
                    println!("\n{}\n",content);
                }
            }
            5 => {
                break;
            },
            _ => { eprintln!("\nPlease select a valid choice\n"); }
        }
    }
}
