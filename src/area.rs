//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Div};
use crate::value::{Value,ValueTrait};
use crate::length::LengthUnit;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<AreaUnit> {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.value(), self.unit_string())
   }
}

// ========================================
// calculations
// ========================================
impl Add for Value<AreaUnit> {
    type Output = Value<AreaUnit>;
    fn add(self, other: Value<AreaUnit>) -> Value<AreaUnit> {
        Value { unit: AreaUnit::SquareMeter, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<AreaUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: AreaUnit::SquareMeter, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<AreaUnit> {
    type Output = Value<AreaUnit>;
    fn sub(self, other: Value<AreaUnit>) -> Value<AreaUnit> {
        Value { unit: AreaUnit::SquareMeter, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<AreaUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: AreaUnit::SquareMeter, value: self.base_value() - other.base_value() };
    }
}

impl Div<Value<LengthUnit>> for Value<AreaUnit> {
    type Output = Value<LengthUnit>;

    fn div(self, rhs: Value<LengthUnit>) -> Value<LengthUnit> {
        Value { unit: LengthUnit::Meter, value: self.base_value() / rhs.base_value() }
    }
}

// ========================================
// Area module
// ========================================
#[derive(Copy, Clone)]
pub enum AreaUnit {
    SquareMillimeter,
    SquareCentimeter,
    SquareMeter,
    SquareKilometer
}

impl AreaUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            AreaUnit::SquareKilometer => num_rational::Ratio::new(1000000, 1),
            AreaUnit::SquareMeter => num_rational::Ratio::new(1, 1),
            AreaUnit::SquareCentimeter => num_rational::Ratio::new(1, 10000),
            AreaUnit::SquareMillimeter => num_rational::Ratio::new(1, 1000000)
            }
    }
    
    fn unit(&self) -> String
    {   
        match self {
            AreaUnit::SquareKilometer => String::from("km²"),
            AreaUnit::SquareMeter => String::from("m²"),
            AreaUnit::SquareCentimeter => String::from("cm²"),
            AreaUnit::SquareMillimeter => String::from("mm²")
            }
    }
}

impl ValueTrait<AreaUnit> for Value<AreaUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> AreaUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: AreaUnit) -> Value<AreaUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}

#[cfg(test)]
mod test {
    use super::{AreaUnit};
    use crate::value::{Value,ValueTrait};
    use crate::length::LengthUnit;

    #[test]
    fn test_factories_() {
        let km = Value { unit: AreaUnit::SquareKilometer, value: 3.0 };
        assert_eq!(3.0, km.value());
        assert_eq!(3000000.0, km.base_value());

        let m = Value { unit: AreaUnit::SquareMeter, value: 2.0 };
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());

        let cm = Value { unit: AreaUnit::SquareCentimeter, value: 50.0 };
        assert_eq!(50.0, cm.value());
        assert_eq!(0.005, cm.base_value());

        let mm = Value { unit: AreaUnit::SquareMillimeter, value: 20.0 };
        assert_eq!(20.0, mm.value());
        assert_eq!(0.00002, mm.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Value { unit: AreaUnit::SquareMeter, value: 2.0 };
        let cm = Value { unit: AreaUnit::SquareCentimeter, value: 50.0 };

        let a = m + cm;
        assert_eq!(2.005, a.value());
        let b = m - cm;
        assert_eq!(1.995, b.value());
        m += cm;
        assert_eq!(2.005, m.value());
        m -= cm;
        assert_eq!(2.0, m.value());
    }

    #[test]
    fn test_strings() {
        let km = Value { unit: AreaUnit::SquareKilometer, value: 3.0 };
        let m = Value { unit: AreaUnit::SquareMeter, value: 2.5 };
        let cm = Value { unit: AreaUnit::SquareCentimeter, value: 50.0 };
        let mm = Value { unit: AreaUnit::SquareMillimeter, value: 20.2 };
        assert_eq!("3km²", km.to_string());
        assert_eq!("2.5m²", m.to_string());
        assert_eq!("50cm²", cm.to_string());
        assert_eq!("20.2mm²", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value { unit: AreaUnit::SquareKilometer, value: 1.5 };
        let m = km.convert(AreaUnit::SquareMeter);
        assert_eq!(1500000.0, m.value());
    }

    #[test]
    fn test_derived_units()
    {
        let m = Value { unit: LengthUnit::Meter, value: 2.0 };
        let cm = Value { unit: LengthUnit::Centimeter, value: 300.0 };
        let area = m * cm;
        assert_eq!(6.0, area.value());
        let x = area / m;
        assert_eq!(cm.base_value(), x.value());
    }
}
