//extern crate num_rational;
use std::fmt;
use std::ops::{Add,AddAssign,Sub,SubAssign,Div};
use crate::length::Length;

// ========================================
// Display trait
// ========================================
impl std::fmt::Display for Area {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "{}{}", self.value(), unit_string(self.unit()))
   }
}

// ========================================
// calculations
// ========================================
impl Add for Area {
    type Output = Area;
    fn add(self, other: Area) -> Area {
        Area::square_meter(self.base_value() + other.base_value())
    }
}

impl AddAssign for Area {
    fn add_assign(&mut self, other: Self) {
        *self = Area::square_meter(self.base_value() + other.base_value());
    }
}

impl Sub for Area {
    type Output = Area;
    fn sub(self, other: Area) -> Area {
        Area::square_meter(self.base_value() - other.base_value())
    }
}

impl SubAssign for Area {
    fn sub_assign(&mut self, other: Self) {
        *self = Area::square_meter(self.base_value() - other.base_value());
    }
}

impl Div<Length> for Area {
    // The division of rational numbers is a closed operation.
    type Output = Length;

    fn div(self, rhs: Length) -> Length {
        Length::meter(self.base_value() / rhs.base_value())
    }
}

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

pub fn area_ratio(unit: AreaUnit) -> num_rational::Ratio<i32>
{
    match unit {
        AreaUnit::SquareKilometer => num_rational::Ratio::new(1000000, 1),
        AreaUnit::SquareMeter => num_rational::Ratio::new(1, 1),
        AreaUnit::SquareCentimeter => num_rational::Ratio::new(1, 10000),
        AreaUnit::SquareMillimeter => num_rational::Ratio::new(1, 1000000)
    }
}

pub fn unit_string(unit: AreaUnit) -> String
{
    match unit {
        AreaUnit::SquareKilometer => String::from("km²"),
        AreaUnit::SquareMeter => String::from("m²"),
        AreaUnit::SquareCentimeter => String::from("cm²"),
        AreaUnit::SquareMillimeter => String::from("mm²")
    }
}

#[derive(Copy, Clone)]
pub struct Area
{
    value: f64,
    unit: AreaUnit,
    ratio: num_rational::Ratio<i32>,
}

impl Area {
    pub fn new(value: f64, unit: AreaUnit) -> Area
    {
        Area { value: value, ratio: area_ratio(unit), unit: unit }
    }

    pub fn square_millimeter(value: f64) -> Area
    {
        Area { value: value, ratio: area_ratio(AreaUnit::SquareMillimeter), unit: AreaUnit::SquareMillimeter }
    }
    pub fn square_centimeter(value: f64) -> Area
    {
        Area { value: value, ratio: area_ratio(AreaUnit::SquareCentimeter), unit: AreaUnit::SquareCentimeter }
    }
    pub fn square_meter(value: f64) -> Area
    {
        Area { value: value, ratio: area_ratio(AreaUnit::SquareMeter), unit: AreaUnit::SquareMeter }
    }
    pub fn square_kilometer(value: f64) -> Area
    {
        Area { value: value, ratio: area_ratio(AreaUnit::SquareKilometer), unit: AreaUnit::SquareKilometer }
    }

    pub fn value(&self) -> f64
    {
        self.value
    }

    pub fn base_value(&self) -> f64
    {
        (*self.ratio.numer() as f64) * self.value / (*self.ratio.denom() as f64)
    }

    pub fn unit(&self) -> AreaUnit
    {
        self.unit
    }

    pub fn convert(&self, unit: AreaUnit) -> Area
    {
        let ratio = area_ratio(unit);
        Area { value: self.base_value() * (*ratio.numer() as f64) / (*ratio.denom() as f64), ratio: ratio, unit: unit }
    }
}

#[cfg(test)]
mod test {
    use super::{Area, AreaUnit};
    use crate::length::Length;

    #[test]
    fn test_factories_() {
        let km = Area::square_kilometer(3.0);
        assert_eq!(3.0, km.value());
        assert_eq!(3000000.0, km.base_value());

        let m = Area::square_meter(2.0);
        assert_eq!(2.0, m.value());
        assert_eq!(2.0, m.base_value());

        let cm = Area::square_centimeter(50.0);
        assert_eq!(50.0, cm.value());
        assert_eq!(0.005, cm.base_value());

        let mm = Area::square_millimeter(20.0);
        assert_eq!(20.0, mm.value());
        assert_eq!(0.00002, mm.base_value());

        let n = Area::new(2.0, AreaUnit::SquareMeter);
        assert_eq!(2.0, n.value());
        assert_eq!(2.0, n.base_value());
    }

    #[test]
    fn test_calculations() {
        let mut m = Area::square_meter(2.0);
        let cm = Area::square_centimeter(50.0);

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
        let km = Area::square_kilometer(3.0);
        assert_eq!("3km²", km.to_string());

        let m = Area::square_meter(2.5);
        assert_eq!("2.5m²", m.to_string());

        let cm = Area::square_centimeter(50.0);
        assert_eq!("50cm²", cm.to_string());

        let mm = Area::square_millimeter(20.2);
        assert_eq!("20.2mm²", mm.to_string());
    }

    #[test]
    fn test_conversion() {
        let km = Area::square_kilometer(1.5);
        let m = km.convert(AreaUnit::SquareMeter);
        assert_eq!(1500000.0, m.value());

        let m = Length::meter(2.0);
        let cm = Length::centimeter(300.0);
        let area = m * cm;
        assert_eq!(6.0, area.value());
        let x = area / m;
        assert_eq!(cm.base_value(), x.value());
    }
}
