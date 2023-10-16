use std::fs::File;
use csv::Reader;

#[derive(Debug)]
struct Person {
    name : String , 
    age : u32
}

impl Person {
    fn new( name : String , age : u32 ) -> Self {
        Self {
            name , 
            age
        }
    }

    fn name( &self ) -> &str {
        &self.name
    }

    fn age( &self ) -> u32 {
        self.age
    }

    fn set_name( &mut self , name : String ) {
        self.name = name;
    }

    fn set_age( &mut self , age : u32 ) {
        self.age = age;
    }
}

fn read_csv_file( filename : &str ) -> Result< Vec<Person> , Box< dyn std::error::Error > > {
    let file = File::open(filename);
    let mut reader = Reader::from_reader( file? );
    let mut name = String::new();
    let mut age = 0;
    let mut people : Vec< Person > = Vec::new();

    for result in reader.records() {
        let record = result?;

        name = record[0].to_string();
        age = record[1].trim().parse()?;

        people.push( Person::new( name , age ) );
    }
    Ok(people)
}

fn main() {
    let result = read_csv_file("C://Users//SAIF UR REHMAN//Desktop//RUST Training//week5//src//Book.csv");

    if let Err(e) = result {
        println!("{:?}",e.to_string());
    }
    else if let Ok(data) = result {
        println!("{:#?}",data);
    }
}