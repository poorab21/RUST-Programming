trait Area {
    fn area( &self ) -> f32; 
}

trait Perimeter {
    fn perimeter( &self ) -> f32;
}

struct Circle {
    radius : f32
}

impl Circle {
    fn new( radius : f32 ) -> Self {
        Self {
            radius
        }
    }

    fn radius( &self ) -> f32 {
        self.radius
    }

    fn set_radius( &mut self , radius : f32 ) {
        self.radius = radius;
    }
}

impl Area for Circle {
    fn area( &self ) -> f32 {
        self.radius * self.radius * 3.14_f32
    }
}

impl Perimeter for Circle {
    fn perimeter( &self ) -> f32 {
        self.radius * 2_f32 * 3.14_f32
    }
}

struct Rectangle {
    width : f32 ,
    height : f32
}

impl Rectangle {
    fn new( width : f32 , height : f32 ) -> Self {
        Self {
            width , 
            height
        }
    }

    fn width( &self ) -> f32 {
        self.width
    }

    fn height( &self ) -> f32 {
        self.height
    }

    fn set_width( &mut self , width : f32 ) {
        self.width = width;
    }

    fn set_height( &mut self , height : f32 ) {
        self.height = height;
    }
}

impl Perimeter for Rectangle {
    fn perimeter( &self ) -> f32 {
        ( 2_f32 * self.width ) + ( 2_f32 * self.height )
    } 
}

impl Area for Rectangle {
    fn area( &self ) -> f32 {
        self.width * self.height
    }
}

fn main() {
    let rec1 = Rectangle::new( 12.2 , 22.2 );
    println!("{}",rec1.area());
    println!("{}",rec1.perimeter());

    let circle1 = Circle::new( 3.33 );
    println!("{}",circle1.area());
    println!("{}",circle1.perimeter());
}