pub fn is_big(mut input: Vec<u64>, x : u64) -> bool{
    input.sort();

    if input[input.len() - 1] == x{

        return true;
        
    }

    false
    
}

pub fn is_small(input: &[Vec<u64>], x: usize, y: usize) -> bool{

    let num = input[x][y];
    for i in 0.. input.len(){

        if input[i][y] < num{

            return false;
            
        }
        
    }

    true
    
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    // todo!("find the saddle points of the following matrix: {input:?}")

    let mut vec = Vec::<(usize, usize)>::new();
    
    for x in 0.. input.len(){

        for y in 0.. input[x].len(){

            if is_big(input[x].clone(), input[x][y]) && is_small(input, x, y){

                vec.push((x, y));
                
            }
            
            
        }
        
    }

    vec
}
