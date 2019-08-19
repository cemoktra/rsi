use crate::value::Unit;

// ========================================
// Velocity module
// ========================================
#[derive(Copy, Clone)]
pub enum VelocityUnit {
    MeterPerSecond,
    KilometerPerHour,
}

impl Unit for VelocityUnit {
    type UnitEnum = VelocityUnit;

    fn ratio(&self) -> f64
    {
        match self {
            VelocityUnit::KilometerPerHour => 1.0 / 3.6,
            VelocityUnit::MeterPerSecond => 1.0,
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            VelocityUnit::KilometerPerHour => String::from("km/h"),
            VelocityUnit::MeterPerSecond => String::from("m/s"),
        }
    }
}