//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Div};
use crate::value::{Value,ValueTrait};
use crate::length::LengthUnit;
use crate::area::AreaUnit;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<VolumeUnit> {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.value(), self.unit_string())
   }
}

// ========================================
// calculations
// ========================================
impl Add for Value<VolumeUnit> {
    type Output = Value<VolumeUnit>;
    fn add(self, other: Value<VolumeUnit>) -> Value<VolumeUnit> {
        Value { unit: VolumeUnit::CubicMeter, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<VolumeUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: VolumeUnit::CubicMeter, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<VolumeUnit> {
    type Output = Value<VolumeUnit>;
    fn sub(self, other: Value<VolumeUnit>) -> Value<VolumeUnit> {
        Value { unit: VolumeUnit::CubicMeter, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<VolumeUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: VolumeUnit::CubicMeter, value: self.base_value() - other.base_value() };
    }
}


impl Div<Value<LengthUnit>> for Value<VolumeUnit> {
    type Output = Value<AreaUnit>;

    fn div(self, rhs: Value<LengthUnit>) -> Value<AreaUnit> {
        Value { unit: AreaUnit::SquareMeter, value: self.base_value() / rhs.base_value() }
    }
}

impl Div<Value<AreaUnit>> for Value<VolumeUnit> {
    type Output = Value<LengthUnit>;

    fn div(self, rhs: Value<AreaUnit>) -> Value<LengthUnit> {
        Value { unit: LengthUnit::Meter, value: self.base_value() / rhs.base_value() }
    }
}

// ========================================
// Area module
// ========================================
#[derive(Copy, Clone)]
pub enum VolumeUnit {
    CubicMillimeter,
    CubicCentimeter,
    CubicMeter,
    CubicKilometer
}

impl VolumeUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            VolumeUnit::CubicKilometer => num_rational::Ratio::new(1000000000, 1),
            VolumeUnit::CubicMeter => num_rational::Ratio::new(1, 1),
            VolumeUnit::CubicCentimeter => num_rational::Ratio::new(1, 1000000),
            VolumeUnit::CubicMillimeter => num_rational::Ratio::new(1, 1000000000)
            }
    }
    
    fn unit(&self) -> String
    {   
        match self {
            VolumeUnit::CubicKilometer => String::from("km³"),
            VolumeUnit::CubicMeter => String::from("m³"),
            VolumeUnit::CubicCentimeter => String::from("cm³"),
            VolumeUnit::CubicMillimeter => String::from("mm³")
            }
    }
}

impl ValueTrait<VolumeUnit> for Value<VolumeUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> VolumeUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: VolumeUnit) -> Value<VolumeUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}

#[cfg(test)]
mod test {
    use super::{VolumeUnit};
    use crate::value::{Value,ValueTrait};
    use crate::length::LengthUnit;
    use crate::area::AreaUnit;

    #[test]
    fn test_factories_() {
        let km = Value { unit: VolumeUnit::CubicKilometer, value: 3.0 };
        assert_eq!(3.0, km.value());
        assert_eq!(3000000000.0, km.base_value());

        let m = Value { unit: VolumeUnit::CubicMeter, value: 2.0 };
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());

        let cm = Value { unit: VolumeUnit::CubicCentimeter, value: 50.0 };
        assert_eq!(50.0, cm.value());
        assert_eq!(0.00005, cm.base_value());

        let mm = Value { unit: VolumeUnit::CubicMillimeter, value: 20.0 };
        assert_eq!(20.0, mm.value());
        assert_eq!(0.00000002, mm.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Value { unit: VolumeUnit::CubicMeter, value: 2.0 };
        let cm = Value { unit: VolumeUnit::CubicCentimeter, value: 50.0 };

        let a = m + cm;
        assert_eq!(2.00005, a.value());
        let b = m - cm;
        assert_eq!(1.99995, b.value());
        m += cm;
        assert_eq!(2.00005, m.value());
        m -= cm;
        assert_eq!(2.0, m.value().round());
    }

    #[test]
    fn test_strings() {
        let km = Value { unit: VolumeUnit::CubicKilometer, value: 3.0 };
        let m = Value { unit: VolumeUnit::CubicMeter, value: 2.5 };
        let cm = Value { unit: VolumeUnit::CubicCentimeter, value: 50.0 };
        let mm = Value { unit: VolumeUnit::CubicMillimeter, value: 20.2 };
        assert_eq!("3km³", km.to_string());
        assert_eq!("2.5m³", m.to_string());
        assert_eq!("50cm³", cm.to_string());
        assert_eq!("20.2mm³", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value { unit: VolumeUnit::CubicKilometer, value: 1.5 };
        let m = km.convert(VolumeUnit::CubicMeter);
        assert_eq!(1500000000.0, m.value());
    }

    #[test]
    fn test_derived_units()
    {
        let m2 = Value { unit: AreaUnit::SquareMeter, value: 4.0 };
        let m = Value { unit: LengthUnit::Meter, value: 3.0 };
        let vol = m * m2;
        assert_eq!(12.0, vol.value());
        let x = vol / m;
        assert_eq!(m2.base_value(), x.value());
    }
}
