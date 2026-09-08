use std::char::from_digit;
pub fn find_diagonal(garden: &[&str], x: usize, y: usize) -> u32 {

    let mut num: u32 = 0;
    
    if x as i32 - 1 >= 0 && y as i32 - 1 >= 0{
        if garden[x - 1].as_bytes()[y - 1] == b'*'{
            
            num += 1;
            
        }
    }
    
    if x + 1 < garden.len() && y + 1 < garden[0].len(){
        if garden[x + 1].as_bytes()[y + 1] == b'*'{
            
            num += 1;
            
        }
    }
    
    if x + 1 < garden.len() && y as i32 - 1 >= 0{
        if garden[x + 1].as_bytes()[y - 1] == b'*'{
            
            num += 1;
            
        }
    }
    
    if x as i32 - 1 >= 0 && y + 1 < garden[0].len(){
        if garden[x - 1].as_bytes()[y + 1] == b'*'{
            
            num += 1;
            
        }
    }
    num
    
}

pub fn find_vertical(garden: &[&str], x: usize, y: usize) -> u32 {

    let mut num: u32 = 0;
    
    if x as i32 - 1 >= 0{
        if garden[x - 1].as_bytes()[y] == b'*'{
            
            num += 1;
            
        }
    }
    
    if x + 1 < garden.len(){
        if garden[x + 1].as_bytes()[y] == b'*'{
            
            num += 1;
            
        }
    }
    
    num
    
}

pub fn find_horizontal(garden: &[&str], x: usize, y: usize) -> u32 {

    let mut num: u32 = 0;
    
    if y as i32 - 1 >= 0{
        if garden[x].as_bytes()[y-1] == b'*'{
            
            num += 1;
            
        }
    }
    
    if y + 1 < garden[0].len(){
        if garden[x].as_bytes()[y+1] == b'*'{
            
            num += 1;
            
        }
    }
    num
    
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    // todo!(
    //     "\nAnnotate each square of the given garden with the number of flowers that surround said square (blank if there are no surrounding flowers):\n{garden:#?}\n"
    // );
    let mut res = vec![String::from(""); garden.len()];
    
    if garden.is_empty() || garden[0].is_empty(){
    
        return res;
        
    }
    
    let mut num;
    
    for x in 0..garden.len(){
        
        for y in 0..garden[0].len(){
        
            if garden[x].as_bytes()[y] == b' '{
            
                num = find_horizontal(garden, x, y) + find_vertical(garden, x, y) + find_diagonal(garden, x, y);
                
                if num == 0{
                    
                    res[x].push(' ');
                    
                }
                else{
                    
                    res[x].push(from_digit(num, 10).unwrap());
                    
                }
                
            }
            else {
                
                res[x].push('*');
                
            }
        }
    }    
    
   res
}
