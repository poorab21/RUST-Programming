use std::io::Error;
use std::io::ErrorKind;

#[derive(Debug)]
pub struct Guess {
    num : u32
}

impl Guess {
    pub fn new( num : u32 ) -> Result< Self , Error > {
        if num < 1 || num > 100 { return Err( Error::new( ErrorKind::Other , "Number must be in range 1-100 inclusive" ) ); }
        
        Ok(Self {
            num
        })
    }

    pub fn num( &self ) -> u32 {
        self.num
    }

    pub fn set_num( &mut self , num : u32 ) -> Result< String , Error > {
        if num < 1 || num > 100 { return Err(  Error::new( ErrorKind::Other ,"number must be in range 1-100 inclusive")) }
        self.num = num;
        Ok("Number successfully updated".to_string())
    }
}

