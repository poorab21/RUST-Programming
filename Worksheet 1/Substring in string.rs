fn find_substr( s : &str , substr : &str ) -> Vec<usize> {
    let mut i = 0;
    let mut indices : Vec<usize> = Vec::new();
    
    'outerloop: while i <= ( s.len() - substr.len() ) {
        if s.chars().nth(i).unwrap() != substr.chars().nth(0).unwrap() {
            i += 1;
            continue;
        }
        
        'innerloop: for j in 0..substr.len() {
            if substr.chars().nth(j).unwrap() != s.chars().nth( i + j ).unwrap() {
                i += 1;
                continue 'outerloop;
            }
        }
        indices.push(i);
        i += 1;
    }
    indices
}

fn main() {
    let s = String::from("rust train");
    let substr = String::from("rain");
    
    println!("{:?}",find_substr(&s,&substr));
}