//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign};
use crate::value::{Value,ValueTrait};

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<TimeUnit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), self.unit_string())
    }
}


// ========================================
// calculations
// ========================================
impl Add for Value<TimeUnit> {
    type Output = Value<TimeUnit>;
    fn add(self, other: Value<TimeUnit>) -> Value<TimeUnit> {
        Value { unit: TimeUnit::Seconds, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<TimeUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: TimeUnit::Seconds, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<TimeUnit> {
    type Output = Value<TimeUnit>;
    fn sub(self, other: Value<TimeUnit>) -> Value<TimeUnit> {
        Value { unit: TimeUnit::Seconds, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<TimeUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: TimeUnit::Seconds, value: self.base_value() - other.base_value() };
    }
}


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

impl TimeUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            TimeUnit::Milliseconds => num_rational::Ratio::new(1, 1000),
            TimeUnit::Seconds => num_rational::Ratio::new(1, 1),
            TimeUnit::Minutes => num_rational::Ratio::new(60, 1),
            TimeUnit::Hours => num_rational::Ratio::new(60 * 60, 1),
            TimeUnit::Days => num_rational::Ratio::new(24 * 60* 60, 1)
        }
    }
    
    fn unit(&self) -> String
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

impl ValueTrait<TimeUnit> for Value<TimeUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> TimeUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: TimeUnit) -> Value<TimeUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}

#[cfg(test)]
mod test {
    use super::{TimeUnit};
    use crate::value::{Value,ValueTrait};

    #[test]
    fn test_factories_() {
        let d = Value { unit: TimeUnit::Days, value: 1.0 };
        let h = Value { unit: TimeUnit::Hours, value: 2.0 };
        let min = Value { unit: TimeUnit::Minutes, value: 5.0 };
        let s = Value { unit: TimeUnit::Seconds, value: 20.0 };
        let ms = Value { unit: TimeUnit::Milliseconds, value: 20.0 };
        assert_eq!(1.0, d.value());
        assert_eq!(24.0 * 60.0 * 60.0, d.base_value());
        assert_eq!(2.0, h.value());
        assert_eq!(2.0 * 60.0 * 60.0, h.base_value());
        assert_eq!(5.0, min.value());
        assert_eq!(300.0, min.base_value());
        assert_eq!(20.0, s.value());
        assert_eq!(20.0, s.base_value());
        assert_eq!(20.0, ms.value());
        assert_eq!(0.02, ms.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut s = Value { unit: TimeUnit::Seconds, value: 30.0 };
        let min = Value { unit: TimeUnit::Minutes, value: 2.0 };

        let a = min + s;
        assert_eq!(150.0, a.value());
        let b = min - s;
        assert_eq!(90.0, b.value());
        s += min;
        assert_eq!(150.0, s.value());
        s -= min;
        assert_eq!(30.0, s.value());
    }

    #[test]
    fn test_strings() {
        let d = Value { unit: TimeUnit::Days, value: 3.0 };
        let h = Value { unit: TimeUnit::Hours, value: 2.0 };
        let min = Value { unit: TimeUnit::Minutes, value: 50.0 };
        let s = Value { unit: TimeUnit::Seconds, value: 2.5 };
        let ms = Value { unit: TimeUnit::Milliseconds, value: 20.2 };
        assert_eq!("3d", d.to_string());
        assert_eq!("2h", h.to_string());
        assert_eq!("50min", min.to_string());
        assert_eq!("2.5s", s.to_string());
        assert_eq!("20.2ms", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let h = Value { unit: TimeUnit::Hours, value: 1.5 };
        let min = h.convert(TimeUnit::Minutes);
        assert_eq!(90.0, min.value());
    }
}
