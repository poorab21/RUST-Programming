use std::collections::HashMap;
use crate::recipe::Recipe;
use std::fs::{ self , File };
use std::io::Write;

pub struct Manager {
    id_counter : u32 , 
    recipes : HashMap< u32 , Recipe >
}

impl Manager {
    pub fn new() -> Self {
        Self {
            id_counter : 1 ,
            recipes : HashMap::new()
        }
    }

    pub fn recipes( &self ) -> &HashMap< u32 , Recipe > {
        &self.recipes
    }

    pub fn push( &mut self , recipe : Recipe ) -> Result< String , Box< dyn std::error::Error > > {

        if recipe.name().trim().is_empty() || recipe.how_to_make_it().len() == 0 {
            return Err("Recipe name and Steps are mandatory to fill".into());
        }

        let mut file = File::create( format!("{}.txt",recipe.name()).as_str() )?;
        file.write( format!("Recipe Name: {} \n\nSteps\n{:#?}",recipe.name(),recipe.how_to_make_it()).as_bytes() )?;

        self.recipes.insert( self.id_counter , recipe );
        self.id_counter += 1;

        Ok(format!("Recipe successfully saved with ID {}",self.id_counter-1).into())
    }

    pub fn remove_recipe( &mut self , id : u32 ) -> Result< String , Box< dyn std::error::Error > > {

        if self.recipes.contains_key(&id) {
            fs::remove_file( format!("{}.txt",self.recipes.get(&id).unwrap().name()) )?;
            self.recipes.remove(&id);
        }
        else {
            return Err("recipe with specified ID does not exist".into());
        }

        Ok("Recipe successfully removed".into())
    }

    pub fn view( &self ) {
        println!("{:#?}",self.recipes);
    }

    pub fn search( &self , id : u32 ) -> Option<&Recipe> {
        self.recipes.get(&id)
    }
} 