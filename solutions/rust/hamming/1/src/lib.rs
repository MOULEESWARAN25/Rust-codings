/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len(){
        
        return None;
        
    }
    else if s1 == s2{
        
        return Some(0);
        
    }
    
    let mut count: usize = 0;
    
    for i in 0..s1.len(){
        
        if s1.as_bytes()[i] != s2.as_bytes()[i]{
            
            count += 1;
            
        }
        
    }
    
    Some(count)
    
}
