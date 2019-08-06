//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul};
use crate::value::{Value,ValueTrait};
use crate::length::LengthUnit;
use crate::time::TimeUnit;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<VelocityUnit> {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.value(), self.unit_string())
   }
}

// ========================================
// calculations
// ========================================
impl Add for Value<VelocityUnit> {
    type Output = Value<VelocityUnit>;
    fn add(self, other: Value<VelocityUnit>) -> Value<VelocityUnit> {
        Value { unit: VelocityUnit::MeterPerSecond, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<VelocityUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: VelocityUnit::MeterPerSecond, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<VelocityUnit> {
    type Output = Value<VelocityUnit>;
    fn sub(self, other: Value<VelocityUnit>) -> Value<VelocityUnit> {
        Value { unit: VelocityUnit::MeterPerSecond, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<VelocityUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: VelocityUnit::MeterPerSecond, value: self.base_value() - other.base_value() };
    }
}


impl Mul<Value<TimeUnit>> for Value<VelocityUnit> {
    type Output = Value<LengthUnit>;

    fn mul(self, rhs: Value<TimeUnit>) -> Value<LengthUnit> {
        Value { unit: LengthUnit::Meter, value: self.base_value() * rhs.base_value() }
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

impl VelocityUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            VelocityUnit::KilometerPerHour => num_rational::Ratio::new(1000, 3600),
            VelocityUnit::MeterPerSecond => num_rational::Ratio::new(1, 1),
        }
    }
    
    fn unit(&self) -> String
    {   
        match self {
            VelocityUnit::KilometerPerHour => String::from("km/h"),
            VelocityUnit::MeterPerSecond => String::from("m/s"),
        }
    }
}

impl ValueTrait<VelocityUnit> for Value<VelocityUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> VelocityUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: VelocityUnit) -> Value<VelocityUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}

#[cfg(test)]
mod test {
    use super::VelocityUnit;
    use crate::length::LengthUnit;
    use crate::time::TimeUnit;
    use crate::value::{Value,ValueTrait};

    #[test]
    fn test_factories_() {
        let kmh = Value { unit: VelocityUnit::KilometerPerHour, value: 3.6 };
        let ms = Value { unit: VelocityUnit::MeterPerSecond, value: 2.0 };
        assert_eq!(3.6, kmh.value());
        assert_eq!(1.0, kmh.base_value());
        assert_eq!(2.0, ms.value());
        assert_eq!(2.0, ms.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut ms = Value { unit: VelocityUnit::MeterPerSecond, value: 5.0 };
        let kmh = Value { unit: VelocityUnit::KilometerPerHour, value: 3.6 };
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
        let kmh = Value { unit: VelocityUnit::KilometerPerHour, value: 3.0 };
        let ms = Value { unit: VelocityUnit::MeterPerSecond, value: 2.5 };
        assert_eq!("3km/h", kmh.to_string());
        assert_eq!("2.5m/s", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let kmh = Value { unit: VelocityUnit::KilometerPerHour, value: 3.6 };
        let ms = kmh.convert(VelocityUnit::MeterPerSecond);
        assert_eq!(1.0, ms.value());
    }

    #[test]
    fn test_derived_units()
    {
        let m = Value { unit: LengthUnit::Meter, value: 6.0 };
        let s = Value { unit: TimeUnit::Seconds, value: 2.0 };
        let v = m / s;
        assert_eq!(3.0, v.value());
        let m2 = v * s;
        assert_eq!(m.value(), m2.value());
        let s2 = m / v;
        assert_eq!(s.value(), s2.value());
    }
}
