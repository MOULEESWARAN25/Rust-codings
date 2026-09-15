use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock{

    hours: i32,
    minutes: i32,
    
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let min = self.minutes.to_string();
        let hrs = self.hours.to_string();
        
        
        if hrs.len() == 2 && min.len() == 2{
            return write!(f, "{}:{}", self.hours, self.minutes);
        }
        else if hrs.len() == 1 && min.len() == 2{
            return write!(f, "0{}:{}", self.hours, self.minutes);
        }
        else if hrs.len() == 2 && min.len() == 1{
            return write!(f, "{}:0{}", self.hours, self.minutes);
        }
        else{
            return write!(f, "0{}:0{}", self.hours, self.minutes);
        }
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");

        let mut new_hour = hours;
        let mut new_minute = minutes;
        
        while new_minute > 59{

            new_minute -= 60;
            new_hour += 1;
            
        }

        while new_minute < 0{

            new_minute += 60;
            new_hour -= 1;
            
        }
        
        while new_hour > 23{

            new_hour -= 24;
            
        }

        while new_hour < 0{

            new_hour += 24;
            
        }
    
        Clock{hours: new_hour, minutes: new_minute}

    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        
        let mut new_hour = self.hours;
        let mut new_minute = self.minutes + minutes;
        
        while new_minute > 59{

            new_minute -= 60;
            new_hour += 1;
            
        }

        while new_minute < 0{

            new_minute += 60;
            new_hour -= 1;
            
        }
        
        while new_hour > 23{

            new_hour -= 24;
            
        }

        while new_hour < 0{

            new_hour += 24;
            
        }
    
        Self{hours: new_hour, minutes: new_minute}
    }
    
}
