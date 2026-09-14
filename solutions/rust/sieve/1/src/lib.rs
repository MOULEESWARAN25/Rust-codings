pub fn is_prime(num: u64) -> bool{

    let mut i = 2;
    while  i * i <= num{

        if num % i == 0{

            return false;
            
        }
        i += 1;
    }
    true
    
}


pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // todo!("Construct a vector of all primes up to {upper_bound}");

    let mut vec = vec![];
    if upper_bound == 1{

        return vec;
        
    }
    for i in 2..=upper_bound{

        if is_prime(i){

            vec.push(i);
            
        }

        
    }

    vec
    
}
