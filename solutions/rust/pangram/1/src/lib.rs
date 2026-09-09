/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    // todo!("Is {sentence} a pangram?");
    let txt = sentence.to_ascii_uppercase();
    let mut count = 0;
    
    let mut arr: [i8; 26] = [0; 26];
    
    for x in txt.as_bytes(){
        
        if (*x as usize) > 64 && (*x as usize) < 92{
        
            arr[*x as usize - 65] += 1;
        }
    }
    
    for x in arr{
        
        if x >= 1{
            
            count += 1;
            
        }
        
    }
    
    if count >= 26{
        
        return true;
        
    }
    
    false
    
}
