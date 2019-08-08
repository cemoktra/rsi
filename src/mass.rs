use crate::value::{Unit};

// ========================================
// Mass module
// ========================================
#[derive(Copy, Clone)]
pub enum MassUnit {
    Kilogram,
    Gram,
}

impl Unit for MassUnit {
    type UnitEnum = MassUnit;

    fn ratio(&self) -> f64
    {
        match self {
            MassUnit::Kilogram => 1.0,
            MassUnit::Gram => 0.001,
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            MassUnit::Kilogram => String::from("kg"),
            MassUnit::Gram => String::from("g"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::MassUnit;
    use crate::value::Value;

    #[test]
    fn test_factories_() {
        let kg = Value::new(MassUnit::Kilogram, 3.0);
        let g  = Value::new(MassUnit::Gram, 200.0);
        assert_eq!(3.0, kg.value());
        assert_eq!(3.0, kg.base_value());
        assert_eq!(200.0, g.value());
        assert_eq!(0.2, g.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut kg = Value::new(MassUnit::Kilogram, 2.0);
        let g      = Value::new(MassUnit::Gram, 500.0);

        let a = kg + g;
        assert_eq!(2.5, a.value());
        let b = kg - g;
        assert_eq!(1.5, b.value());
        kg += g;
        assert_eq!(2.5, kg.value());
        kg -= g;
        assert_eq!(2.0, kg.value());
    }

    #[test]
    fn test_strings() {
        let kg = Value::new(MassUnit::Kilogram, 3.0);
        let g  = Value::new(MassUnit::Gram, 2.5);
        assert_eq!("3kg", kg.to_string());
        assert_eq!("2.5g", g.to_string());
    }

    #[test]
    fn test_conversion() {
        let kg = Value::new(MassUnit::Kilogram, 1.5);
        let g = kg.convert(MassUnit::Gram);
        assert_eq!(1500.0, g.value());
    }
}
