fn divide( a : f32 , b : f32 ) -> Result< f32 , Box< dyn std::error::Error > > {
    if b == 0.0 { return Err("Division by Zero error".into()); }
    Ok(a/b)
}

fn main() {
    let a = 15.1 as f32;
    let b = 0.0 as f32;
    
    let result = divide( a , b );
    
    if let Err(e) = result {
        println!("{}",e.to_string());
    }
    else if let Ok(val) = result {
        println!("{}",val);
    }
}