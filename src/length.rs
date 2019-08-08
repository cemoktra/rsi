use std::ops::{Mul,Div};
use crate::value::{Value,Unit};
use crate::area::AreaUnit;
use crate::volume::VolumeUnit;
use crate::time::TimeUnit;
use crate::velocity::VelocityUnit;

// ========================================
// calculations
// ========================================
impl Mul for Value<LengthUnit> {
    type Output = Value<AreaUnit>;
    fn mul(self, rhs: Value<LengthUnit>) -> Value<AreaUnit> {
        Value::new(AreaUnit::SquareMeter, self.base_value() * rhs.base_value())
    }
}

impl Mul<Value<AreaUnit>> for Value<LengthUnit> {
    type Output = Value<VolumeUnit>;
    fn mul(self, rhs: Value<AreaUnit>) -> Value<VolumeUnit> {
        Value::new(VolumeUnit::CubicMeter, self.base_value() * rhs.base_value())
    }
}

impl Div<Value<TimeUnit>> for Value<LengthUnit> {
    type Output = Value<VelocityUnit>;

    fn div(self, rhs: Value<TimeUnit>) -> Value<VelocityUnit> {
        Value::new(VelocityUnit::MeterPerSecond, self.base_value() / rhs.base_value())
    }
}

impl Div<Value<VelocityUnit>> for Value<LengthUnit> {
    type Output = Value<TimeUnit>;

    fn div(self, rhs: Value<VelocityUnit>) -> Value<TimeUnit> {
        Value::new(TimeUnit::Seconds, self.base_value() / rhs.base_value())
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

impl Unit for LengthUnit {
    type UnitEnum = LengthUnit;

    fn ratio(&self) -> f64
    {
        match self {
            LengthUnit::Kilometer => 1000.0,
            LengthUnit::Meter => 1.0,
            LengthUnit::Centimeter => 0.01,
            LengthUnit::Millimeter => 0.001,
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            LengthUnit::Kilometer => String::from("km"),
            LengthUnit::Meter => String::from("m"),
            LengthUnit::Centimeter => String::from("cm"),
            LengthUnit::Millimeter => String::from("mm")
        }
    }
}

#[cfg(test)]
mod test {
    use super::{LengthUnit};
    use crate::value::{Value};

    #[test]
    fn test_factories_() {
        let km = Value::new(LengthUnit::Kilometer, 3.0);
        let m  = Value::new(LengthUnit::Meter, 2.0);
        let cm = Value::new(LengthUnit::Centimeter, 50.0);
        let mm = Value::new(LengthUnit::Millimeter, 20.0);

        assert_eq!(3.0, km.value());
        assert_eq!(3000.0, km.base_value());
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());
        assert_eq!(50.0, cm.value());
        assert_eq!(0.5, cm.base_value());
        assert_eq!(20.0, mm.value());
        assert_eq!(0.02, mm.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m  = Value::new(LengthUnit::Meter, 2.0);
        let cm     = Value::new(LengthUnit::Centimeter, 50.0);

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
        let km = Value::new(LengthUnit::Kilometer, 3.0);
        let m  = Value::new(LengthUnit::Meter, 2.5);
        let cm = Value::new(LengthUnit::Centimeter, 50.0);
        let mm = Value::new(LengthUnit::Millimeter, 20.2);
        assert_eq!("3km", km.to_string());
        assert_eq!("2.5m", m.to_string());
        assert_eq!("50cm", cm.to_string());
        assert_eq!("20.2mm", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value::new(LengthUnit::Kilometer, 1.5);
        let m = km.convert(LengthUnit::Meter);
        let cm = km.convert(LengthUnit::Centimeter);
        assert_eq!(1500.0, m.value());
        assert_eq!(150000.0, cm.value());
    }
}