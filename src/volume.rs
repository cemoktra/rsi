use crate::value::Unit;

// ========================================
// Volume module
// ========================================
#[derive(Copy, Clone)]
pub enum VolumeUnit {
    CubicMillimeter,
    CubicCentimeter,
    CubicMeter,
    CubicKilometer
}

impl Unit for VolumeUnit {
    type UnitEnum = VolumeUnit;

    fn ratio(&self) -> f64
    {
        match self {
            VolumeUnit::CubicKilometer => 1000000000.0,
            VolumeUnit::CubicMeter => 1.0,
            VolumeUnit::CubicCentimeter => 0.000001,
            VolumeUnit::CubicMillimeter => 0.000000001
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            VolumeUnit::CubicKilometer => String::from("km³"),
            VolumeUnit::CubicMeter => String::from("m³"),
            VolumeUnit::CubicCentimeter => String::from("cm³"),
            VolumeUnit::CubicMillimeter => String::from("mm³")
        }
    }
}

#[cfg(test)]
mod test {
    use super::VolumeUnit;
    use crate::value::Value;

    #[test]
    fn test_factories_() {
        let km = Value::new(VolumeUnit::CubicKilometer, 3.0);
        let m  = Value::new(VolumeUnit::CubicMeter, 2.0);
        let cm = Value::new(VolumeUnit::CubicCentimeter, 50.0);
        let mm = Value::new(VolumeUnit::CubicMillimeter, 20.0);

        assert_eq!(3.0, km.value());
        assert_eq!(3000000000.0, km.base_value());
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());
        assert_eq!(50.0, cm.value());        
        assert!((0.00005 - cm.base_value()).abs() < std::f64::EPSILON);
        assert_eq!(20.0, mm.value());
        assert_eq!(0.00000002, mm.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Value::new(VolumeUnit::CubicMeter, 2.0);
        let cm    = Value::new(VolumeUnit::CubicCentimeter, 50.0);

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
        let km = Value::new(VolumeUnit::CubicKilometer, 3.0);
        let m  = Value::new(VolumeUnit::CubicMeter, 2.5);
        let cm = Value::new(VolumeUnit::CubicCentimeter, 50.0);
        let mm = Value::new(VolumeUnit::CubicMillimeter, 20.2);
        assert_eq!("3km³", km.to_string());
        assert_eq!("2.5m³", m.to_string());
        assert_eq!("50cm³", cm.to_string());
        assert_eq!("20.2mm³", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value::new(VolumeUnit::CubicKilometer, 1.5);
        let m = km.convert(VolumeUnit::CubicMeter);
        assert_eq!(1500000000.0, m.value());
    }
}
