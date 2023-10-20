use blog_engine_lib::{ 
    blog::{ Blog } 
};
use std::io;
use clearscreen::clear;
use std::io::BufRead;

fn create_post( blog : &mut Blog ) -> Result< String , Box< dyn std::error::Error > > {
    let mut username = String::new();
    let mut title = String::new();
    let mut content = String::new();

    println!("Enter Post Title:");
    io::stdin().read_line( &mut title ).expect("Something went wrong while processing input for post title");
    title.pop();

    println!("Enter Editor\'s Name:");
    io::stdin().read_line( &mut username ).expect("Something went wrong while processing input for post editor");
    username.pop();
    
    println!("Enter Post Content:");
    let mut reader = io::stdin().lock();;

    for line in reader.lines() {
        if let Ok(line) = line {  
            if line.is_empty() { break; }
            content += format!("{}\n",line.trim()).as_str();
        }
    }

    Ok(blog.post( title , content , username )?)
}

fn edit_post( blog : &mut Blog ) -> Result< String , Box< dyn std::error::Error > > {
    let mut username = String::new();
    let mut title = String::new();
    let mut content = String::new();
    let mut id = String::new();

    println!("Enter Post ID:");
    io::stdin().read_line( &mut id ).expect("Something went wrong while processing input for post ID");
    id.pop();

    println!("Enter Post Title:");
    io::stdin().read_line( &mut title ).expect("Something went wrong while processing input for post title");
    title.pop();

    println!("Enter Editor\'s Name:");
    io::stdin().read_line( &mut username ).expect("Something went wrong while processing input for post editor");
    username.pop();

    println!("Enter Post Content:");
    let mut reader = io::stdin().lock();;

    for line in reader.lines() {
        if let Ok(line) = line {  
            if line.is_empty() { break; }
            content += format!("{}\n",line.trim()).as_str();
        }
    }

    if let Err(_) = id.trim().parse::<u32>() {
        return Err("Please enter ID properly".into());
    }

    Ok( blog.edit_post( id.trim().parse().unwrap() , title , content , username )? )
}

fn delete_post( blog : &mut Blog ) -> Result< String , Box< dyn std::error::Error > > {
    let mut id = String::new();

    println!("Enter Post ID:");
    io::stdin().read_line( &mut id ).expect("Something went wrong while processing input for post ID");
    id.pop();

    if let Err(_) = id.trim().parse::<u32>() {
        return Err("Please enter ID properly".into());
    }

    Ok(blog.remove_post( id.trim().parse::<u32>().unwrap() )?)
}

fn main() {
    let mut choice = String::from("");
    let mut blog = Blog::new("Poorab\'s Blog".to_string());

    loop {
        choice.clear();

        println!("1. Create Post");
        println!("2. Edit Post");
        println!("3. Delete Post");
        println!("4. Exit");
        println!("Enter Choice:");
        io::stdin().read_line( &mut choice ).expect("Something went wrong while processing your input for choice");
        choice.pop();

        if let Err(_) = choice.trim().parse::<u32>() {
            println!("Please enter your choice properly");
            continue;
        }

        match choice.trim().parse::<u32>().unwrap() {
            1 => {  
                let result = create_post( &mut blog );
                
                clear();
                
                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            },
            2 => {
                let result = edit_post( &mut blog );
                
                clear();
                
                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            },
            3 => {
                let result = delete_post( &mut blog );
                
                clear();
                
                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            }
            4 => { break; }
            _ => { println!("Please enter valid choice"); } 
        }

    }
}
