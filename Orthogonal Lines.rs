use std::io;

fn main(){
    let mut input = String::new();
    let mut line1_Coordinates : [ (i32,i32) ; 2 ] = [ (0,0) ; 2 ];
    let mut line2_Coordinates : [ (i32,i32) ; 2 ] = [ (0,0) ; 2 ];
    
    for i in 0..line1_Coordinates.len() {
        println!("Enter X coordinate for line 1 :");
        io::stdin().read_line(&mut input).expect("Something went wrong while processing input for X coordinate for line 1");
        input.pop();
        
        line1_Coordinates[i].0 = input.trim().parse().unwrap();
        input.clear();
        
        println!("Enter Y coordinate for line 1 :");
        io::stdin().read_line(&mut input).expect("Something went wrong while processing input for Y coordinate for line 1");
        input.pop();
        
        line1_Coordinates[i].1 = input.trim().parse().unwrap();
        input.clear();
    }
    
    for i in 0..line2_Coordinates.len() {
        println!("Enter X coordinate for line 2 :");
        io::stdin().read_line(&mut input).expect("Something went wrong while processing input for X coordinate for line 2");
        input.pop();
        
        line2_Coordinates[i].0 = input.trim().parse().unwrap();
        input.clear();
        
        println!("Enter Y coordinate for line 2 :");
        io::stdin().read_line(&mut input).expect("Something went wrong while processing input for Y coordinate for line 2");
        input.pop();
        
        line2_Coordinates[i].1 = input.trim().parse().unwrap();
        input.clear();
    }
    
    if line1_Coordinates[0].0 - line1_Coordinates[1].0 == 0 && getSlope(line2_Coordinates) == 0 as f32 {
        println!("Lines are Orthogonal");
    }
    else if line2_Coordinates[0].0 - line2_Coordinates[1].0 == 0 && getSlope(line1_Coordinates) == 0 as f32 {
        println!("Lines are Orthogonal");
    }
    else if getSlope(line1_Coordinates) * getSlope(line2_Coordinates) == -1 as f32  {
        println!("Lines are Orthogonal");
    }
    else {
        println!("Lines are not orthogonal");
    }
}

fn getSlope(line : [ (i32,i32) ; 2 ]) -> f32 {
    return ( line[0].1 as f32 - line[1].1 as f32 ) / (line[0].0 as f32 - line[1].0 as f32 );
}