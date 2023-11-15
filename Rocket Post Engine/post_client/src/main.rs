use std::io;
use postlib::{ post , account };
use account::{ Account , create_account , login , posts };
use post::{ Access , Category , Post , categorized_post , delete_post , update_post };
use tokio;
use regex::Regex;
use clearscreen::clear;

async fn new_account() -> Result< String , Box< dyn std::error::Error > > {
    let mut username = String::new();
    let mut email = String::new();
    let mut password = String::new();
    let email_addr_format = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();

    println!("Enter User Name:");
    io::stdin().read_line( &mut username ).expect("Could not process input for username in account creation");
    username.pop();

    if username.trim().is_empty() { return Err("username must be specified".into()); }

    println!("Enter Email:");
    io::stdin().read_line( &mut email ).expect("Could not process input for email in account creation");
    email.pop();

    if email_addr_format.find( email.trim() ).is_none()  { return Err("Email must be in proper email format".into()); }

    println!("Enter Password:");
    io::stdin().read_line( &mut password ).expect("Could not process input for password in account creation");
    password.pop();

    if password.trim().is_empty() { return Err("Password must be specified".into()); }

    let account = Account::new( username , email , password );
    let result = create_account( account ).await?;

    Ok(result)
}

async fn user_login() -> Result< String , Box< dyn std::error::Error > > {
    let mut email = String::new();
    let mut password = String::new();
    let email_addr_format = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+(?:\.[a-zA-Z0-9-]+)*$").unwrap();

    println!("Enter Email:");
    io::stdin().read_line( &mut email ).expect("Could not process input for email in user login");
    email.pop();

    if email_addr_format.find( email.trim() ).is_none() { return Err("Email must be in proper email format".into()); }

    println!("Enter Password:");
    io::stdin().read_line( &mut password ).expect("Could not process input for password in user login");
    password.pop();

    if password.trim().is_empty() { return Err("Password must be specified".into()); }
    let result = login( email , password ).await?;
    if let Err(_) = result.trim().parse::<u32>() { return Err(result.into()); }
    Ok(result)
}

async fn create_post( user_id : u32 ) -> Result< String , Box< dyn std::error::Error > > {
    let mut post_title = String::new();
    let mut post_content = String::new();
    let mut post_type = String::new();
    let mut post_accessibility = String::new();
    let mut user_post = Post::new( "".into() , "".into() , Category::MEDICAL , Access::PUBLIC , user_id );

    println!("Enter Post Title:");
    io::stdin().read_line( &mut post_title ).expect("Could not process post title during post creation");
    post_title.pop();

    if post_title.trim().is_empty() { return Err("Post title must be specified in post creation".into()); }
    else { user_post.set_title(post_title); }

    println!("Enter Post Content:");
    io::stdin().read_line( &mut post_content ).expect("Could not process post content during post creation");
    post_content.pop();    

    if post_content.trim().is_empty() { return Err("Post content must be specified in post creation".into()); }
    else { user_post.set_content(post_content); }

    println!("Enter Post Type ( 1 for Medical , 2 for Scientific ):");
    io::stdin().read_line( &mut post_type ).expect("Could not process post type during post creation");
    post_type.pop();

    let post_type = post_type.trim().parse::<u8>();
    if let Err(_) = post_type { return Err("Specify post type in valid format during post creation".into()); }
    if post_type.clone().unwrap() != 1 && post_type.clone().unwrap() != 2 {
        return Err("Specify valid post type during post creation".into());
    }
    else if post_type.clone().unwrap() == 2 { user_post.set_post_type( Category::SCIENCE ); }

    println!("Enter Post Access Control ( 1 for Private , 2 for Public ):");
    io::stdin().read_line( &mut post_accessibility ).expect("Could not process post accessibility during post creation");
    post_accessibility.pop();

    let post_accessibility = post_accessibility.trim().parse::<u8>();
    if let Err(_) = post_accessibility { return Err("Specify post access control setting in valid format during post creation".into()); }
    if post_accessibility.clone().unwrap() != 1 && post_accessibility.clone().unwrap() != 2 {
        return Err("Specify valid post access control setting during post creation".into());
    }
    else if post_accessibility.clone().unwrap() == 1 { user_post.set_accessibility( Access::PRIVATE ); }

    let result = post::post( user_post ).await?;
    Ok(result)
}

async fn all_public_posts() -> Result< () , Box< dyn std::error::Error > > {
    let mut category = String::new();

    println!("Enter Desired Category ( 1 for Medical , 2 for Scientific ):");
    io::stdin().read_line( &mut category ).expect("Could not process input category");
    category.pop();

    let category = category.trim().parse::<u32>();
    if let Err(_) = category { return Err("Please enter category in required format".into()); }
    if category.clone().unwrap() != 1 && category.clone().unwrap() != 2 {
        return Err("Please specify a valid category".into());
    }

    let result = if category.clone().unwrap() == 1 {
        categorized_post( Category::MEDICAL ).await?
    }
    else {
        categorized_post( Category::SCIENCE ).await?
    };

    println!("{:#?}",result);
    Ok(())
}

async fn all_user_posts( user_id : u32 ) -> Result< () , Box< dyn std::error::Error > > {
    let result = posts( user_id ).await?;
    println!("{:#?}",result);
    Ok(())
}

async fn post_deletion( user_id : u32 ) -> Result< String , Box< dyn std::error::Error > > {
    let mut post_id = String::new();

    println!("Enter Post ID:");
    io::stdin().read_line( &mut post_id ).expect("Could not process input for post id in post deletion");
    post_id.pop();

    if let Err(_) = post_id.trim().parse::<usize>() { return Err("Please specify Post ID in proper format".into()); }
    let result = delete_post( post_id.trim().parse().unwrap() , user_id as usize ).await?;

    Ok(result)
}

async fn post_update( user_id : u32 ) -> Result< String , Box< dyn std::error::Error > > {
    let mut post_id = String::new();
    let mut input = String::new();

    println!("Enter Post ID:");
    io::stdin().read_line( &mut post_id ).expect("Could not process post ID in post update");
    post_id.pop();

    if post_id.trim().is_empty() { return Err("Please specify ID for the post to be updated".into()) }
    if let Err(_) = post_id.trim().parse::<u32>() { return Err("Please specify Post ID in proper format in post update".into()); }
    let mut new_post = Post::new( "".to_string() , "".to_string() , Category::MEDICAL , Access::PUBLIC , post_id.trim().parse().unwrap() );

    println!("Enter Post Title:");
    io::stdin().read_line( &mut input ).expect("Could not process new post title in  post update");
    input.pop();

    if input.trim().is_empty() { return Err("Please specify new Post title".into()); }
    new_post.set_title( input.clone() );
    input.clear();


    println!("Enter Post Content:");
    io::stdin().read_line( &mut input ).expect("Could not process new post content in  post update");
    input.pop();

    if input.trim().is_empty() { return Err("Please specify new Post Content".into()); }
    new_post.set_content( input.clone() );
    input.clear();


    println!("Enter Post Type ( 1 for Medical , 2 for Scientific ):");
    io::stdin().read_line( &mut input ).expect("Could not process new post type in  post update");
    input.pop();

    let category_input = input.trim().parse::<u32>();
    if input.trim().is_empty() { return Err("Please specify new Post type".into()); }
    if let Err(_) = category_input { return Err("Please make sure the new post type is specified in proper format".into()); }
    if category_input.clone().unwrap() != 1 && category_input.clone().unwrap() != 2 {
        return Err("Please select a valid new category for post type".into());
    }
    
    if category_input.clone().unwrap() == 1 { new_post.set_post_type( Category::MEDICAL ); }
    else { new_post.set_post_type( Category::SCIENCE ); }
    input.clear();

    println!("Enter Post Access Control ( 1 for Public , 2 for Private ):");
    io::stdin().read_line( &mut input ).expect("Could not process input for new access control setting in post update");
    input.pop();

    let accessibility_input = input.trim().parse::<u32>();
    if input.trim().is_empty() { return Err("Please specify new post access control setting".into()); }
    if let Err(_) = accessibility_input { return Err("Please make sure the new post access control setting is specified in proper format".into()); }
    if accessibility_input.clone().unwrap() != 1 && accessibility_input.clone().unwrap() != 2 {
        return Err("Please select a valid new access control setting for post".into());
    }

    if accessibility_input.unwrap() == 1 { new_post.set_accessibility( Access::PUBLIC ); }
    else { new_post.set_accessibility( Access::PRIVATE ); }
    
    let result = update_post( user_id as usize , post_id.trim().parse().unwrap() , new_post ).await?;
    Ok(result)
}

#[tokio::main]
async fn main() {
    let mut choice = String::new();
    let mut current_user : u32 = 0;

    loop {
        if current_user == 0 {

            choice.clear();
            println!("1. Create Account");
            println!("2. Log In");
            println!("3. Exit");
            println!("Enter Choice:");
            io::stdin().read_line( &mut choice ).expect("Could not process your input for choice");
            choice.pop();

            clear().unwrap();
            if let Err(_) = choice.trim().parse::<u64>() { println!("\nPlease enter choice in valid format\n"); }
            else {

                match choice.trim().parse().unwrap() {
                    1 => {
                        let result = new_account().await;
                        clear().unwrap();
                        
                        if let Err(e) = result { println!("\n{}\n",e.to_string()) ; continue ; }
                        else if let Ok(msg) = result { println!("\n{}\n",msg) ; continue ; }
                    },
                    2 => {
                        let result = user_login().await;
                        clear().unwrap();
                        
                        if let Err(e) = result { println!("\n{}\n",e.to_string()) ; continue ; }
                        else if let Ok(id) = result { current_user = id.trim().parse().unwrap() ; continue ; } 
                    },
                    3 => {
                        break;
                    },
                    _ => {
                        println!("\nPlease select from the specified list of choices\n");
                        continue;
                    }
                };
            }
        }
        else {
            choice.clear();
    
            println!("1. Create Post");
            println!("2. Your Posts");
            println!("3. Delete Post");
            println!("4. Update Post");
            println!("5. View all Public Posts");
            println!("6. Exit");
            println!("Enter Choice:");
            io::stdin().read_line( &mut choice ).expect("Could not process input for your choice");
            choice.pop();
    
            clear().unwrap();
            if let Err(_) = choice.trim().parse::<u64>() { println!("\nPlease enter your choice in a valid format\n"); }
    
            else {
                match choice.trim().parse().unwrap() {
                    1 => {
                        let result = create_post( current_user ).await;
                        clear().unwrap();
                        
                        if let Err(e) = result { println!("\n{}\n",e.to_string()); }
                        else if let Ok(msg) = result { println!("\n{}\n",msg); }
                    },
                    2 => {
                        let result = all_user_posts( current_user ).await;

                        if let Err(e) = result { println!("\n{}\n",e.to_string()) }
                    },
                    3 => {
                        let result = post_deletion( current_user ).await;
                        clear().unwrap();

                        if let Err(e) = result { println!("\n{}\n",e.to_string()); }
                        else if let Ok(msg) = result { println!("\n{}\n",msg); }
                    },
                    4 => {
                        let result = post_update( current_user ).await;
                        clear().unwrap();

                        if let Err(e) = result { println!("\n{}\n",e.to_string()); }
                        else if let Ok(msg) = result { println!("\n{}\n",msg); }
                    },
                    5 => {
                        let result = all_public_posts().await;

                        if let Err(msg) = result { println!("\n{}\n",msg.to_string()); }
                    },
                    6 => {
                        current_user = 0;
                    },
                    _ => {
                        println!("\nPlease select from the specified list of choices\n");
                    }
                };
            }
        }
    }
}
