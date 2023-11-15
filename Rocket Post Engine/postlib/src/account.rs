use serde::{ Serialize , Deserialize };
use serde_json::{ self , Value , json };
use reqwest::Client;

#[ derive( Deserialize , Serialize , Debug , Clone ) ]
pub struct Account {
    username : String , 
    email : String , 
    password : String
}

impl Account {
    pub fn new( username : String , email : String , password : String ) -> Self {
        Self {
            username ,
            email ,
            password
        }
    }

    pub fn username( &self ) -> &str {
        &self.username
    }

    pub fn email( &self ) -> &str {
        &self.email
    }

    pub fn password( &self ) -> &str {
        &self.password
    }

    pub fn set_password( &mut self , password : String ) {
        self.password = password;
    }

    pub fn set_email( &mut self , email : String ) {
        self.email = email;
    }

    pub fn set_username( &mut self , username : String ) {
        self.username = username;
    }
}

pub async fn does_account_exist( account : &Account ) -> Result< bool , Box< dyn std::error::Error > > {
    let client = Client::new();
    let json_account = serde_json::to_value( account )?;
    let result = client.post("http://localhost:8000/find_credentials").json( &json_account ).send().await?;
    
    if result.text().await?.contains( "false" ) { return Ok(false); }
    Ok(true)
}

pub async fn create_account( account : Account ) -> Result< String , Box< dyn std::error::Error > > {
    let result = does_account_exist( &account ).await?;
    if result == true { return Err("The submitted Email or Password already exists for another user".into()) }

    let client = Client::new();
    let json_account = serde_json::to_value( &account )?;
    client.post( "http://localhost:8000/create_account" ).json( &json_account ).send().await?;

    Ok("Account successfully created".into())
}

pub async fn posts( id : u32 ) -> Result< Value , Box< dyn std::error::Error > > {
    let client = Client::new();
    let json_data = serde_json::to_value(id)?;
    let response = client.post(
        format!("http://localhost:8000/user_posts/{}",id)
    ).json( &json_data ).send().await?.json::<serde_json::Value>().await?;

    Ok( response )
}

pub async fn login( email : String , password : String ) -> Result< String , Box< dyn std::error::Error > > {
    let client = Client::new();
    let json_data = json!( { "email" : email , "password" : password } );

    let result = client.post("http://localhost:8000/account").json( &json_data ).send().await?;
    Ok( result.text().await? )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn check_account_existence() -> Result< () , Box< dyn std::error::Error > > {
        let result = does_account_exist( 
            &Account::new( "ron".to_string() , "ron2@gmail.com".to_string() , "ron123".to_string() )
        ).await?;

        if result == true || result == false { return Ok(()); }

        Err( format!("{}",result).into() )
    }

    #[tokio::test]
    async fn check_personal_posts() -> Result< () , Box< dyn std::error::Error > > {
        let _ = posts( 6 ).await?;
        Ok(())
    }

    #[tokio::test]
    async fn check_login() -> Result< () , Box< dyn std::error::Error > > {
        let result = login( "ron@gmail.com".to_string() , "ron123".to_string() ).await;

        match result {
            Ok(_) => Ok(()) ,
            Err(e) if e.to_string() == "Account with specified email and password does not exist" => Ok(()) ,
            Err(e) => Err(e)
        }
    }
}
