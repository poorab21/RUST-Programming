use std::any::type_name;

fn type_of<T>( _ : T ) -> &'static str {
    &type_name::<T>()[1..]
}

struct User {
   name : String ,
   age : i32
}

enum IpAddr {
   V4 ,
   V6
}

fn main() {
    let i = 12;    
    let bool_val = true;
    let tuple = ( 12 , 55.3 , 'c' );

    let user1 = User {
	name : String::from("Poorab") ,
	age : 12
    };    

    let v1 = vec![ 1 , 2 , 3 , 4 ]; 
    let arr1 = [ 1.1 , 2.2 , 3.3 ];
    

    println!("{}",type_of(&i));
    println!("{}",type_of(&bool_val));
    println!("{}",type_of(&user1));    
    println!("{}",type_of(&IpAddr::V4));
    println!("{}",type_of(&tuple));
    println!("{}",type_of(&v1));
    println!("{}",type_of(arr1));
}