use std::ops::{Mul,Div};
use crate::value::Value;
use crate::length::LengthUnit;
use crate::volume::VolumeUnit;
use crate::area::AreaUnit;
use crate::time::TimeUnit;
use crate::velocity::VelocityUnit;

// length / area
impl Div<Value<LengthUnit>> for Value<AreaUnit> {
    type Output = Value<LengthUnit>;
    fn div(self, rhs: Value<LengthUnit>) -> Value<LengthUnit> {
        Value::new(LengthUnit::Meter, self.base_value() / rhs.base_value())
    }
}

impl Mul<Value<LengthUnit>> for Value<AreaUnit> {
    type Output = Value<VolumeUnit>;
    fn mul(self, rhs: Value<LengthUnit>) -> Value<VolumeUnit> {
        Value::new(VolumeUnit::CubicMeter, self.base_value() * rhs.base_value())
    }
}

impl Mul for Value<LengthUnit> {
    type Output = Value<AreaUnit>;
    fn mul(self, rhs: Value<LengthUnit>) -> Value<AreaUnit> {
        Value::new(AreaUnit::SquareMeter, self.base_value() * rhs.base_value())
    }
}

// volume / area / length
impl Mul<Value<AreaUnit>> for Value<LengthUnit> {
    type Output = Value<VolumeUnit>;
    fn mul(self, rhs: Value<AreaUnit>) -> Value<VolumeUnit> {
        Value::new(VolumeUnit::CubicMeter, self.base_value() * rhs.base_value())
    }
}

impl Div<Value<LengthUnit>> for Value<VolumeUnit> {
    type Output = Value<AreaUnit>;

    fn div(self, rhs: Value<LengthUnit>) -> Value<AreaUnit> {
        Value::new(AreaUnit::SquareMeter, self.base_value() / rhs.base_value())
    }
}

impl Div<Value<AreaUnit>> for Value<VolumeUnit> {
    type Output = Value<LengthUnit>;

    fn div(self, rhs: Value<AreaUnit>) -> Value<LengthUnit> {
        Value::new(LengthUnit::Meter, self.base_value() / rhs.base_value())
    }
}

// length / time /velocity
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

impl Mul<Value<TimeUnit>> for Value<VelocityUnit> {
    type Output = Value<LengthUnit>;

    fn mul(self, rhs: Value<TimeUnit>) -> Value<LengthUnit> {
        Value::new(LengthUnit::Meter, self.base_value() * rhs.base_value())
    }
}

#[cfg(test)]
mod test {
    use crate::value::Value;
    use crate::length::LengthUnit;
    use crate::time::TimeUnit;
    use crate::area::AreaUnit;

    #[test]
    fn test_length_area()
    {
        let m  = Value::new(LengthUnit::Meter, 2.0);
        let cm = Value::new(LengthUnit::Centimeter, 300.0);
        let area = m * cm;
        assert_eq!(6.0, area.value());
        let x = area / m;
        assert_eq!(cm.base_value(), x.value());
    }

    #[test]
    fn test_velocity()
    {
        let m = Value::new(LengthUnit::Meter, 6.0);
        let s = Value::new(TimeUnit::Seconds, 2.0);

        let v = m / s;
        assert_eq!(3.0, v.value());
        let m2 = v * s;
        assert_eq!(m.value(), m2.value());
        let s2 = m / v;
        assert_eq!(s.value(), s2.value());
    }

    #[test]
    fn test_volume_units()
    {
        let m2 = Value::new(AreaUnit::SquareMeter, 4.0);
        let m  = Value::new(LengthUnit::Meter, 3.0);
        let vol = m * m2;
        assert_eq!(12.0, vol.value());
        let x = vol / m;
        assert_eq!(m2.base_value(), x.value());
    }
}
