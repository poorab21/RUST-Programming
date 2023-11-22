use serde::{ Serialize , Deserialize };

#[derive(Serialize,Deserialize,Debug)]
pub enum Access {
    PUBLIC , 
    PRIVATE
}

#[derive(Serialize,Deserialize,Debug)]
pub struct UserFile {
    category : usize , 
    title : String , 
    description : String ,
    content : String , 
    ip_address : String ,
    visibility : Access
}

impl UserFile {
    pub fn new( title : String , description : String , content : String , ip_address : String , visibility : Access , category : usize ) -> Self {
        Self {
            category ,
            title ,
            description ,
            content ,
            ip_address ,
            visibility
        }
    }

    pub fn title( &self ) -> &str {
        &self.title
    }

    pub fn description( &self ) -> &str {
        &self.description
    }

    pub fn content( &self ) -> &str {
        &self.content
    }

    pub fn ip_address( &self ) -> &str {
        &self.ip_address
    }

    pub fn set_content( &mut self , content : String ) {
        self.content = content;
    }

    pub fn set_title( &mut self , title : String ) {
        self.title = title;
    }

    pub fn set_description( &mut self , description : String ) {
        self.description = description;
    }

    pub fn set_ip_address( &mut self , ip_address : String ) {
        self.ip_address = ip_address;
    }

    pub fn category( &self ) -> usize {
        self.category
    }

    pub fn set_category( &mut self , category : usize ) {
        self.category = category;
    }

    pub fn set_visibility( &mut self , visibility : Access ) {
        self.visibility = visibility;
    }

    pub fn visibility( &self ) -> &Access {
        &self.visibility
    }
}

