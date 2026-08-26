pub fn square_of_sum(n: u32) -> u32 {
    // todo!("square of sum of 1...{n}")
    let mut sum: u32 = 0;

    for x in 1 ..= n{sum += x;}

    sum.pow(2)
    
}

pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")

    let mut sum: u32 = 0;

    for x in 1 ..= n{sum += x.pow(2);}

    sum
    
}

pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
    square_of_sum(n) - sum_of_squares(n)
}
