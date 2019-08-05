//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul};
use crate::length::Length;
use crate::time::Time;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Velocity {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.value(), unit_string(self.unit()))
   }
}

// ========================================
// calculations
// ========================================
impl Add for Velocity {
    type Output = Velocity;
    fn add(self, other: Velocity) -> Velocity {
        Velocity::meter_per_second(self.base_value() + other.base_value())
    }
}

impl AddAssign for Velocity {
    fn add_assign(&mut self, other: Self) {
        *self = Velocity::meter_per_second(self.base_value() + other.base_value());
    }
}

impl Sub for Velocity {
    type Output = Velocity;
    fn sub(self, other: Velocity) -> Velocity {
        Velocity::meter_per_second(self.base_value() - other.base_value())
    }
}

impl SubAssign for Velocity {
    fn sub_assign(&mut self, other: Self) {
        *self = Velocity::meter_per_second(self.base_value() - other.base_value());
    }
}

impl Mul<Time> for Velocity {
    type Output = Length;

    fn mul(self, rhs: Time) -> Length {
        Length::meter(self.base_value() * rhs.base_value())
    }
}

// ========================================
// Velocity module
// ========================================
#[derive(Copy, Clone)]
pub enum VelocityUnit {
    MeterPerSecond,
    KilometerPerHour,
}

pub fn velocity_ratio(unit: VelocityUnit) -> num_rational::Ratio<i32>
{
    match unit {
        VelocityUnit::KilometerPerHour => num_rational::Ratio::new(3600, 1000),
        VelocityUnit::MeterPerSecond => num_rational::Ratio::new(1, 1),
    }
}

pub fn unit_string(unit: VelocityUnit) -> String
{
    match unit {
        VelocityUnit::KilometerPerHour => String::from("km/h"),
        VelocityUnit::MeterPerSecond => String::from("m/s"),
    }
}

#[derive(Copy, Clone)]
pub struct Velocity
{
    value: f64,
    unit: VelocityUnit,
    ratio: num_rational::Ratio<i32>,
}

impl Velocity {
    pub fn new(value: f64, unit: VelocityUnit) -> Velocity
    {
        Velocity { value: value, ratio: velocity_ratio(unit), unit: unit }
    }

    pub fn meter_per_second(value: f64) -> Velocity
    {
        Velocity { value: value, ratio: velocity_ratio(VelocityUnit::MeterPerSecond), unit: VelocityUnit::MeterPerSecond }
    }

    pub fn kilometer_per_hour(value: f64) -> Velocity
    {
        Velocity { value: value, ratio: velocity_ratio(VelocityUnit::KilometerPerHour), unit: VelocityUnit::KilometerPerHour }
    }

    pub fn value(&self) -> f64
    {
        self.value
    }

    pub fn base_value(&self) -> f64
    {
        (*self.ratio.denom() as f64) * self.value / (*self.ratio.numer() as f64)
    }

    pub fn unit(&self) -> VelocityUnit
    {
        self.unit
    }

    pub fn convert(&self, unit: VelocityUnit) -> Velocity
    {
        let ratio = velocity_ratio(unit);
        Velocity { value: self.base_value() * (*ratio.numer() as f64) / (*ratio.denom() as f64), ratio: ratio, unit: unit }
    }
}

#[cfg(test)]
mod test {
    use super::{Velocity, VelocityUnit};
    use crate::length::Length;
    use crate::time::Time;

    #[test]
    fn test_factories_() {
        let kmh = Velocity::kilometer_per_hour(3.6);
        assert_eq!(3.6, kmh.value());
        assert_eq!(1.0, kmh.base_value());

        let ms = Velocity::meter_per_second(2.0);
        assert_eq!(2.0, ms.value());
        assert_eq!(2.0, ms.base_value());

        let n = Velocity::new(2.0, VelocityUnit::MeterPerSecond);
        assert_eq!(2.0, n.value());
        assert_eq!(2.0, n.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut ms = Velocity::meter_per_second(5.0);
        let kmh = Velocity::kilometer_per_hour(3.6);

        let a = ms + kmh;
        assert_eq!(6.0, a.value());
        let b = ms - kmh;
        assert_eq!(4.0, b.value());
        ms += kmh;
        assert_eq!(6.0, ms.value());
        ms -= kmh;
        assert_eq!(5.0, ms.value());
    }

    #[test]
    fn test_strings() {
        let kmh = Velocity::kilometer_per_hour(3.0);
        assert_eq!("3km/h", kmh.to_string());

        let ms = Velocity::meter_per_second(2.5);
        assert_eq!("2.5m/s", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let kmh = Velocity::kilometer_per_hour(3.6);
        let ms = kmh.convert(VelocityUnit::MeterPerSecond);
        assert_eq!(1.0, ms.value());
    }

    #[test]
    fn test_derived_units()
    {
        let m = Length::meter(6.0);
        let s = Time::seconds(2.0);
        let v = m / s;
        assert_eq!(3.0, v.value());
        let m2 = v * s;
        assert_eq!(m.value(), m2.value());
        let s2 = m / v;
        assert_eq!(s.value(), s2.value());
    }
}
