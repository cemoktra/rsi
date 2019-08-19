#[cfg(test)]
mod test {
    use crate::value::Value;
    use crate::length::LengthUnit;
    use crate::area::AreaUnit;
    use crate::mass::MassUnit;
    use crate::time::TimeUnit;
    use crate::velocity::VelocityUnit;
    use crate::volume::VolumeUnit;

    #[test]
    fn test_add_sub() {
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
        let km  = Value::new(LengthUnit::Kilometer, 3.0);
        let m   = Value::new(LengthUnit::Meter, 2.5);
        let cm  = Value::new(LengthUnit::Centimeter, 50.0);
        let mm  = Value::new(LengthUnit::Millimeter, 20.2);
        let km2 = Value::new(AreaUnit::SquareKilometer, 3.0);
        let m2  = Value::new(AreaUnit::SquareMeter, 2.5);
        let cm2 = Value::new(AreaUnit::SquareCentimeter, 50.0);
        let mm2 = Value::new(AreaUnit::SquareMillimeter, 20.2);
        let kg  = Value::new(MassUnit::Kilogram, 3.0);
        let g   = Value::new(MassUnit::Gram, 2.5);
        let d   = Value::new(TimeUnit::Days, 3.0);
        let h   = Value::new(TimeUnit::Hours, 2.0);
        let min = Value::new(TimeUnit::Minutes, 50.0);
        let s   = Value::new(TimeUnit::Seconds, 2.5);
        let ms  = Value::new(TimeUnit::Milliseconds, 20.2);
        let kmh = Value::new(VelocityUnit::KilometerPerHour, 3.0);
        let mps = Value::new(VelocityUnit::MeterPerSecond, 2.5);        
        let km3 = Value::new(VolumeUnit::CubicKilometer, 3.0);
        let m3  = Value::new(VolumeUnit::CubicMeter, 2.5);
        let cm3 = Value::new(VolumeUnit::CubicCentimeter, 50.0);
        let mm3 = Value::new(VolumeUnit::CubicMillimeter, 20.2);
        assert_eq!("3km", km.to_string());
        assert_eq!("2.5m", m.to_string());
        assert_eq!("50cm", cm.to_string());
        assert_eq!("20.2mm", mm.to_string());
        assert_eq!("3km²", km2.to_string());
        assert_eq!("2.5m²", m2.to_string());
        assert_eq!("50cm²", cm2.to_string());
        assert_eq!("20.2mm²", mm2.to_string());
        assert_eq!("3kg", kg.to_string());
        assert_eq!("2.5g", g.to_string());
        assert_eq!("3d", d.to_string());
        assert_eq!("2h", h.to_string());
        assert_eq!("50min", min.to_string());
        assert_eq!("2.5s", s.to_string());
        assert_eq!("20.2ms", ms.to_string());
        assert_eq!("3km/h", kmh.to_string());
        assert_eq!("2.5m/s", mps.to_string());
        assert_eq!("3km³", km3.to_string());
        assert_eq!("2.5m³", m3.to_string());
        assert_eq!("50cm³", cm3.to_string());
        assert_eq!("20.2mm³", mm3.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Value::new(LengthUnit::Kilometer, 1.5);
        let m = km.convert(LengthUnit::Meter);
        assert_eq!(1500.0, m.value());
    }
}
