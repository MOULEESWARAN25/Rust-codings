pub struct Triangle{

    sides: [u64; 3],
    
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // todo!(
        //     "Construct new Triangle from following sides: {sides:?}. Return None if the sides are invalid."
        // );

        if sides[0] <= 0 || sides[1] <= 0 || sides[2] <= 0{

            return None;
            
        }
        else if sides[0] + sides[1] < sides[2] || sides[1] + sides[2] < sides[0] || sides[0] + sides[2] < sides[1]{

            return None;
            
        }

        Some(Triangle{sides: sides})
        
        
    }

    pub fn is_equilateral(&self) -> bool {
        // todo!("Determine if the Triangle is equilateral.");
        if self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2] && self.sides[0] == self.sides[2] {

            return true;
            
        }

        false
        
    }

    pub fn is_scalene(&self) -> bool {
        // todo!("Determine if the Triangle is scalene.");
        if self.sides[0] != self.sides[1] && self.sides[1] != self.sides[2] && self.sides[0] != self.sides[2] {

            return true;
            
        }

        false
        
    }

    pub fn is_isosceles(&self) -> bool {
        // todo!("Determine if the Triangle is isosceles.");

        let mut count = 0;

        if self.sides[0] == self.sides[1]{count += 1;}
        if self.sides[1] == self.sides[2]{count += 1;}
        if self.sides[0] == self.sides[2]{count += 1;}

        if count >= 1{

            return true;
            
        }
        false
    }
}