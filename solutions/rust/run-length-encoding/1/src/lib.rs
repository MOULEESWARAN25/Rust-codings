pub fn encode(source: &str) -> String {
    // todo!("Return the run-length encoding of {source}.");
    let mut res = String::from("");
    if source.is_empty(){return res;}
    
    let mut prev ='1';
    let mut count: u32 = 1;
    for i in source.chars(){
        
        if prev == i{count += 1;}
        else if prev == '1'{prev = i;}
        else{

            if count == 1{

                res.push(prev);
                
            }
            else{
                res.push_str(&count.to_string());
                res.push(prev);
            }
            count = 1;
        }
        
        prev = i;
    }
    
    if count == 1{

        res.push(prev);
        
    }
    else{
        res.push_str(&count.to_string());
        res.push(prev);
    }
    
    res
}


pub fn decode(source: &str) -> String {
    // todo!("Return the run-length decoding of {source}.");
    let mut res = String::from("");
    
    if source.is_empty(){return res;}
    
    let mut num_vec = Vec::<String>::new();
    let mut txt_vec = Vec::<char>::new();
    
    let mut num = String::from("");
    
    for i in source.chars(){
        
        if i.is_ascii_digit(){
            
            num.push(i);
            
        }
        else{
            
            txt_vec.push(i);
            num_vec.push(num.clone());
            
            num.clear();
            
        }

    }
    
    for i in 0.. num_vec.len(){
        
        if num_vec[i].is_empty(){
            
            res.push(txt_vec[i]);
            
        }
        else{
            
            let mut n = num_vec[i].parse::<i32>().unwrap();
            
            while n != 0{
                
                res.push(txt_vec[i]);
                n -= 1;
            }
            
            
        }
        
        
    }
    
    res
}
