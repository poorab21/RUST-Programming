use serde::ser::{ Serialize , Serializer ,  SerializeStruct };

pub trait convertible {
    fn to_string(&self) -> String;
}

struct Person {
    name : String ,
    age : u32
}

impl Person {
    fn new( name : String , age :u32 ) -> Self {
        Self {
            name ,
            age
        }
    }

    fn set_name( &mut self , name : String ) {
        self.name = name;
    }

    fn set_age( &mut self , age : u32 ) {
        self.age = age;
    }

    fn name( &self ) -> &str {
        &self.name
    }

    fn age( &self ) -> u32 {
        self.age
    }
}

impl Serialize for Person {
    fn serialize<S>( &self , serializer : S ) -> Result< S::Ok , S::Error > where S: Serializer {
        let mut state = serializer.serialize_struct( "Person" , 2 )?;

        state.serialize_field( "name" , &self.name )?;
        state.serialize_field( "age" , &self.age )?;
        state.end()
    }
}

impl convertible for Person {
    fn to_string( &self ) -> String {
        serde_json::to_string(self).unwrap()
    }
}

fn main() {
    let mut person1 = Person {
        name : "franky".to_string() ,
        age : 12
    };


    println!("{:#?}",person1.to_string());

    person1.set_name("Johnny".to_string());
    person1.set_age(55);

    println!("{:#?}",person1.to_string());
}