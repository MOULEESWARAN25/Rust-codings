pub fn collatz(n: u64) -> Option<u64> {
    // todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    let mut num: u64 = 0;
    let mut val = n;
    
    if val == 0{
        None
    }
    else{
        
        while val != 1{
            
            if val.is_multiple_of(2){
                
                val /= 2;
                
            }
            else{
                
                val *= 3;
                val += 1;
                
            }
            num += 1;
            
        }
        Some(num)
        
    }
}
