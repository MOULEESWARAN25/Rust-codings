pub fn factors(n: u64) -> Vec<u64> {
    // todo!("This should calculate the prime factors of {n}")

    let mut temp = n;
    let mut div: u64 = 2;
    let mut res = Vec::<u64>::new();
    
    while temp != 1{

        if temp % div == 0{res.push(div); temp /= div;}
        else{div += 1;}
        
    }
    res
}
