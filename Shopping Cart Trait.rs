pub trait Collection {
    fn add_item( &mut self , item : &'static str );
    fn remove_item( &mut self , index : usize ) -> Option<&str>;
}

struct Shopping_cart<'a> {
    items : Vec<&'a str> ,
}

impl<'a> Shopping_cart<'a> {
    fn new() -> Self {
        Self {
            items : vec![] ,
        }
    }
    
    fn items( &self ) -> &Vec<&'a str> {
        &self.items
    }
    
    fn total( &self ) -> usize {
        self.items.len()
    }
}

impl<'a> Collection for Shopping_cart<'a> {
    fn add_item( &mut self , item : &'static str  ) {
        self.items.push( item );
    }
    
    fn remove_item( &mut self , index : usize ) -> Option<&str> {
        if index < self.items.len() {
            Some( self.items.remove(index) )
        }
        else {
            None
        }
    }
    
    
}

fn main() {
    let mut shopping_cart1 = Shopping_cart::new();
    
    shopping_cart1.add_item("milk");
    println!("{:?}",shopping_cart1.items());
    println!("{}",shopping_cart1.total());
    
    shopping_cart1.add_item("banana");
    println!("{:?}",shopping_cart1.items());
    println!("{}",shopping_cart1.total());
    
    shopping_cart1.add_item("cereal");
    println!("{:?}",shopping_cart1.items());
    println!("{}",shopping_cart1.total());
    
    if let Some(val) = shopping_cart1.remove_item(1) {
        println!("{} removed from cart",val);
        println!("{:?}",shopping_cart1.items());
    }
    else {
        println!("index out of bounds");
    }
    
    if let Some(val) = shopping_cart1.remove_item(1) {
        println!("{} removed from cart",val);
        println!("{:?}",shopping_cart1.items());
    }
    else {
        println!("index out of bounds");
    }
    
    shopping_cart1.add_item("Parathas");
    println!("{:?}",shopping_cart1.items());
    println!("{}",shopping_cart1.total());
    
    if let Some(val) = shopping_cart1.remove_item(0) {
        println!("{} removed from cart",val);
        println!("{:?}",shopping_cart1.items());
    }
    else {
        println!("index out of bounds");
    }
    
    if let Some(val) = shopping_cart1.remove_item(0) {
        println!("{} removed from cart",val);
        println!("{:?}",shopping_cart1.items());
    }
    else {
        println!("index out of bounds");
    }
    
    if let Some(val) = shopping_cart1.remove_item(0) {
        println!("{} removed from cart",val);
        println!("{:?}",shopping_cart1.items());
    }
    else {
        println!("index out of bounds");
    }
    
    shopping_cart1.add_item("Ice Cream");
    println!("{:?}",shopping_cart1.items());
    println!("{}",shopping_cart1.total());
    
}