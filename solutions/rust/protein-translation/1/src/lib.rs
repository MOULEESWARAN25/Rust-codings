pub fn translate(rna: &str) -> Option<Vec<&str>> {

    let mut vec = Vec::<&str>::new();
    let mut txt = rna;

    if rna.is_empty(){

        return Some(vec);
        
    }

    // if (!rna.contains("UAA") || !rna.contains("UAG") || !rna.contains("UGA")) && rna.len() % 3 != 0{

    //     return None;
        
    // }
    
    let mut count = rna.len() / 3;
    
    if rna.len() % 3 != 0{count += 1;}

    while count != 0{
        
        // if !txt.contains("U"){

        //     return None;
        
        // }
        
        let s = txt.get(0..3)?;

        // if s.len() < 3{return None;}
        // println!("{}", s);
        // println!("{}", count);
        match s {

            "AUG" => {vec.push("Methionine");}
            "UUU" | "UUC" => {vec.push("Phenylalanine");}
            "UUA" | "UUG" => {vec.push("Leucine");}
            "UCU" | "UCC" | "UCA" | "UCG" => {vec.push("Serine");}
            "UAU" | "UAC" => {vec.push("Tyrosine");}
            "UGU" | "UGC" => {vec.push("Cysteine");}
            "UGG" => {vec.push("Tryptophan");}
            "UAA" | "UAG" | "UGA" => {break;}
            _ => {return None;}
        }

        txt = txt.strip_prefix(s).unwrap();
        count -= 1;
        
        
        
    }

    Some(vec)
}