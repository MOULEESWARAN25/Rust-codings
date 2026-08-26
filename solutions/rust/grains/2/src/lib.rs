pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    let mut grains: u64 = 2;    
    if s == 1 {return 1_u64;}
    else if s == 2 {return grains;}
    else if s == 0 {panic!();}

    for _x in 3 ..= s {

        grains *= 2;
        
    }

    grains
    
}

pub fn total() -> u64 {
    // todo!();
    let grains: u64 = 18446744073709551615;
    grains
    
}
