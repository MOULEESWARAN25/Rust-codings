#[derive(Debug, PartialEq, Eq)]
pub struct Dna{
    
    dna: String,
    
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna{
    
    rna: String,
    
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        // todo!(
        //     "Construct new Dna from '{dna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"
        // );
        if dna.is_empty(){return Ok(Dna{dna: String::from("")});}
        for i in dna.chars(){

            if !"GCTA".contains(i){

                return Err(dna.find(i).unwrap());
                
            }            
        }

        Ok(Dna{dna: dna.to_string()})
    }

        pub fn into_rna(self) -> Rna {
        // todo!("Transform Dna {self:?} into corresponding Rna");
        let mut rna = String::from("");
        for i in self.dna.chars(){

            match i {

                'G' => {rna.push('C');}
                'C' => {rna.push('G');}
                'T' => {rna.push('A');}
                _ => {rna.push('U');}
            }
            
        }

        Rna::new(rna.as_str()).unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        // todo!(
        //     "Construct new Rna from '{rna}' string. If string contains invalid nucleotides return index of first invalid nucleotide"
        // );
        if rna.is_empty(){return Ok(Rna{rna: String::from("")});}
        for i in rna.chars(){

            if !"GCUA".contains(i){

                return Err(rna.find(i).unwrap());
                
            }            
        }

        Ok(Rna{rna: rna.to_string()})
        
        
    }
}
