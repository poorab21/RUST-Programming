enum Color {
    Red ,
    Blue ,
    Green
}

fn get_color( color : &Color ) -> ( u8 , u8 , u8 ) {
    match color {
        Color::Red => ( 255 , 0 , 0 ) ,
        Color::Blue => ( 0 , 0 , 255 ) ,
        Color::Green => ( 0 , 255 , 0 )
    }
}

fn main() {
    let mut color = Color::Red;
    
    println!("{:?}",get_color(&color));
    
    let color2 = Color::Blue;
    
    println!("{:?}",get_color(&color2));

    color = Color::Green;
    println!("{:?}",get_color(&color));
}