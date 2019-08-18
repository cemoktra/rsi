use crate::value::Unit;

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

impl Unit for AreaUnit {
    type UnitEnum = AreaUnit;

    fn ratio(&self) -> f64
    {
        match self {
            AreaUnit::SquareKilometer => 1000000.0,
            AreaUnit::SquareMeter => 1.0,
            AreaUnit::SquareCentimeter => 0.0001,
            AreaUnit::SquareMillimeter => 0.000001
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            AreaUnit::SquareKilometer => String::from("km²"),
            AreaUnit::SquareMeter => String::from("m²"),
            AreaUnit::SquareCentimeter => String::from("cm²"),
            AreaUnit::SquareMillimeter => String::from("mm²")
            }
    }
}

#[cfg(test)]
mod test {
    use super::AreaUnit;
    use crate::value::Value;

    #[test]
    fn test_factories_() {
        let km = Value::new(AreaUnit::SquareKilometer, 3.0);
        let m  = Value::new(AreaUnit::SquareMeter, 2.0);
        let cm = Value::new(AreaUnit::SquareCentimeter, 50.0);
        let mm = Value::new(AreaUnit::SquareMillimeter, 20.0);
        assert_eq!(3.0, km.value());
        assert_eq!(3000000.0, km.base_value());
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());
        assert_eq!(50.0, cm.value());
        assert_eq!(0.005, cm.base_value());
        assert_eq!(20.0, mm.value());
        assert!((0.00002 - mm.base_value()).abs() < std::f64::EPSILON);
    }

    #[test]
    fn test_calculations() {
        let mut m = Value::new(AreaUnit::SquareMeter, 2.0);
        let cm    = Value::new(AreaUnit::SquareCentimeter, 50.0);

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
        let km = Value::new(AreaUnit::SquareKilometer, 3.0);
        let m  = Value::new(AreaUnit::SquareMeter, 2.5);
        let cm = Value::new(AreaUnit::SquareCentimeter, 50.0);
        let mm = Value::new(AreaUnit::SquareMillimeter, 20.2);
        assert_eq!("3km²", km.to_string());
        assert_eq!("2.5m²", m.to_string());
        assert_eq!("50cm²", cm.to_string());
        assert_eq!("20.2mm²", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value::new(AreaUnit::SquareKilometer, 1.5);
        let m = km.convert(AreaUnit::SquareMeter);
        assert_eq!(1500000.0, m.value());
    }
}
