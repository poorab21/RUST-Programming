use std::collections::HashMap;

fn higher_scores_than( student_scores : &HashMap<String,f32> , threshold : f32 ) -> HashMap<String,f32> {
    let mut satisfying_scores : HashMap< String , f32 > = HashMap::new();
    
    for ( ref student , &score ) in student_scores {
        if score > threshold {
            satisfying_scores.insert( student.to_string() , score );
        }
    }
    satisfying_scores
}

fn main() {
    let student_scores = HashMap::from(
        [
            ( "Poorab".to_string() , 76f32 ) ,
            ( "Ajay".to_string() , 89f32 ) ,
            ( "Ronald".to_string() , 94f32 ) ,
            ( "Bruce".to_string() , 80.92f32 ) ,
            ( "Douglas".to_string() , 81.2 )
        ]
    );
    
    println!("{:#?}",higher_scores_than( &student_scores , 88f32 ));
}