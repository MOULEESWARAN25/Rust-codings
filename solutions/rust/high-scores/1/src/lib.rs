#[derive(Debug)]
pub struct HighScores{

    score: Vec<u32>,
    
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        // todo!("Construct a HighScores struct, given the scores: {scores:?}")
        let mut vec = vec![]; 
        for i in scores{
            vec.push(*i);
        }
        HighScores{score: vec}
    }

    pub fn scores(&self) -> &[u32] {
        // todo!("Return all the scores as a slice")

        self.score.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        // todo!("Return the latest (last) score")

        if self.score.is_empty(){

            return None;
            
        }
        Some(self.score[self.score.len() - 1])
        
    }

    pub fn personal_best(&self) -> Option<u32> {
        // todo!("Return the highest score")
        if self.score.is_empty(){return None;}
        let mut arr = self.score.clone();
        arr.sort();
        Some(arr[arr.len() - 1])
        
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // todo!("Return 3 highest scores")
        let mut vec = vec![];

        if self.score.is_empty(){return vec;}

        let mut arr = self.score.clone();

        arr.sort();

        if arr.len() == 1{

            vec.push(arr[0]);
            return vec
            
        }
        if arr.len() == 2{

            vec.push(arr[1]);
            vec.push(arr[0]);
            return vec
            
        }
      
        vec.push(arr[arr.len() - 1]);
        vec.push(arr[arr.len() - 2]);
        vec.push(arr[arr.len() - 3]);        
        
        vec
    }
}
