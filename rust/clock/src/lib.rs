#![feature(euclidean_division)]

use std::fmt;
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours = hours.mod_euc(24) + minutes.div_euc(60);
        Clock {
            hours: hours.mod_euc(24),
            minutes: minutes.mod_euc(60),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        &self + &Clock::new(0, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl<'a, 'b> Add<&'b Clock> for &'a Clock {
    type Output = Clock;

    fn add(self, other: &'b Clock) -> Clock {
        Clock::new(self.hours + other.hours, self.minutes + other.minutes)
    }
}
