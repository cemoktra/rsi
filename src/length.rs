use crate::value::Unit;

// ========================================
// Length module
// ========================================
#[derive(Copy, Clone)]
pub enum LengthUnit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer
}

impl Unit for LengthUnit {
    type UnitEnum = LengthUnit;

    fn ratio(&self) -> f64
    {
        match self {
            LengthUnit::Kilometer => 1000.0,
            LengthUnit::Meter => 1.0,
            LengthUnit::Centimeter => 0.01,
            LengthUnit::Millimeter => 0.001,
        }
    }
    
    fn abbr(&self) -> String
    {   
        match self {
            LengthUnit::Kilometer => String::from("km"),
            LengthUnit::Meter => String::from("m"),
            LengthUnit::Centimeter => String::from("cm"),
            LengthUnit::Millimeter => String::from("mm")
        }
    }
}