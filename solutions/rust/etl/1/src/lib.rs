use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    // todo!("How will you transform the tree {h:?}?")
    let mut res = BTreeMap::<char, i32>::new();
    
    for (key, value) in h.iter(){
        
        for x in value{
            
            res.insert(x.to_ascii_lowercase(), *key);
            
        }
        
        
    }
    
    res
}
