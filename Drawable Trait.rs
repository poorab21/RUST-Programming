pub trait Drawable {
    fn draw(&self);
    fn default_func(&self) {
        println!("default function called");
    }
}

impl Drawable for i32 {
    fn draw(&self) {
        println!("draw() function called for value of type i32");
    }
}

fn main() {
    let x : i32 = 12;
    x.draw();
    x.default_func();
}