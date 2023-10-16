fn append_world( s : &mut String ) {
    s.push_str(" world");
} 

fn main() {
    let mut s = String::from("Hello");
    
    append_world( &mut s );
    
    println!("{}",s);
}