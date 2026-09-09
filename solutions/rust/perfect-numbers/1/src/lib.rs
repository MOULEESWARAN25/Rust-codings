#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    // todo!("classify {num}");
    if num == 0{return None;}
    
    let mut sum = 0;

    for i in 1..num{

        if num.is_multiple_of(i){
            
            sum += i;
            
        }
        
    }
    
    if sum == num{
        
        Some(Classification::Perfect)
        
    }
    else if sum > num{
        
        Some(Classification::Abundant)
        
    }
    else{
        
        Some(Classification::Deficient)
        
    }
}
