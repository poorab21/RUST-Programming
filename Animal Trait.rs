pub trait Animal {
    fn make_sound(&self) -> &'static str ;
}

struct Cat<'a> {
    color : &'a str 
}

struct Dog<'a> {
    color : &'a str
}

impl<'a> Cat<'a> {
    fn new( color : &'a str ) -> Self {
        Self {
            color
        }
    }
    
    fn set_color( &mut self , color : &'a str ) {
        self.color = color;
    }
    
    fn color(&self) -> &str {
        self.color
    }
}

impl<'a> Dog<'a> {
    fn new( color : &'a str ) -> Self {
        Self {
            color
        }
    }
    
    fn set_color( &mut self , color : &'a str ) {
        self.color = color;
    }
    
    fn color( &self ) -> &str {
        self.color
    }
}

impl<'a> Animal for Cat<'a> {
    fn make_sound( &self ) -> &'static str {
        "meow"
    }
}

impl<'a> Animal for Dog<'a> {
    fn make_sound( &self ) -> &'static str {
        "bark"
    }
}

fn main() {
    let mut cat1 = Cat::new("black");
    
    cat1.set_color("white");
    println!("{}",cat1.color);
    
    let mut dog1 = Dog::new("maroon");
    dog1.set_color("black");
    
    println!("{}",dog1.color());
    
    println!("{}",cat1.make_sound());
    println!("{}",dog1.make_sound());
}