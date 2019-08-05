//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign};

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), unit_string(self.unit()))
    }
}

// ========================================
// calculations
// ========================================
impl Add for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        Time::seconds(self.base_value() + other.base_value())
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        *self = Time::seconds(self.base_value() + other.base_value());
    }
}

impl Sub for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Time {
        Time::seconds(self.base_value() - other.base_value())
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        *self = Time::seconds(self.base_value() - other.base_value());
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

pub fn length_ratio(unit: TimeUnit) -> num_rational::Ratio<i32>
{
    match unit {
        TimeUnit::Milliseconds => num_rational::Ratio::new(1, 1000),
        TimeUnit::Seconds => num_rational::Ratio::new(1, 1),
        TimeUnit::Minutes => num_rational::Ratio::new(60, 1),
        TimeUnit::Hours => num_rational::Ratio::new(60 * 60, 1),
        TimeUnit::Days => num_rational::Ratio::new(24 * 60* 60, 1)
    }
}

pub fn unit_string(unit: TimeUnit) -> String
{
    match unit {
        TimeUnit::Milliseconds => String::from("ms"),
        TimeUnit::Seconds => String::from("s"),
        TimeUnit::Minutes => String::from("min"),
        TimeUnit::Hours => String::from("h"),
        TimeUnit::Days => String::from("d")
    }
}

#[derive(Copy, Clone)]
pub struct Time
{
    value: f64,
    unit: TimeUnit,
    ratio: num_rational::Ratio<i32>,
}

impl Time {
    pub fn new(value: f64, unit: TimeUnit) -> Time
    {
        Time { value: value, ratio: length_ratio(unit), unit: unit }
    }

    pub fn milliseconds(value: f64) -> Time
    {
        Time { value: value, ratio: length_ratio(TimeUnit::Milliseconds), unit: TimeUnit::Milliseconds }
    }
    pub fn seconds(value: f64) -> Time
    {
        Time { value: value, ratio: length_ratio(TimeUnit::Seconds), unit: TimeUnit::Seconds }
    }
    pub fn minutes(value: f64) -> Time
    {
        Time { value: value, ratio: length_ratio(TimeUnit::Minutes), unit: TimeUnit::Minutes }
    }
    pub fn hours(value: f64) -> Time
    {
        Time { value: value, ratio: length_ratio(TimeUnit::Hours), unit: TimeUnit::Hours }
    }
    pub fn days(value: f64) -> Time
    {
        Time { value: value, ratio: length_ratio(TimeUnit::Days), unit: TimeUnit::Days }
    }

    pub fn value(&self) -> f64
    {
        self.value
    }

    pub fn base_value(&self) -> f64
    {
        (*self.ratio.numer() as f64) * self.value / (*self.ratio.denom() as f64)
    }

    pub fn unit(&self) -> TimeUnit
    {
        self.unit
    }

    pub fn convert(&self, unit: TimeUnit) -> Time
    {
        let ratio = length_ratio(unit);
        Time { value: self.base_value() * (*ratio.denom() as f64) / (*ratio.numer() as f64), ratio: ratio, unit: unit }
    }
}


#[cfg(test)]
mod test {
    use super::{Time, TimeUnit};

    #[test]
    fn test_factories_() {
        let d = Time::days(1.0);
        assert_eq!(1.0, d.value());
        assert_eq!(24.0 * 60.0 * 60.0, d.base_value());

        let h = Time::hours(2.0);
        assert_eq!(2.0, h.value());
        assert_eq!(2.0 * 60.0 * 60.0, h.base_value());

        let min = Time::minutes(5.0);
        assert_eq!(5.0, min.value());
        assert_eq!(300.0, min.base_value());

        let s = Time::seconds(20.0);
        assert_eq!(20.0, s.value());
        assert_eq!(20.0, s.base_value());

        let ms = Time::milliseconds(20.0);
        assert_eq!(20.0, ms.value());
        assert_eq!(0.02, ms.base_value());

        let n = Time::new(2.0, TimeUnit::Seconds);
        assert_eq!(2.0, n.value());
        assert_eq!(2.0, n.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut s = Time::seconds(30.0);
        let min = Time::minutes(2.0);

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
        let d = Time::days(3.0);
        assert_eq!("3d", d.to_string());

        let h = Time::hours(2.0);
        assert_eq!("2h", h.to_string());
        let s = Time::seconds(2.5);

        assert_eq!("2.5s", s.to_string());

        let min = Time::minutes(50.0);
        assert_eq!("50min", min.to_string());

        let ms = Time::milliseconds(20.2);
        assert_eq!("20.2ms", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let hours = Time::hours(1.5);
        let min = hours.convert(TimeUnit::Minutes);
        assert_eq!(90.0, min.value());
    }
}
