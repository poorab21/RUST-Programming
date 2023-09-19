use std::collections::HashMap;

fn main() {
    let mut arr : [ [ char ; 5 ] ; 4 ] = [
        [ '.' , '.' , '.' , '.' , '.' ] ,
        [ '.' , '.' , '.' , '.' , '.' ] ,
        [ '.' , '.' , '.' , '.' , '.' ] ,
        [ '*' , '*' , '*' , '*' , '*' ]
    ];
    let mut count = 0;
    let mut h1 : HashMap<(usize,usize),i32> = HashMap::new();
    let digits = HashMap::from([
        (0,'0') ,
        (1,'1') ,
        (2,'2') ,
        (3,'3') ,
        (4,'4') ,
        (5,'5') ,
        (6,'6') ,
        (7,'7') ,
        (8,'8')
    ]);
    
    for row in 0..4 {
        for column in 0..5 {
            if arr[row][column] == '*' {
                continue;
            }
            
            if (row as i32) - 1 >= 0 && (column as i32) - 1 >= 0 && arr[row-1][column-1] == '*' {
                count += 1;
            }
            
            if (row as i32) - 1 >= 0 && (column as i32) >= 0 && arr[row-1][column] == '*' {
                count += 1;
            }
            
            if (row as i32) - 1 >= 0 && (column as i32) + 1 < 5 && arr[row-1][column+1] == '*' {
                count += 1;
            }
            
            if (row as i32) >= 0 && (column as i32) - 1 >= 0 && arr[row][column-1] == '*' {
                count += 1;
            }
            
            if (row as i32) >= 0 && (column as i32) + 1 < 5 && arr[row][column+1] == '*' {
                count += 1;
            }
            
            if (row as i32) + 1 < 4 && (column as i32) - 1 >= 0 && arr[row+1][column-1] == '*' {
                count += 1;
            }
            
            if (row as i32) + 1 < 4 && (column as i32) >= 0 && arr[row+1][column] == '*' {
                count += 1;
            }
            
            if (row as i32) + 1 < 4 && (column as i32) + 1 < 5 && arr[row+1][column+1] == '*' {
                count += 1;
            }
            
            if count > 0 {
                h1.insert((row,column),count);
                count = 0;
            }
        }
    }
    
    for ((row,column),count) in &h1 {
        arr[*row][*column] = digits[count];
    }
    
    for row in arr {
        for column in row {
            print!("{}",column);
        }
        println!("");
    }
}