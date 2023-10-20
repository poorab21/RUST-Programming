use blog_engine_lib::blog::{ Blog };

#[test]
fn check_post_deletion() -> Result< () , Box<dyn std::error::Error > > {
    let mut blog = Blog::new( "Poorab\'s Blog".to_string() );

    if let Err(e) = blog.post( "title".to_string() , "content".to_string() , "username".to_string() ) {
        return Err(e);
    }

    if let Err(e) = blog.remove_post( 1 ) { return Err(e); }

    Ok(())
}