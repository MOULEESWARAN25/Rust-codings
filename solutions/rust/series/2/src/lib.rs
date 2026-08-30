pub fn series(digits: &str, len: usize) -> Vec<String> {
    // todo!("What are the series of length {len} in string {digits:?}")
    let mut substr = Vec::<String>::new();
    let mut m = 0; let mut n = len - 1;
    
    if digits.is_empty() || len > digits.len(){
        
        substr
        
    }
    else if len == digits.len(){ 
    
        substr.push(digits.to_string());
        substr
        
    }
    else{
        
        while n < digits.len(){
            
            substr.push(digits[m..=n].to_string());
            m += 1;
            n += 1;
            
        }         
        substr
                
    }
}
