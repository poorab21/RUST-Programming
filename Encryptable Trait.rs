pub trait Encryptable {
    fn encrypt( &self ) -> Self;
    fn decrypt( &self ) -> Self;
}

impl Encryptable for String {
    fn encrypt( &self ) -> Self {
        self.to_ascii_lowercase().bytes().map( |c| {
            if c + 15 > 122 { (((c + 15) % 123) + 97) as char }
            else { (c + 15) as char }
        } ).collect::<String>()
    }
    
    fn decrypt( &self ) -> Self {
        self.to_ascii_lowercase().bytes().map( |c| {
            if c - 15 >= 97 { (c - 15) as char } 
            else { (123 - (97 - ( c - 15 ))) as char }
        } ).collect::<String>()
    }
}

impl Encryptable for Vec<String> {
    fn encrypt( &self ) -> Self {
        self.iter().map( |word| {
            word.bytes().map( |c| {
                if c + 15 > 122 { (((c + 15) % 123) + 97) as char }
                else { (c + 15) as char }
            }).collect::<String>()
        }).collect()
    }
    
    fn decrypt( &self ) -> Self {
        self.iter().map( |word| {
            word.bytes().map( |c| {
                if c - 15 >= 97 { (c - 15) as char } 
                else { (123 - (97 - ( c - 15 ))) as char }
            }).collect::<String>()
        } ).collect()
    }
}

fn main() {
    let mut s = String::from("hello");
    
    let mut s2 : String = s.encrypt();
    println!("{}",s2);
    
    s2 = s2.decrypt();
 
    println!("{}",s2);
    
    let mut vec = vec![
        "hello".to_string() ,
        "there".to_string() ,
        "sir".to_string()
    ];
    
    let encrypted_vector = vec.encrypt();
    println!("{:?}",encrypted_vector);
    
    vec = encrypted_vector.decrypt();
    println!("{:?}",vec);
}