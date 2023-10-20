pub struct Post {
    id : u32 ,
    title : String ,
    content : String ,
    username : String ,
}

impl Post {
    pub fn new( id : u32 , title : String , content : String , username : String ) -> Self {
        Self {
            id , 
            title ,
            content ,
            username
        }
    }

    pub fn title( &self ) -> &str {
        &self.title
    }

    pub fn content( &self ) -> &str {
        &self.content
    }

    pub fn username( &self ) -> &str {
        &self.username
    }

    pub fn set_username( &mut self , username : String ) {
        self.username = username;
    }

    pub fn set_content( &mut self , content : String ) {
        self.content = content;
    }

    pub fn set_title( &mut self , title : String ) {
        self.title = title;
    }

    pub fn id( &self ) -> u32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_post_title() {
        let post = Post::new( 1 , "My first Post".to_string() , "content".to_string() , "username".to_string() );

        assert!( post.title() == "My first Post" );
    }

    #[test]
    fn check_post_content() {
        let post = Post::new( 1 , "My first Post".to_string() , "My blog\'s Post".to_string() , "username".to_string() );
        assert_eq!( post.content() , "My blog\'s Post" );
    }

    #[test]
    fn check_post_username() {
        let post = Post::new( 1 , "My Post".to_string() , "My Blog Post".to_string() , "Poorab Gangwani".to_string() );
        assert_eq!( post.username() , "Poorab Gangwani" );
    }

    #[test]
    fn check_title_update() -> Result< () , Box< dyn std::error::Error > > {
        let mut post = Post::new( 1 , "My Post".to_string() , "My Blog Post".to_string() , "Username".to_string() );

        post.set_title("My newest Post".to_string());
        if post.title() == "My newest Post" { return Ok(()); }
        
        Err("Post title update is malfunctioning".into())
    }

    #[test]
    fn check_username_update() -> Result< () , Box< dyn std::error::Error > > {
        let mut post = Post::new( 1 , "My first Post".to_string() , "My First blog post".to_string() , "Poorab Gangwani".to_string() );

        post.set_username( "Poorab Singh".to_string() );

        if post.username() == "Poorab Singh" { return Ok(()); }

        Err("username update functionality is not working properly".into())
    }

    #[test]
    fn check_content_update() {
        let mut post = Post::new( 1 , "My first Post".to_string() , "My First blog post content".to_string() , "Poorab Gangwani".to_string() );
        post.set_content( "Today i will talk about RUST".to_string() );

        assert_eq!( post.content() , "Today i will talk about RUST" );
    }
}