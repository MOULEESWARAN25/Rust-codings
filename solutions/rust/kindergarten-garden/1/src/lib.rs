pub fn adding(letter: char) -> &'static str {

            match letter{
                'V' => {return "violets";}
                'R' => {return "radishes";}
                'G' => {return "grass";}
                'C' => {return "clover";}
                _ => {return "";}
            }
 
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for")
    
    
    let mut res = Vec::<&str>::new();
    let v: Vec<&str> = diagram.split('\n').collect();
    
    match student {
        
        "Alice" => { 
            res.push(adding(v[0].as_bytes()[0] as char));
            res.push(adding(v[0].as_bytes()[1] as char));
            res.push(adding(v[1].as_bytes()[0] as char));
            res.push(adding(v[1].as_bytes()[1] as char));
        }
        "Bob" => { 
            res.push(adding(v[0].as_bytes()[2] as char));
            res.push(adding(v[0].as_bytes()[3] as char));
            res.push(adding(v[1].as_bytes()[2] as char));
            res.push(adding(v[1].as_bytes()[3] as char));
        }
        "Charlie" => { 
            res.push(adding(v[0].as_bytes()[4] as char));
            res.push(adding(v[0].as_bytes()[5] as char));
            res.push(adding(v[1].as_bytes()[4] as char));
            res.push(adding(v[1].as_bytes()[5] as char));
        }
        "David" => { 
            res.push(adding(v[0].as_bytes()[6] as char));
            res.push(adding(v[0].as_bytes()[7] as char));
            res.push(adding(v[1].as_bytes()[6] as char));
            res.push(adding(v[1].as_bytes()[7] as char));
        }
        "Eve" => { 
            res.push(adding(v[0].as_bytes()[8] as char));
            res.push(adding(v[0].as_bytes()[9] as char));
            res.push(adding(v[1].as_bytes()[8] as char));
            res.push(adding(v[1].as_bytes()[9] as char));
        }
        "Fred" => { 
            res.push(adding(v[0].as_bytes()[10] as char));
            res.push(adding(v[0].as_bytes()[11] as char));
            res.push(adding(v[1].as_bytes()[10] as char));
            res.push(adding(v[1].as_bytes()[11] as char));
        }
        "Ginny" => { 
            res.push(adding(v[0].as_bytes()[12] as char));
            res.push(adding(v[0].as_bytes()[13] as char));
            res.push(adding(v[1].as_bytes()[12] as char));
            res.push(adding(v[1].as_bytes()[13] as char));
        }
        "Harriet" => { 
            res.push(adding(v[0].as_bytes()[14] as char));
            res.push(adding(v[0].as_bytes()[15] as char));
            res.push(adding(v[1].as_bytes()[14] as char));
            res.push(adding(v[1].as_bytes()[15] as char));
        }
        "Ileana" => { 
            res.push(adding(v[0].as_bytes()[16] as char));
            res.push(adding(v[0].as_bytes()[17] as char));
            res.push(adding(v[1].as_bytes()[16] as char));
            res.push(adding(v[1].as_bytes()[17] as char));
        }
        "Joseph" => { 
            res.push(adding(v[0].as_bytes()[18] as char));
            res.push(adding(v[0].as_bytes()[19] as char));
            res.push(adding(v[1].as_bytes()[18] as char));
            res.push(adding(v[1].as_bytes()[19] as char));
        }
        "Kincaid" => { 
            res.push(adding(v[0].as_bytes()[20] as char));
            res.push(adding(v[0].as_bytes()[21] as char));
            res.push(adding(v[1].as_bytes()[20] as char));
            res.push(adding(v[1].as_bytes()[21] as char));
        }
        "Larry" => { 
            res.push(adding(v[0].as_bytes()[22] as char));
            res.push(adding(v[0].as_bytes()[23] as char));
            res.push(adding(v[1].as_bytes()[22] as char));
            res.push(adding(v[1].as_bytes()[23] as char));
        }
        _ => {}
    }


    res
    
}
