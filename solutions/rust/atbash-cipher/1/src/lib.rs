/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    // todo!("Encoding of {plain:?} in Atbash cipher.");
    if plain.is_empty(){return "".to_string();}
    let mut res = String::from("");
    let mut temp = String::from("");
    let arr = ['z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h', 'g', 'f', 'e', 'd', 'c', 'b', 'a'];
    let s = plain.to_ascii_lowercase();
    for i in s.chars(){

        if i.is_ascii_alphabetic(){
            if temp.len() == 5{

                res.push_str(&temp);
                res.push(' ');

                temp.clear();
                
            }
            temp.push(arr[(i as u8 - 97) as usize])
            
        }
        else if i.is_ascii_digit(){            
                
            if temp.len() == 5{

                res.push_str(&temp);
                res.push(' ');

                temp.clear();
                
            }
            temp.push(i);}
        
    }
    res.push_str(&temp);
    res
}
pub fn decode(cipher: &str) -> String {
    // todo!("Decoding of {cipher:?} in Atbash cipher.");
    if cipher.is_empty(){return "".to_string();}
    let mut res = String::from("");
    let arr = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let s = cipher.to_ascii_lowercase();
    for i in s.chars(){

        if i.is_ascii_alphabetic(){

            res.push(arr[((i as u8).abs_diff(123)) as usize - 1])
            
        }
        else if i.is_ascii_digit(){

            res.push(i);
            
        }

    }
    
    res
    
}
