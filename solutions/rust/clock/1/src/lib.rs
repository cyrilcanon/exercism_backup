use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        
        let total_minutes = ((hours % 24) + 24) * 60 + (minutes % (24 * 60)) + 24 * 60;
        let new_minutes = total_minutes % 60;
        Clock{hours: ((total_minutes - new_minutes) / 60 ) % 24, minutes: new_minutes}
        
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}
