pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut len = 0;
    let mut numbers = std::collections::HashSet::new();

    while len < factors.len(){

        if factors[len] == 0{}
        else{
            for x in 1 .. limit{
            
                if x % factors[len] == 0{

                    numbers.insert(x);
                
                }
            }
        }
        len += 1;
    }

    numbers.iter().sum()
    
}
