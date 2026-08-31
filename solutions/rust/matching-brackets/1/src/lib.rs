pub fn brackets_are_balanced(string: &str) -> bool {
    // todo!("Check if the string \"{string}\" contains balanced brackets");

    let mut stack = Vec::<char>::new();
    
    if string.is_empty(){return true;}

    for x in string.chars(){

        if x == '['{stack.push(x);}
        else if x == '{'{stack.push(x);}
        else if x == '('{stack.push(x);}
        else if x == ']'{
            
            if !stack.is_empty() && stack.pop().unwrap() == '['{}
            else{
                
                return false;
                
            }
            
        }
        else if x == '}'{
            
            if !stack.is_empty() && stack.pop().unwrap() == '{'{}
            else{
                
                return false;
                
            }
            
        }
        else if x == ')'{
            
            if !stack.is_empty() && stack.pop().unwrap() == '('{}
            else{
                
                return false;
                
            }
            
        }
 
    }
    stack.is_empty()

}

