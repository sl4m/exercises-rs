use std::fmt;

static MAX_HOURS: i32 = 24;
static MAX_MINUTES: i32 = 60;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::calculate(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::calculate(self.hours, self.minutes + minutes)
    }

    fn calculate(hours: i32, minutes: i32) -> Self {
        let new_minutes = minutes.rem_euclid(MAX_MINUTES);
        let new_hours = (hours + (minutes.div_euclid(MAX_MINUTES))).rem_euclid(MAX_HOURS);
        Self {hours: new_hours, minutes: new_minutes}
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
