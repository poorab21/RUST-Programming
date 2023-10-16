// sentence must only contain words and no punctuations 
fn already_exists( index : usize , words : &Vec<&str> ) -> bool {
    for i in 0..index {
        if words[i].to_ascii_lowercase() == words[index].to_ascii_lowercase() { return true; }
    }
    false
}

fn sort( words : &mut Vec<&str> ) {
    let mut temp = "";
    
    for i in 0..words.len() {
        for j in i+1..words.len() {
            if words[i].to_ascii_lowercase() > words[j].to_ascii_lowercase() {
                temp = words[i].clone();
                words[i] = words[j].clone();
                words[j] = temp;
            }
        }
    }
}

fn unique_and_alphabetic<'a>( sentence: &'a str ) -> Vec<&'a str> {
    let words = sentence.split_whitespace().collect::<Vec<&str>>();
    let mut vec : Vec<&str> = Vec::new();
    
    for i in 0..words.len() {
        if !already_exists( i , &words ) {
            vec.push(words[i]);
        }
    }
    
    
    
    sort( &mut vec );
    
    vec
    
}
fn main() {
    println!("{:?}",unique_and_alphabetic("rousey hosey"))
}