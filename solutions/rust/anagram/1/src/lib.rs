use std::collections::HashSet;
use std::collections::HashMap;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let mut resset = HashSet::new();
    let mut map1: HashMap<char, i32> = HashMap::new();
    let mut map2: HashMap<char, i32> = HashMap::new();
    
    
    let a = word.to_lowercase();
    let txt = a.as_str();
    
    for i in txt.chars(){
        
        if let Some(x) = map1.get_mut(&i) {
            *x += 1;
        }
        else{
            map1.insert(i, 1);
        }
    }
    
    for i in possible_anagrams{
        
        let b = i.to_lowercase();
        let txt2 = b.as_str();
        
        if txt != txt2 && txt.len() == txt2.len(){
            
            for j in txt2.chars(){
        
                if let Some(x) = map2.get_mut(&j) {
                    *x += 1;
                }
                else{
                    map2.insert(j, 1);
                }
        
            }
            
            if map1 == map2{
                
                resset.insert(*i);
                
            }
            
        }     
        map2.clear();
    }
    
    resset
}
