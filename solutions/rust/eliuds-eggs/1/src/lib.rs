pub fn egg_count(display_value: u32) -> usize {
    // todo!("count the eggs in {display_value}")
    let mut ones: usize = 0;
    let mut num = display_value;
    
    while num > 0{
        
        if num % 2 == 1{
            
            ones += 1;
            
        }
        
        num /= 2;
        
    }
    
    ones
    
}
