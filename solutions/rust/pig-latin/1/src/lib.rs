pub fn rule1(input: &str) -> Option<String>{

        if input.starts_with("a") ||  input.starts_with("e") || input.starts_with("i") || input.starts_with("o") || input.starts_with("u") || input.starts_with("xr") || input.starts_with("yt"){

        return Some(input.to_string() + "ay");
        
    }

    None
    
}

pub fn rule4(input: &str) -> Option<String>{

    let mut count = 0;
    let mut txt = String::from("");
    for i in input.chars(){
        if !"aeiouy".contains(i){

            count += 1;
            txt.push(i);
            
        }
        else {

            break;
            
        } 
    }

    if input.as_bytes()[count] == b'y' && count != 0{
        
        // txt.push('y');
        let res = input.strip_prefix(txt.as_str()).unwrap();

        return Some(res.to_string() + txt.as_str() + "ay");
    }
    else {return None;}
        
}


pub fn rule3(input: &str) -> Option<String>{

    let mut count = 0;
    let mut txt = String::from("");
    for i in input.chars(){
        if !"aeioqu".contains(i){

            count += 1;
            txt.push(i);
            
        }
        else {

            break;
            
        } 
    }
    if count < input.len() && input.as_bytes()[count] == b'q' && input.as_bytes()[count + 1] == b'u'{

        txt.push('q'); txt.push('u');
        let res = input.strip_prefix(txt.as_str()).unwrap();

        return Some(res.to_string() + txt.as_str() + "ay");
    }
    else {return None;}
  
}


pub fn rule2(input: &str) -> Option<String>{

    let mut count = 0;
    let mut txt = String::from("");
    for i in input.chars(){
        if !"aeiou".contains(i){

            count += 1;
            txt.push(i);
            
        }
        else {

            break;
            
        } 
    }
    if count == 0{

        return None;
        
    }

    let res = input.strip_prefix(txt.as_str()).unwrap();

    Some(res.to_string() + txt.as_str() + "ay")
    
}

pub fn translate(input: &str) -> String {
    // todo!("Using the Pig Latin text transformation rules, convert the given input '{input}'");
    let mut res = String::from("");
    let input_vec: Vec<&str> = input.split(" ").collect();

    for i in &input_vec {
        
        let mut a = rule1(i);
        match a{
    
            Some(val)  => {res = res + val.as_str();res.push(' ');continue;}
            None => {}
            
        }
        
        a = rule3(i);
        match a{
    
            Some(val)  => {res = res + val.as_str();res.push(' ');continue;}
            None => {}
            
        }
        
        a = rule4(i);
        match a{
    
            Some(val)  => {res = res + val.as_str();res.push(' ');continue;}
            None => {}
            
        }
        
        a = rule2(i);
        match a{
    
            Some(val)  => {res = res + val.as_str();res.push(' ');continue;}
            None => {}
            
        }
            
    }

    res.trim().to_string()
}