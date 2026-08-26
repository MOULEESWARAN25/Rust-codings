pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let len = num.to_string().len() as u32;
    let mut temp = num;
    let mut n = 0;
    while temp != 0{

        n += (temp % 10).pow(len);
        temp /= 10;
        
    }

    if n == num{true}
    else{false}
    
}
