/// Check a Luhn checksum.
pub fn double(val: u32) -> u32{
    
    if val * 2 > 9{
        
        (val * 2) - 9
        
    }
    else{
        
        val * 2
        
    }
    
    
}

pub fn is_valid(code: &str) -> bool {
    // todo!("Is the Luhn checksum for {code} valid?");
        if code.trim().len() < 2{return false;}
    
    let res1: Vec<&str> = code.split(" ").collect();
    let mut res2 = Vec::<u32>::new();
    let mut num: u32 = 0;
    
    for x in res1{
        
        for y in x.chars(){
            
            if y.is_numeric(){
                res2.push(y.to_digit(10).unwrap());
            }
            else{
                
                return false;
                
            }
        }
    }
    
    let mut len = res2.len() as i32 - 2;
    
    while len >= 0{
        
        res2[len as usize] = double(res2[len as usize]);
        
        len -= 2;
        
    }
    
    for x in res2{
        
        num += x;
        
    }
    
    return num % 10 == 0;
}
