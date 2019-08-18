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

#[cfg(test)]
mod test {
    use super::VelocityUnit;
    use crate::value::Value;

    #[test]
    fn test_factories_() {
        let kmh = Value::new(VelocityUnit::KilometerPerHour, 3.6);
        let ms  = Value::new(VelocityUnit::MeterPerSecond, 2.0);
        assert_eq!(3.6, kmh.value());
        assert_eq!(1.0, kmh.base_value());
        assert_eq!(2.0, ms.value());
        assert_eq!(2.0, ms.base_value());
    }

    #[test]
    fn test_calculations() {
        let kmh    = Value::new(VelocityUnit::KilometerPerHour, 3.6);
        let mut ms = Value::new(VelocityUnit::MeterPerSecond, 5.0);
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
        let kmh = Value::new(VelocityUnit::KilometerPerHour, 3.0);
        let ms  = Value::new(VelocityUnit::MeterPerSecond, 2.5);
        assert_eq!("3km/h", kmh.to_string());
        assert_eq!("2.5m/s", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let kmh = Value::new(VelocityUnit::KilometerPerHour, 3.6);
        let ms  = kmh.convert(VelocityUnit::MeterPerSecond);
        assert_eq!(1.0, ms.value());
    }
}
