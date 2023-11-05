use std::fmt;

#[derive(Eq, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {

    fn to_min(&self) -> i32 {
        self.hours * 60 + self.minutes
    }

    fn from_min(mins: i32) -> Self {
        Clock {
            hours: (mins / 60) % 24,
            minutes: mins % 60,
        }
    }
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::from_min(hours * 60 + minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::from_min(self.to_min() + minutes)
    }
}
