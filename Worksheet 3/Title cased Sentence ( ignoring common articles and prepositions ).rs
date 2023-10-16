fn main() {
    let mut s = String::from("Around the pond");
    let words_to_ignore = [
        "the" ,
        "a" ,
        "an" ,
        "about" ,
        "above" ,
        "across" ,
        "after" ,
        "against" ,
        "along" ,
        "among" ,
        "around" ,
        "as" ,
        "at" ,
        "before" ,
        "behind" ,
        "below" ,
        "beneath" ,
        "beside" ,
        "between" ,
        "beyond" ,
        "by" ,
        "concerning" ,
        "considering" ,
        "despite" ,
        "down" ,
        "during" ,
        "except" ,
        "for" ,
        "from" ,
        "in" ,
        "inside" ,
        "into" ,
        "like" ,
        "near" ,
        "of" ,
        "off" ,
        "on" ,
        "onto" ,
        "out" ,
        "outside" ,
        "over" ,
        "past" ,
        "regarding" ,
        "round" ,
        "since" ,
        "through" ,
        "throughout" ,
        "to" ,
        "toward" ,
        "under" , 
        "underneath" ,
        "until" ,
        "unto" ,
        "the" ,
        "up" ,
        "upon" ,
        "with" ,
        "within" ,
        "without"
    ];
    
    let title_cased : String = s.split_whitespace().map( |word| {
       if words_to_ignore.contains(&word.to_ascii_lowercase().as_str())  {
            format!("{} ",word)
       }
       else { 
           let c = word.chars().nth(0).unwrap();
           let mut new_word = word.to_string();
           new_word.replace_range(0..1, format!("{}", c.to_ascii_uppercase() ).as_str() );
           format!("{} ",new_word)
       }
    }).collect();
    
    println!("{}",title_cased);
}