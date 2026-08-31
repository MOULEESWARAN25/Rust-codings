pub fn reply(message: &str) -> &str {
    // todo!("have Bob reply to the incoming message: {message}")

    if message.trim().ends_with("?"){
    
        if message.chars().any(|c| c.is_ascii_uppercase()) && !message.chars().any(|c| c.is_ascii_lowercase()){
            return "Calm down, I know what I'm doing!";
        }
        return "Sure."; 
    }
    
    if message.chars().any(|c| c.is_ascii_uppercase()) && !message.chars().any(|c| c.is_ascii_lowercase()){
        
        return "Whoa, chill out!";
        
    }
    
    if message.chars().all(|x| x.is_ascii_whitespace()){
        
        return "Fine. Be that way!";
        
    }
    
    return "Whatever.";
}
