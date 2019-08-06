//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Mul,Div};
use crate::value::{Value,ValueTrait};
use crate::area::AreaUnit;
use crate::time::TimeUnit;
use crate::velocity::VelocityUnit;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<LengthUnit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), self.unit_string())
    }
}

// ========================================
// calculations
// ========================================
impl Add for Value<LengthUnit> {
    type Output = Value<LengthUnit>;
    fn add(self, other: Value<LengthUnit>) -> Value<LengthUnit> {
        Value { unit: LengthUnit::Meter, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<LengthUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: LengthUnit::Meter, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<LengthUnit> {
    type Output = Value<LengthUnit>;
    fn sub(self, other: Value<LengthUnit>) -> Value<LengthUnit> {
        Value { unit: LengthUnit::Meter, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<LengthUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: LengthUnit::Meter, value: self.base_value() - other.base_value() };
    }
}

impl Mul for Value<LengthUnit> {
    type Output = Value<AreaUnit>;
    fn mul(self, rhs: Value<LengthUnit>) -> Value<AreaUnit> {
        Value { unit: AreaUnit::SquareMeter, value: self.base_value() * rhs.base_value() }
    }
}

impl Div<Value<TimeUnit>> for Value<LengthUnit> {
    type Output = Value<VelocityUnit>;

    fn div(self, rhs: Value<TimeUnit>) -> Value<VelocityUnit> {
        Value { unit: VelocityUnit::MeterPerSecond, value: self.base_value() / rhs.base_value() }
    }
}

impl Div<Value<VelocityUnit>> for Value<LengthUnit> {
    type Output = Value<TimeUnit>;

    fn div(self, rhs: Value<VelocityUnit>) -> Value<TimeUnit> {
        Value { unit: TimeUnit::Seconds, value: self.base_value() / rhs.base_value() }
    }
}

// ========================================
// Length module
// ========================================
#[derive(Copy, Clone)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer
}

impl LengthUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            LengthUnit::Kilometer => num_rational::Ratio::new(1000, 1),
            LengthUnit::Meter => num_rational::Ratio::new(1, 1),
            LengthUnit::Centimeter => num_rational::Ratio::new(1, 100),
            LengthUnit::Millimeter => num_rational::Ratio::new(1, 1000),
        }
    }
    
    fn unit(&self) -> String
    {   
        match self {
            LengthUnit::Kilometer => String::from("km"),
            LengthUnit::Meter => String::from("m"),
            LengthUnit::Centimeter => String::from("cm"),
            LengthUnit::Millimeter => String::from("mm")
        }
    }
}

impl ValueTrait<LengthUnit> for Value<LengthUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> LengthUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: LengthUnit) -> Value<LengthUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}


#[cfg(test)]
mod test {
    use super::{LengthUnit};
    use crate::value::{Value,ValueTrait};

    #[test]
    fn test_factories_() {
        let km = Value { unit: LengthUnit::Kilometer, value: 3.0 };
        assert_eq!(3.0, km.value());
        assert_eq!(3000.0, km.base_value());

        let m = Value { unit: LengthUnit::Meter, value: 2.0 };
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());

        let cm = Value { unit: LengthUnit::Centimeter, value: 50.0 };
        assert_eq!(50.0, cm.value());
        assert_eq!(0.5, cm.base_value());

        let mm = Value { unit: LengthUnit::Millimeter, value: 20.0 };
        assert_eq!(20.0, mm.value());
        assert_eq!(0.02, mm.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Value { unit: LengthUnit::Meter, value: 2.0 };
        let cm = Value { unit: LengthUnit::Centimeter, value: 50.0 };

        let a = m + cm;
        assert_eq!(2.5, a.value());
        let b = m - cm;
        assert_eq!(1.5, b.value());
        m += cm;
        assert_eq!(2.5, m.value());
        m -= cm;
        assert_eq!(2.0, m.value());
    }

    #[test]
    fn test_strings() {
        let km = Value { unit: LengthUnit::Kilometer, value: 3.0 };
        let m = Value { unit: LengthUnit::Meter, value: 2.5 };
        let cm = Value { unit: LengthUnit::Centimeter, value: 50.0 };
        let mm = Value { unit: LengthUnit::Millimeter, value: 20.2 };
        assert_eq!("3km", km.to_string());
        assert_eq!("2.5m", m.to_string());
        assert_eq!("50cm", cm.to_string());
        assert_eq!("20.2mm", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value { unit: LengthUnit::Kilometer, value: 1.5 };
        let m = km.convert(LengthUnit::Meter);
        let cm = km.convert(LengthUnit::Centimeter);
        assert_eq!(1500.0, m.value());
        assert_eq!(150000.0, cm.value());
    }
}
