extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign};


// ========================================
// Display trait
// ========================================
impl std::fmt::Display for si_length::Length {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.value(), si_length::unit_string(self.unit()))
    }
}

// ========================================
// calculations
// ========================================
impl Add for si_length::Length {
    type Output = si_length::Length;
    fn add(self, other: si_length::Length) -> si_length::Length {
        si_length::Length::meter(self.base_value() + other.base_value())
    }
}

impl AddAssign for si_length::Length {
    fn add_assign(&mut self, other: Self) {
        *self = si_length::Length::meter(self.base_value() + other.base_value());
    }
}

impl Sub for si_length::Length {
    type Output = si_length::Length;
    fn sub(self, other: si_length::Length) -> si_length::Length {
        si_length::Length::meter(self.base_value() - other.base_value())
    }
}

impl SubAssign for si_length::Length {
    fn sub_assign(&mut self, other: Self) {
        *self = si_length::Length::meter(self.base_value() - other.base_value());
    }
}


// ========================================
// Length module
// ========================================
mod si_length {
    #[derive(Copy, Clone)]
    pub enum LengthUnit {
        Millimeter,
        Centimeter,
        Meter,
        Kilometer
    }

    pub fn length_ratio(unit: LengthUnit) -> num_rational::Ratio<i32>
    {
        match unit {
            LengthUnit::Kilometer => num_rational::Ratio::new(1000, 1),
            LengthUnit::Meter => num_rational::Ratio::new(1, 1),
            LengthUnit::Centimeter => num_rational::Ratio::new(1, 100),
            LengthUnit::Millimeter => num_rational::Ratio::new(1, 1000)
        }
    }

    pub fn unit_string(unit: LengthUnit) -> String
    {
        match unit {
            LengthUnit::Kilometer => String::from("km"),
            LengthUnit::Meter => String::from("m"),
            LengthUnit::Centimeter => String::from("cm"),
            LengthUnit::Millimeter => String::from("mm")
        }
    }

    #[derive(Copy, Clone)]
    pub struct Length
    {
        value: f64,
        unit: LengthUnit,
        ratio: num_rational::Ratio<i32>,
    }

    impl Length {
        pub fn new(value: f64, unit: LengthUnit) -> Length
        {
            Length { value: value, ratio: length_ratio(unit), unit: unit }
        }

        pub fn millimeter(value: f64) -> Length
        {
            Length { value: value, ratio: length_ratio(LengthUnit::Millimeter), unit: LengthUnit::Millimeter }
        }
        pub fn centimeter(value: f64) -> Length
        {
            Length { value: value, ratio: length_ratio(LengthUnit::Centimeter), unit: LengthUnit::Centimeter }
        }
        pub fn meter(value: f64) -> Length
        {
            Length { value: value, ratio: length_ratio(LengthUnit::Meter), unit: LengthUnit::Meter }
        }
        pub fn kilometer(value: f64) -> Length
        {
            Length { value: value, ratio: length_ratio(LengthUnit::Kilometer), unit: LengthUnit::Kilometer }
        }

        pub fn value(&self) -> f64
        {
            self.value
        }

        pub fn base_value(&self) -> f64
        {
            (*self.ratio.numer() as f64) * self.value / (*self.ratio.denom() as f64)
        }

        pub fn unit(&self) -> LengthUnit
        {
            self.unit
        }

        pub fn convert(&self, unit: LengthUnit) -> Length
        {
            let ratio = length_ratio(unit);
            Length { value: self.base_value() * (*ratio.numer() as f64) / (*ratio.denom() as f64), ratio: ratio, unit: unit }
        }
    }
}


#[cfg(test)]
mod test {
    use super::si_length::{Length, LengthUnit};

    #[test]
    fn test_factories_() {
        let km = Length::kilometer(3.0);
        assert_eq!(3.0, km.value());
        assert_eq!(3000.0, km.base_value());

        let m = Length::meter(2.0);
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());

        let cm = Length::centimeter(50.0);
        assert_eq!(50.0, cm.value());
        assert_eq!(0.5, cm.base_value());

        let mm = Length::millimeter(20.0);
        assert_eq!(20.0, mm.value());
        assert_eq!(0.02, mm.base_value());

        let n = Length::new(2.0, LengthUnit::Meter);
        assert_eq!(2.0, n.value());
        assert_eq!(2.0, n.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Length::meter(2.0);
        let cm = Length::centimeter(50.0);

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
        let km = Length::kilometer(3.0);
        assert_eq!("3km", km.to_string());

        let m = Length::meter(2.5);
        assert_eq!("2.5m", m.to_string());

        let cm = Length::centimeter(50.0);
        assert_eq!("50cm", cm.to_string());

        let mm = Length::millimeter(20.2);
        assert_eq!("20.2mm", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Length::kilometer(1.5);
        let m = km.convert(LengthUnit::Meter);
        assert_eq!(1500.0, m.value());
    }
}
