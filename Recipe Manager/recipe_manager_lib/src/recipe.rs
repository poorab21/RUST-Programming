#[derive(Debug)]
pub struct Recipe {
    name : String ,
    how_to_make_it : Vec<String>
}

impl Recipe {
    pub fn new( name : String , how_to_make_it : Vec<String> ) -> Self {
        Self {
            name ,
            how_to_make_it
        }
    }

    pub fn name( &self ) -> &str {
        &self.name
    }

    pub fn how_to_make_it( &self ) -> &Vec<String> {
        &self.how_to_make_it
    }

    pub fn set_name( &mut self , name : String ) {
        self.name = name;
    }

    pub fn set_steps( &mut self , how_to_make_it : Vec<String> ) {
        self.how_to_make_it = how_to_make_it;
    }
}