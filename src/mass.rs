//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign};
use crate::value::{Value,ValueTrait};

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Value<MassUnit> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), self.unit_string())
    }
}

// ========================================
// calculations
// ========================================
impl Add for Value<MassUnit> {
    type Output = Value<MassUnit>;
    fn add(self, other: Value<MassUnit>) -> Value<MassUnit> {
        Value { unit: MassUnit::Kilogram, value: self.base_value() + other.base_value() }
    }
}

impl AddAssign for Value<MassUnit> {
    fn add_assign(&mut self, other: Self) {
        *self = Value { unit: MassUnit::Kilogram, value: self.base_value() + other.base_value() };
    }
}

impl Sub for Value<MassUnit> {
    type Output = Value<MassUnit>;
    fn sub(self, other: Value<MassUnit>) -> Value<MassUnit> {
        Value { unit: MassUnit::Kilogram, value: self.base_value() - other.base_value() }
    }
}

impl SubAssign for Value<MassUnit> {
    fn sub_assign(&mut self, other: Self) {
        *self = Value { unit: MassUnit::Kilogram, value: self.base_value() - other.base_value() };
    }
}

// ========================================
// Mass module
// ========================================
#[derive(Copy, Clone)]
pub enum MassUnit {
    Kilogram,
    Gram,
}

impl MassUnit {
    fn ratio(&self) -> num_rational::Ratio<i32>
    {
        match self {
            MassUnit::Kilogram => num_rational::Ratio::new(1, 1),
            MassUnit::Gram => num_rational::Ratio::new(1, 1000),
        }
    }
    
    fn unit(&self) -> String
    {   
        match self {
            MassUnit::Kilogram => String::from("kg"),
            MassUnit::Gram => String::from("g"),
        }
    }
}

impl ValueTrait<MassUnit> for Value<MassUnit> {
    fn value(&self) -> f64
    {
        self.value
    }
    fn base_value(&self) -> f64
    {
        self.value * (*self.unit().ratio().numer() as f64) / (*self.unit().ratio().denom() as f64)
    }
    fn unit(&self) -> MassUnit
    {
        self.unit
    }    
    fn unit_string(&self) -> String
    {
        self.unit.unit()
    }
    fn convert(&self, unit: MassUnit) -> Value<MassUnit>
    {
        Value { unit: unit, value: self.value() * (*self.unit().ratio().numer() as f64) * (*unit.ratio().denom() as f64) / ((*self.unit().ratio().denom() as f64) * (*unit.ratio().numer() as f64)) }
    }
}


#[cfg(test)]
mod test {
    use super::{MassUnit};
    use crate::value::{Value,ValueTrait};

    #[test]
    fn test_factories_() {
        let kg = Value { unit: MassUnit::Kilogram, value: 3.0 };
        assert_eq!(3.0, kg.value());
        assert_eq!(3.0, kg.base_value());

        let g = Value { unit: MassUnit::Gram, value: 200.0 };
        assert_eq!(200.0, g.value());
        assert_eq!(0.2, g.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut kg = Value { unit: MassUnit::Kilogram, value: 2.0 };
        let g = Value { unit: MassUnit::Gram, value: 500.0 };

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
        let kg = Value { unit: MassUnit::Kilogram, value: 3.0 };
        let g = Value { unit: MassUnit::Gram, value: 2.5 };
        assert_eq!("3kg", kg.to_string());
        assert_eq!("2.5g", g.to_string());
    }

    #[test]
    fn test_conversion() {
        let kg = Value { unit: MassUnit::Kilogram, value: 1.5 };
        let g = kg.convert(MassUnit::Gram);
        assert_eq!(1500.0, g.value());
    }
}
