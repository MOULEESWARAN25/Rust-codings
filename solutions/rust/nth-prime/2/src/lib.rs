pub fn is_prime(n : u32) -> bool {
    if n < 2 {return false;}
    for x in 2 ..= n.isqrt() {

        if n % x == 0{ return false;}
        
    }
    true
    
}

pub fn nth(n: u32) -> u32 {
    // todo!("What is the 0-indexed {n}th prime number?")

    if n == 0 {return 2_u32;}

    let mut num : u32 = 3;
    let mut temp: u32 = 0;

    while temp < n{

        if is_prime(num) {temp += 1;}
        if temp == n{break;}
        num += 2;
        
    }

    num
}
