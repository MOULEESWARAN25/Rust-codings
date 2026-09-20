pub fn abbreviate(phrase: &str) -> String {
    // todo!("Given the phrase '{phrase}', return its acronym");

    let mut res = String::from("");

    let mut prev = 'a';

    for i in phrase.chars(){

        if i.is_ascii_uppercase() || prev == ' ' || prev == '-'{

            if !prev.is_ascii_uppercase() && i.is_ascii_alphabetic(){
                res.push(i.to_ascii_uppercase());
            }
        }
        
        prev = i;
    }
    
    res
}
