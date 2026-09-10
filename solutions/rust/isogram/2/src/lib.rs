pub fn check(candidate: &str) -> bool {
    // todo!("Is {candidate} an isogram?");
    if candidate.is_empty(){return true;}
    
    let txt = candidate.to_ascii_lowercase();
    
    let mut arr: [i8; 26] = [0; 26];
    
    for i in txt.chars(){
        
        if i.is_ascii_alphabetic(){
            
            arr[i as usize - 97] += 1;
            
        }
        
    }
    
    for i in arr{
        
        if i > 1{
            
            return false;
            
        }
  
    }
    
    true
    
}