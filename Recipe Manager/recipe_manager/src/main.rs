use recipe_manager_lib::{ recipe::Recipe , manager::Manager  };
use std::io;
use std::io::BufRead;
use clearscreen::clear;

fn create_recipe( manager : &mut Manager ) -> Result< String , Box< dyn std::error::Error > > {
    let mut name = String::new();
    let mut steps : Vec<String> = Vec::new();

    println!("Enter Recipe Name:");
    io::stdin().read_line( &mut name ).expect("Something went wrong while processing input for recipe name");
    name.pop();

    if name.trim().is_empty() { return Err("submitting a name for recipe is mandatory".into()); }

    let reader = io::stdin().lock();
    println!("Please provide the recipe steps");
    for line in reader.lines() {
        if let Err(e) = line  { return Err(e.into()); }
        let content = line.unwrap();

        if content.trim().is_empty() { break; }
        else { steps.push(content.trim().to_string()); }
    }

    if steps.is_empty() { return Err("Submitting the recipe steps is mandatory".into()); }

    manager.push( Recipe::new( name.trim().to_string() , steps ) )?;

    Ok("Recipe successfully stored".into())
}

fn delete_recipe( manager : &mut Manager ) -> Result< String , Box< dyn std::error::Error > > {
    let mut id = String::new();

    println!("Enter Recipe ID:");
    io::stdin().read_line( &mut id ).expect("Something went wrong while processing input for recipe ID");
    id.pop();

    if let Err(_) = id.trim().parse::<u32>() { return Err("Please enter recipe ID properly".into()); }
    
    manager.remove_recipe( id.trim().parse().unwrap() )?;
    Ok("Recipe successfully removed".into())
}

fn search_recipe( manager : &mut Manager ) -> Result< &Recipe , Box< dyn std::error::Error > > {
    let mut id = String::new();

    println!("Enter Recipe ID:");
    io::stdin().read_line( &mut id ).expect("Something went wrong while processing recipe ID");
    id.pop();

    if let Err(_) = id.trim().parse::<u32>() { return Err("Please enter recipe ID properly".into()); }

    let result = manager.search( id.trim().parse().unwrap() );
    if let None = result { return Err("Recipe with specified ID does not exist".into()); }
    Ok(result.unwrap())
}

fn main() {
    let mut choice = String::from("");
    let mut manager = Manager::new();

    loop {

        choice.clear();

        println!("1. Create Recipe");
        println!("2. Delete Recipe");
        println!("3. View Recipe");
        println!("4. Search Recipe");
        println!("5. Exit");
        println!("Choose:");
        io::stdin().read_line( &mut choice ).expect("Something went wrong while processing input for choice");
        choice.pop();

        clear();

        if let Err(_) = choice.trim().parse::<u32>() {
            println!("Please enter your choice properly");
            continue;
        }

        match choice.trim().parse().unwrap() {
            1 => {
                let result = create_recipe( &mut manager );

                clear();

                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            },
            2 => {
                let result = delete_recipe( &mut manager );

                clear();

                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(msg) = result {
                    println!("\n{}\n",msg);
                }
            },
            3 => {
                clear();
                println!("\n{:#?}\n",manager.recipes());
            },
            4 => {
                let result = search_recipe( &mut manager );

                clear();

                if let Err(e) = result {
                    println!("\n{}\n",e.to_string());
                }
                else if let Ok(recipe) = result {
                    println!("\n{:#?}\n",recipe);
                }
            },
            5 => { break; }
            _ => { println!("\nPlease select valid choice\n"); }
        };
    }
}
