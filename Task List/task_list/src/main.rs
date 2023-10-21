extern crate tasklist;

use tasklist::{ list::List , task::Task , task::Status };
use std::io;

fn add_task( tasklist : &mut List , id : &mut i32 ) -> Result< String , Box<dyn std::error::Error> > {
    let mut title = String::new();
    let mut description = String::new();
    
    println!("Enter Task Title:");
    io::stdin().read_line( &mut title ).expect("Something went wrong while processing task title");
    title.pop();

    println!("Enter Task Description:");
    io::stdin().read_line( &mut description ).expect("Something went wrong while processing task title");
    description.pop();   
    
    if title.trim().is_empty() || description.trim().is_empty() { 
        return Err("Task Title and Description must be provided".into()); 
    }

    if tasklist.size() == 0 && *id == 0 { *id = 1;  }
    else { *id += 1; }

    tasklist.push( Task::new( id.clone() as u32 , title.trim().to_string() , description.trim().to_string() , Status::InProgress ) );
    Ok("Task was successfully added".into())
}

fn view_tasks( tasklist : &List ) -> &Vec<Task> {
    tasklist.tasks()
}

fn remove_task( tasklist : &mut List ) -> Result< String , Box<dyn std::error::Error> > {
    let mut id = String::new();

    println!("Enter ID:");
    io::stdin().read_line(&mut id).expect("Something went wrong while processing input for ID");
    id.pop();

    let id = id.trim().parse();

    if let Err(_) = id { return Err("Please enter valid ID".into()); }

    let removed_tasked = tasklist.remove_with_id( id.unwrap() )?;

    Ok( format!("{:#?} has been removed",removed_tasked) )
}

fn main() {
    let mut choice = String::from("");
    let mut id_counter = 0;
    let mut tasklist = List::new();

    loop {

        choice.clear();

        println!("1.Enter Task");
        println!("2.View Task");
        println!("3.Remove Task");
        println!("4.Exit");
        println!("\nEnter Choice:");
        io::stdin().read_line(&mut choice).expect("\nSomething went wrong while processing input for choice\n");

        let result = choice.trim().parse::<i32>();
        
        clearscreen::clear().expect("Failed to clear screen");
        
        if let Err(_) = choice.trim().parse::<i32>() {
            println!("\nError in processing input ! please try again\n");
            continue;
        }
        
        match result.unwrap() {
            1 => {
                if let Err(e) = add_task( &mut tasklist , &mut id_counter ) {
                    println!("\n{:#?}\n",e);
                }
                else {
                    println!("\nTask successfully added to list\n");
                }
            } ,
            2 => {
                let x = view_tasks( &tasklist );
                println!("\n{:#?}\n",x);
            },
            3 => {
                let x = remove_task( &mut tasklist );

                if let Ok(message) = x {
                    println!("\n{}\n",message);
                }
                else if let Err(e) = x {
                    println!("\n{}\n",e);
                }
            }
            4 => {
                break;
            }
            _ => { println!("\nPlease select valid choice\n") ; }
        }
    }
}
