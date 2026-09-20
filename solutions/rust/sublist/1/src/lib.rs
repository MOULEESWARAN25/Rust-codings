#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // todo!(
    //     "Determine if the {first_list:?} is equal to, sublist of, superlist of or unequal to {second_list:?}."
    // );
    if first_list == second_list{

        return Comparison::Equal;
        
    }
    else if first_list.is_empty() && !second_list.is_empty(){

        return Comparison::Sublist;
        
    }
    else if !first_list.is_empty() && second_list.is_empty(){

        return Comparison::Superlist;
        
    }
    else if first_list.len() < second_list.len(){

        let mut start = 0;
        let mut end = first_list.len();
        let mut vec = Vec::<i32>::new();
        while end <= second_list.len(){            

            vec = second_list[start..end].to_vec();

            if first_list == vec{return Comparison::Sublist;}

            start += 1;end += 1;

        }

        return Comparison::Unequal;
    }
    else if second_list.len() < first_list.len(){

        let mut start = 0;
        let mut end = second_list.len();
        let mut vec = Vec::<i32>::new();
        while end <= first_list.len(){            

            vec = first_list[start..end].to_vec();
            
            if second_list == vec{return Comparison::Superlist;}

            start += 1;end += 1;

        }
        
        return Comparison::Unequal;
    }
    else{return Comparison::Unequal;}
}
