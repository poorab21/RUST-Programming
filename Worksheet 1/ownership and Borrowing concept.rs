fn take_and_append( mut s : String ) -> String {
    s.push_str(" there");
    s
}

fn borrow_and_print( s : &str ) {
    println!("{s}");
}

fn main() {
    let mut s = String::from("hello");
    
    let mut s2 = take_and_append(s);
    borrow_and_print(&s2);
}