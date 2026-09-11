use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
        let mut sum: usize = 0;
    
    if nucleotide != 'A' && nucleotide != 'C' && nucleotide != 'G' && nucleotide != 'T'{
        
        return Err(nucleotide);
        
    }
    
    for i in dna.chars(){
        
        if i == 'A' || i == 'C' || i == 'G' || i == 'T'{
        
            if i == nucleotide{
            
                sum += 1;
            
            }
        
        }
        else{
            
            return Err(i);
            
        }
        
    }
    
    Ok(sum)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.insert('A', 0);
    map.insert('C', 0);
    map.insert('G', 0);
    map.insert('T', 0);
    
    if dna.is_empty(){
        
        return Ok(map);
        
    }
    
    for i in dna.chars(){
        
        if map.contains_key(&i){
            
            let x = map.get_mut(&i).unwrap();
            *x += 1;
            
        }
        else{
            
            return Err(i);
            
        }
        
    }
    
    Ok(map)
}
