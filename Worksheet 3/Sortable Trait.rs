use std::any::type_name;

trait Sortable {
    fn sort( &mut self );
}

fn type_of<T>( _ : T ) -> &'static str {
    &type_name::<T>()[..]
}

impl<T: std::cmp::PartialOrd + Clone + std::fmt::Display> Sortable for Vec<T> {
    fn sort( &mut self ) {
        println!("sort() from Sortable for type Vec");
        if type_of(self[0].clone()).contains("&str") || type_of(self[0].clone()).contains("String") {
            for i in 0..(self.len()) {
                for j in i+1..(self.len()) {
                    if format!("{}",self.get(i).unwrap()).len() > format!("{}",self.get(j).unwrap()).len() {
                        self.swap(i, j);
                    }
                }
            }
        }
        else {
            for i in 0..(self.len() - 1) {
                for j in 0..(self.len() - 1 - i) {
                    if self.get(j).unwrap() > self.get(j+1).unwrap() {
                        self.swap( j , j + 1 );
                    }
                }
            }
        }
    }
}

impl<T: Clone + std::fmt::Display + std::cmp::PartialOrd , const S : usize> Sortable for [ T ; S ] {
    fn sort( &mut self ) {
        println!("sort() from Sortable for type Array");
        if type_of(self[0].clone()).contains("&str") || type_of(self[0].clone()).contains("String") {
            for i in 0..(self.len()) {
                for j in i+1..(self.len()) {
                    if format!("{}",self.get(i).unwrap()).len() > format!("{}",self.get(j).unwrap()).len() {
                        self.swap(i, j);
                    }
                }
            }
        }
        else {
            for i in 0..self.len() {
                for j in i+1..self.len() {
                    if self.get(i).unwrap() > self.get(j).unwrap() {
                        self.swap(i,j);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut arr : [ String ; 3 ] = [ "raymond".to_string() , "lorenzo von". to_string() , "wrestledream".to_string() ];
    arr.sort();
    println!("{:?}",arr);
    
    let mut vec = vec![  "raymond".to_string() , "lorenzo von".to_string() , "wrestlemania".to_string() ];
    vec.sort();
    println!("{:?}",vec);
    
}