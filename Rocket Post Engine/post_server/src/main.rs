#![feature(plugin)]
#![feature(decl_macro)]

extern crate rocket;
use rocket_codegen::routes;
use rocket::{ get , post };
use rusqlite::Connection;
use postlib::post::{ Post , Category };
use postlib::account::Account;
use rocket_contrib::json::Json;
use serde::{ Serialize , Deserialize };

#[derive(Debug,Serialize,Deserialize)]
struct RequestBody {
    email : String , 
    password : String
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the Post App"
}

#[post("/Post/Create", format = "application/json" , data="<post>")]
fn post_creation( post : Json<Post> ) -> Result< String , Box<dyn std::error::Error> > {
    let conn = Connection::open("Post.db").expect("Could not open connection to Post.db");
    
    let result = conn.execute(
        format!(
        "INSERT INTO POST ( TITLE , CONTENT , POST_TYPE , ACCESSIBILITY , USER_ID ) VALUES( \"{}\" , \"{}\" , \"{:?}\" , \"{:?}\" , {} )" ,
            post.title() ,
            post.content() ,
            post.post_type() ,
            post.accessibility() ,
            post.user_id()
        ).as_str() ,
            []
    );

    if let Err(e) = result { panic!("{}",e.to_string()); }

    Ok("Post Successfully created".into())
}

#[post("/find_credentials", format = "application/json" , data = "<account>" )]
fn do_credentials_exist( account : Json<Account> ) -> Result< String , Box< dyn std::error::Error > > {
    let conn = Connection::open("Post.db").expect("Could not open connection to Post.db");
    let query = format!("SELECT COUNT(*) FROM Account WHERE Email = ? OR Password = ?");

    let result = conn.prepare( query.as_str() );

    if let Err(e) = result { panic!("{}",e.to_string()); }

    let result = result.unwrap().query_row( &[ account.email() , account.password() ] , |row| row.get::<_,i64>(0));

    if let Err(e) = result { panic!("{}",e.to_string()) }
    
    if result.unwrap() > 0 { return Ok("true".into()) }
    
    Ok("false".into())
}

#[post("/account", data = "<account>")] 
fn account( account : Json<RequestBody> ) -> Result< String , &'static str > {
    let conn = Connection::open("Post.db").expect("Could not open connection to Post.db");
    let query = "SELECT * FROM Account WHERE Email = ? AND Password = ?";

    let mut result = conn.prepare(query).unwrap();
    let row = result.query_row( [ account.email.clone() , account.password.clone() ] , |row| {
        Ok(row.get::<_,u64>(0).unwrap())
    });
    
    if let Err(_) = row { return Err("Account with specified email and password does not exist"); }
    
    Ok( format!("{}",row.unwrap()) )
}

#[post( "/create_account", format = "application/json" , data = "<account>" )]
fn create_account( account : Json<Account> ) -> Result< String , Box< dyn std::error::Error > > {

    let conn = Connection::open("Post.db").expect("Could not open connection to Post.db");

    let _ = conn.execute(
        format!(
            "INSERT INTO Account (  
                Username , 
                Email , 
                Password
            )
            VALUES (
                \"{}\" ,
                \"{}\" ,
                \"{}\"
            )
            "
            ,
            account.username() ,
            account.email() ,
            account.password()
    ).as_str() , []
    ).unwrap();

    Ok("Account successfully created".into())
}

#[get("/type/<post_type>")]
fn type_of_posts( post_type : String ) -> Result< Json<Vec<( String , String , Category , String )>> , Box< dyn std::error::Error > > {
    let conn = Connection::open("Post.db").expect("Could not open connection to Post.db");
    let query = "SELECT POST.TITLE , POST.CONTENT , Account.Username FROM POST INNER JOIN Account ON POST.USER_ID = Account.ID WHERE POST.POST_TYPE = ? AND POST.ACCESSIBILITY = ?";
    let mut posts = Vec::new();
    let mut result = conn.prepare( query ).unwrap();
    let category = if post_type.contains("Medical") {
        Category::MEDICAL
    }
    else { Category::SCIENCE };

    let rows = result.query_map( [ &(post_type.to_ascii_uppercase()) , "PUBLIC" ] , |row| {
        Ok(( row.get::<_,String>(0).unwrap() , row.get::<_,String>(1).unwrap() , category , row.get::<_,String>(2).unwrap() ))
    }).unwrap();

    for row in rows {
        posts.push(row?);
    }
    
    Ok(Json(posts))
}

#[post("/user_posts/<id>")]
fn user_posts( id : u32 ) -> Result< Json<Vec<( usize , String , String , String , String )>> , Box< dyn std::error::Error > > {
    let conn = Connection::open("Post.db").unwrap();
    let query = "SELECT ID , TITLE , CONTENT , POST_TYPE , ACCESSIBILITY FROM POST WHERE USER_ID = ?";
    let mut result = conn.prepare( query ).unwrap();
    let mut posts = Vec::new();

    let rows = result.query_map( [ id ] , |row| {
        Ok(( row.get::<_,usize>(0).unwrap() , row.get::<_,String>(1).unwrap() , row.get::<_,String>(2).unwrap() , row.get::<_,String>(3).unwrap() , row.get::<_,String>(4).unwrap() ))
    }).unwrap();

    for row in rows {
        posts.push(row?);
    }

    Ok( Json(posts) )
}

#[post("/update_post/<uid>/<pid>",data = "<data>")] 
fn update_post( data : Json< Post > , uid : usize , pid : usize ) -> String {
    let conn = Connection::open("Post.db").unwrap();
    let query = format!(
        "UPDATE POST SET TITLE = \"{}\" , CONTENT = \"{}\" , POST_TYPE = \"{:?}\" , ACCESSIBILITY = \"{:?}\" WHERE ID = {} AND USER_ID = {}"
        ,
        data.title() ,
        data.content() ,
        data.post_type() ,
        data.accessibility() ,
        pid , 
        uid
    );

    let result = conn.execute( 
        &query , 
        [] ).unwrap();

    if result == 0 { "Fault with the Input IDs or the Post data".into() }
    else { "Post Successfully Updated".into() }
}

#[post("/delete_post/<uid>/<pid>")]
fn delete_post( uid : usize , pid : usize ) -> Result< String , String > {
    let conn = Connection::open("Post.db").unwrap();
    let query = "DELETE FROM POST WHERE ID = ? AND USER_ID = ?";

    let result = conn.execute( query , [ pid , uid ] ).unwrap();
    
    if result == 0 { return Err("Post ID or User ID is invalid".into()) }

    Ok("Post Successfully Deleted".into())
}

fn main() {
    // let conn = Connection::open("Post.db").unwrap();
    // let table_creation_result = conn.execute(
    //     "CREATE TABLE POST ( 
    //         ID INTEGER PRIMARY KEY NOT NULL , 
    //         TITLE TEXT NOT NULL , 
    //         CONTENT TEXT NOT NULL , 
    //         POST_TYPE TEXT NOT NULL , 
    //         ACCESSIBILITY TEXT NOT NULL ,
    //         USER_ID INTEGER NOT NULL
    //     )", []
    // );

    // if let Err(e) = table_creation_result { println!("{}",e.to_string()) ; return; }
    // else { println!("Post table successfully created"); }
    
    // let table_creation_result = conn.execute(
    //     "Create table Account (
    //         ID INTEGER PRIMARY KEY NOT NULL ,
    //         Username TEXT NOT NULL ,
    //         Email TEXT NOT NULL ,
    //         Password TEXT NOT NULL
    //     )" , []
    // );

    // if let Err(e) = table_creation_result {
    //     println!("{}",e.to_string());
    //     return;
    // }
    // else { println!("Account table successfully created"); }

    rocket::ignite().mount(
        "/", 
        routes![
            index ,
            post_creation ,
            do_credentials_exist ,
            create_account ,
            type_of_posts ,
            user_posts ,
            delete_post ,
            update_post ,
            account
        ]
    ).launch();
}
