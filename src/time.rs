use crate::value::{Unit};

// ========================================
// Time module
// ========================================
#[derive(Copy, Clone)]
pub enum TimeUnit {
    Days,
    Hours,
    Minutes,
    Seconds,
    Milliseconds
}

impl Unit for TimeUnit {
    type UnitEnum = TimeUnit;

    fn ratio(&self) -> f64
    {
        match self {
            TimeUnit::Milliseconds => 0.001,
            TimeUnit::Seconds => 1.0,
            TimeUnit::Minutes => 60.0,
            TimeUnit::Hours => 60.0 * 60.0,
            TimeUnit::Days => 24.0 * 60.0 * 60.0
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            TimeUnit::Milliseconds => String::from("ms"),
            TimeUnit::Seconds => String::from("s"),
            TimeUnit::Minutes => String::from("min"),
            TimeUnit::Hours => String::from("h"),
            TimeUnit::Days => String::from("d")        
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TimeUnit};
    use crate::value::{Value};

    #[test]
    fn test_factories_() {
        let d   = Value::new(TimeUnit::Days, 1.0);
        let h   = Value::new(TimeUnit::Hours, 2.0);
        let min = Value::new(TimeUnit::Minutes, 5.0);
        let s   = Value::new(TimeUnit::Seconds, 20.0);
        let ms  = Value::new(TimeUnit::Milliseconds, 20.0);

        assert_eq!(1.0, d.value());
        assert_eq!(24.0 * 60.0 * 60.0, d.base_value());
        assert_eq!(2.0, h.value());
        assert_eq!(2.0 * 60.0 * 60.0, h.base_value());
        assert_eq!(5.0, min.value());
        assert_eq!(300.0, min.base_value());
        assert_eq!(20.0, s.value());
        assert_eq!(20.0, s.base_value());
        assert_eq!(20.0, ms.value());
        assert_eq!(0.02, ms.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut s = Value::new(TimeUnit::Seconds, 30.0);
        let min   = Value::new(TimeUnit::Minutes, 2.0);

        let a = min + s;
        assert_eq!(2.5, a.value());
        let b = min - s;
        assert_eq!(1.5, b.value());
        s += min;
        assert_eq!(150.0, s.value());
        s -= min;
        assert_eq!(30.0, s.value());
    }

    #[test]
    fn test_strings() {
        let d   = Value::new(TimeUnit::Days, 3.0);
        let h   = Value::new(TimeUnit::Hours, 2.0);
        let min = Value::new(TimeUnit::Minutes, 50.0);
        let s   = Value::new(TimeUnit::Seconds, 2.5);
        let ms  = Value::new(TimeUnit::Milliseconds, 20.2);
        assert_eq!("3d", d.to_string());
        assert_eq!("2h", h.to_string());
        assert_eq!("50min", min.to_string());
        assert_eq!("2.5s", s.to_string());
        assert_eq!("20.2ms", ms.to_string());
    }

    #[test]
    fn test_conversion() {
        let h = Value::new(TimeUnit::Hours, 1.5);
        let min = h.convert(TimeUnit::Minutes);
        assert_eq!(90.0, min.value());
    }
}
