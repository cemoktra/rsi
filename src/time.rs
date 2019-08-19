use crate::value::{Unit};

// ========================================
// Time module
// ========================================
#[derive(Copy, Clone)]
pub enum TimeUnit {
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds
}

impl Unit for TimeUnit {
    type UnitEnum = TimeUnit;

    fn ratio(&self) -> f64
    {
        match self {
            TimeUnit::Milliseconds => 0.001,
            TimeUnit::Seconds => 1.0,
            TimeUnit::Minutes => 60.0,
            TimeUnit::Hours => 60.0 * 60.0,
            TimeUnit::Days => 24.0 * 60.0 * 60.0
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            TimeUnit::Milliseconds => String::from("ms"),
            TimeUnit::Seconds => String::from("s"),
            TimeUnit::Minutes => String::from("min"),
            TimeUnit::Hours => String::from("h"),
            TimeUnit::Days => String::from("d")        
        }
    }
}
