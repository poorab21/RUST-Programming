use crate::post::{ self , Post };
use regex::Regex;
use std::fs::{ self , File };
use std::io::Write;

pub struct Blog {
    posts : Vec< post::Post > ,
    name : String , 
    id_counter : u32
}

impl Blog {
    pub fn new( name : String ) -> Self {
        Self {
            posts : Vec::new() ,
            id_counter : 1 ,
            name 
        }
    }

    pub fn posts( &self ) -> &Vec< post::Post > {
        &self.posts
    }

    pub fn name( &self ) -> &str {
        &self.name
    }

    pub fn post( &mut self , title : String , content : String , username : String ) -> Result< String , Box< dyn std::error::Error > > {
        let username_regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();

        if title.trim().is_empty() || content.trim().is_empty() { 
            return Err("Post title and content must not be empty".into()) 
        }

        if let None = username_regex.find(username.trim()) {
            return Err("Username can only contain alphabetic characters".into());
        }

        let mut file = File::create( format!("{}_post_id_{}",self.name,self.id_counter).as_str() )?;

        file.write( format!("Post ID {}\n({} by {})\n{}",self.id_counter,title,username,content).as_bytes() )?;

        self.posts.push( 
            Post::new( 
                self.id_counter , 
                title.clone(),
                content.clone() ,
                username.clone()
            )
        );

        self.id_counter += 1;

        Ok("Post was successfully created".into())
    }

    pub fn remove_post( &mut self , id : u32 ) -> Result< String , Box< dyn std::error::Error > > {
        let result = self.find_by_id( id );

        if let None = result { return Err(format!("Post with ID {} does not exist",id).into()); }
        
        for ( index , post ) in self.posts.iter().enumerate() {
            if post.id() == id { 
                self.posts.remove(index); 
                fs::remove_file( format!("{}_post_id_{}",self.name,id).as_str() )?;
                break; 
            }
        }

        Ok("Post successfully removed".to_string())
    }

    pub fn edit_post( &mut self , id : u32 , title : String , content : String , username : String ) -> Result< String , Box< dyn std::error::Error > > {
        let result = self.find_by_id( id );
        let username_regex = Regex::new(r"^[a-zA-Z\s]+$").unwrap();

        if let None = result {
            return Err( format!("Post with ID {} does not exist",id).into() );
        }

        if title.trim().is_empty() || content.trim().is_empty() { 
            return Err("Post title and content must not be empty".into()) 
        }

        if let None = username_regex.find(username.trim()) {
            return Err("Username can only contain alphabetic characters".into());
        }

        let mut file = File::create( format!("{}_post_id_{}",self.name,id).as_str() );

        if let Err(_) = file {
            return Err("Problem in processing file containing post".into());
        }

        for ( index , post ) in self.posts.iter().enumerate() {
            if post.id() == id {
                file?.write( format!("Post ID {}\n({} by {})\n{}",id,title,username,content).as_bytes() )?;
                self.posts[ index ] = Post::new( id , title , content , username );
                break;
            }
        } 

        Ok("Post successfully edited".into())
    }

    fn find_by_id( &self , id : u32 ) -> Option<usize> {
        self.posts.iter().position( |post| post.id() == id )
    }
} 

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_post_creation() -> Result< () , Box< dyn std::error::Error > > {
        let mut blog = Blog::new("Poorab\'s Blog".to_string());

        if let Err(e) = blog.post( "title".to_string() , "content".to_string() , "username".to_string() ) {
            return Err(e);
        }
        
        Ok(())
    }
}