/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.is_empty(){
        
        return false;
        
    }
    
    let res: Vec<&str> = isbn.split('-').collect();
    
    let mut count = 0;
    let mut sum: u32 = 0;
    let byte = res[res.len() - 1].as_bytes()[(res[res.len() - 1]).len() - 1];
    
    for i in &res{
        
        count += i.len() as u32;
        
        
    }
    
    if count != 10 || !byte.is_ascii_digit(){

        if byte == b'X'{sum += 10;}
        else{return false;}
        
    }
    
    for i in res{
        
        for j in i.chars(){
            
            if j.is_ascii_digit(){
                
                sum += j.to_digit(10).unwrap() * count;
                count -= 1;
            }
            else if count == 1 && byte == b'X'{
                
                break;
                
            }
            else{
                
                return false;
                
            }
        }
    }
    
    if sum.is_multiple_of(11){
        
        return true;
        
    }
    
    false
    
}