pub trait Validator {
    fn validate( &self ) -> bool;
}

impl Validator for String {
    fn validate( &self ) -> bool {
        self.len() > 3
    }
}

impl Validator for i32 {
    fn validate( &self ) -> bool {
        *self > 3
    }
}

impl<T> Validator for Vec<T> {
    fn validate( &self ) -> bool {
        self.len() > 3
    }
}

fn main() {
    let mut x = 2;
    println!("{}",x.validate());
    
    let mut s = String::from("He");
    println!("{}",s.validate());
    
    let mut vec = vec!['c','h','u','i','o'];
    println!("{}",vec.validate());
}