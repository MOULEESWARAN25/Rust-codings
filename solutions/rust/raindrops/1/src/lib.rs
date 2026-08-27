pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
    let mut res = String::from("");
    
    if n.is_multiple_of(3){
        res.push_str("Pling");    
    }
    if n.is_multiple_of(5){
        res.push_str("Plang");    
    }
    if n.is_multiple_of(7){
        res.push_str("Plong");    
    }
    if res.len() < 1{return n.to_string();}
    res
}
