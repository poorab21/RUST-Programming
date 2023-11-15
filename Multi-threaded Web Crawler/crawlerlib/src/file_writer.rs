use std::fs::{ OpenOptions , File };
use std::io::Write;

pub fn create( filename : &str ) -> Result< () , Box< dyn std::error::Error > > {
    let _ = File::create(filename)?;
    Ok(())
}

pub fn record( webpage_descriptions : ( String , String ) , depth : usize , filename : &str ) -> Result< () , Box< dyn std::error::Error > > {
    let mut file = OpenOptions::new().append(true).open(filename)?;
    let ( link , description ) = webpage_descriptions;
    
    file.write_all( format!("Depth = {} , Link = {} , Title = {}\n",depth,link,description).as_bytes() )?;
    
    Ok(())
}

pub fn clear( filename : &str ) -> Result< () , Box< dyn std::error::Error > > {
    let file = OpenOptions::new().write(true).open(filename)?;
    file.set_len(0)?;
    Ok(())
}