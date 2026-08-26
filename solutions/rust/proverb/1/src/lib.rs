pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")    
    let mut text = String::from("");
    if list.len() == 0{return text;}
    let last_text = format!("And all for the want of a {}.", list[0]);
    
    let mut temp = list.len() - 1;
    if temp > 0{
        let mut i = 0;
        while temp > 0{

            let temp_str = format!("For want of a {} the {} was lost.\n", list[i], list[i + 1]);
            // text.push('\n');
            text.push_str(&temp_str);
            temp -= 1;
            i += 1;
            
        }
    }  
        
    text.push_str(&last_text);
    text
}
