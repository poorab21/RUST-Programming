use serde::{ Deserialize , Serialize };
use reqwest::Client;
use serde_json::{ self , Value };

#[derive(Copy,Clone,Deserialize,Debug,Serialize)]
pub enum Access {
    PRIVATE ,
    PUBLIC
}

#[derive(Copy,Clone,Deserialize,Debug,Serialize)]
pub enum Category {
    MEDICAL ,
    SCIENCE
}

#[derive(Deserialize,Debug,Serialize)]
pub struct Post {
    title : String , 
    content : String ,
    post_type : Category ,
    accessibility : Access ,
    user_id : u32
}

impl Post {
    pub fn new( title : String , content : String , post_type : Category , accessibility : Access , user_id : u32 ) -> Self {
        Self {
            title , 
            content ,
            post_type ,
            accessibility ,
            user_id
        }
    }

    pub fn set_title( &mut self , title : String ) {
        self.title = title;
    }

    pub fn set_content( &mut self , content : String ) {
        self.content = content;
    }

    pub fn set_post_type( &mut self , post_type : Category ) {
        self.post_type = post_type;
    }

    pub fn set_user_id( &mut self , user_id : u32 ) {
        self.user_id = user_id;
    }

    pub fn set_accessibility( &mut self , accessibility : Access ) {
        self.accessibility = accessibility;
    }

    pub fn accessibility( &self ) -> Access {
        self.accessibility
    }

    pub fn post_type( &self ) -> &Category {
        &self.post_type
    }

    pub fn title( &self ) -> &str {
        &self.title
    }

    pub fn content( &self ) -> &str {
        &self.content
    }

    pub fn user_id( &self ) -> u32 {
        self.user_id
    }
}

pub async fn post( new_post : Post ) -> Result< String , Box< dyn std::error::Error> > {
    let client = Client::new();

    if new_post.title().is_empty() || new_post.content().is_empty() {
        return Err("Post Title and Content must be properly filled".into());
    }
    
    let json_post = serde_json::to_value(&new_post)?;

    let response = client.
    post("http://localhost:8000/Post/Create").
    header("Content-Type","application/json").
    json( &json_post ).
    send().
    await?;

    Ok(response.text().await?)
}

pub async fn categorized_post( category : Category ) -> Result< Value , Box< dyn std::error::Error > > {
    let client = Client::new();
    let _ : Vec< (String , String , String , String) > = Vec::new();

    let post_category_query = if let Category::MEDICAL = category {
        format!("http://localhost:8000/type/Medical")
    }
    else {
        format!("http://localhost:8000/type/Science")
    };

    let result = client.get(
        post_category_query
    ).send().await?.json::<serde_json::Value>().await?;
    
    Ok( result )
}

pub async fn delete_post( post_id : usize , user_id : usize ) -> Result< String , Box< dyn std::error::Error > > {
    let client = Client::new();
    let result = client.post(
        format!("http://localhost:8000/delete_post/{}/{}",user_id,post_id)
    ).send().await?;

    Ok( result.text().await? )
}

pub async fn update_post( user_id : usize , post_id : usize , post : Post ) -> Result< String , Box< dyn std::error::Error > > {
    let client = Client::new();
    let json_post = serde_json::to_value( &post )?;
    let result = client.post(
        format!("http://localhost:8000/update_post/{}/{}",user_id,post_id)
    ).json( &json_post ).send().await?;

    Ok( result.text().await? )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_categorized_posts() -> Result< () , Box< dyn std::error::Error > > {
        let _ = categorized_post( Category::MEDICAL ).await?;
        Ok(())
    }
}