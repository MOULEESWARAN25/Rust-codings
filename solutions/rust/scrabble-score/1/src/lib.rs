/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    // todo!("Score {word} in Scrabble.");
    let mut sum: u64 = 0;

    let txt = word.to_ascii_uppercase();

    
    for i in txt.chars(){

        if "AEIOULNRST".contains(i){

            sum += 1;
            
        }
        else if "DG".contains(i){

            sum += 2;
            
        }
        else if "BCMP".contains(i){

            sum += 3;
            
        }
        else if "FHVWY".contains(i){

            sum += 4;
            
        }
        else if "K".contains(i){

            sum += 5;
            
        }
        else if "JX".contains(i){

            sum += 8;
            
        }
        else if "QZ".contains(i){

            sum += 10;
            
        }
        
    }

    sum
    
}
