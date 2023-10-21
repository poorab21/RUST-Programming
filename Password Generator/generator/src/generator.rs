use rand::Rng;

pub fn lowercase_character() -> char {
    (rand::thread_rng().gen_range(97..122) as u8) as char 
}

pub fn uppercase_character() -> char {
    (rand::thread_rng().gen_range(65..90) as u8) as char
}

pub fn digit() -> char {
    ( (rand::thread_rng().gen_range(48..57) as u8) as char )
}

pub fn special_characters() -> char {
    let characters = ['$','=','+','-','/','*','^','#','@','!','_','?'];
    
    characters[ rand::thread_rng().gen_range(0..12) ]
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn check_lowercase_generation() {
        let character = lowercase_character();

        assert!( character >= 'a' && character <= 'z' );
    }

    #[test]
    fn check_uppercase_generation() {
        let character = uppercase_character();

        assert!( character >= 'A' && character <= 'Z' );
    }

    #[test]
    fn check_digit_generation() {
        let digit = digit();

        assert!( digit >= '0' && digit <= '9' );
    }

    #[test]
    fn check_special_generation() {
        let special = special_characters();

        assert!( ['$','=','+','-','/','*','^','#','@','!','_','?'].contains(&special) );
    }
}
