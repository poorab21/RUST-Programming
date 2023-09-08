// all the elements in both vectors must be in ascending form.
fn main() {
    let mut vec1 = vec![3, 4, 8, 11, 13, 15, 20 , -1];
    let mut vec2 = vec![21];
    let mut j = 0;
    let mut nullpos = 0;
    for i in 0..vec1.len() {
        if vec1[i] != -1 { continue; }
        
        j = i;
        while j < vec1.len() && vec1[j] == -1 {
            j += 1;
        }
        
        if j == vec1.len() { break; }
        
        vec1[i] += vec1[j];
        vec1[j] = vec1[i] - vec1[j];
        vec1[i] -= vec1[j];
    }
    
    j = 0;
    nullpos = vec1.len() - vec2.len();
    
    for i in 0..vec1.len() {
        if j < vec2.len() && vec2[j] >= vec1[i] && vec1[i] != -1 { continue; }
        else if j >= vec2.len() { break; }
        
        if vec1[i] == -1 {
            vec1[i] = vec2[j];
            j += 1;
            continue;
        }
        
        for k in (i+1..nullpos+1).rev() {
            
            vec1[k] = vec1[k-1];
        }
        nullpos += 1;
        vec1[i] = vec2[j];
        j += 1;
    }
    
    println!("{:?}",vec1);
}